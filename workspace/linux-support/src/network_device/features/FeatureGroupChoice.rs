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
	
	/// Generic Send Offload (GSO) encapsulation.
	generic_send_offload_encapsulation,

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
			&generic_send_offload_encapsulation => generic_send_offload_encapsulation,
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
			(&generic_send_offload_encapsulation, &generic_send_offload_encapsulation) => true,
			(&OtherToEnable(ref left_feature_group), &OtherToEnable(ref right_feature_group)) => left_feature_group.eq(right_feature_group),
			(&OtherToDisable(ref left_feature_group), &OtherToDisable(ref right_feature_group)) => left_feature_group.eq(right_feature_group),
			_ => false,
		}
	}
}

impl Eq for FeatureGroupChoice
{
}

impl<'a> Sub<Feature> for &'a FeatureGroupChoice
{
	type Output = FeatureGroupChoice;
	
	#[inline(always)]
	fn sub(self, rhs: Feature) -> Self::Output
	{
		use self::FeatureGroupChoice::*;
		
		#[inline(always)]
		fn non_static_lifetime(feature_group: &FeatureGroup, feature: &Feature, enable: bool) -> FeatureGroupChoice
		{
			let mut hash_set = feature_group.0.clone();
			hash_set.remove(feature);
			let feature_group = FeatureGroup::from(hash_set);
			if enable
			{
				OtherToEnable(feature_group)
			}
			else
			{
				OtherToDisable(feature_group)
			}
		}
		
		let feature_group = match &self
		{
			&ethtool_sg => FeatureGroup::ethtool_sg(),
			&ethtool_tx => FeatureGroup::ethtool_tx(),
			&ethtool_txvlan => FeatureGroup::ethtool_txvlan(),
			&ethtool_rxvlan => FeatureGroup::ethtool_rxvlan(),
			&ethtool_gso => FeatureGroup::ethtool_gso(),
			&ethtool_gro => FeatureGroup::ethtool_gro(),
			&ethtool_lro => FeatureGroup::ethtool_lro(),
			&ethtool_tso => FeatureGroup::ethtool_tso(),
			&ethtool_ntuple => FeatureGroup::ethtool_ntuple(),
			&ethtool_rxhash => FeatureGroup::ethtool_rxhash(),
			&ethtool_rx => FeatureGroup::ethtool_rx(),
			&internet_protocols_checksum => FeatureGroup::internet_protocols_checksum(),
			&internet_protocols_checksum_in_hardware => FeatureGroup::internet_protocols_checksum_in_hardware(),
			&generic_send_offload_encapsulation => FeatureGroup::generic_send_offload_encapsulation(),
			
			&OtherToEnable(ref feature_group) => return non_static_lifetime(feature_group, &rhs, true),
			&OtherToDisable(ref feature_group) => return non_static_lifetime(feature_group, &rhs, false),
		};
		
		if feature_group.contains(&rhs)
		{
			let mut hash_set = feature_group.0.clone();
			hash_set.remove(&rhs);
			OtherToEnable(FeatureGroup::from(hash_set))
		}
		else
		{
			self.clone()
		}
	}
}

impl Sub<Feature> for FeatureGroupChoice
{
	type Output = Self;
	
	#[inline(always)]
	fn sub(self, rhs: Feature) -> Self::Output
	{
		(&self) - rhs
	}
}

impl FeatureGroupChoice
{
	/// Enable one.
	#[inline(always)]
	pub fn enable_one(feature: Feature) -> Self
	{
		Self::enable(fast_secure_hash_set! [feature])
	}
	
	/// Enable.
	#[inline(always)]
	pub fn enable(features: HashSet<Feature>) -> Self
	{
		FeatureGroupChoice::OtherToEnable(FeatureGroup::from(features))
	}
	
	/// Disable one.
	#[inline(always)]
	pub fn disable_one(feature: Feature) -> Self
	{
		Self::disable(fast_secure_hash_set! [feature])
	}
	
	/// Disable.
	#[inline(always)]
	pub fn disable(features: HashSet<Feature>) -> Self
	{
		FeatureGroupChoice::OtherToDisable(FeatureGroup::from(features))
	}
	
	#[inline(always)]
	pub(crate) fn iter(feature_group_choices: &Vec<Self>) -> impl Iterator<Item=HashMap<Feature, bool>> + '_
	{
		feature_group_choices.iter().map(|feature_group_choice| feature_group_choice.to_feature_settings())
	}
	
	#[inline(always)]
	pub(crate) fn to_feature_settings(&self) -> HashMap<Feature, bool>
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
			generic_send_offload_encapsulation => FeatureGroup::generic_send_offload_encapsulation().enable(),
			OtherToEnable(ref feature_group) => feature_group.enable(),
			OtherToDisable(ref feature_group) => feature_group.disable(),
		}
	}
}
