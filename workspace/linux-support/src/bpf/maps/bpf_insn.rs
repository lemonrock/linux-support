// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub(crate) struct bpf_insn
{
	pub(crate) code: u8,
	pub(crate) _bitfield_1: __BindgenBitfieldUnit<[u8; 1], u8>,
	pub(crate) off: i16,
	pub(crate) imm: i32,
}

impl bpf_insn
{
	#[inline(always)]
	pub(crate) fn dst_reg(&self) -> u8
	{
		unsafe { transmute(self._bitfield_1.get(0, 4u8) as u8) }

	}

	#[inline(always)]
	pub(crate) fn set_dst_reg(&mut self, val: u8)
	{
		unsafe
		{
			let val: u8 = transmute(val);
			self._bitfield_1.set(0, 4u8, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn src_reg(&self) -> u8
	{
		unsafe { transmute(self._bitfield_1.get(4, 4u8) as u8) }
	}

	#[inline(always)]
	pub(crate) fn set_src_reg(&mut self, val: u8)
	{
		unsafe
		{
			let val: u8 = transmute(val);
			self._bitfield_1.set(4, 4u8, val as u64)
		}

	}

	#[inline(always)]
	pub(crate) fn new_bitfield_1(dst_reg: u8, src_reg: u8) -> __BindgenBitfieldUnit<[u8; 1], u8>
	{
		let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1], u8> = Default::default();
		__bindgen_bitfield_unit.set
		(
			0,
			4,
		{
			let dst_reg: u8 = unsafe { transmute(dst_reg) };
			dst_reg as u64
			}
		);
		__bindgen_bitfield_unit.set
		(
			4,
			4,
		{
				let src_reg: u8 = unsafe { transmute(src_reg) };
				src_reg as u64
			}
		);
		__bindgen_bitfield_unit
	}
}
