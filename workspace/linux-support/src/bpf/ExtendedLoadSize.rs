// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Size of .
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[repr(u8)]
pub enum ExtendedLoadSize
{
	/// 8-bit.
	_8 = BPF_SIZE(BPF_B) as u8,
	
	/// 16-bit.
	_16 = BPF_SIZE(BPF_H) as u8,
	
	/// 32-bit.
	_32 = BPF_SIZE(BPF_W) as u8,
	
	/// 64-bit.
	_64 = BPF_SIZE(BPF_DW as u16) as u8,
}
