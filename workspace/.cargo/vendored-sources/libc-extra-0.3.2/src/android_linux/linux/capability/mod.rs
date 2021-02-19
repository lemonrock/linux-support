// This file is part of libc-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT. No part of libc-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of libc-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT.


#![allow(non_camel_case_types)]

use ::std::default::Default;
use ::std::mem::zeroed;
use ::libc::c_int;
use ::libc::c_longlong;
use ::libc::c_uchar;
use ::libc::c_uint;
use ::libc::size_t;


include!("constants/_LINUX_CAPABILITY.rs");
include!("constants/VFS_CAP.rs");
include!("constants/CAP.rs");
include!("task_struct.rs");
include!("__user_cap_header_struct.rs");
include!("cap_user_header_t.rs");
include!("__user_cap_data_struct.rs");
include!("cap_user_data_t.rs");
include!("vfs_cap_data.rs");
include!("Data.rs");
