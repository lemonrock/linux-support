// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use errno::errno;
use libc::c_void;
use libc::AF_INET;
use libc::close;
use libc::EACCES;
use libc::EAFNOSUPPORT;
use libc::EMFILE;
use libc::ENFILE;
use libc::ENOBUFS;
use libc::ENOMEM;
use libc::EPROTOTYPE;
use libc::EPROTONOSUPPORT;
use libc::ioctl;
use libc::IPPROTO_IP;
use libc::SOCK_DGRAM;
use libc::socket;
use libc_extra::android_linux::linux::ethtool::*;
use libc_extra::android_linux::linux::sockios::SIOCETHTOOL;
use libc_extra::android_linux::net::if_::ifreq;
use serde::Deserialize;
use serde::Serialize;
use std::error;
use std::ffi::{CStr, FromBytesWithNulError};
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::mem::transmute;
use std::os::unix::io::RawFd;
use std::ptr::write;
use std::convert::TryFrom;
use std::num::ParseIntError;
use std::str::Utf8Error;


/// Classifications
pub mod classification;


include!("OpenPciExpressBusInformationError.rs");
include!("PciDeviceAddress.rs");
include!("PciDeviceAddressStringParseError.rs");
include!("PciExpressBusInformation.rs");


