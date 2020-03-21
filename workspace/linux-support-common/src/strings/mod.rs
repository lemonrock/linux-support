// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use libc::*;
use likely::unlikely;
use std::ffi::CStr;
use std::ffi::CString;
use std::ffi::OsStr;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::path::PathBuf;


include!("c_string_pointer_to_path_buf.rs");
include!("ConstCStr.rs");
include!("OsStrExtMore.rs");
include!("path_to_cstring.rs");
include!("replace.rs");
include!("split.rs");
include!("splitn.rs");
include!("to_c_string_robustly.rs");
