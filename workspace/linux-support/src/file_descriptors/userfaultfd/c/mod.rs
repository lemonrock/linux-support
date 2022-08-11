// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use crate::ioctl::_IOWR;
use crate::ioctl::_IOR;
use std::os::unix::io::RawFd;
use crate::thread::ThreadIdentifier;


include!("_UFFDIO.rs");
include!("UFFD_API_FEATURES.rs");
include!("UFFDIO.rs");
include!("UFFDIO_.rs");
include!("UFFD_EVENT.rs");
include!("UFFD_FEATURE.rs");
include!("UFFD_PAGEFAULT.rs");
include!("UFFD_COPY_MODE.rs");
include!("UFFD_REGISTER_MODE.rs");
include!("UFFD_ZEROPAGE_MODE.rs");
include!("UFFD_USER_MODE_ONLY.rs");
include!("UFFD_WRITEPROTECT_MODE.rs");
include!("uffd_msg.rs");
include!("uffd_msg_arg.rs");
include!("uffd_msg_arg_fork.rs");
include!("uffd_msg_arg_pagefault.rs");
include!("uffd_msg_arg_pagefault_feat.rs");
include!("uffd_msg_arg_remap.rs");
include!("uffd_msg_arg_remove.rs");
include!("uffd_msg_arg_reserved.rs");
include!("uffdio_api.rs");
include!("uffdio_copy.rs");
include!("uffdio_range.rs");
include!("uffdio_register.rs");
include!("uffdio_writeprotect.rs");
include!("uffdio_zeropage.rs");
