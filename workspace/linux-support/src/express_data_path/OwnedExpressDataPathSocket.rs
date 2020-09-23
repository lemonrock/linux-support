// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Owned socket.
#[derive(Debug)]
pub struct OwnedExpressDataPathSocket<ROTOB: ReceiveOrTransmitOrBoth, FFQ: FreeFrameQueue>
{
	common: ManuallyDrop<CommonExpressDataPathSocket<ROTOB>>,
	
	instance: ManuallyDrop<ExpressDataPathInstance<ROTOB, FFQ>>,
}

impl<ROTOB: ReceiveOrTransmitOrBoth, FFQ: FreeFrameQueue> SpecializationHackOfDropToOvercomeComplierErrorE0367ExpressDataPathSocket for OwnedExpressDataPathSocket<ROTOB, FFQ>
{
	#[inline(always)]
	default fn specialization_of_drop(&mut self)
	{
		self.manually_drop()
	}
}

impl<ROTOB: ReceiveOrTransmitOrBoth<RP=RP> + Receives<CommonReceiveOnly<RP>>, FFQ: FreeFrameQueue, RP: ReceivePoll> SpecializationHackOfDropToOvercomeComplierErrorE0367ExpressDataPathSocket for OwnedExpressDataPathSocket<ROTOB, FFQ>
{
	#[inline(always)]
	fn specialization_of_drop(&mut self)
	{
		self.common.remove_receive_map_queue_identifier(&self.instance);
		self.manually_drop()
	}
}

impl<ROTOB: ReceiveOrTransmitOrBoth, FFQ: FreeFrameQueue> Drop for OwnedExpressDataPathSocket<ROTOB, FFQ>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.specialization_of_drop()
	}
}

impl<ROTOB: ReceiveOrTransmitOrBoth, FFQ: FreeFrameQueue> OwnedExpressDataPathSocket<ROTOB, FFQ>
{
	/// Based on `libbpf`'s `xsk_socket__delete()`.
	#[inline(always)]
	fn manually_drop(&mut self)
	{
		unsafe
		{
			ManuallyDrop::drop(&mut self.common);
			ManuallyDrop::drop(&mut self.instance);
		}
	}
}

impl<ROTOB: ReceiveOrTransmitOrBoth, FFQ: FreeFrameQueue> ExpressDataPathSocket<ROTOB, FFQ> for OwnedExpressDataPathSocket<ROTOB, FFQ>
{
	#[inline(always)]
	fn user_memory(&self) -> &UserMemory<FFQ>
	{
		self.instance.user_memory()
	}
	
	#[inline(always)]
	fn common(&self) -> &CommonExpressDataPathSocket<ROTOB>
	{
		&self.common
	}
	
	#[inline(always)]
	fn express_data_path_socket_file_descriptor(&self) -> &ExpressDataPathSocketFileDescriptor
	{
		self.user_memory().user_memory_socket_file_descriptor()
	}
}

impl<ROTOB: ReceiveOrTransmitOrBoth + Receives<CommonReceiveOnly<RP>>, FFQ: FreeFrameQueue, RP: ReceivePoll> ReceivesExpressDataPathSocket<ROTOB, FFQ, RP> for OwnedExpressDataPathSocket<ROTOB, FFQ>
{
	#[inline(always)]
	fn lock_fill_queue(&self)
	{
	}
	
	#[inline(always)]
	fn unlock_fill_queue(&self)
	{
	}
}

impl<ROTOB: ReceiveOrTransmitOrBoth + Transmits<CommonTransmitOnly>, FFQ: FreeFrameQueue> TransmitsExpressDataPathSocket<ROTOB, FFQ> for OwnedExpressDataPathSocket<ROTOB, FFQ>
{
	#[inline(always)]
	fn lock_completion_queue(&self)
	{
	}
	
	#[inline(always)]
	fn unlock_completion_queue(&self)
	{
	}
}
