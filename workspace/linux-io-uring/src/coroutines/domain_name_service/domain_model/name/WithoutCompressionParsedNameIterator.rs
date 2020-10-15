// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Does not contain the root, empty label.
///
/// RFC 2065 asserts that the maximum number of labels is 127; this makes sense if every label bar the last (which is Root) is 1 byte long and so occupies 2 bytes.
/// However, the maximum reasonable length is an IPv6 reverse DNS look up, which requires 33 labels (32 for each nibble and 2 for `ip6.arpa` less 1 for the omitted root label) of a `SRV` entry such as `_mqtt._tcp`, thus 35 labels.
#[derive(Default, Debug, Clone)]
pub struct WithoutCompressionParsedNameIterator<'message>
{
	pointer_to_label: usize,
	marker: PhantomData<&'message ()>,
}

impl<'message> Iterator for WithoutCompressionParsedNameIterator<'message>
{
	type Item = LabelBytes<'message>;

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		let (label, pointer_to_label) = iterator_next_label!(self);
		bytes_label!(self, label, pointer_to_label)
	}
}

impl<'message> WithoutCompressionParsedNameIterator<'message>
{
	#[inline(always)]
	pub(crate) fn new(pointer_to_label: usize) -> Self
	{
		Self
		{
			pointer_to_label,
			marker: PhantomData,
		}
	}
}
