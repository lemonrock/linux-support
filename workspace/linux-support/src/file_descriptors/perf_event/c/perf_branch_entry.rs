// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord)]
pub(crate) struct perf_branch_entry
{
	pub(crate) from: u64,
	pub(crate) to: u64,
	pub(crate) _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize], u64>,
}

impl perf_branch_entry
{
	#[inline(always)]
	pub(crate) fn mispred(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(0usize, 1u8) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_mispred(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(0usize, 1u8, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn predicted(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(1usize, 1u8) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_predicted(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(1usize, 1u8, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn in_tx(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(2usize, 1u8) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_in_tx(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(2usize, 1u8, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn abort(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(3usize, 1u8) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_abort(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(3usize, 1u8, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn cycles(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(4usize, 16u8) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_cycles(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(4usize, 16u8, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn type_(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(20usize, 4u8) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_type(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(20usize, 4u8, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn reserved(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(24usize, 40u8) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_reserved(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(24usize, 40u8, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn new_bitfield_1(mispred: u64, predicted: u64, in_tx: u64, abort: u64, cycles: u64, type_: u64, reserved: u64) -> __BindgenBitfieldUnit<[u8; 8], u64> {
		let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8], u64> = Default::default();
		__bindgen_bitfield_unit.set(0usize, 1, {
			let mispred: u64 = unsafe { transmute(mispred) };
			mispred as u64
		});
		__bindgen_bitfield_unit.set(1, 1, {
			let predicted: u64 = unsafe { transmute(predicted) };
			predicted as u64
		});
		__bindgen_bitfield_unit.set(2, 1, {
			let in_tx: u64 = unsafe { transmute(in_tx) };
			in_tx as u64
		});
		__bindgen_bitfield_unit.set(3, 1, {
			let abort: u64 = unsafe { transmute(abort) };
			abort as u64
		});
		__bindgen_bitfield_unit.set(4, 16, {
			let cycles: u64 = unsafe { transmute(cycles) };
			cycles as u64
		});
		__bindgen_bitfield_unit.set(20, 4, {
			let type_: u64 = unsafe { transmute(type_) };
			type_ as u64
		});
		__bindgen_bitfield_unit.set(24, 40, {
			let reserved: u64 = unsafe { transmute(reserved) };
			reserved as u64
		});
		__bindgen_bitfield_unit
	}
}
