// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) const PR_SPEC_STORE_BYPASS: usize = 0;

pub(crate) const PR_SPEC_INDIRECT_BRANCH: usize = 1;

#[allow(dead_code)]
pub(crate) const PR_SPEC_NOT_AFFECTED: i32 = 0;

pub(crate) const PR_SPEC_PRCTL: i32 = 1 << 0;

pub(crate) const PR_SPEC_ENABLE: i32 = 1 << 1;

pub(crate) const PR_SPEC_DISABLE: i32 = 1 << 2;

pub(crate) const PR_SPEC_FORCE_DISABLE: i32 = 1 << 3;

pub(crate) const PR_SPEC_DISABLE_NOEXEC: i32 = 1 << 4;
