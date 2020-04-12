// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use crate::file_descriptors::CreationError;
use super::*;
use super::huge_pages::*;
use errno::errno;
use libc::c_void;
use libc::EACCES;
use libc::EAGAIN;
use libc::EBADF;
use libc::EEXIST;
use libc::EINVAL;
use libc::ENFILE;
use libc::ENODEV;
use libc::ENOMEM;
use libc::EOVERFLOW;
use libc::EPERM;
use libc::ETXTBSY;
use libc::MAP_ANONYMOUS;
use libc::MAP_FAILED;
use libc::MAP_FIXED;
use libc::MAP_HUGETLB;
use libc::MAP_NORESERVE;
use libc::MAP_POPULATE;
use libc::MAP_PRIVATE;
use libc::mmap;
use libc::munmap;
use libc::PROT_EXEC;
use libc::PROT_NONE;
use libc::PROT_READ;
use libc::PROT_WRITE;
use likely::*;
use std::borrow::Borrow;
use std::convert::TryInto;
use std::fs::OpenOptions;
use std::fs::File;
use std::fmt::Debug;
use std::num::NonZeroU64;
use std::ops::Deref;
use std::ops::DerefMut;
use std::os::unix::io::AsRawFd;
use std::ptr::null_mut;
use std::ptr::read_volatile;
use std::ptr::write_volatile;
use std::slice::from_raw_parts;
use std::slice::from_raw_parts_mut;


include!("AddressHint.rs");
include!("MAP_32BIT.rs");
include!("MAP_FIXED_NOREPLACE.rs");
include!("MAP_SHARED_VALIDATE.rs");
include!("MAP_SYNC.rs");
include!("MappedMemory.rs");
include!("Protection.rs");
include!("Sharing.rs");
