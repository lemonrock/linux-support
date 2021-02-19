// This file is part of libc-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT. No part of libc-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of libc-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ether_arp
{
    pub ea_hdr: arphdr,
    pub arp_sha: [uint8_t; 6],
    pub arp_spa: [uint8_t; 4],
    pub arp_tha: [uint8_t; 6],
    pub arp_tpa: [uint8_t; 4],
}

impl Default for ether_arp
{
    fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
