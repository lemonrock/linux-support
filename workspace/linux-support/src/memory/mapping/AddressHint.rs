// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Address hint.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum AddressHint
{
	/// Let the Linux kernel choose any location.
	Any
	{
		/// Force the mapping to be in the first 2Gb of address space.
		///
		/// Useful today only for generated machine code.
		#[cfg(target_arch = "x86_64")]
		constrain_to_first_2Gb: bool,
	},

	/// Prefer this location.
	Prefer
	{
		/// Force the mapping to be in the first 2Gb of address space.
		///
		/// Useful today only for generated machine code.
		#[cfg(target_arch = "x86_64")]
		constrain_to_first_2Gb: bool,

		/// Preferred virtual address.
		///
		/// Will be forced to be equal to or greater than `/proc/sys/vm/mmap_min_addr`.
		///
		/// Must be a multiple of the regular or huge page sized used.
		virtual_address_preference: VirtualAddress,
	},

	/// Forcibly map at this location or fail.
	///
	/// Uses `MAP_FIXED` and `MAP_FIXED_NOREPLACE`.
	Fixed
	{
		/// Required virtual address.
		///
		/// Must be equal to or greater than `/proc/sys/vm/mmap_min_addr`.
		///
		/// Must be a multiple of the regular or huge page sized used.
		virtual_address_required: VirtualAddress,
	},
}

impl Default for AddressHint
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::any()
	}
}

impl AddressHint
{
	/// Convenience constructor for `Any`.
	#[inline(always)]
	pub const fn any() -> Self
	{
		AddressHint::Any
		{
			#[cfg(target_arch = "x86_64")] constrain_to_first_2Gb: false,
		}
	}
	
	/// Convenience constructor for `Fixed`.
	#[inline(always)]
	pub fn fixed(virtual_address_base: VirtualAddress, virtual_address_offset: u64) -> Self
	{
		AddressHint::Fixed
		{
			virtual_address_required: virtual_address_base + virtual_address_offset,
		}
	}

	/// Convenience constructor for `Prefer`.
	#[inline(always)]
	pub const fn prefer(virtual_address_preference: VirtualAddress) -> Self
	{
		AddressHint::Prefer
		{
			#[cfg(target_arch = "x86_64")] constrain_to_first_2Gb: false,
			virtual_address_preference,
		}
	}

	#[cfg(not(target_arch = "x86_64"))]
	#[inline(always)]
	fn to_address_and_flags(self, page_size: PageSizeOrHugePageSize, _length_in_bytes: u64) -> (*mut c_void, i32)
	{
		use self::AddressHint::*;

		match self
		{
			Any { } => (null_mut(), 0),

			Prefer { virtual_address_preference } => (Self::round_up_virtual_address_to_page_boundary(page_size, virtual_address_preference), 0),

			Fixed { virtual_address_required } => (Self::round_up_virtual_address_to_page_boundary(page_size, virtual_address_preference), MAP_FIXED | MAP_FIXED_NOREPLACE),
		}
	}

	#[cfg(target_arch = "x86_64")]
	#[inline(always)]
	fn to_address_and_flags(self, page_size: PageSizeOrHugePageSize, length_in_bytes: u64) -> (*mut c_void, i32)
	{
		use self::AddressHint::*;

		const _2Gb: u64 = 2 * 1024 * 1024 * 1024;

		#[inline(always)]
		fn constrain_to_first_2Gb_flags(constrain_to_first_2Gb: bool, virtual_address: *mut c_void, length_in_bytes: u64) -> i32
		{
			if constrain_to_first_2Gb
			{
				if cfg!(debug_assertions)
				{
					debug_assert!((virtual_address as u64) + length_in_bytes <= _2Gb, "constrain_to_first_2Gb is true but address + length_in_bytes would exceed that")
				}
				MAP_32BIT
			}
			else
			{
				0
			}
		}

		match self
		{
			Any { constrain_to_first_2Gb } =>
			{
				if cfg!(debug_assertions)
				{
					if constrain_to_first_2Gb
					{
						debug_assert!(length_in_bytes <= _2Gb, "constrain_to_first_2Gb is true but length_in_bytes alone would exceed that")
					}
				}

				const virtual_address: *mut c_void = null_mut();

				(virtual_address, constrain_to_first_2Gb_flags(constrain_to_first_2Gb, virtual_address, length_in_bytes))
			},

			Prefer { constrain_to_first_2Gb, virtual_address_preference } =>
			{
				let virtual_address = Self::round_up_virtual_address_to_page_boundary(page_size, virtual_address_preference);

				(virtual_address, constrain_to_first_2Gb_flags(constrain_to_first_2Gb, virtual_address, length_in_bytes))
			},

			Fixed { virtual_address_required } => (Self::round_up_virtual_address_to_page_boundary(page_size, virtual_address_required), MAP_FIXED | MAP_FIXED_NOREPLACE),
		}
	}

	#[inline(always)]
	fn round_up_virtual_address_to_page_boundary(page_size: PageSizeOrHugePageSize, virtual_address: VirtualAddress) -> *mut c_void
	{
		let address: u64 = virtual_address.into();
		page_size.number_of_bytes_rounded_up_to_multiple_of_page_size(address) as *mut c_void
	}
}
