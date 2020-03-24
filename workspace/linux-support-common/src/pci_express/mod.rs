// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use crate::paths::*;
use crate::cpu::*;
use crate::linux_kernel_modules::*;
use crate::user_and_groups::assert_effective_user_id_is_root;
use self::classification::*;
use errno::errno;
use file_descriptors::RawFdExt;
use libc::c_void;
use libc::AF_INET;
use libc::EACCES;
use libc::EAFNOSUPPORT;
use libc::EBADF;
use libc::EFAULT;
use libc::EINVAL;
use libc::EMFILE;
use libc::ENFILE;
use libc::ENOBUFS;
use libc::ENOMEM;
use libc::ENOTTY;
use libc::EPROTOTYPE;
use libc::EPROTONOSUPPORT;
use libc::IF_NAMESIZE;
use libc::if_nametoindex;
use libc::ioctl;
use libc::IPPROTO_IP;
use libc::mmap;
use libc::MAP_FAILED;
use libc::MAP_SHARED;
use libc::munmap;
use libc::PROT_READ;
use libc::PROT_WRITE;
use libc::SOCK_DGRAM;
use libc::socket;
use libc_extra::android_linux::linux::ethtool::*;
use libc_extra::android_linux::linux::sockios::SIOCETHTOOL;
use libc_extra::android_linux::net::if_::ifreq;
use likely::likely;
use likely::unlikely;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeSet;
use std::convert::TryFrom;
use std::error;
use std::io;
use std::io::ErrorKind;
use std::ffi::*;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fs::*;
use std::mem::transmute;
use std::num::*;
use std::ops::Deref;
use std::os::unix::io::*;
use std::path::PathBuf;
use std::ptr::NonNull;
use std::ptr::null_mut;
use std::str::Utf8Error;
use std::borrow::Cow;


/// Classifications.
pub mod classification;


/// Definitions.
pub mod definitions;


/// Registers.
pub mod registers;


include!("ConvertNetworkInterfaceIndexToPciDeviceAddressError.rs");include!("LinuxPciUserspaceKernelDriverModule.rs");
include!("NetworkInterfaceIndex.rs");
include!("NetworkInterfaceName.rs");
include!("NetworkInterfaceNameToIndexConversionError.rs");
include!("NetworkInterfaceNameToPciDeviceAddressConversionError.rs");
include!("PciDevice.rs");
include!("PciDeviceAddress.rs");
include!("PciDeviceAddressStringParseError.rs");
include!("PciDevicePhysicalOrVirtualFunction.rs");
include!("PciResource.rs");
