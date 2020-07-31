// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A block of 32 features which are being requested.
#[derive(Debug, Copy, Clone)]
pub(crate) struct ethtool_set_features_block
{
	/// Mask of features to be changed.
	pub(crate) valid: u32,
	
	/// Values of features to be changed.
	pub(crate) requested: u32,
}
