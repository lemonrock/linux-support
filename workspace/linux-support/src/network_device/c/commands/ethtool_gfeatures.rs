// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Command to get state of device's features
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub(crate) struct ethtool_gfeatures
{
	/// Always `ETHTOOL_GFEATURES`.
	cmd: u32,
	
	/// On entry, the number of elements in the `features[]` array.
	/// On return, the number of elements in `features[]` needed to hold all features (`NETIF_F::ETHTOOL_DEV_FEATURE_WORDS`).
	///
	/// If `size` is not large enough, `EFAULT` is returned.
	size: u32,
	
	/// State of features.
	features: __IncompleteArrayField<ethtool_get_features_block>,
}

impl EthtoolCommand for ethtool_gfeatures
{
	#[inline(always)]
	fn command(&self) -> u32
	{
		self.cmd
	}
}

impl VariablySizedEthtoolCommand for ethtool_gfeatures
{
	type ArrayElement = ethtool_get_features_block;
	
	#[inline(always)]
	fn array_length(&self) -> u32
	{
		self.size
	}
}

/*

#define FEATURE_FIELD_FLAG(index)		(1U << (index) % 32U)

#define FEATURE_WORD(blocks, index, field)	((blocks)[(index) / 32U].field)

#define FEATURE_BIT_IS_SET(blocks, index, field)		\
	(FEATURE_WORD(blocks, index, field) & FEATURE_FIELD_FLAG(index))
	
	
	we can not change a feature if:-
		- it is not in the 'available' set;
		- it is in the 'never_changed' set.
	
	
	let i = 0;
	while i < off_flag_def.len()
	{
		let mut fixed = 1;
		let feature_bit = 0;
		while feature_bit < defs.n_features
		{
			if defs.def[feature_bit].off_flag_index != i
			{
				continue
			}
			if old_state.features.features.available_is_not_set(feature_bit)
			{
				continue
			}
			if old_state.features.features.never_changed_is_set(feature_bit)
			{
				continue
			}
			
			fixed = 0;
			
			if efeatures.features.valid.is_not_set(feature_bit)
			{
				efeatures.features.valid.set(feature_bit);
				
				if off_flags_wanted & off_flag_def[i].value
				{
					efeatures.features.requested.set(feature_bit)
				}
			}
			
			feature_bit += 1
		}
		
		if fixed
		{
			warn("Cannot change {}", off_flag_def[i].long_name)
		}
 */

impl VariablySizedEthtoolCommandWrapper<ethtool_gfeatures>
{
	/// Returns `true` if one or more features were added to `set_features`.
	pub(crate) fn change_features_if_possible(&self, features_to_change: &HashMap<NETIF_F, bool>, set_features: &mut VariablySizedEthtoolCommandWrapper<ethtool_sfeatures>) -> bool
	{
		let mut have_added_to_set_features = false;
		for (feature, desired_value) in features_to_change
		{
			let added_to_set_features = self.change_feature_if_possible(*feature, set_features, *desired_value);
			have_added_to_set_features |= added_to_set_features;
		}
		have_added_to_set_features
	}
	
	/// Returns `true` if the feature was added to `set_features`.
	pub(crate) fn change_feature_if_possible(&self, feature: NETIF_F, set_features: &mut VariablySizedEthtoolCommandWrapper<ethtool_sfeatures>, desired_value: bool) -> bool
	{
		if let Some(feature_value) = self.feature_is_available_and_can_be_changed_with_value(feature)
		{
			feature_value.update_if_desired(feature, set_features, desired_value)
		}
		else
		{
			false
		}
	}
	
	fn feature_is_available_and_can_be_changed_with_value(&self, feature: NETIF_F) -> Option<FeatureValue>
	{
		if self.feature_is_available_and_can_be_changed(feature)
		{
			Some
			(
				FeatureValue
				{
					requested: self.requested_is_set(feature),
					active: self.active_is_set(feature),
				}
			)
		}
		else
		{
			None
		}
	}
	
	#[inline(always)]
	fn feature_is_available_and_can_be_changed(&self, feature: NETIF_F) -> bool
	{
		self.available_is_set(feature) && !self.never_changed_is_set(feature)
	}
	
	#[inline(always)]
	pub(crate) fn available_is_set(&self, feature: NETIF_F) -> bool
	{
		feature.is_set_field_locator(self.array_elements(), |ethtool_get_features_block| ethtool_get_features_block.available)
	}
	
	#[inline(always)]
	pub(crate) fn requested_is_set(&self, feature: NETIF_F) -> bool
	{
		feature.is_set_field_locator(self.array_elements(), |ethtool_get_features_block| ethtool_get_features_block.requested)
	}
	
	#[inline(always)]
	pub(crate) fn active_is_set(&self, feature: NETIF_F) -> bool
	{
		feature.is_set_field_locator(self.array_elements(), |ethtool_get_features_block| ethtool_get_features_block.active)
	}
	
	#[inline(always)]
	pub(crate) fn never_changed_is_set(&self, feature: NETIF_F) -> bool
	{
		feature.is_set_field_locator(self.array_elements(), |ethtool_get_features_block| ethtool_get_features_block.never_changed)
	}
}

impl ethtool_gfeatures
{
	#[inline(always)]
	pub(crate) fn new() -> VariablySizedEthtoolCommandWrapper<Self>
	{
		Self::new_with_initialized_header_but_uninitialized_array
		(
			Self
			{
				cmd: ETHTOOL_GFEATURES,
				size: NETIF_F::SafeSize as u32,
				features: __IncompleteArrayField::new(),
			}
		)
	}
}
