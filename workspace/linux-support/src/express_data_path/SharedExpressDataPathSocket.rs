// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Receive and transmit memory ring queues.
///
/// Created by `OwnedExpressDataPathSocket::share()`.
#[derive(Debug)]
pub struct SharedExpressDataPathSocket<ROTOB: ReceiveOrTransmitOrBoth, FFQ: FreeFrameQueue>
{
	common: ManuallyDrop<CommonExpressDataPathSocket<ROTOB>>,
	
	instance: ShareableExpressDataPathInstance<ROTOB, FFQ>,
	
	express_data_path_socket_file_descriptor: ManuallyDrop<ExpressDataPathSocketFileDescriptor>,

	queue_identifier: QueueIdentifier,
}

impl<ROTOB: ReceiveOrTransmitOrBoth, FFQ: FreeFrameQueue> SpecializationHackOfDropToOvercomeComplierErrorE0367ExpressDataPathSocket for SharedExpressDataPathSocket<ROTOB, FFQ>
{
	#[inline(always)]
	default fn specialization_of_drop(&mut self)
	{
		self.manually_drop()
	}
}

impl<ROTOB: ReceiveOrTransmitOrBoth<RP=RP> + Receives<CommonReceiveOnly<RP>>, FFQ: FreeFrameQueue, RP: ReceivePoll> SpecializationHackOfDropToOvercomeComplierErrorE0367ExpressDataPathSocket for SharedExpressDataPathSocket<ROTOB, FFQ>
{
	#[inline(always)]
	fn specialization_of_drop(&mut self)
	{
		self.common.remove_receive_map_queue_identifier(&self.instance);
		self.manually_drop()
	}
}

impl<ROTOB: ReceiveOrTransmitOrBoth, FFQ: FreeFrameQueue> SharedExpressDataPathSocket<ROTOB, FFQ>
{
	#[inline(always)]
	fn manually_drop(&mut self)
	{
		self.specialization_of_drop()
	}
}

impl<ROTOB: ReceiveOrTransmitOrBoth, FFQ: FreeFrameQueue> ExpressDataPathSocket<ROTOB, FFQ> for SharedExpressDataPathSocket<ROTOB, FFQ>
{
	#[inline(always)]
	fn user_memory(&self) -> &UserMemory<FFQ>
	{
		&self.instance.user_memory()
	}
	
	#[inline(always)]
	fn common(&self) -> &CommonExpressDataPathSocket<ROTOB>
	{
		&self.common
	}
	
	#[inline(always)]
	fn express_data_path_socket_file_descriptor(&self) -> &ExpressDataPathSocketFileDescriptor
	{
		&self.express_data_path_socket_file_descriptor
	}
}

impl<ROTOB: ReceiveOrTransmitOrBoth + Receives<CommonReceiveOnly<RP>>, FFQ: FreeFrameQueue, RP: ReceivePoll> ReceivesExpressDataPathSocket<ROTOB, FFQ, RP> for SharedExpressDataPathSocket<ROTOB, FFQ>
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

impl<ROTOB: ReceiveOrTransmitOrBoth + Transmits<CommonTransmitOnly>, FFQ: FreeFrameQueue> TransmitsExpressDataPathSocket<ROTOB, FFQ> for SharedExpressDataPathSocket<ROTOB, FFQ>
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

impl<ROTOB: ReceiveOrTransmitOrBoth<RP=RP> + Receives<CommonReceiveOnly<RP>>, FFQ: FreeFrameQueue, RP: ReceivePoll> SharedExpressDataPathSocket<ROTOB, FFQ>
{
	#[inline(always)]
	fn fill_queue_spin_lock(&self) -> &BestForCompilationTargetSpinLock
	{
		self.instance.fill_queue_spin_lock()
	}
}

impl<ROTOB: ReceiveOrTransmitOrBoth + Transmits<CommonTransmitOnly>, FFQ: FreeFrameQueue> SharedExpressDataPathSocket<ROTOB, FFQ>
{
	#[inline(always)]
	fn completion_queue_spin_lock(&self) -> &BestForCompilationTargetSpinLock
	{
		self.instance.completion_queue_spin_lock()
	}
}
