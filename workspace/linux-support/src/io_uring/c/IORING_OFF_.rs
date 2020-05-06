// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


// Magic offsets for the application to `mmap()` the data it needs.
pub(super) const IORING_OFF_SQ_RING: u64 = 0;
pub(super) const IORING_OFF_CQ_RING: u64 = 0x8000000;
pub(super) const IORING_OFF_SQES: u64 = 0x10000000;
