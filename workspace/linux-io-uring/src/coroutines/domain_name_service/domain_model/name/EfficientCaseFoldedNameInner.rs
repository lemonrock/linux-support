// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Deserialize, Serialize)]
struct EfficientCaseFoldedNameInner
{
	/// There is a maximum of 127 labels.
	///
	/// Since we exclude root, this is a maximum of 126 labels.
	///
	/// These offsets are stored in reverse order, ie the leftmost label is the top-level domain.
	///
	/// For example, for the name `mail.example.com.`, it would be stored as:-
	///
	/// ```bash
	/// [
	/// 	(0, 3) => b"com"
	/// 	(3, 7) => b"example"
	/// 	(10, 4) => b"mail"
	/// ]
	/// ```
	///
	/// Thus the offset for the top-level domain (`com`) is always `0`; all other domains can never have an offset of `0`, and their offsets must always be greater than that of the previous entry.
	///
	/// Lengths are stored excluding the trailing period's length of `1`.
	label_offsets_and_lengths_excluding_root: ArrayVec<(u8, NonZeroU8), EfficientCaseFoldedNameInner::LabelsCount>,
	
	/// A label can not exceed 63 bytes; all labels must have one byte except root.
	///
	/// So, the longest possible label data excluding periods is `63, 63, 63, 61, 0`, which, with labels, is 255 bytes, the longest permitted.
	///
	/// Labels are stored:-
	///
	/// * contiguously with no leading or trailing sentinels or lengths.
	/// * in reverse order, ie the leftmost label is the top-level domain.
	///
	/// So, for the name `mail.example.com.`, it would be stored as `b"comexamplemail"`.
	/// This makes it easy to create a parent name, such as `example.com.` using just a memory copy.
	label_data: [u8; EfficientCaseFoldedNameInner::LabelDataSize],
}

impl EfficientCaseFoldedNameInner
{
	const LabelsCount: usize = 126;
	
	const LabelDataSize: usize = 250;
	
	#[inline(always)]
	fn new(label_offsets_and_lengths_excluding_root: ArrayVec<(u8, NonZeroU8), Self::LabelsCount>, label_data: [u8; Self::LabelDataSize]) -> Rc<Self>
	{
		Rc::new
		(
			Self
			{
				label_offsets_and_lengths_excluding_root,
				
				label_data,
			}
		)
	}
	
	#[inline(always)]
	fn collect_excluding_root(labels_excluding_root_top_level_domain_last: &[EfficientCaseFoldedLabel]) -> Rc<Self>
	{
		debug_assert!(labels_excluding_root_top_level_domain_last.len() <= Self::LabelsCount);
		
		match labels_excluding_root_top_level_domain_last.len()
		{
			0 => Self::root(),
			
			length @ _ =>
			{
				let mut label_offsets_and_lengths_excluding_root = ArrayVec::new();
				let mut label_data: [u8; EfficientCaseFoldedNameInner::LabelDataSize] = unsafe_uninitialized();
				let mut label_offset = 0u8;
				for label in labels_excluding_root_top_level_domain_last.into_iter().rev()
				{
					let label_length = label.not_root_length_checked(label_offset);
					
					unsafe { label_data.as_mut_ptr().add(label_offset as usize).copy_from_nonoverlapping(label.bytes_pointer(), label_length as usize) };
					label_offsets_and_lengths_excluding_root.push((label_offset, new_non_zero_u8(label_length)));
					
					label_offset += label_length
				}
				
				Self::new(label_offsets_and_lengths_excluding_root, label_data)
			}
		}
	}
	
	#[inline(always)]
	fn root() -> Rc<Self>
	{
		lazy_static!
		{
			static ref root: Rc<EfficientCaseFoldedNameInner> = EfficientCaseFoldedNameInner::new(ArrayVec::new(), unsafe_uninitialized());
		}
		
		let this: &'static Rc<Self> = &root;
		this.clone()
	}
	
	#[inline(always)]
	fn is_root(&self) -> bool
	{
		self.label_offsets_and_lengths_excluding_root.is_empty()
	}
	
	#[inline(always)]
	fn number_of_labels_including_root(&self) -> NonZeroU8
	{
		new_non_zero_u8((self.label_offsets_and_lengths_excluding_root.len() as u8) + 1)
	}
	
