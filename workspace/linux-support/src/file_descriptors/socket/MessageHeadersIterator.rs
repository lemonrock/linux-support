// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[allow(dead_code)]
pub(crate) struct MessageHeadersIterator<'a>
{
	parent: &'a msghdr,
	next: Option<&'a cmsghdr>,
}

impl<'a> Iterator for MessageHeadersIterator<'a>
{
	type Item = &'a cmsghdr;

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		let next_message_header = self.next.take();
		if let Some(next_message_header) = next_message_header
		{
			self.next = next_message_header.next(self.parent);
		}

		next_message_header
	}
}
