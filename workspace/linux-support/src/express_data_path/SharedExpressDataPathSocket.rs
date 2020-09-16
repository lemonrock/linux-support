// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Receive and transmit memory ring queues.
///
/// Created by `OwnedExpressDataPathSocket::share()`.
#[derive(Debug)]
pub struct SharedExpressDataPathSocket<'shared, ReceiveTransmit: ReceiveOrTransmitOrBoth<ReceiveQueue, TransmitQueue>, RP: ReceivePoll, CA: ChunkAlignment>
{
	user_memory: &'shared UserMemory,
	
	express_data_path_extended_bpf_program: &'shared RedirectMapAndAttachedProgram,
	
	express_data_path_socket_file_descriptor: ManuallyDrop<ExpressDataPathSocketFileDescriptor>,
	
	common: CommonSharedExpressDataPathSocket<ReceiveTransmit, RP>,
}

impl<'shared, ReceiveTransmit: ReceiveOrTransmitOrBoth<ReceiveQueue, TransmitQueue>, RP: ReceivePoll, CA: ChunkAlignment> Drop for SharedExpressDataPathSocket<'shared, ReceiveTransmit, RP, CA>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.common.remove_receive_map_queue_identifier(self.express_data_path_extended_bpf_program);
		
		unsafe
		{
			ManuallyDrop::drop(&mut self.common);
			ManuallyDrop::drop(&mut self.express_data_path_socket_file_descriptor);
		}
	}
}

impl<'shared, ReceiveTransmit: ReceiveOrTransmitOrBoth<ReceiveQueue, TransmitQueue>, RP: ReceivePoll, CA: ChunkAlignment> ExpressDataPathSocket for SharedExpressDataPathSocket<'shared, ReceiveTransmit, RP, CA>
{
	#[inline(always)]
	fn user_memory(&self) -> &UserMemory<CA>
	{
		self.user_memory
	}
	
	#[inline(always)]
	fn common(&self) -> &CommonSharedExpressDataPathSocket<RP>
	{
		&self.common
	}
	
	#[inline(always)]
	fn express_data_path_socket_file_descriptor(&self) -> &ExpressDataPathSocketFileDescriptor
	{
		&self.express_data_path_socket_file_descriptor
	}
}
