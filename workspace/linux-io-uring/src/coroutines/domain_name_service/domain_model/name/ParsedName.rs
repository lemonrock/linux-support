// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A parsed name.
#[derive(Clone)]
pub struct ParsedName<'message>
{
	label_data_starts_at_pointers_and_label_length_excluding_trailing_period: ArrayVec<[(usize, u8); LabelMaximumNumber]>,
	
	name_length_including_trailing_periods_after_labels: NonZeroU8,

	marker: PhantomData<&'message ()>
}

impl<'message> Debug for ParsedName<'message>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		self.display(f)
	}
}

impl<'message> Display for ParsedName<'message>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		self.display(f)
	}
}

impl<'message> PartialEq for ParsedName<'message>
{
	#[inline(always)]
	fn eq(&self, rhs: &Self) -> bool
	{
		self.equals(rhs)
	}
}

impl<'message> Eq for ParsedName<'message>
{
}

impl<'message> PartialOrd for ParsedName<'message>
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
	{
		self.partial_compare(rhs)
	}
}

impl<'message> Ord for ParsedName<'message>
{
	#[inline(always)]
	fn cmp(&self, rhs: &Self) -> Ordering
	{
		self.compare(rhs)
	}
}

impl<'message> Hash for ParsedName<'message>
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		self.hash_slice(state)
	}
}

impl<'message> PartialEq<EfficientCaseFoldedName> for ParsedName<'message>
{
	#[inline(always)]
	fn eq(&self, rhs: &EfficientCaseFoldedName) -> bool
	{
		self.equals(rhs)
	}
}

impl<'message> PartialOrd<EfficientCaseFoldedName> for ParsedName<'message>
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &EfficientCaseFoldedName) -> Option<Ordering>
	{
		self.partial_compare(rhs)
	}
}

impl<'message> HasTypeEquality for ParsedName<'message>
{
	type TypeEquality = ParsedTypeEquality;
}

impl<'message> Name for ParsedName<'message>
{
	type Label<'label> = ParsedLabel<'label>;
	
	#[inline(always)]
	fn parent(&self) -> Option<Self>
	{
		if unlikely!(self.is_root())
		{
			None
		}
		else
		{
			let mut label_data_starts_at_pointers_and_label_length_excluding_trailing_period = ArrayVec::new();
			label_data_starts_at_pointers_and_label_length_excluding_trailing_period.try_extend_from_slice(&self.label_data_starts_at_pointers_and_label_length_excluding_trailing_period[1 .. ]).unwrap();
			
			let child_label_length = self.label(0).len();
			Some(Self::new(label_data_starts_at_pointers_and_label_length_excluding_trailing_period, self.name_length_including_trailing_periods_after_labels.get() - child_label_length))
		}
	}
	
	#[inline(always)]
	fn label<'label>(&'label self, index: u8) -> Self::Label<'label>
	{
		let (label_data_starts_at_pointer, label_length_excluding_trailing_period) = self.label_data_starts_at_pointers_and_label_length_excluding_trailing_period.get_unchecked_value_safe(index);
		
		ParsedLabel(unsafe { from_raw_parts(label_data_starts_at_pointer as *const u8, label_length_excluding_trailing_period as usize) })
	}
	
	#[inline(always)]
	fn number_of_labels_including_root(&self) -> NonZeroU8
	{
		new_non_zero_u8(self.label_data_starts_at_pointers_and_label_length_excluding_trailing_period.len() as u8)
	}
	
	#[inline(always)]
	fn name_length_including_trailing_periods_after_labels(&self) -> NonZeroU8
	{
		self.name_length_including_trailing_periods_after_labels
	}
}

impl<'message> ParsedName<'message>
{
	#[inline(always)]
	pub(crate) fn new(label_data_starts_at_pointers_and_label_length_excluding_trailing_period: ArrayVec<[(usize, u8); LabelMaximumNumber]>, name_length_including_trailing_periods_after_labels: u8) -> Self
	{
		debug_assert_ne!(name_length_including_trailing_periods_after_labels, 0);
		
		Self
		{
			label_data_starts_at_pointers_and_label_length_excluding_trailing_period,
			name_length_including_trailing_periods_after_labels: new_non_zero_u8(name_length_including_trailing_periods_after_labels),
			marker: PhantomData,
		}
	}
}
