// This file is part of libc-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT. No part of libc-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of libc-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT.


pub const WAKE_PHY: c_uchar = 1;
pub const WAKE_UCAST: c_uchar = 2;
pub const WAKE_MCAST: c_uchar = 4;
pub const WAKE_BCAST: c_uchar = 8;
pub const WAKE_ARP: c_uchar = 16;
pub const WAKE_MAGIC: c_uchar = 32;
pub const WAKE_MAGICSECURE: c_uchar = 64;
