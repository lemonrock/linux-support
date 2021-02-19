// This file is part of libc-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT. No part of libc-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of libc-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT.


pub const RXH_L2DA: c_uchar = 2;
pub const RXH_VLAN: c_uchar = 4;
pub const RXH_L3_PROTO: c_uchar = 8;
pub const RXH_IP_SRC: c_uchar = 16;
pub const RXH_IP_DST: c_uchar = 32;
pub const RXH_L4_B_0_1: c_uchar = 64;
pub const RXH_L4_B_2_3: c_uchar = 128;
pub const RXH_DISCARD: c_uint = 2147483648;
