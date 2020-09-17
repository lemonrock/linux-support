// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Owned socket.
#[derive(Debug)]
pub struct OwnedExpressDataPathSocket<ROTOB: ReceiveOrTransmitOrBoth, CA: ChunkAlignment>
{
	common: ManuallyDrop<CommonExpressDataPathSocket<ROTOB>>,
	
	instance: ManuallyDrop<ExpressDataPathInstance<CA>>,
}

impl<ROTOB: ReceiveOrTransmitOrBoth + Receives<CommonReceiveOnly<RP>>, CA: ChunkAlignment, RP: ReceivePoll> Drop for OwnedExpressDataPathSocket<ROTOB, CA>
{
	/// Based on `libbpf`'s `xsk_socket__delete()`.
	#[inline(always)]
	fn drop(&mut self)
	{
		self.common.remove_receive_map_queue_identifier(&self.redirect_map_and_attached_program);
		self.manually_drop()
	}
}

impl<ROTOB: ReceiveOrTransmitOrBoth + Transmits<CommonTransmitOnly>, CA: ChunkAlignment> Drop for OwnedExpressDataPathSocket<ROTOB, CA>
{
	/// Based on `libbpf`'s `xsk_socket__delete()`.
	#[inline(always)]
	fn drop(&mut self)
	{
		self.manually_drop()
	}
}

impl<ROTOB: ReceiveOrTransmitOrBoth, CA: ChunkAlignment> OwnedExpressDataPathSocket<ROTOB, CA>
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

impl<ROTOB: ReceiveOrTransmitOrBoth, CA: ChunkAlignment> ExpressDataPathSocket<ROTOB, CA> for OwnedExpressDataPathSocket<ROTOB, CA>
{
	#[inline(always)]
	fn user_memory(&self) -> &UserMemory<CA>
	{
		&self.user_memory
	}
	
	#[inline(always)]
	fn common(&self) -> &CommonExpressDataPathSocket<ROTOB>
	{
		&self.common
	}
	
	#[inline(always)]
	fn express_data_path_socket_file_descriptor(&self) -> &ExpressDataPathSocketFileDescriptor
	{
		&self.user_memory().user_memory_socket_file_descriptor
	}
}

impl<ROTOB: ReceiveOrTransmitOrBoth + Receives<CommonReceiveOnly<RP>>, CA: ChunkAlignment, RP: ReceivePoll> ReceivesExpressDataPathSocket<ROTOB, CA> for OwnedExpressDataPathSocket<ROTOB, CA>
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

impl<ROTOB: ReceiveOrTransmitOrBoth + Transmits<CommonTransmitOnly>, CA: ChunkAlignment> TransmitsExpressDataPathSocket<ROTOB, CA> for OwnedExpressDataPathSocket<ROTOB, CA>
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
