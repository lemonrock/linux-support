// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) const RWF_HIPRI: u32 = 0x00000001;
pub(crate) const RWF_DSYNC: u32 = 0x00000002;
pub(crate) const RWF_SYNC: u32 = 0x00000004;
pub(crate) const RWF_NOWAIT: u32 = 0x00000008;
pub(crate) const RWF_APPEND: u32 = 0x00000010;

#[allow(dead_code)]
pub(crate) const RWF_SUPPORTED: u32 = RWF_HIPRI | RWF_DSYNC | RWF_SYNC | RWF_NOWAIT | RWF_APPEND;
