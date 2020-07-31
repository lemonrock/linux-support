// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Command to request change in device's features.
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub(crate) struct ethtool_sfeatures
{
	/// Always `ETHTOOL_SFEATURES`.
	cmd: u32,
	
	/// The number of elements in the `features[]` array.
	///
	/// Internally in Linux this size is `ETHTOOL_DEV_FEATURE_WORDS`.
	pub(crate) size: u32,
	
	/// Feature change masks.
	features: __IncompleteArrayField<ethtool_set_features_block>,
}

impl EthtoolCommand for ethtool_sfeatures
{
	#[inline(always)]
	fn command(&self) -> u32
	{
		self.cmd
	}
}

impl VariablySizedEthtoolCommand for ethtool_sfeatures
{
	type ArrayElement = ethtool_set_features_block;
	
	#[inline(always)]
	fn array_length(&self) -> u32
	{
		self.size
	}
}

impl VariablySizedEthtoolCommandWrapper<ethtool_sfeatures>
{
	#[inline(always)]
	pub(crate) fn valid_is_set(&self, feature: NETIF_F) -> bool
	{
		feature.is_set_field_locator(self.array_elements(), |ethtool_set_features_block| ethtool_set_features_block.valid)
	}
	
	#[inline(always)]
	pub(crate) fn requested_is_set(&self, feature: NETIF_F) -> bool
	{
		feature.is_set_field_locator(self.array_elements(), |ethtool_set_features_block| ethtool_set_features_block.requested)
	}
	
	#[inline(always)]
	pub(crate) fn set_to_be_changed(&mut self, feature: NETIF_F, value: bool)
	{
		feature.set_field_locator(self.array_elements_mut(), |ethtool_set_features_block| &mut ethtool_set_features_block.valid);
		if value
		{
			feature.set_field_locator(self.array_elements_mut(), |ethtool_set_features_block| &mut ethtool_set_features_block.requested);
		}
		else
		{
			feature.unset_field_locator(self.array_elements_mut(), |ethtool_set_features_block| &mut ethtool_set_features_block.requested);
		}
	}
}

impl ethtool_sfeatures
{
	#[inline(always)]
	pub(crate) fn new() -> VariablySizedEthtoolCommandWrapper<Self>
	{
		let array_size = NETIF_F::ETHTOOL_DEV_FEATURE_WORDS + 2;
		
		Self::new_with_initialized_header_but_uninitialized_array
		(
			Self
			{
				cmd: ETHTOOL_SFEATURES,
				size: NETIF_F::SafeSize as u32,
				features: __IncompleteArrayField::new(),
			}
		)
	}
}
