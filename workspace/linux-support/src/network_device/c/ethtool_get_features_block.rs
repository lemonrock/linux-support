// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A block of 32 features.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct ethtool_get_features_block
{
	/// Mask of changeable features.
	///
	/// If a feature is in this bit set, then it is supported by the driver.
	/// If a feature is not in this bit then it is unsupported; trying to set it on or off we cause an error.
	pub(crate) available: BitSetWord,
	
	/// Mask of features requested to be enabled if possible.
	pub(crate) requested: BitSetWord,
	
	/// Mask of currently enabled features.
	pub(crate) active: BitSetWord,
	
	/// Mask of features not changeable.
	///
	/// If a feature is in this bit set, then it can not be changed on or off; trying to do so will cause an error.
	pub(crate) never_changed: BitSetWord,
}