	#[inline(always)]
	fn label<'label>(&'label self, label_index: u8) -> EfficientCaseFoldedLabel<'label>
	{
		if unlikely!(label_index == 0)
		{
			EfficientCaseFoldedLabel::Root
		}
		else
		{
			let (label_offset, label_length_exluding_trailing_period) = self.label_offset_and_length_excluding_root(new_non_zero_u8(label_index));
			let label_offset = label_offset as usize;
			let label_length_exluding_trailing_period = label_length_exluding_trailing_period.get() as usize;
			EfficientCaseFoldedLabel(&self.label_data[label_offset .. (label_offset + label_length_exluding_trailing_period)])
		}
	}
	
	#[inline(always)]
	fn label_offset_and_length_excluding_root(&self, label_index: NonZeroU8) -> (u8, NonZeroU8)
	{
		self.label_offset_and_length_excluding_root_adjusted_index((label_index.get() - 1) as usize)
	}
	
	#[inline(always)]
	fn label_offset_and_length_excluding_root_adjusted_index(&self, adjusted_index: usize) -> (u8, NonZeroU8)
	{
		self.label_offsets_and_lengths_excluding_root.get_unchecked_safe_value(adjusted_index)
	}
	
	/// Returns the parent and the length of the child label removed.
	///
	/// Requires a memory copy.
	#[inline(always)]
	fn parent(&self) -> Option<(Rc<Self>, NonZeroU8)>
	{
		match self.number_of_labels_including_root().get()
		{
			0 => unreachable_code_const("NonZeroU8"),
			
			1 => None,
			
			2 =>
			{
				let (child_label_offset, child_label_length_excluding_trailing_period) = self.label_offset_and_length_excluding_root_adjusted_index(0);
				debug_assert_eq!(child_label_offset, 0);
				
				Some((Self::root(), child_label_length_excluding_trailing_period))
			},
			
			_ =>
			{
				let label_offsets_and_lengths_excluding_root = self.label_offsets_and_lengths_excluding_root.clone();
				let (child_label_offset, child_label_length_excluding_trailing_period) = label_offsets_and_lengths_excluding_root.pop().unwrap();
				debug_assert_ne!(child_label_offset, 0);
				
				let mut label_data: [u8; Self::LabelDataSize] = unsafe_uninitialized();
				unsafe { label_data.as_mut_ptr().copy_from_nonoverlapping(self.label_data.as_ptr(), child_label_offset) };
				Some
				(
					(
						Self::new
						(
							label_offsets_and_lengths_excluding_root,
							label_data,
						),
						child_label_length_excluding_trailing_period
					)
				)
			}
		}
	}
	
	#[inline(always)]
	fn child_clone(&self, child: EfficientCaseFoldedLabel) -> Rc<Self>
	{
		debug_assert!(!child.is_root());
		
		let mut label_data: [u8; Self::LabelDataSize] = unsafe_uninitialized();
		
		let label_offsets_and_lengths_excluding_root = if unlikely!(self.is_root())
		{
			let mut label_offsets_and_lengths_excluding_root = ArrayVec::new();
			Self::push_child_to_label_offsets_and_lengths_excluding_root(&mut label_offsets_and_lengths_excluding_root, child, child.len());
		
			Self::copy_to_new_label_data(&mut label_data, child);
			
			label_offsets_and_lengths_excluding_root
		}
		else
		{
			let old_label_data_length = self.label_data_length();
			
			let mut label_offsets_and_lengths_excluding_root = self.label_offsets_and_lengths_excluding_root.clone();
			Self::push_child_to_label_offsets_and_lengths_excluding_root(&mut label_offsets_and_lengths_excluding_root, child, old_label_data_length);
			
			Self::copy_child_to_label_data(self.label_data.as_ptr(), &mut label_data, child, old_label_data_length);
			
			label_offsets_and_lengths_excluding_root
		};
		
		Self::new
		(
			label_offsets_and_lengths_excluding_root,
			label_data,
		)
	}
	
	#[inline(always)]
	fn child_mutate(&mut self, child: EfficientCaseFoldedLabel)
	{
		debug_assert!(child.is_root());
		
		if unlikely!(self.is_root())
		{
			Self::push_child_to_label_offsets_and_lengths_excluding_root(&mut self.label_offsets_and_lengths_excluding_root, child, child.len());
			
			Self::copy_to_new_label_data(&mut self.label_data, child);
		}
		else
		{
			let old_label_data_length = self.label_data_length();
			
			Self::push_child_to_label_offsets_and_lengths_excluding_root(&mut self.label_offsets_and_lengths_excluding_root, child, old_label_data_length);
			
			Self::copy_child_to_label_data(self.label_data.as_ptr(), &mut self.label_data, child, old_label_data_length);
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn copy_child_to_label_data(old_label_data_ptr: *const u8, new_label_data: &mut [u8; Self::LabelDataSize], child: EfficientCaseFoldedLabel, old_label_data_length: u8)
	{
		let new_label_data = new_label_data.as_mut_ptr();
		let old_label_data_length = old_label_data_length as usize;
		unsafe
		{
			new_label_data.copy_from_nonoverlapping(old_label_data_ptr, old_label_data_length);
			new_label_data.add(old_label_data_length).copy_from_nonoverlapping(child.bytes_pointer(), child.len() as usize);
		};
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn copy_to_new_label_data(new_label_data: &mut [u8; Self::LabelDataSize], child: EfficientCaseFoldedLabel)
	{
		unsafe { new_label_data.as_mut_ptr().copy_from_nonoverlapping(child.bytes_pointer(), child.len() as usize) };
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn push_child_to_label_offsets_and_lengths_excluding_root(label_offsets_and_lengths_excluding_root: &mut ArrayVec<(u8, NonZeroU8), Self::LabelsCount>, child: EfficientCaseFoldedLabel, old_label_data_length: u8)
	{
		label_offsets_and_lengths_excluding_root.push((old_label_data_length, new_non_zero_u8(child.len())));
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn label_data_length(&self) -> u8
	{
		let (label_offset, label_length) = self.label_offsets_and_lengths_excluding_root.get_unchecked_safe_value(self.label_offsets_and_lengths_excluding_root.len() - 1);
		label_offset + label_length.get()
	}
}
