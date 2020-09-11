// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A message processor for route messages.
pub(crate) trait MessageProcessor
{
	/// Expected reply header.
	type Header: Sized;
	
	/// Type of state used when processing a message.
	type ProcessingMessageState: Sized;
	
	/// Type of processed message.
	type ProcessedMessage: Sized;
	
	type NAT: NetlinkAttributeType;
	
	#[doc(hidden)]
	#[inline(always)]
	fn process_message(&self, _message_type: RouteNetlinkMessageType, data: &[u8]) -> Result<Option<Self::ProcessedMessage>, String>
	{
		let message_header = unsafe { & * (data.as_ptr() as *const Self::Header) };
		let mut processing_message_state = match self.process_message_header(message_header)?
		{
			None => return Ok(None),
			Some(processing_message_state) => processing_message_state,
		};
		
		let pointer = data.as_ptr();
		let data_length = data.len();
		let mut potential_message_attribute_pointer = unsafe { pointer.add(NETLINK_ALIGN::<Self::Header>()) as *const rtattr<Self::NAT> };
		let end = unsafe { pointer.add(data_length) as *const rtattr<Self::NAT> };
		while rtattr::ok(potential_message_attribute_pointer, end)
		{
			let message_attribute = unsafe { & * potential_message_attribute_pointer };
			self.process_message_attribute(message_attribute, &mut processing_message_state)?;
			potential_message_attribute_pointer = message_attribute.next()
		}
		Ok(Some(self.finalize(processing_message_state)?))
	}
	
	/// `Ok(Self::ProcessedMessage)` can be `Ok(Self::ProcessingMessageState)`.
	fn process_message_header(&self, message_header: &Self::Header) -> Result<Option<Self::ProcessingMessageState>, String>;
	
	/// Process a message attribute.
	fn process_message_attribute(&self, message_attribute: &rtattr<Self::NAT>, processing_message_state: &mut Self::ProcessingMessageState) -> Result<(), String>;
	
	/// Convert to a processed message.
	fn finalize(&self, processing_message_state: Self::ProcessingMessageState) -> Result<Self::ProcessedMessage, String>;
}
