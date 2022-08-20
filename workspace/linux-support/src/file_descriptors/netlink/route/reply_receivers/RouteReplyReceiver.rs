// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A reply receiver for route netlink.
pub(crate) struct RouteReplyReceiver<'a, RMP: 'a + MessageProcessor>
{
	message_sets: HashMap<MultipartMessagePartIdentification, (Vec<RMP::ProcessedMessage>, Vec<String>, Result<bool, SystemCallErrorNumber>)>,
	current_message_set: Option<(MultipartMessagePartIdentification, Vec<RMP::ProcessedMessage>, Vec<String>)>,
	could_not_start_messages_errors: Vec<io::Error>,
	route_message_processor: &'a RMP,
}

impl<'a, RMP: 'a + MessageProcessor> ReplyReceiver<RouteNetlinkProtocol> for RouteReplyReceiver<'a, RMP>
{
	#[inline(always)]
	fn start_of_set_of_messages(&mut self, message_identification: &MultipartMessagePartIdentification)
	{
		debug_assert!(self.current_message_set.is_none());
		debug_assert_eq!(self.message_sets.contains_key(message_identification), false);
		
		self.current_message_set = Some((message_identification.clone(), Vec::new(), Vec::new()));
	}
	
	#[inline(always)]
	fn could_not_start_messages(&mut self, error: io::Error)
	{
		debug_assert!(self.current_message_set.is_none());
		
		self.could_not_start_messages_errors.push(error);
	}
	
	#[inline(always)]
	fn could_not_continue_multipart_messages(&mut self, error: io::Error)
	{
		debug_assert!(self.current_message_set.is_some());
		
		self.could_not_start_messages_errors.push(error);
	}
	
	#[inline(always)]
	fn unexpected_end_of_set_of_multipart_messages(&mut self)
	{
		debug_assert!(self.current_message_set.is_some());
		
		self.could_not_start_messages_errors.push(ErrorKind::UnexpectedEof.into())
	}
	
	#[inline(always)]
	fn message(&mut self, message_type: RouteNetlinkMessageType, data: &[u8])
	{
		debug_assert!(self.current_message_set.is_some());
		
		let current_message_set = self.current_message_set.as_mut().unwrap();
		
		match self.route_message_processor.process_message(message_type, data)
		{
			Ok(Some(processed_message)) => current_message_set.1.push(processed_message),
			
			Ok(None) => (),
			
			Err(explanation) => current_message_set.2.push(explanation),
		}
	}
	
	#[inline(always)]
	fn end_of_set_of_messages(&mut self, result: Result<bool, SystemCallErrorNumber>)
	{
		debug_assert!(self.current_message_set.is_some());
		
		let (message_identification, messages, message_parsing_errors) = self.current_message_set.take().unwrap();
		
		let previous = self.message_sets.insert(message_identification, (messages, message_parsing_errors, result));
		debug_assert!(previous.is_none())
	}
}

impl<'a, RMP: 'a + MessageProcessor> RouteReplyReceiver<'a, RMP>
{
	// Not sure we need to try receiving more than once but nothing about netlink seems obvious.
	#[doc(hidden)]
	pub(super) fn try_receiving_until_get_reply(netlink_socket_file_descriptor: &NetlinkSocketFileDescriptor<RouteNetlinkProtocol>, route_message_processor: &RMP, message_identification: MultipartMessagePartIdentification) -> Result<Option<Vec<RMP::ProcessedMessage>>, Either<Vec<String>, SystemCallErrorNumber>>
	{
		let mut reply_receiver = RouteReplyReceiver::new(route_message_processor);
		loop
		{
			netlink_socket_file_descriptor.receive_replies(&mut reply_receiver);
			
			reply_receiver.panic_if_has_could_not_start_messages_errors();
			
			match reply_receiver.messages(&message_identification)
			{
				Err(reply_receiver_again) =>
				{
					reply_receiver = reply_receiver_again;
					continue
				}
				
				Ok(something) => return something,
			}
		}
	}
	
	/// New instance.
	#[inline(always)]
	fn new(route_message_processor: &'a RMP) -> Self
	{
		Self
		{
			message_sets: Default::default(),
			current_message_set: None,
			could_not_start_messages_errors: vec![],
			route_message_processor,
		}
	}
	
	#[inline(always)]
	fn panic_if_has_could_not_start_messages_errors(&self)
	{
		if !self.could_not_start_messages_errors.is_empty()
		{
			panic!("Could not gather messages: {:?}", self.could_not_start_messages_errors);
		}
	}
	
	/// Messages.
	#[inline(always)]
	fn messages(mut self, message_identification: &MultipartMessagePartIdentification) -> Result<Result<Option<Vec<RMP::ProcessedMessage>>, Either<Vec<String>, SystemCallErrorNumber>>, Self>
	{
		match self.message_sets.remove(message_identification)
		{
			None => Err(self),
			
			Some((processed_messages, message_parsing_errors, end_of_set_of_messages_result)) => Ok
			(
				{
					let has_message_parsing_errors = !message_parsing_errors.is_empty();
					if has_message_parsing_errors
					{
						Err(Left(message_parsing_errors))
					}
					else
					{
						match end_of_set_of_messages_result
						{
							Ok(DumpWasInterrupted) => Ok(None),
							
							Ok(DumpCompleted) => Ok(Some(processed_messages)),
							
							Err(error) => Err(Right(error)),
						}
					}
				}
			)
		}
	}
}
