// This file is part of libc-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT. No part of libc-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of libc-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT.


pub const SECCOMP_RET_KILL: c_uint = 0;
pub const SECCOMP_RET_DATA: c_uint = 65535;
pub const SECCOMP_RET_TRAP: c_uint = 196608;
pub const SECCOMP_RET_ERRNO: c_uint = 327680;
pub const SECCOMP_RET_ALLOW: c_uint = 2147418112;
pub const SECCOMP_RET_TRACE: c_uint = 2146435072;
pub const SECCOMP_RET_ACTION: c_uint = 2147418112;
