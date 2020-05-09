// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Remap memory hints.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RemapMemoryHints
{
	/// Do not move.
	DoNotMove,

	/// May move the base address.
	MayMove,

	/// Forcibly map at this location or fail.
	MoveToFixedAddress
	{
		/// Required virtual address.
		///
		/// Must be equal to or greater than `/proc/sys/vm/mmap_min_addr`.
		///
		/// Must be a multiple of the regular or huge page sized used.
		virtual_address_required: VirtualAddress,
	}
}

impl RemapMemoryHints
{
	#[inline(always)]
	fn to_address_and_flags(&self, page_size: PageSizeOrHugePageSize, virtual_address: VirtualAddress) -> (*mut c_void, i32, VirtualAddress)
	{
		use self::RemapMemoryHints::*;

		match self
		{
			&DoNotMove => (null_mut(), 0, virtual_address),

			&MayMove => (null_mut(), MREMAP_MAYMOVE, virtual_address),

			&MoveToFixedAddress { virtual_address_required } =>
			{
				let address = Self::round_up_virtual_address_to_page_boundary(page_size, virtual_address_required);
				(address.into(), MREMAP_FIXED, address)
			}
		}
	}

	#[inline(always)]
	fn round_up_virtual_address_to_page_boundary(page_size: PageSizeOrHugePageSize, virtual_address: VirtualAddress) -> VirtualAddress
	{
		let address: u64 = virtual_address.into();
		VirtualAddress::from(page_size.number_of_bytes_rounded_up_to_multiple_of_page_size(address))
	}
}
