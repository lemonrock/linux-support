// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Rx/Tx descriptor.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(C)]
pub(crate) struct xdp_desc
{
	/// This is the `addr` in the functions `xsk_umem__extract_addr()` and `xsk_umem__add_offset_to_addr()` in the Linux source `tools/lib/bpf/xsk.h`.
	///
	/// If using aligned chunks, then this is an address.
	/// If using unaligned chunks, this is a bit field:-
	///
	/// * The top 16 bits contain an offset to the start of an Ethernet packet.
	/// * The bottom 48 bits contain the same address kind as for an aligned chunk.
	addr: FrameDescriptorBitfield,
	
	len: u32,
	
	options: u32,
}

impl xdp_desc
{
	#[inline(always)]
	pub(crate) fn received_relative_addresses_and_offsets_if_aligned(&self, frame_headroom: FrameHeadroom) -> RelativeAddressesAndOffsets
	{
		RelativeAddressesAndOffsets::from_received_frame_descriptor_if_aligned(self.addr, self.len, frame_headroom)
	}
	
	#[inline(always)]
	pub(crate) fn received_relative_addresses_and_offsets_if_unaligned(&self, frame_headroom: FrameHeadroom) -> RelativeAddressesAndOffsets
	{
		RelativeAddressesAndOffsets::from_received_frame_descriptor_if_unaligned(self.addr, self.len, frame_headroom)
	}
	
	#[inline(always)]
	pub(crate) fn write_for_transmit_copying_from(transmit_descriptor: NonNull<Self>, other: &Self)
	{
		unsafe { other.copy_to_nonoverlapping(transmit_descriptor.as_ptr(), size_of::<FrameDescriptor>()) }
	}
	
	/// Parameters are named by analogy to the fields in `ReceivedRelativeAddressesAndOffsets`.
	#[inline(always)]
	pub(crate) fn write_for_transmit_aligned(transmit_descriptor: NonNull<Self>, start_of_packet: u64, length_of_packet: usize)
	{
		Self::write(transmit_descriptor, FrameDescriptorBitfield::for_aligned(start_of_packet), length)
	}
	
	/// Parameters are named by analogy to the fields in `ReceivedRelativeAddressesAndOffsets`.
	#[inline(always)]
	pub(crate) fn write_for_transmit_unaligned(transmit_descriptor: NonNull<Self>, orig_addr: u64, offset: u64, length_of_packet: usize)
	{
		Self::write(transmit_descriptor, FrameDescriptorBitfield::for_unaligned(orig_addr, offset), length_of_packet)
	}
	
	#[inline(always)]
	fn write(transmit_descriptor: NonNull<Self>, address_and_offset_bitfield: FrameDescriptorBitfield, length_of_packet: usize)
	{
		const OptionsMustCurrentlyBeAlwaysZero: u32 = 0;
		
		unsafe
		{
			let this = transmit_descriptor.as_mut();
			write(&mut this.addr, address_and_offset_bitfield.into());
			write(&mut this.len, length_of_packet.try_into().unwrap());
			write(&mut this.options, OptionsMustCurrentlyBeAlwaysZero);
		}
	}
}
