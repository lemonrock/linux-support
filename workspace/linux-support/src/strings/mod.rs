// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(deprecated)] use std::mem::uninitialized;
use crate::strings::parse_number::ParseNumber;
use crate::strings::parse_number::ParseNumberError;
use libc::*;
use likely::unlikely;
use self::to_number::NumberAsBytes;
use std::borrow::Cow;
use std::cmp::min;
use std::error;
use std::ffi::CStr;
use std::ffi::CString;
use std::ffi::OsStr;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::path::PathBuf;


include!("c_string_pointer_to_path_buf.rs");
include!("ConstCStr.rs");
include!("FromBytes.rs");
include!("IntoLineFeedTerminatedByteString.rs");
include!("NonNumericDigitCase.rs");
include!("OsStrExtMore.rs");
include!("path_to_cstring.rs");
include!("Radix.rs");
include!("replace.rs");
include!("to_c_string_robustly.rs");


/// Conversions to numbers.
pub mod to_number;


/// Conversions from numbers.
pub mod parse_number;
