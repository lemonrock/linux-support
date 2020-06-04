// This file is part of libc-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT. No part of libc-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of libc-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT.


pub const TCP_V4_FLOW: c_uchar = 1;
pub const UDP_V4_FLOW: c_uchar = 2;
pub const SCTP_V4_FLOW: c_uchar = 3;
pub const AH_ESP_V4_FLOW: c_uchar = 4;
pub const TCP_V6_FLOW: c_uchar = 5;
pub const UDP_V6_FLOW: c_uchar = 6;
pub const SCTP_V6_FLOW: c_uchar = 7;
pub const AH_ESP_V6_FLOW: c_uchar = 8;
pub const AH_V4_FLOW: c_uchar = 9;
pub const ESP_V4_FLOW: c_uchar = 10;
pub const AH_V6_FLOW: c_uchar = 11;
pub const ESP_V6_FLOW: c_uchar = 12;
pub const IP_USER_FLOW: c_uchar = 13;
pub const IPV4_FLOW: c_uchar = 16;
pub const IPV6_FLOW: c_uchar = 17;
pub const ETHER_FLOW: c_uchar = 18;
