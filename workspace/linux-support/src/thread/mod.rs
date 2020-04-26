// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2019 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use crate::paths::*;
use crate::process::*;
use crate::strings::parse_number::*;
use crate::syscall::SYS::gettid;
use arrayvec::ArrayVec;
use libc::c_char;
use libc::pid_t;
use libc::PR_GET_NAME;
use libc::PR_SET_NAME;
use libc::prctl;
use likely::likely;
use memchr::memchr;
use serde::Deserialize;
use serde::Serialize;
use std::convert::TryFrom;
use std::io;
use std::num::NonZeroI32;
use std::ops::Deref;
use std::slice::from_raw_parts;
use std::ffi::CStr;


include!("ThreadIdentifier.rs");
include!("ThreadIdentifierChoice.rs");
include!("ThreadName.rs");
