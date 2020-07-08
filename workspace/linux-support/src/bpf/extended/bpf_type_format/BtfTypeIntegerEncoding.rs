// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Integer encoding.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum BtfTypeIntegerEncoding
{
	/// Unsigned.
	Unsigned = 0,
	
	/// Signed.
	Signed = BTF_INT_SIGNED,
	
	/// Char.
	Char = BTF_INT_CHAR,
	
	/// Boolean.
	Boolean = BTF_INT_BOOL,
}

impl Default for BtfTypeIntegerEncoding
{
	#[inline(always)]
	fn default() -> Self
	{
		BtfTypeIntegerEncoding::Unsigned
	}
}
