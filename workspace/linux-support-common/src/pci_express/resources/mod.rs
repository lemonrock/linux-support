// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use crate::pci_express::PciDevice;
use errno::errno;
use libc::c_void;
use libc::MAP_FAILED;
use libc::MAP_SHARED;
use libc::mmap;
use libc::munmap;
use libc::PROT_READ;
use libc::PROT_WRITE;
use likely::*;
use std::collections::BTreeMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::ErrorKind;
use std::io::BufReader;
use std::io::BufRead;
use std::ops::Deref;
use std::os::unix::io::AsRawFd;
use std::ptr::NonNull;
use std::ptr::null_mut;
use std::str::from_utf8;


include!("MemoryMappedResource.rs");
include!("ResourceEntry.rs");
include!("Resources.rs");
