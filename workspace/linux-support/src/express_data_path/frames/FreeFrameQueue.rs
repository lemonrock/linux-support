// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A free frame queue contains is a memory allocator for frames.
pub trait FreeFrameQueue: Send + Sync
{
	/// Aligned or Unaligned.
	type CS: ChunkSize;
	
	/// Constructor.
	fn new(number_of_chunks: NonZeroU32, user_memory: &MappedMemory) -> Self;
	
	/// Return a frame no longer in use.
	fn push(&self, newly_freed_frame_identifier: <Self::CS as ChunkSize>::FrameIdentifier);
	
	/// Get a frame to use.
	fn pop(&self) -> Option<<Self::CS as ChunkSize>::FrameIdentifier>;
}
