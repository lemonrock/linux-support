// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use crate::process::status::StatusStatisticParseError;
use crate::strings::FromBytes;
use crate::strings::parse_number::*;
use errno::errno;
use likely::likely;
use likely::unlikely;
use libc::*;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeSet;
use std::ffi::CStr;
use std::ffi::CString;
#[allow(deprecated)] use std::mem::uninitialized;
use std::ops::Deref;
use std::ptr::NonNull;
use std::hash::Hash;


include!("assert_effective_user_id_is_root.rs");
include!("GroupIdentifier.rs");
include!("GroupIdentifiers.rs");
include!("Groups.rs");
include!("initgroups_wrapper.rs");
include!("OriginalRealUserAndGroupIdentifierUser.rs");
include!("setresgid_wrapper.rs");
include!("setresuid_wrapper.rs");
include!("UserAndGroupSettings.rs");
include!("UserOrGroupIdentifier.rs");
include!("UserIdentifier.rs");
include!("UserIdentifiers.rs");
