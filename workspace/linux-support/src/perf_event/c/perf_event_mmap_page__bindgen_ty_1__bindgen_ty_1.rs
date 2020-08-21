// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C, align(8))]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct perf_event_mmap_page__bindgen_ty_1__bindgen_ty_1
{
	pub(crate) _bitfield_1: __BindgenBitfieldUnit<[u8; 8], u64>,
}

#[allow(dead_code)]
impl perf_event_mmap_page__bindgen_ty_1__bindgen_ty_1
{
	#[inline(always)]
	pub(crate) fn cap_bit0(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(0usize, 1u8) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_cap_bit0(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(0usize, 1u8, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn cap_bit0_is_deprecated(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(1usize, 1u8) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_cap_bit0_is_deprecated(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(1usize, 1u8, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn cap_user_rdpmc(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(2usize, 1u8) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_cap_user_rdpmc(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(2usize, 1u8, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn cap_user_time(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(3usize, 1u8) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_cap_user_time(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(3usize, 1u8, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn cap_user_time_zero(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(4usize, 1u8) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_cap_user_time_zero(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(4usize, 1u8, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn cap_____res(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(5usize, 59u8) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_cap_____res(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(5usize, 59u8, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn new_bitfield_1
	(
		cap_bit0: u64,
		cap_bit0_is_deprecated: u64,
		cap_user_rdpmc: u64,
		cap_user_time: u64,
		cap_user_time_zero: u64,
		cap_____res: u64,
	) -> __BindgenBitfieldUnit<[u8; 8], u64>
	{
		let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8], u64> = Default::default();
		__bindgen_bitfield_unit.set(0, 1,
		{
			let cap_bit0: u64 = unsafe { transmute(cap_bit0) };
			cap_bit0 as u64
		});
		__bindgen_bitfield_unit.set(1, 1,
		{
			let cap_bit0_is_deprecated: u64 = unsafe { transmute(cap_bit0_is_deprecated) };
			cap_bit0_is_deprecated as u64
		});
		__bindgen_bitfield_unit.set(2, 1,
		{
			let cap_user_rdpmc: u64 = unsafe { transmute(cap_user_rdpmc) };
			cap_user_rdpmc as u64
		});
		__bindgen_bitfield_unit.set(3, 1,
		{
			let cap_user_time: u64 = unsafe { transmute(cap_user_time) };
			cap_user_time as u64
		});
		__bindgen_bitfield_unit.set(4, 1,
		{
			let cap_user_time_zero: u64 = unsafe { transmute(cap_user_time_zero) };
			cap_user_time_zero as u64
		});
		__bindgen_bitfield_unit.set(5, 59,
		{
			let cap_____res: u64 = unsafe { transmute(cap_____res) };
			cap_____res as u64
		});
		__bindgen_bitfield_unit
	}
}
