// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use std::num::NonZeroU8;
use std::num::NonZeroU16;
use std::num::NonZeroU32;
#[cfg(target_pointer_width = "64")] use std::num::NonZeroU64;
use std::ops::RangeInclusive;
use std::ops::RangeFrom;
use std::ops::RangeFull;
use std::ops::Range;
use std::ops::RangeTo;
use std::ops::RangeToInclusive;
use std::slice::SliceIndex;
use std::str::from_utf8_unchecked;
use std::str::from_utf8_unchecked_mut;


include!("AsUsizeIndex.rs");
include!("AsUsizeRange.rs");
include!("GetUnchecked.rs");
