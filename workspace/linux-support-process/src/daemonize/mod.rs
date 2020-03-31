// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use errno::errno;
use libc::*;
use likely::likely;
use likely::unlikely;
use linux_support_common::get_program_name;
use linux_support_common::logging::redirect_standard_out_and_standard_error_to_syslog;
use linux_support_common::paths::{DevPath, PathBufExt};
use linux_support_common::paths::PathExt;
use linux_support_common::strings::OsStrExtMore;
use linux_support_common::user_and_groups::OriginalRealUserAndGroupIdentifierUser;
use serde::Deserialize;
use serde::Serialize;
use std::cell::UnsafeCell;
use std::ffi::CStr;
use std::fs::remove_file;
use std::io;
use std::os::unix::io::AsRawFd;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use linux_support_common::process::{UserIdentifier, GroupIdentifier};


include!("Daemonize.rs");
include!("DaemonizeCleanUpOnExit.rs");
