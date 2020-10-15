// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) struct Message;

impl Message
{
	pub(crate) const MinimumMessageSize: usize = MessageHeader::Size + Self::MinimumMessageBodySize;

	const MinimumMessageBodySize: usize = 0;

	const MaximumQueryMessageSize: usize = MessageHeader::Size + QuerySectionEntry::MaximumSizeOfOneQuery + ResourceRecord::ExtendedDns0OptRecordWithoutOptionsSize;

	/// Message header.
	#[inline(always)]
	pub(crate) fn message_header(&self) -> &MessageHeader
	{
		self.unsafe_cast::<MessageHeader>()
	}

	/// Message body.
	#[inline(always)]
	pub(crate) fn message_body_as_query_section_entry(&mut self) -> &mut QuerySectionEntry
	{
		let message_header_pointer = self.as_usize_pointer() + MessageHeader::Size;
		message_header_pointer.unsafe_cast_mut::<QuerySectionEntry>()
	}
}
