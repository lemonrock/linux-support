// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


struct Label
{
	bitfield: LabelBitfield,
	bytes: UpTo63Bytes,
}

impl Label
{
	pub(crate) const MaximumNumber: usize = 127;

	#[inline(always)]
	fn is_root(&self) -> bool
	{
		self.bitfield.is_root()
	}

	/// Two bits, `u2`.
	#[inline(always)]
	fn raw_kind(&self) -> LabelKind
	{
		self.bitfield.raw_kind()
	}

	/// Actually `u6` (an inclusive maximum of 63).
	#[inline(always)]
	fn length(&self) -> usize
	{
		self.bitfield.bottom_6_bits_as_usize()
	}

	/// Actually `u14`.
	#[inline(always)]
	fn offset(&self) -> usize
	{
		let top_6_bits = self.bitfield.bottom_6_bits_as_usize() << 8;
		let bottom_8_bits = *self.bytes().unsafe_cast::<u8>() as usize;
		top_6_bits | bottom_8_bits
	}

	#[inline(always)]
	fn bytes(&self) -> &UpTo63Bytes
	{
		&self.bytes
	}

	#[inline(always)]
	fn label<'message>(label_starts_at_pointer: usize) -> &'message Label
	{
		label_starts_at_pointer.unsafe_cast::<Label>()
	}
}
