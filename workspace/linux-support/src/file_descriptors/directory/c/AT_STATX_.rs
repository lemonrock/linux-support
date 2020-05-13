// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) const AT_STATX_SYNC_AS_STAT: c_int = 0x0000;
pub(crate) const AT_STATX_FORCE_SYNC: c_int = 0x2000;
pub(crate) const AT_STATX_DONT_SYNC: c_int = 0x4000;

/// Used as a mask for the above values.
#[allow(dead_code)] pub(super) const AT_STATX_SYNC_TYPE: c_int = 0x6000;
