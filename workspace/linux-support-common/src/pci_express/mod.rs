// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use errno::errno;
use libc::c_void;
use libc::AF_INET;
use libc::close;
use libc::EACCES;
use libc::EAFNOSUPPORT;
use libc::EBADF;
use libc::EINTR;
use libc::EIO;
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
use std::error;
use std::ffi::CStr;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::mem::transmute;
use std::os::unix::io::RawFd;
use std::ptr::write;


include!("OpenPciBusInformationError.rs");
include!("PciExpressBusInformation.rs");


