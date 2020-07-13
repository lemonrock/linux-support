// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C, align(8))]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct perf_mem_data_src__bindgen_ty_1
{
	pub(crate) _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize], u32>,
}

#[allow(dead_code)]
impl perf_mem_data_src__bindgen_ty_1
{
	#[inline(always)]
	pub(crate) fn mem_op(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(0usize, 5u8) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_mem_op(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(0usize, 5u8, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn mem_lvl(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(5usize, 14u8) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_mem_lvl(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(5usize, 14u8, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn mem_snoop(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(19usize, 5u8) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_mem_snoop(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(19usize, 5u8, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn mem_lock(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(24usize, 2u8) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_mem_lock(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(24usize, 2u8, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn mem_dtlb(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(26usize, 7u8) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_mem_dtlb(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(26usize, 7u8, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn mem_lvl_num(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(33usize, 4u8) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_mem_lvl_num(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(33usize, 4u8, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn mem_remote(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(37usize, 1u8) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_mem_remote(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(37usize, 1u8, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn mem_snoopx(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(38usize, 2u8) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_mem_snoopx(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(38usize, 2u8, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn mem_rsvd(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(40usize, 24u8) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_mem_rsvd(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(40usize, 24u8, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn new_bitfield_1
	(
		mem_op: u64,
		mem_lvl: u64,
		mem_snoop: u64,
		mem_lock: u64,
		mem_dtlb: u64,
		mem_lvl_num: u64,
		mem_remote: u64,
		mem_snoopx: u64,
		mem_rsvd: u64,
	) -> __BindgenBitfieldUnit<[u8; 8], u32> {
		let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8], u32> = Default::default();
		__bindgen_bitfield_unit.set(0, 5, {
			let mem_op: u64 = unsafe { transmute(mem_op) };
			mem_op as u64
		});
		__bindgen_bitfield_unit.set(5, 14, {
			let mem_lvl: u64 = unsafe { transmute(mem_lvl) };
			mem_lvl as u64
		});
		__bindgen_bitfield_unit.set(19, 5, {
			let mem_snoop: u64 = unsafe { transmute(mem_snoop) };
			mem_snoop as u64
		});
		__bindgen_bitfield_unit.set(24, 2, {
			let mem_lock: u64 = unsafe { transmute(mem_lock) };
			mem_lock as u64
		});
		__bindgen_bitfield_unit.set(26, 7, {
			let mem_dtlb: u64 = unsafe { transmute(mem_dtlb) };
			mem_dtlb as u64
		});
		__bindgen_bitfield_unit.set(33, 4, {
			let mem_lvl_num: u64 = unsafe { transmute(mem_lvl_num) };
			mem_lvl_num as u64
		});
		__bindgen_bitfield_unit.set(37, 1, {
			let mem_remote: u64 = unsafe { transmute(mem_remote) };
			mem_remote as u64
		});
		__bindgen_bitfield_unit.set(38, 2, {
			let mem_snoopx: u64 = unsafe { transmute(mem_snoopx) };
			mem_snoopx as u64
		});
		__bindgen_bitfield_unit.set(40, 24, {
			let mem_rsvd: u64 = unsafe { transmute(mem_rsvd) };
			mem_rsvd as u64
		});
		__bindgen_bitfield_unit
	}
}
