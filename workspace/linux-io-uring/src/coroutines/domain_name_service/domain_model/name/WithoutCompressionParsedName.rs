// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Iterating this *excludes* the root label.
#[derive(Default, Debug, Clone)]
pub struct WithoutCompressionParsedName<'message>
{
	/// This *includes* the root label.
	pub(crate) number_of_labels: u8,

	/// This *includes* the root label.
	pub(crate) name_length: u8,

	iterator: WithoutCompressionParsedNameIterator<'message>,
}

impl<'message> IntoIterator for WithoutCompressionParsedName<'message>
{
	type Item = LabelBytes<'message>;

	type IntoIter = WithoutCompressionParsedNameIterator<'message>;

	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter
	{
		self.iterator
	}
}

impl<'a, 'message> IntoIterator for &'a WithoutCompressionParsedName<'message>
{
	type Item = LabelBytes<'message>;

	type IntoIter = WithoutCompressionParsedNameIterator<'message>;

	#[inline(always)]
	fn into_iter(&self) -> Self::IntoIter
	{
		self.iterator.clone()
	}
}

impl<'message> PartialEq for WithoutCompressionParsedName<'message>
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		if likely!(self.number_of_labels != other.number_of_labels)
		{
			return false
		}
		if likely!(self.name_length != other.name_length)
		{
			return false
		}
		let left = self.into_iter();
		let right = other.into_iter();
		for (left_label, right_label) in left.zip(right)
		{
			if likely!(left_label != right_label)
			{
				return false
			}
		}
		true
	}
}

impl<'message> PartialEq<WithCompressionParsedName<'message>> for WithoutCompressionParsedName<'message>
{
	#[inline(always)]
	fn eq(&self, other: &WithCompressionParsedName<'message>) -> bool
	{
		other.eq(self)
	}
}

impl<'message, Allocator: AllocRef> PartialEq<UncompressedName<Allocator>> for WithoutCompressionParsedName<'message>
{
	#[inline(always)]
	fn eq(&self, other: &UncompressedName<Allocator>) -> bool
	{
		self.eq(other.name())
	}
}

impl<'message> Eq for WithoutCompressionParsedName<'message>
{
}

impl<'message> Hash for WithoutCompressionParsedName<'message>
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		for label in self
		{
			label.hash(state)
		}
	}
}

impl<'message> WithoutCompressionParsedName<'message>
{
	#[inline(always)]
	pub(crate) fn new(number_of_labels: u8, name_length: u8, pointer_to_label: usize) -> Self
	{
		Self
		{
			number_of_labels,
			name_length,
			iterator: WithoutCompressionParsedNameIterator::new(pointer_to_label),
		}
	}

	#[inline(always)]
	pub(crate) fn parent(&self) -> Option<Self>
	{
		if unlikely!(self.number_of_labels == 1)
		{
			None
		}
		else
		{
			let pointer_to_label = self.iterator.pointer_to_label;
			let label_length = pointer_to_label.dereference_u8();
			let parent_pointer_to_label = pointer_to_label + LabelKind::LabelKindSize + label_length;
			let parent_name_length = self.name_length - label_length;
			Some(Self::new(self.number_of_labels - 1, parent_name_length, parent_pointer_to_label))
		}
	}

	#[inline(always)]
	pub(crate) fn ends_with(&self, shorter_or_same_length_name: &WithCompressionParsedName<'message>) -> bool
	{
		if unlikely!(self.name_length < shorter_or_same_length_name.name_length)
		{
			return false
		}

		if unlikely!(self.number_of_labels < shorter_or_same_length_name.number_of_labels)
		{
			return false
		}

		let mut labels_to_pop = self.number_of_labels - shorter_or_same_length_name.number_of_labels;
		debug_assert_ne!(labels_to_pop, self.number_of_labels, "Can not pop all labels");

		if likely!(labels_to_pop == 0)
		{
			self.eq(shorter_or_same_length_name)
		}
		else
		{
			let mut shorter = self.parent().unwrap();
			labels_to_pop -= 1;
			while labels_to_pop != 0
			{
				shorter = shorter.parent().unwrap();
				labels_to_pop -= 1;
			}
			shorter.eq(shorter_or_same_length_name)
		}
	}

	#[inline(always)]
	pub(crate) fn parse_without_compression_but_register_labels_for_compression(parsed_labels: &mut ParsedLabels, start_of_name_pointer: usize, end_of_data_section_containing_name_pointer: usize) -> Result<(Self, usize), DnsProtocolError>
	{
		let mut labels_register: LabelsRegister = ArrayVec::new();
		let labels_register_reference = &mut labels_register;
		let (pointer_to_label, true_end_of_name_pointer, number_of_labels, name_length) = parse_name!(start_of_name_pointer, end_of_data_section_containing_name_pointer, labels_register_reference, parsed_labels, parse_and_register_bytes_label, compressed_error);

		let this = Self::new(number_of_labels, name_length, pointer_to_label);

		Ok((this, true_end_of_name_pointer))
	}

	#[inline(always)]
	pub(crate) fn parse_without_compression(start_of_name_pointer: usize, end_of_data_section_containing_name_pointer: usize) -> Result<(Self, usize), DnsProtocolError>
	{
		const _labels_register_reference: usize = 0;
		const _parsed_labels: usize = 0;
		let (pointer_to_label, true_end_of_name_pointer, number_of_labels, name_length) = parse_name!(start_of_name_pointer, end_of_data_section_containing_name_pointer, _labels_register_reference, _parsed_labels, parse_and_ignore_bytes_label, compressed_error);

		let this = Self::new(number_of_labels, name_length, pointer_to_label);

		Ok((this, true_end_of_name_pointer))
	}
}
