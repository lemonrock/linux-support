// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use self::c::*;
use crate::paths::ProcPath;
use bitflags::bitflags;
use libc::ST_MANDLOCK;
use libc::ST_NOATIME;
use libc::ST_NODIRATIME;
use libc::ST_NODEV;
use libc::ST_NOEXEC;
use libc::ST_NOSUID;
use libc::ST_RDONLY;
use libc::ST_SYNCHRONOUS;

use libc::statvfs;
use likely::likely;
use std::collections::HashMap;
use std::error;
use std::ffi::CStr;
use std::ffi::CString;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::ErrorKind;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;


mod c;


include!("FileSystemMetadata.rs");
include!("FileSystemMountFlags.rs");
include!("FileSystemMountIdentifier.rs");
include!("FileSystemSupportedError.rs");
include!("FileSystemType.rs");
include!("FileSystemTypeList.rs");
include!("HasNoAssociatedDevice.rs");
