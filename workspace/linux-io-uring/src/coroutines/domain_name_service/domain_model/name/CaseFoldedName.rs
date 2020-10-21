// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A case-folded (normalized to lower case) name which consists of labels, including a terminal root label.
#[derive(Default, Debug, Clone)]
pub struct CaseFoldedName
{
	labels: Box<[CaseFoldedLabel]>,
	
	name_length_including_trailing_periods_after_labels: NonZeroU8,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CaseFoldedNameParseError
{
	RootLabelWasNotEmpty,

	NonRootLabelWasEmpty,

	LabelExceeded63Bytes,
}

impl Display for CaseFoldedNameParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for CaseFoldedNameParseError
{
}

impl<'a> TryFrom<&'a [u8]> for CaseFoldedName
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: &'a [u8]) -> Result<Self, Self::Error>
	{
		use self::ParseNumberError::*;
		
		let mut labels = Vec::new();
		let mut name_length_including_trailing_periods_after_labels = 0u8;
		
		let mut iterator = value.split_bytes_reverse(b'.');
		let should_be_root_label = iterator.next().unwrap();
		if unlikely!(!should_be_root_label.is_empty())
		{
			return Err(TooLarge)
		}
		
		
		
		for label in value.split_bytes(b'.')
		{
			labels.push(CaseFoldedLabel::new_cloned_and_case_folded(label)?);
			name_length_including_trailing_periods_after_labels = name_length_including_trailing_periods_after_labels.checked_add(label.len() as u8).ok_or(TooLarge)?;
		}
		
		// All but last label should not be empty.
		let last_index = labels.len() - 1;
		for index in 0 .. last_index
		{
			if unlikely!((unsafe { labels.get_unchecked(index) }).len() == 0)
			{
				return Err(WasZero)
			}
		}
		
		// Last label should be root.
		if unlikely!((unsafe { labels.get_unchecked(last_index) }).len() != 0)
		{
			return Err(TooLarge)
		}
		
		Ok
		(
			Self
			{
				labels: labels.into_boxed_slice(),
				
				name_length_including_trailing_periods_after_labels: unsafe { NonZeroU8::new_unchecked(name_length_including_trailing_periods_after_labels) }
			}
		)
	}
}

impl<'message> From<ParsedName<'message>> for CaseFoldedName
{
	#[inline(always)]
	fn from(value: ParsedName<'message>) -> Self
	{
		Self
		{
			labels:
			{
				let mut labels = Vec::with_capacity(value.label_data_starts_at_pointers_and_label_length_excluding_trailing_period.len());
				
				for index in 0 .. value.number_of_labels_including_root().get()
				{
					let parsed_label_cow = value.label(index);
					labels.push(Self::from(parsed_label_cow.deref()))
				}
				
				labels.into_boxed_slice()
			},
			name_length_including_trailing_periods_after_labels: value.name_length_including_trailing_periods_after_labels
		}
	}
}

impl<'label> Name<'label> for CaseFoldedName
{
	type Label = CaseFoldedLabel;
	
	#[inline(always)]
	fn parent(&self) -> Option<Self>
	{
		if unlikely!(self.number_of_labels_including_root().get() == 1)
		{
			None
		}
		else
		{
			let child_label_length = self.label(0).len();
			Some
			(
				Self
				{
					labels: (self.labels[1 .. ]).to_vec().into_boxed_slice(),
					
					name_length_including_trailing_periods_after_labels: unsafe { NonZeroU8::new_unchecked(self.name_length_including_trailing_periods_after_labels.get() - child_label_length) },
				}
			)
		}
	}
	
	#[inline(always)]
	fn label<'label>(&'label self, index: u8) -> Cow<'label, Self::Label>
	{
		Cow::Borrowed(unsafe { self.labels.get_unchecked(index as usize) })
	}
	
	#[inline(always)]
	fn number_of_labels_including_root(&self) -> NonZeroU8
	{
		unsafe { NonZeroU8::new_unchecked(self.labels.len() as u8) }
	}
	
	#[inline(always)]
	fn name_length_including_trailing_periods_after_labels(&self) -> NonZeroU8
	{
		self.name_length_including_trailing_periods_after_labels
	}
}

impl PartialEq for CaseFoldedName
{
	#[inline(always)]
	fn eq(&self, rhs: &Self) -> bool
	{
		self.equals(rhs)
	}
}

impl Eq for CaseFoldedName
{
}

impl PartialOrd for CaseFoldedName
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
	{
		self.partial_compare(rhs)
	}
}

impl Ord for CaseFoldedName
{
	#[inline(always)]
	fn cmp(&self, rhs: &Self) -> Ordering
	{
		self.compare(rhs)
	}
}

impl Hash for CaseFoldedName
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		self.hash_slice(state)
	}
}

impl<'message> PartialEq<ParsedName<'message>> for CaseFoldedName
{
	#[inline(always)]
	fn eq(&self, rhs: &CaseFoldedName) -> bool
	{
		self.equals(rhs)
	}
}

impl<'message> PartialOrd<ParsedName<'message>> for CaseFoldedName
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
	{
		self.partial_compare(rhs)
	}
}

impl CaseFoldedName
{ RFC 6762, Appendix G: Private DNS Namespace.
	pub(crate) fn top_level(label: &[u8]) -> HashSet<Self>
	{
		hashset!
		{
			Self::from()
		}
	}
	
	/// Based on
	pub(crate) fn recommended_local_names_in_rfc_6762_appendix_g()
	{
	
	}
	
}
