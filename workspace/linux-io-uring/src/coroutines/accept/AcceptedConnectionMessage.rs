// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


struct AcceptedConnectionMessage<SD: SocketData>
{
	accepted_connection: AcceptedConnection<SD>,
	service_protocol_identifier: ServiceProtocolIdentifier,
	value: Arc<AccessControlValue>,
}

impl<SD: SocketData> Message for AcceptedConnectionMessage<SD>
{
	type ConstructMessageArguments = (AcceptedConnection<SD>, ServiceProtocolIdentifier, Arc<AccessControlValue>);
	
	#[inline(always)]
	unsafe fn construct_message(uninitialized_memory: NonNull<Self>, construct_message_arguments: Self::ConstructMessageArguments)
	{
		let (accepted_connection, service_protocol_identifier, value) = construct_message_arguments;
		
		let pointer = uninitialized_memory.as_ptr();
		pointer.write
		(
			Self
			{
				accepted_connection,
				service_protocol_identifier,
				value,
			}
		);
	}
	
	type MessageHandlerArguments = ();
	
	type DequeuedMessageProcessingError = DequeuedMessageProcessingError;
	
	#[inline(always)]
	fn handle_message(&mut self, _message_handler_arguments: &Self::MessageHandlerArguments) -> Result<(), Self::DequeuedMessageProcessingError>
	{
		unimplemented!("TODO")
	}
}
