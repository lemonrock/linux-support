// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Creation of ring queues.
pub trait CreateReceiveOrTransmitOrBoth: RingQueueDepths
{
	/// Arguments required to create a receive or transmit queue.
	type Arguments: Sized;
	
	// It would be nice to constrain this further but Rust does not support this.
	#[doc(hidden)]
	type ReceiveOrTransmitOrBoth: ReceiveOrTransmitOrBoth;
	
	#[doc(hidden)]
	fn set_ring_queue_depths(&self, express_data_path_socket_file_descriptor: &ExpressDataPathSocketFileDescriptor);
	
	#[doc(hidden)]
	fn create_receive_or_transmit_or_both(self, express_data_path_socket_file_descriptor: &ExpressDataPathSocketFileDescriptor, memory_map_offsets: &xdp_mmap_offsets, queue_identifier: QueueIdentifier, redirect_map_and_attached_program: &RedirectMapAndAttachedProgram, arguments: Self::Arguments) -> Result<Self::ReceiveOrTransmitOrBoth, ExpressDataPathSocketCreationError>;
}
