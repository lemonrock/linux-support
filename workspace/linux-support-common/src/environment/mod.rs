// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::strings::ConstCStr;
use errno::errno;
use libc::*;
use likely::likely;
use likely::unlikely;
use std::collections::BTreeSet;
use std::env::join_paths;
use std::env::set_var;
use std::ffi::CStr;
use std::ffi::CString;
use std::path::PathBuf;


include!("clearenv_wrapper.rs");
include!("populate_clean_environment.rs");
include!("setenv_wrapper.rs");
