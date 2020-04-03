// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use crate::bit_set::BitSet;
use crate::cpu::*;
use crate::linux_kernel_modules::*;
use crate::file_descriptors::RawFdExt;
use crate::memory::numa::NumaNode;
use crate::paths::*;
use crate::strings::*;
use crate::strings::parse_number::*;
use crate::user_and_groups::assert_effective_user_id_is_root;
use self::classification::*;
use self::classification::subclasses::NetworkController;
use self::configuration_space::MemoryMappedConfigurationSpace;
use self::link::*;
use self::resources::Resources;
use either::Either;
use either::Either::Left;
use errno::errno;
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
use libc::strnlen;
use libc_extra::android_linux::linux::ethtool::*;
use libc_extra::android_linux::linux::sockios::SIOCETHTOOL;
use libc_extra::android_linux::net::if_::ifreq;
use likely::*;
use serde::Deserialize;
use serde::Serialize;
use std::convert::TryFrom;
use std::error;
use std::io;
use std::ffi::*;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::mem::transmute;
use std::num::*;
use std::ops::Deref;
use std::os::unix::io::*;
use std::path::PathBuf;
use std::str::Utf8Error;
use std::borrow::Cow;


/// Classifications.
pub mod classification;


/// Configuration Space.
pub mod configuration_space;


/// Definitions.
pub mod definitions;


/// Link.
pub mod link;


/// Registers.
pub mod registers;


/// Resources.
pub mod resources;


include!("ConvertNetworkInterfaceIndexToPciDeviceAddressError.rs");
include!("LinuxPciUserspaceKernelDriverModule.rs");
include!("NetworkInterfaceIndex.rs");
include!("NetworkInterfaceName.rs");
include!("NetworkInterfaceNameToIndexConversionError.rs");
include!("NetworkInterfaceNameToPciDeviceAddressConversionError.rs");
include!("PciDevice.rs");
include!("PciDeviceDetails.rs");
include!("PciDeviceAddress.rs");
include!("PciDeviceAddressStringParseError.rs");
include!("PciDevicePhysicalOrVirtualFunction.rs");
