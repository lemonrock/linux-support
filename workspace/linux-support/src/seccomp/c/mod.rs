// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use crate::VariablySized;
use super::UserNotificationFlags;
use crate::process::ProcessIdentifier;
use crate::signals::AuditArchitecture;


include!("PR_SET_SECCOMP.rs");
include!("SECCOMP_.rs");
include!("seccomp_data.rs");
include!("SECCOMP_FILTER_FLAG_.rs");
include!("SECCOMP_IOCTL_NOTIF_.rs");
include!("SECCOMP_MODE_.rs");
include!("seccomp_notif.rs");
include!("seccomp_notif_resp.rs");
include!("seccomp_notif_sizes.rs");
include!("SECCOMP_RET_.rs");
include!("SECCOMP_USER_NOTIF_FLAG_.rs");
