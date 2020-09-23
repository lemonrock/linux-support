// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[doc(hidden)]
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommonExpressDataPathSocket<ROTOB: ReceiveOrTransmitOrBoth>(ROTOB);

impl<ROTOB: ReceiveOrTransmitOrBoth> CommonExpressDataPathSocket<ROTOB>
{
	/// Based on `libbpf`'s `xsk_socket__create()`.
	fn new<RingQueueDepths: CreateReceiveOrTransmitOrBoth<ReceiveOrTransmitOrBoth=ROTOB>>(express_data_path_socket_file_descriptor: &ExpressDataPathSocketFileDescriptor, network_interface_index: NetworkInterfaceIndex, receive_or_transmit_or_both_ring_queue_depths: RingQueueDepths, owned_or_shared: XdpSocketAddressFlags, force_copy: bool, force_zero_copy: bool, user_memory_socket_file_descriptor: &ExpressDataPathSocketFileDescriptor, queue_identifier: QueueIdentifier, defaults: &DefaultPageSizeAndHugePageSizes, redirect_map_and_attached_program: &RedirectMapAndAttachedProgram, arguments: RingQueueDepths::Arguments) -> Result<ManuallyDrop<Self>, ExpressDataPathSocketCreationError>
	{
		// NOTE: Needs to happen before the socket is bound below.
		let receive_or_transmit_or_both =
		{
			receive_or_transmit_or_both_ring_queue_depths.set_ring_queue_depths(express_data_path_socket_file_descriptor);
			
			// NOTE: Valid memory map offsets are not available until the socket options in `ring_queue_depths.set_ring_queue_depths()` have been set.
			let memory_map_offsets = express_data_path_socket_file_descriptor.get_memory_map_offsets();
			
			receive_or_transmit_or_both_ring_queue_depths.create_receive_or_transmit_or_both(express_data_path_socket_file_descriptor, defaults, &memory_map_offsets, queue_identifier, redirect_map_and_attached_program, arguments)
		};
		
		{
			let socket_address = sockaddr_xdp
			{
				sxdp_family: AF_XDP as u16,
				sxdp_flags: owned_or_shared.sxdp_flags(force_copy, force_zero_copy, true),
				sxdp_ifindex: network_interface_index,
				sxdp_queue_id: queue_identifier,
				sxdp_shared_umem_fd: user_memory_socket_file_descriptor.as_raw_fd(),
			};
			bind_socket(express_data_path_socket_file_descriptor, &socket_address)?;
		}
		
		Ok(Self(ManuallyDrop::new(receive_or_transmit_or_both)))
	}
}

impl<ROTOB: ReceiveOrTransmitOrBoth<RP=RP> + Receives<CommonReceiveOnly<RP>>, RP: ReceivePoll> CommonExpressDataPathSocket<ROTOB>
{
	#[inline(always)]
	fn receive_queue(&self) -> &ReceiveQueue
	{
		self.receive().receive_queue()
	}
	
	#[inline(always)]
	fn frames_received(&self) -> u64
	{
		self.receive().frames_received()
	}
	
	#[inline(always)]
	fn increase_frames_received(&self, number_of_frames: NonZeroU32)
	{
		self.receive().increase_frames_received(number_of_frames)
	}
	
	#[inline(always)]
	fn receive_poll(&self)
	{
		self.receive().receive_poll()
	}
	
	#[inline(always)]
	fn remove_receive_map_queue_identifier<FFQ: FreeFrameQueue>(&mut self, instance: &ExpressDataPathInstance<ROTOB, FFQ>)
	{
		let _ignored = self.receive().remove_receive_map_queue_identifier(&instance.redirect_map_and_attached_program);
	}
	
	#[inline(always)]
	fn receive(&self) -> &CommonReceiveOnly<ROTOB::RP>
	{
		self.common_receive_or_transmit_or_both.receive()
	}
}

impl<ROTOB: ReceiveOrTransmitOrBoth + Transmits<CommonTransmitOnly>> CommonExpressDataPathSocket<ROTOB>
{
	#[inline(always)]
	fn transmit_queue(&self) -> &TransmitQueue
	{
		self.transmit().transmit_queue()
	}
	
	#[inline(always)]
	fn frames_transmitted(&self) -> u64
	{
		self.transmit().frames_transmitted()
	}
	
	#[inline(always)]
	fn outstanding_frames_to_transmit(&self) -> u32
	{
		self.transmit().outstanding_frames_to_transmit()
	}
	
	#[inline(always)]
	fn increment_outstanding_frames_to_transmit(&self, number_of_frames: NonZeroU32)
	{
		self.transmit().increment_outstanding_frames_to_transmit(number_of_frames)
	}
	
	#[inline(always)]
	fn change_frames_transmitted(&self, completed_number_of_frames: NonZeroU32)
	{
		self.transmit().decrement_outstanding_frames_to_transmit(completed_number_of_frames);
		self.transmit().increase_frames_transmitted(completed_number_of_frames);
	}
	
	#[inline(always)]
	fn transmit(&self) -> &CommonTransmitOnly
	{
		self.common_receive_or_transmit_or_both.transmit()
	}
}
