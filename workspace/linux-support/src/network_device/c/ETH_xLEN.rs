// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Octets in one ethernet address.
pub(crate) const ETH_ALEN: usize = 6;

/// Octets in ethernet type field.
#[allow(dead_code)]
pub(crate) const ETH_TLEN: usize = 2;

/// Total octets in header.
#[allow(dead_code)]
pub(crate) const ETH_HLEN: usize = 14;

/// Minimum octets in frame without Frame Check Sequence (FCS) of 4 bytes.
#[allow(dead_code)]
pub(crate) const ETH_ZLEN: usize = 60;
