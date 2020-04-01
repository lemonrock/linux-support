// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


use super::*;
use self::syscall::*;
use libc::AT_FDCWD;
use libc::c_uchar;
use libc::c_ushort;
use libc::c_ulonglong;
use libc::c_uint;
use libc::O_APPEND;
use libc::O_DSYNC;
use libc::O_LARGEFILE;
use libc::O_NOATIME;
use libc::O_SYNC;


mod syscall;


include!("EventFlags.rs");
include!("FanotifyFileDescriptor.rs");
include!("FanotifyMarkError.rs");
include!("FanotifyReadOrWrite.rs");
include!("FileStatusFlags.rs");
include!("MarkEventFlags.rs");
include!("MarkFlags.rs");
include!("MarkPath.rs");
