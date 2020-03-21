// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use errno::errno;
use likely::likely;
use likely::unlikely;
use libc::*;
use serde::Deserialize;
use serde::Serialize;
use std::convert::TryInto;
use std::ffi::CStr;
use std::ffi::CString;
use std::ptr::NonNull;


include!("assert_effective_user_id_is_root.rs");
include!("initgroups_wrapper.rs");
include!("OriginalRealUserAndGroupIdentifierUser.rs");
include!("setresgid_wrapper.rs");
include!("setresuid_wrapper.rs");
include!("UserAndGroupSettings.rs");
