// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use crate::syscall::SYS;


include!("AT_STATX_.rs");
include!("dirent.rs");
include!("execveat.rs");
include!("getdents.rs");
include!("MAX_HANDLE_SZ.rs");
include!("file_handle.rs");
include!("name_to_handle_at.rs");
include!("open_by_handle_at.rs");
include!("open_how.rs");
include!("openat2.rs");
include!("renameat2.rs");
include!("statx.rs");
include!("STATX_.rs");
include!("STATX_ATTR_.rs");
include!("statx_timestamp.rs");
