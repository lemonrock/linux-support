// This file is part of libc-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT. No part of libc-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of libc-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ifmap
{
	pub mem_start: c_ulong,
	pub mem_end: c_ulong,
	pub base_addr: c_ushort,
	pub irq: c_uchar,
	pub dma: c_uchar,
	pub port: c_uchar,
}

impl Default for ifmap
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
