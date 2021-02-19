// This file is part of libc-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT. No part of libc-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of libc-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT.


#![allow(non_camel_case_types)] 

use ::std::default::Default;
use ::std::mem::transmute;
use ::std::mem::zeroed;
use ::libc::c_char;
use ::libc::c_int;
use ::libc::c_short;
use ::libc::c_uchar;
use ::libc::c_ulong;
use ::libc::c_ushort;
use ::libc::c_void;
use ::libc::IF_NAMESIZE;
use ::libc::sockaddr;
use ::libc::socklen_t;


include!("cmsghdr.rs");
include!("functions.rs");
include!("iface.rs");
include!("ifaddr.rs");
include!("ifconf.rs");
include!("ifmap.rs");
include!("ifreq.rs");
