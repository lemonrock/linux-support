// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


// Valid flags for `SECCOMP_SET_MODE_FILTER`.

/// Can not be used with `SECCOMP_FILTER_FLAG_NEW_LISTENER`.
pub(crate) const SECCOMP_FILTER_FLAG_TSYNC: u32 = 1 << 0;

pub(crate) const SECCOMP_FILTER_FLAG_LOG: u32 = 1 << 1;

pub(crate) const SECCOMP_FILTER_FLAG_SPEC_ALLOW: u32 = 1 << 2;

/// Can not be used with `SECCOMP_FILTER_FLAG_TSYNC` (or `SECCOMP_FILTER_FLAG_TSYNC_ESRCH`).
pub(crate) const SECCOMP_FILTER_FLAG_NEW_LISTENER: u32 = 1 << 3;

/// Can only be used with `SECCOMP_FILTER_FLAG_TSYNC`.
#[allow(dead_code)]
pub(crate) const SECCOMP_FILTER_FLAG_TSYNC_ESRCH: u32 = 1 << 4;
