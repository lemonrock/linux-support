// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use crate::paths::PathBufExt;
use crate::strings::into_line_feed_terminated_byte_string::*;
use super::inode::Inode;
use super::paths::PathExt;
use super::paths::ProcPath;
use crate::process::*;
use libc::*;
use likely::unlikely;
use std::collections::BTreeMap;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::io::Write;
use std::num::NonZeroU32;
use std::num::ParseIntError;
use std::ops::Deref;
use std::os::unix::io::IntoRawFd;
use std::os::unix::io::RawFd;
use std::path::Path;
use std::path::PathBuf;
use std::borrow::Cow;
use crate::user_and_groups::{UserOrGroupIdentifier, UserIdentifier, GroupIdentifier};


include!("Intervals.rs");
include!("NamespaceInodeParseError.rs");
include!("NamespacesProcPath.rs");
include!("setup_uts_namespace.rs");
include!("UserOrGroupIdentifierMap.rs");
include!("write_uid_and_gid_maps.rs");
