// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) struct AcceptPublisher<SA: SocketAddress>(Rc<Publisher<AcceptedConnectionMessage<SA::SD>, MessageHandlerArguments, DequeuedMessageProcessingError>>);

impl<SA: SocketAddress> Clone for AcceptPublisher<SA>
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		Self(self.0.clone())
	}
}

impl<SA: SocketAddress> AcceptPublisher<SA>
{
	#[inline(always)]
	pub(crate) fn new(queues: &Queues<MessageHandlerArguments, DequeuedMessageProcessingError>, our_hyper_thread: HyperThread) -> Self
	{
		Self(Rc::new(queues.publisher(our_hyper_thread)))
	}
	
	#[inline(always)]
	pub(crate) fn publish(&self, socket_hyper_thread: HyperThread, accepted_connection: AcceptedConnection<SA::SD>, service_protocol_identifier: ServiceProtocolIdentifier, value: &Arc<RemotePeerAddressBasedAccessControlValue>) -> HyperThread
	{
		self.0.publish(socket_hyper_thread, (accepted_connection, service_protocol_identifier, value.clone()))
	}
}
