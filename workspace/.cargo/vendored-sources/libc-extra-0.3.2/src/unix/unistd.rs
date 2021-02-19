// This file is part of libc-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT. No part of libc-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of libc-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT.


use ::libc::c_int;
use ::libc::gid_t;


#[link(name = "c")]
extern "C"
{
	pub fn setegid(egid: gid_t) -> c_int;
	
	pub fn getpagesize() -> c_int;
}