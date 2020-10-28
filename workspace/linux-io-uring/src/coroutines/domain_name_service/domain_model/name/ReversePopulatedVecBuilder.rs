// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// The purpose of this struct is to build a vec from the end to the start.
///
/// This is actually quite an unsafe thing to do.
struct ReversePopulatedVecBuilder<T>
{
	temporary: Vec<T>,
	
	maximum_capacity: usize,
	
	count: usize,

	copied: bool,
}

impl<T> Drop for ReversePopulatedVecBuilder<T>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if unlikely!(!self.copied)
		{
			let start_pointer = self.start_pointer();
			let start_index = self.start_index();
			for index in start_index .. self.maximum_capacity
			{
				let element_pointer = unsafe { start_pointer.add(index) };
				unsafe { drop_in_place(element_pointer) }
			}
		}
		
		unsafe { self.temporary.set_len(0) };
	}
}

impl<T> ReversePopulatedVecBuilder<T>
{
	#[inline(always)]
	fn new(maximum_capacity: usize) -> Self
	{
		Self
		{
			temporary:
			{
				let mut temporary = Vec::with_capacity(maximum_capacity);
				unsafe { temporary.set_len(maximum_capacity) };
				temporary
			},
			
			maximum_capacity,
			
			count: 0,
		
			copied: false,
		}
	}
	
	#[inline(always)]
	fn try_reverse_push(&mut self, element: T) -> Result<(), ()>
	{
		let start_index = self.start_index();
		
		if unlikely!(start_index == 0)
		{
			return Err(())
		}
		
		let insertion_index = start_index - 1;
		let element_pointer = unsafe { self.start_pointer().add(insertion_index) };
		unsafe { element_pointer.write(element) }
		
		self.count += 1;
		
		Ok(())
	}
	
	/// NOTE: `copy_nonoverlapping()` is used in preference to `copy()` and re-use of the existing `temporary` Vec because:-
	///
	/// * under the covers it uses the much more efficient `memcpy`.
	/// * it simplifies the logic required in `drop()`.
	#[inline(always)]
	fn to_vec(mut self) -> Vec<T>
	{
		let mut vec = Vec::with_capacity(self.count);
		unsafe
		{
			unsafe { copy_nonoverlapping(self.start_pointer(), vec.as_mut_ptr(), self.count) }
			vec.set_len(self.count)
		}
		
		self.copied = true;
		drop(self);
		
		vec
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn start_pointer(&mut self) -> *mut T
	{
		self.temporary.as_mut_ptr()
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn start_index(&mut self) -> usize
	{
		self.maximum_capacity - self.count
	}
}

impl<'a> ReversePopulatedVecBuilder<CaseFoldedLabel<'a>>
{
	#[inline(always)]
	fn try_push_root_label(&mut self, label: &[u8]) -> Result<NonZeroU8, CaseFoldedNameParseError>
	{
		if likely!(label.is_empty())
		{
			self.try_reverse_push(CaseFoldedLabel::Root).unwrap();
			Ok(unsafe { NonZeroU8::new_unchecked(ParsedNameParser::SizeOfTrailingPeriod) })
		}
		else
		{
			Err(CaseFoldedNameParseError::RootLabelWasNotEmpty)
		}
	}
	
	#[inline(always)]
	fn try_push_non_empty_label(&mut self, label: &[u8], name_length_including_trailing_periods_after_labels: &mut NonZeroU8) -> Result<(), CaseFoldedNameParseError>
	{
		use self::CaseFoldedNameParseError::*;
		
		if label.is_empty()
		{
			return Err(NonRootLabelWasEmpty)
		}
		
		Self::change_name_length_including_trailing_periods_after_labels(label, name_length_including_trailing_periods_after_labels)?;
		self.try_reverse_push(CaseFoldedLabel::try_from(label)?).map_err(|_: ()| NumberOfLabelsExceed127)
	}
	
	#[inline(always)]
	fn change_name_length_including_trailing_periods_after_labels(label: &[u8], name_length_including_trailing_periods_after_labels: &mut NonZeroU8) -> Result<(), CaseFoldedNameParseError>
	{
		use self::CaseFoldedNameParseError::TotalNameLengthExceed255Bytes;
		
		let name_length = (*name_length_including_trailing_periods_after_labels).get();
		let name_length = name_length.checked_add(ParsedNameParser::SizeOfTrailingPeriod).ok_or(TotalNameLengthExceed255Bytes)?;
		let name_length = name_length.checked_add(label.len() as u8).ok_or(TotalNameLengthExceed255Bytes)?;
		
		*name_length_including_trailing_periods_after_labels = unsafe { NonZeroU8::new_unchecked(name_length) };
		Ok(())
	}
}
