// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Endianness operation.
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[repr(u8)]
pub enum EndiannessOperation
{
	/// Convert to little-endian.
	ToLittleEndian = BPF_SRC(BPF_TO_LE as u16) as u8,
	
	/// Convert to big-endian.
	ToBigEndian = BPF_SRC(BPF_TO_BE as u16) as u8,
}

impl EndiannessOperation
{
	/// Convert from little-endian.
	///
	/// `BPF_FROM_LE`.
	pub const FromLittleEndian: Self = EndiannessOperation::ToLittleEndian;
	
	/// Convert from big-endian.
	///
	/// `BPF_FROM_BE`.
	pub const FromBigEndian: Self = EndiannessOperation::ToBigEndian;
}
