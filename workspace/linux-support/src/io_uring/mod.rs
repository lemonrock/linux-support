// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use self::c::*;
use crate::cpu::HyperThread;
use crate::file_descriptors::eventfd::EventFileDescriptor;
use crate::file_descriptors::memfd::MemoryFileDescriptor;
use errno::errno;
use libc::c_void;
use libc::EAGAIN;
use libc::EBADF;
use libc::EBUSY;
use libc::EINTR;
use libc::EFAULT;
use libc::EINVAL;
use libc::ENOMEM;
use libc::ENXIO;
use libc::EOPNOTSUPP;
use libc::EOVERFLOW;
use libc::sigset_t;
use likely::*;
use std::collections::BTreeSet;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fs::File;
use std::io;
use std::mem::transmute;
#[allow(deprecated)] use std::mem::uninitialized;
use std::mem::zeroed;
use std::num::NonZeroU16;
use std::num::NonZeroU32;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::RawFd;
use std::ptr::NonNull;
use std::ptr::null_mut;


mod c;


include!("EnterError.rs");
include!("IoUringFileDescriptor.rs");
include!("PersonalityCredentialsIdentifier.rs");
include!("SupportedFileDescriptor.rs");
