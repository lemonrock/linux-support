// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
#[derive(Default, Debug,  Clone)]
pub(crate) struct ifmap
{
	pub(crate) mem_start: c_ulong,
	pub(crate) mem_end: c_ulong,
	pub(crate) base_addr: c_ushort,
	pub(crate) irq: c_uchar,
	pub(crate) dma: c_uchar,
	pub(crate) port: c_uchar,
}
