// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Simplification of choices of features for common sets (eg those used by ethtool).
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum FeatureGroupChoice
{
	/// Ethtool setting is `sg` or `scatter-gather`.
	ethtool_sg,
	
	/// Ethtool setting is `tx` or `tx-checksumming`.
	ethtool_tx,
	
	/// Ethtool setting is `txvlan` or `tx-vlan-offload`.
	ethtool_txvlan,
	
	/// Ethtool setting is `rxvlan` or `rx-vlan-offload`.
	ethtool_rxvlan,
	
	/// Ethtool setting is `gso` or `generic-segmentation-offload`.
	ethtool_gso,
	
	/// Ethtool setting is `gro` or `generic-receive-offload`.
	ethtool_gro,
	
	/// Ethtool setting is `lro` or `large-receive-offload`.
	ethtool_lro,
	
	/// Ethtool setting is `tso` or `tcp-segmentation-offload`.
	ethtool_tso,
	
	/// Ethtool setting is `ntuple` or `ntuple-filters`.
	ethtool_ntuple,
	
	/// Ethtool setting is `rxhash` or `receive-hashing`.
	ethtool_rxhash,
	
	/// Ethtool setting is `rx` or `rx-checksumming`.
	ethtool_rx,

	/// IP checksums.
	internet_protocols_checksum,

	/// IP checksums in hardware.
	internet_protocols_checksum_in_hardware,

	/// Any combination not represented above.
	OtherToEnable(FeatureGroup),

	/// Any combination not represented above.
	OtherToDisable(FeatureGroup),
}

impl Debug for FeatureGroupChoice
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "FeatureGroupChoice")
	}
}

impl Clone for FeatureGroupChoice
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		use self::FeatureGroupChoice::*;
		
		match self
		{
			&ethtool_sg => ethtool_sg,
			&ethtool_tx => ethtool_tx,
			&ethtool_txvlan => ethtool_txvlan,
			&ethtool_rxvlan => ethtool_rxvlan,
			&ethtool_gso => ethtool_gso,
			&ethtool_gro => ethtool_gro,
			&ethtool_lro => ethtool_lro,
			&ethtool_tso => ethtool_tso,
			&ethtool_ntuple => ethtool_ntuple,
			&ethtool_rxhash => ethtool_rxhash,
			&ethtool_rx => ethtool_rx,
			&internet_protocols_checksum => internet_protocols_checksum,
			&internet_protocols_checksum_in_hardware => internet_protocols_checksum_in_hardware,
			&OtherToEnable(ref feature_group) => OtherToEnable(feature_group.clone()),
			&OtherToDisable(ref feature_group) => OtherToDisable(feature_group.clone()),
		}
	}
}

impl PartialEq for FeatureGroupChoice
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		use self::FeatureGroupChoice::*;
		
		match (self, other)
		{
			(&ethtool_sg, &ethtool_sg) => true,
			(&ethtool_tx, &ethtool_tx) => true,
			(&ethtool_txvlan, &ethtool_txvlan) => true,
			(&ethtool_rxvlan, &ethtool_rxvlan) => true,
			(&ethtool_gso, &ethtool_gso) => true,
			(&ethtool_gro, &ethtool_gro) => true,
			(&ethtool_lro, &ethtool_lro) => true,
			(&ethtool_tso, &ethtool_tso) => true,
			(&ethtool_ntuple, &ethtool_ntuple) => true,
			(&ethtool_rxhash, &ethtool_rxhash) => true,
			(&ethtool_rx, &ethtool_rx) => true,
			(&internet_protocols_checksum, &internet_protocols_checksum) => true,
			(&internet_protocols_checksum_in_hardware, &internet_protocols_checksum_in_hardware) => true,
			(&OtherToEnable(ref left_feature_group), &OtherToEnable(ref right_feature_group)) => left_feature_group.eq(right_feature_group),
			(&OtherToDisable(ref left_feature_group), &OtherToDisable(ref right_feature_group)) => left_feature_group.eq(right_feature_group),
			_ => false,
		}
	}
}

impl Eq for FeatureGroupChoice
{
}

impl FeatureGroupChoice
{
	#[inline(always)]
	pub(crate) fn iter(feature_group_choices: &Vec<Self>) -> impl Iterator<Item=HashMap<NETIF_F, bool>> + '_
	{
		feature_group_choices.iter().map(|feature_group_choice| feature_group_choice.to_feature_settings())
	}
	
	#[inline(always)]
	pub(crate) fn to_feature_settings(&self) -> HashMap<NETIF_F, bool>
	{
		use self::FeatureGroupChoice::*;
		
		match self
		{
			ethtool_sg => FeatureGroup::ethtool_sg().enable(),
			ethtool_tx => FeatureGroup::ethtool_tx().enable(),
			ethtool_txvlan => FeatureGroup::ethtool_txvlan().enable(),
			ethtool_rxvlan => FeatureGroup::ethtool_rxvlan().enable(),
			ethtool_gso => FeatureGroup::ethtool_gso().enable(),
			ethtool_gro => FeatureGroup::ethtool_gro().enable(),
			ethtool_lro => FeatureGroup::ethtool_lro().enable(),
			ethtool_tso => FeatureGroup::ethtool_tso().enable(),
			ethtool_ntuple => FeatureGroup::ethtool_ntuple().enable(),
			ethtool_rxhash => FeatureGroup::ethtool_rxhash().enable(),
			ethtool_rx => FeatureGroup::ethtool_rx().enable(),
			internet_protocols_checksum => FeatureGroup::internet_protocols_checksum().enable(),
			internet_protocols_checksum_in_hardware => FeatureGroup::internet_protocols_checksum_in_hardware().enable(),
			OtherToEnable(ref feature_group) => feature_group.enable(),
			OtherToDisable(ref feature_group) => feature_group.disable(),
		}
	}
}
