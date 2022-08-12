// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Maximum octets in payload.
#[allow(dead_code)]
pub(crate) const ETH_DATA_LEN: usize = 1500;

/// Maximum octets in frame without Frame Check Sequence (FCS) of 4 bytes.
#[allow(dead_code)]
pub(crate) const ETH_FRAME_LEN: usize = 1514;

/// Octets in the Frame Check Sequence (FCS).
#[allow(dead_code)]
pub(crate) const ETH_FCS_LEN: usize = 4;
