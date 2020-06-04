// This file is part of libc-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT. No part of libc-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of libc-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT.


use ::libc::c_ulong;
use ::libc::uint8_t;


pub const SECURE_NOROOT: uint8_t = 0;
pub const SECURE_NOROOT_LOCKED: uint8_t = 1;
pub const SECURE_NO_SETUID_FIXUP: uint8_t = 2;
pub const SECURE_NO_SETUID_FIXUP_LOCKED: uint8_t = 3;
pub const SECURE_KEEP_CAPS: uint8_t = 4;
pub const SECURE_KEEP_CAPS_LOCKED: uint8_t = 5;
pub const SECURE_NO_CAP_AMBIENT_RAISE: uint8_t = 6;
pub const SECURE_NO_CAP_AMBIENT_RAISE_LOCKED: uint8_t = 7;

pub const SECUREBITS_DEFAULT: c_ulong = 0;

pub const SECBIT_NOROOT: c_ulong = 1 << (SECURE_NOROOT as c_ulong);
pub const SECBIT_NOROOT_LOCKED: c_ulong = 1 << (SECURE_NOROOT_LOCKED as c_ulong);
pub const SECBIT_NO_SETUID_FIXUP: c_ulong = 1 << (SECURE_NO_SETUID_FIXUP as c_ulong);
pub const SECBIT_NO_SETUID_FIXUP_LOCKED: c_ulong = 1 << (SECURE_NO_SETUID_FIXUP_LOCKED as c_ulong);
pub const SECBIT_KEEP_CAPS: c_ulong = 1 << (SECURE_KEEP_CAPS as c_ulong);
pub const SECBIT_KEEP_CAPS_LOCKED: c_ulong = 1 << (SECURE_KEEP_CAPS_LOCKED as c_ulong);
pub const SECBIT_NO_CAP_AMBIENT_RAISE: c_ulong = 1 << (SECURE_NO_CAP_AMBIENT_RAISE as c_ulong);
pub const SECBIT_NO_CAP_AMBIENT_RAISE_LOCKED: c_ulong = 1 << (SECURE_NO_CAP_AMBIENT_RAISE_LOCKED as c_ulong);
