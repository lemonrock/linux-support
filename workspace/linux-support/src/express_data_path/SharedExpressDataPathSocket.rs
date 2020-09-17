// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Receive and transmit memory ring queues.
///
/// Created by `OwnedExpressDataPathSocket::share()`.
#[derive(Debug)]
pub struct SharedExpressDataPathSocket<ROTOB: ReceiveOrTransmitOrBoth, CA: ChunkAlignment>
{
	owner: ShareableExpressDataPathSocket<ROTOB, CA>,
	
	express_data_path_socket_file_descriptor: ManuallyDrop<ExpressDataPathSocketFileDescriptor>,
	
	common: CommonExpressDataPathSocket<ROTOB>,
}

impl<ROTOB: ReceiveOrTransmitOrBoth + Receives<CommonReceiveOnly<RP>>, CA: ChunkAlignment, RP: ReceivePoll> Drop for SharedExpressDataPathSocket<RingQueueDepths, CA>
{
	/// Based on `libbpf`'s `xsk_socket__delete()`.
	#[inline(always)]
	fn drop(&mut self)
	{
		self.common.remove_receive_map_queue_identifier(self.owner.redirect_map_and_attached_program());
		self.manually_drop()
	}
}

impl<ROTOB: ReceiveOrTransmitOrBoth + Transmits<CommonTransmitOnly>, CA: ChunkAlignment> Drop for SharedExpressDataPathSocket<RingQueueDepths, CA>
{
	/// Based on `libbpf`'s `xsk_socket__delete()`.
	#[inline(always)]
	fn drop(&mut self)
	{
		self.manually_drop()
	}
}

impl<ROTOB: ReceiveOrTransmitOrBoth, CA: ChunkAlignment> SharedExpressDataPathSocket<RingQueueDepths, CA>
{
	#[inline(always)]
	fn manually_drop(&mut self)
	{
		unsafe
		{
			ManuallyDrop::drop(&mut self.common);
			ManuallyDrop::drop(&mut self.express_data_path_socket_file_descriptor);
		}
	}
}

impl<ROTOB: ReceiveOrTransmitOrBoth, CA: ChunkAlignment> ExpressDataPathSocket<RingQueueDepths::ReceiveOrTransmitOrBoth, CA> for SharedExpressDataPathSocket<RingQueueDepths, CA>
{
	#[inline(always)]
	fn user_memory(&self) -> &UserMemory<CA>
	{
		&self.owner.user_memory()
	}
	
	#[inline(always)]
	fn common(&self) -> &CommonExpressDataPathSocket<RingQueueDepths::ReceiveOrTransmitOrBoth>
	{
		&self.common
	}
	
	#[inline(always)]
	fn express_data_path_socket_file_descriptor(&self) -> &ExpressDataPathSocketFileDescriptor
	{
		&self.express_data_path_socket_file_descriptor
	}
}

impl<ROTOB: ReceiveOrTransmitOrBoth + Receives<CommonReceiveOnly<RP>>, CA: ChunkAlignment, RP: ReceivePoll> ReceivesExpressDataPathSocket<RingQueueDepths::ReceiveOrTransmitOrBoth, CA> for SharedExpressDataPathSocket<RingQueueDepths, CA>
{
	#[inline(always)]
	fn lock_fill_queue(&self)
	{
		self.fill_queue_spin_lock().acquire_spin_lock()
	}
	
	#[inline(always)]
	fn unlock_fill_queue(&self)
	{
		self.fill_queue_spin_lock().unlock_spin_lock()
	}
}

impl<ROTOB: ReceiveOrTransmitOrBoth + Transmits<CommonTransmitOnly>, CA: ChunkAlignment> TransmitsExpressDataPathSocket<RingQueueDepths::ReceiveOrTransmitOrBoth, CA> for SharedExpressDataPathSocket<RingQueueDepths, CA>
{
	#[inline(always)]
	fn lock_completion_queue(&self)
	{
		self.completion_queue_spin_lock().acquire_spin_lock()
	}
	
	#[inline(always)]
	fn unlock_completion_queue(&self)
	{
		self.completion_queue_spin_lock().unlock_spin_lock()
	}
}

impl<ROTOB: ReceiveOrTransmitOrBoth + Receives<CommonReceiveOnly<RP>>, CA: ChunkAlignment, RP: ReceivePoll> SharedExpressDataPathSocket<RingQueueDepths, CA>
{
	#[inline(always)]
	fn fill_queue_spin_lock(&self) -> &BestForCompilationTargetSpinLock
	{
		self.owner.fill_queue_spin_lock()
	}
}

impl<ROTOB: ReceiveOrTransmitOrBoth + Transmits<CommonTransmitOnly>, CA: ChunkAlignment> SharedExpressDataPathSocket<RingQueueDepths, CA>
{
	#[inline(always)]
	fn completion_queue_spin_lock(&self) -> &BestForCompilationTargetSpinLock
	{
		self.owner.completion_queue_spin_lock()
	}
}
