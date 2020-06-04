// This file is part of libc-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT. No part of libc-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of libc-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT.


use ::libc::c_int;


pub const FIOSETOWN: c_int = 0x8901;
pub const SIOCSPGRP: c_int = 0x8902;
pub const FIOGETOWN: c_int = 0x8903;
pub const SIOCGPGRP: c_int = 0x8904;
pub const SIOCATMARK: c_int = 0x8905;
pub const SIOCGSTAMP: c_int = 0x8906;
pub const SIOCGSTAMPNS: c_int = 0x8907;
