// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use crate::file_systems::FileSystemType;
use crate::paths::PathExt;
use crate::paths::ProcPath;
use crate::process::ProcessIdentifierChoice;
use crate::strings::c_string_pointer_to_path_buf;
use crate::strings::ConstCStr;
use bitflags::bitflags;
use errno::errno;
use libc::*;
use libc_extra::android_linux::mntent::endmntent;
use libc_extra::android_linux::mntent::getmntent;
use libc_extra::android_linux::mntent::mntent;
use libc_extra::android_linux::mntent::setmntent;
use likely::unlikely;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::ffi::CStr;
use std::ffi::CString;
use std::io;
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;


include!("Mount.rs");
include!("mount_wrapper.rs");
include!("MountFlags.rs");
include!("Mounts.rs");
include!("MountsWrapper.rs");
include!("UnmountFlags.rs");
