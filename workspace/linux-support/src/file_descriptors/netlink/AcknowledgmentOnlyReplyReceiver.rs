// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) struct AcnowledgmentOnlyReplyReceiver<Protocol: NetlinkProtocol>
{
	expected_message_identification: MultipartMessagePartIdentification,
	start_of_set_of_messages_called: bool,
	acknowledgment: Option<Result<(), Errno>>,
	marker: PhantomData<Protocol>,
}

impl<Protocol: NetlinkProtocol> ReplyReceiver<Protocol> for AcnowledgmentOnlyReplyReceiver<Protocol>
{
	#[inline(always)]
	fn start_of_set_of_messages(&mut self, message_identification: &MultipartMessagePartIdentification)
	{
		assert!(!self.start_of_set_of_messages_called, "start_of_set_of_messages({:?}) should never be called more than once - Linux kernel bug?", message_identification);
		
		assert_eq!(message_identification, &self.expected_message_identification, "start_of_set_of_messages({:?}) does not match expected_message_identification `{:?}` - Linux kernel bug?", message_identification, self.expected_message_identification);
		
		self.start_of_set_of_messages_called = true;
	}
	
	#[inline(always)]
	fn could_not_start_messages(&mut self, error: io::Error)
	{
		panic!("could_not_start_messages({:?}) should never be called - Linux kernel bug?", error)
	}
	
	#[inline(always)]
	fn could_not_continue_multipart_messages(&mut self, error: io::Error)
	{
		panic!("could_not_continue_multipart_messages({:?}) should never be called - Linux kernel bug?", error)
	}
	
	#[inline(always)]
	fn unexpected_end_of_set_of_multipart_messages(&mut self)
	{
		panic!("unexpected_end_of_set_of_multipart_messages() should never be called - Linux kernel bug?")
	}
	
	#[inline(always)]
	fn message(&mut self, message_type: Protocol::MessageType, data: &[u8])
	{
		assert!(self.start_of_set_of_messages_called, "start_of_set_of_messages({:?}, _) should be called before message()", message_type);
		
		panic!("message({:?}, _) should never be called as an acknowledgment is always an error and so end_of_set_of_messages() should be called - Linux kernel bug?", message_type)
	}
	
	#[inline(always)]
	fn end_of_set_of_messages(&mut self, result: Result<bool, Errno>)
	{
		assert!(self.start_of_set_of_messages_called, "start_of_set_of_messages() should be called before end_of_set_of_messages({:?})", result);
		assert!(self.acknowledgment.is_none(), "end_of_set_of_messages({:?}) should never be called more than once", result);
		
		self.acknowledgment = Some
		(
			match result
			{
				Ok(DumpCompleted) => Ok(()),
				
				Ok(DumpWasInterrupted) => panic!("end_of_set_of_messages(DumpWasInterrupted) can never be valid - Linux kernel bug?"),
				
				Err(errno) => Err(errno)
			}
		);
	}
}

impl<Protocol: NetlinkProtocol> AcnowledgmentOnlyReplyReceiver<Protocol>
{
	#[inline(always)]
	pub(crate) const fn new(expected_message_identification: MultipartMessagePartIdentification) -> Self
	{
		Self
		{
			expected_message_identification,
			start_of_set_of_messages_called: false,
			acknowledgment: None,
			marker: PhantomData,
		}
	}
	
	#[inline(always)]
	pub(crate) fn acknowledgment(self) -> Result<(), Errno>
	{
		self.acknowledgment.expect("end_of_set_of_messages() was never called - Linux kernel bug?")
	}
}
