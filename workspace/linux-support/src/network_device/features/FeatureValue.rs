// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) struct FeatureValue
{
	/// Feature requested but not necessarily active.
	pub(crate) requested: bool,
	
	/// Feature active.
	pub(crate) active: bool,
}

impl FeatureValue
{
	/// Returns `true` if the feature was added to `set_features`.
	#[inline(always)]
	pub(crate) fn update_if_desired(&self, feature: NETIF_F, set_features: &mut VariablySizedEthtoolCommandWrapper<ethtool_sfeatures>, desired_value: bool) -> bool
	{
		match (self.requested, self.active, desired_value)
		{
			(true, true, true) => return false,
			
			(true, true, false) => (),
			
			(true, false, true) => return false,
			
			(true, false, false) => (),
			
			(false, true, true) => return false,
			
			(false, true, false) => (),
			
			(false, false, true) => (),
			
			(false, false, false) => return false,
		}
		
		set_features.set_to_be_changed(feature, desired_value);
		true
	}
}
