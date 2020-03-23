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
use libc::SOCK_DGRAM;
use libc::socket;
use libc_extra::android_linux::linux::ethtool::*;
use libc_extra::android_linux::linux::sockios::SIOCETHTOOL;
use libc_extra::android_linux::net::if_::ifreq;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeSet;
use std::convert::TryFrom;
use std::error;
use std::io;
use std::ffi::{CStr, CString, NulError};
use std::ffi::FromBytesWithNulError;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::mem::transmute;
use std::num::*;
use std::os::unix::io::RawFd;
use std::path::PathBuf;
use std::ptr::write;
use std::str::Utf8Error;
use std::ops::Deref;


/// Classifications.
pub mod classification;


/// Registers.
pub mod registers;


include!("ConvertNetworkInterfaceIndexToPciDeviceAddressError.rs");
include!("LinuxPciUserspaceKernelDriverModule.rs");
include!("NetworkInterfaceIndex.rs");
include!("NetworkInterfaceName.rs");
include!("NetworkInterfaceNameToIndexConversionError.rs");
include!("NetworkInterfaceNameToPciDeviceAddressConversionError.rs");
include!("PciDevice.rs");
include!("PciDeviceAddress.rs");
include!("PciDeviceAddressStringParseError.rs");
