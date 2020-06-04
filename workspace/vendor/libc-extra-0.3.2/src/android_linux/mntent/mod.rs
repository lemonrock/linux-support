// This file is part of libc-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT. No part of libc-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of libc-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT.


#![allow(non_camel_case_types)]


use ::libc::c_char;
use ::libc::c_int;
use ::libc::FILE;


include!("mntent.rs");


extern "C"
{
	pub fn addmntent(fp: *mut FILE, mnt: *const mntent) -> c_int;
	pub fn endmntent(fp: *mut FILE) -> c_int;
	pub fn getmntent(fp: *mut FILE) -> *mut mntent;
	pub fn getmntent_r(fp: *mut FILE, mntbuf: *mut mntent, buf: *mut c_char, buflen: c_int) -> *mut mntent;
	pub fn hasmntopt(fp: *const mntent, opt: *const c_char) -> *mut c_char;
	pub fn setmntent(filename: *const c_char, _type: *const c_char) -> *mut FILE;
}
