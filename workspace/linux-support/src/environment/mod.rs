// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use self::c::*;
use super::strings::ConstCStr;
use crate::process::ProcessIdentifierChoice;
use crate::paths::PathExt;
use crate::paths::ProcPath;
use crate::strings::{FromBytes, NulTerminatedCStringArray, CStringFragments, path_to_cstring};
use crate::strings::parse_ascii_nul_string_values;
use errno::errno;
use libc::c_char;
use libc::EINVAL;
use libc::ENOMEM;
use libc::clearenv;
use libc::setenv;
use likely::likely;
use likely::unlikely;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::env::{join_paths, JoinPathsError};
use std::env::set_var;
use std::ffi::CStr;
use std::ffi::CString;
use std::io;
use std::io::ErrorKind;
use std::ops::Deref;
use std::path::{PathBuf, Path};
use crate::user_and_groups::UserName;


mod c;


include!("clearenv_wrapper.rs");
include!("CommandLine.rs");
include!("Environment.rs");
include!("populate_clean_environment.rs");
include!("setenv_wrapper.rs");
