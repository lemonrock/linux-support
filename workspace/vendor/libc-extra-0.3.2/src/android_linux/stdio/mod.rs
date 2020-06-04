// This file is part of libc-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT. No part of libc-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of libc-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT.


use ::libc::c_char;
use ::libc::c_int;
use ::libc::c_void;
use ::libc::FILE;
use ::libc::off_t;
use ::libc::size_t;
use ::libc::ssize_t;


include!("cookie_close_function_t.rs");
include!("cookie_io_functions_t.rs");
include!("cookie_read_function_t.rs");
include!("cookie_seek_function_t.rs");
include!("cookie_write_function_t.rs");
include!("fopencookie.rs");
