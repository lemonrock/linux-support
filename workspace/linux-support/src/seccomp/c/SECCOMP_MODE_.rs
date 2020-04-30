// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


// Valid values for `seccomp.mode` and `prctl(PR_SET_SECCOMP, <mode>)`.

/// seccomp is not in use.
pub(crate) const SECCOMP_MODE_DISABLED: u32 = 0;

/// uses hard-coded filter.
#[allow(unused)]
pub(crate) const SECCOMP_MODE_STRICT: u32 = 1;

/// uses user-supplied filter.
pub(crate) const SECCOMP_MODE_FILTER: u32 = 2;
