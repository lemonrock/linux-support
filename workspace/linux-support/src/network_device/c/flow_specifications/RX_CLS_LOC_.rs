// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


// Special RX classification rule insert location values.

pub(crate) const RX_CLS_LOC_SPECIAL: u32 = 0x80000000;

pub(crate) const RX_CLS_LOC_ANY: u32 = 0xFFFFFFFF;

pub(crate) const RX_CLS_LOC_FIRST: u32 = 0xFFFFFFFE;

pub(crate) const RX_CLS_LOC_LAST: u32 = 0xFFFFFFFD;
