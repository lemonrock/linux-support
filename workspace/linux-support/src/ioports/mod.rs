// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use self::c::*;
use errno::errno;
use likely::likely;
use libc::EINVAL;
use libc::ENOMEM;
use libc::ENOSYS;
use libc::EPERM;
use std::ops::RangeInclusive;


mod c;


include!("remove_ioport_privileges.rs");
include!("remove_ioport_permissions.rs");
include!("set_ioport_permissions.rs");
