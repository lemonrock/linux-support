// This file is part of libc-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT. No part of libc-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of libc-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT.


#![allow(non_camel_case_types)] 

use ::std::default::Default;
use ::std::mem::zeroed;
use ::libc::c_ushort;
use ::libc::uint8_t;
use ::libc::uint16_t;
use ::libc::size_t;


include!("constants/lengths.rs");
include!("constants/ETH_P.EtherType.rs");
include!("constants/ETH_P.FrameType.rs");
include!("ethhdr.rs");
// Needs arphdr
// include!("ether_arp.rs");
