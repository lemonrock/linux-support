// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
#[derive(Deserialize, Serialize)]
pub struct FeatureGroup(HashSet<NETIF_F>);

impl Deref for FeatureGroup
{
	type Target = HashSet<NETIF_F>;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl DerefMut for FeatureGroup
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.0
	}
}

impl From<HashSet<NETIF_F>> for FeatureGroup
{
	#[inline(always)]
	fn from(value: HashSet<NETIF_F>) -> Self
	{
		Self(value)
	}
}

impl Into<HashSet<NETIF_F>> for FeatureGroup
{
	#[inline(always)]
	fn into(self) -> HashSet<NETIF_F>
	{
		self.0
	}
}

impl FeatureGroup
{
	#[inline(always)]
	pub(crate) fn enable(&self) -> HashMap<NETIF_F, bool>
	{
		self.0.iter().map(|key| (*key, true)).collect()
	}
	
	#[inline(always)]
	pub(crate) fn disable(&self) -> HashMap<NETIF_F, bool>
	{
		self.0.iter().map(|key| (*key, false)).collect()
	}
	
	/// All known features.
	#[inline(always)]
	pub fn all() -> &'static Self
	{
		lazy_static!
		{
			static ref Static: FeatureGroup =
			{
				let mut hash_set = HashSet::with_capacity(NETIF_F::NETDEV_FEATURE_COUNT);
				for feature in NETIF_F::iter()
				{
					hash_set.add(feature)
				}
				FeatureGroup(hash_set)
			}
		}
		&Static
	}
	
	/// Ethtool setting is `sg` or `scatter-gather`.
	#[inline(always)]
	pub fn ethtool_sg() -> &'static Self
	{
		lazy_static!
		{
			static ref Static: FeatureGroup = FeatureGroup
			(
				hashset!
				{
					NETIF_F_SG_BIT,
					NETIF_F_FRAGLIST_BIT,
				}
			);
		}
		&Static
	}
	
	/// Ethtool setting is `tx` or `tx-checksumming`.
	#[inline(always)]
	pub fn ethtool_tx() -> &'static Self
	{
		lazy_static!
		{
			static ref Static: FeatureGroup = FeatureGroup
			(
				hashset!
				{
					NETIF_F_IP_CSUM_BIT,
					NETIF_F_HW_CSUM_BIT,
					NETIF_F_IPV6_CSUM_BIT,
					NETIF_F_FCOE_CRC_BIT,
					NETIF_F_SCTP_CRC_BIT,
				}
			);
		}
		&Static
	}
	
	/// Ethtool setting is `txvlan` or `tx-vlan-offload`.
	#[inline(always)]
	pub fn ethtool_txvlan() -> &'static Self
	{
		lazy_static!
		{
			static ref Static: FeatureGroup = FeatureGroup
			(
				hashset!
				{
					NETIF_F_HW_VLAN_CTAG_TX_BIT,
				}
			);
		}
		&Static
	}
	
	/// Ethtool setting is `rxvlan` or `rx-vlan-offload`.
	#[inline(always)]
	pub fn ethtool_rtxvlan() -> &'static Self
	{
		lazy_static!
		{
			static ref Static: FeatureGroup = FeatureGroup
			(
				hashset!
				{
					NETIF_F_HW_VLAN_CTAG_RX_BIT,
				}
			);
		}
		&Static
	}
	
	/// Ethtool setting is `gso` or `generic-segmentation-offload`.
	#[inline(always)]
	pub fn ethtool_gso() -> &'static Self
	{
		lazy_static!
		{
			static ref Static: FeatureGroup = FeatureGroup
			(
				hashset!
				{
					NETIF_F_GSO_BIT,
				}
			);
		}
		&Static
	}
	
	/// Ethtool setting is `gro` or `generic-receive-offload`.
	#[inline(always)]
	pub fn ethtool_gro() -> &'static Self
	{
		lazy_static!
		{
			static ref Static: FeatureGroup = FeatureGroup
			(
				hashset!
				{
					NETIF_F_GRO_BIT,
				}
			);
		}
		&Static
	}
	
	/// Ethtool setting is `lro` or `large-receive-offload`.
	#[inline(always)]
	pub fn ethtool_lro() -> &'static Self
	{
		lazy_static!
		{
			static ref Static: FeatureGroup = FeatureGroup
			(
				hashset!
				{
					NETIF_F_LRO_BIT,
				}
			);
		}
		&Static
	}
	
	/// Ethtool setting is `tso` or `tcp-segmentation-offload`.
	#[inline(always)]
	pub fn ethtool_tso() -> &'static Self
	{
		Self::NETIF_F_ALL_TSO()
	}
	
	/// Ethtool setting is `ufo` or `udp-fragmentation-offload`.
	#[inline(always)]
	#[deprecated]
	#[allow(deprecated)]
	pub fn ethtool_ufo() -> &'static Self
	{
		lazy_static!
		{
			static ref Static: FeatureGroup = FeatureGroup
			(
				hashset!
				{
					NETIF_F_GSO_UDP_BIT,
				}
			);
		}
		&Static
	}
	
	/// Ethtool setting is `ntuple` or `ntuple-filters`.
	#[inline(always)]
	pub fn ethtool_ntuple() -> &'static Self
	{
		lazy_static!
		{
			static ref Static: FeatureGroup = FeatureGroup
			(
				hashset!
				{
					NETIF_F_NTUPLE_BIT,
				}
			);
		}
		&Static
	}
	
	/// Ethtool setting is `rxhash` or `receive-hashing`.
	#[inline(always)]
	pub fn ethtool_rxhash() -> &'static Self
	{
		lazy_static!
		{
			static ref Static: FeatureGroup = FeatureGroup
			(
				hashset!
				{
					NETIF_F_RXHASH_BIT,
				}
			);
		}
		&Static
	}
	
	/// Ethtool setting is `rx` or `rx-checksumming`.
	#[inline(always)]
	pub fn ethtool_rx() -> &'static Self
	{
		lazy_static!
		{
			static ref Static: FeatureGroup = FeatureGroup
			(
				hashset!
				{
					NETIF_F_RXCSUM_BIT,
				}
			);
		}
		&Static
	}
	
	/// Valid IP checksum settings.
	#[inline(always)]
	pub fn internet_protocols_checksum() -> &'static Self
	{
		lazy_static!
		{
			static ref Static: FeatureGroup = FeatureGroup
			(
				hashset!
				{
					NETIF_F_IP_CSUM_BIT,
					NETIF_F_IPV6_CSUM_BIT,
				}
			);
		}
		&Static
	}
	
	/// Valid IP checksum settings (in hardware).
	#[inline(always)]
	pub fn internet_protocols_checksum_in_hardware() -> &'static Self
	{
		lazy_static!
		{
			static ref Static: FeatureGroup = FeatureGroup
			(
				hashset!
				{
					NETIF_F_HW_CSUM_BIT,
				}
			);
		}
		&Static
	}
	
	/// Features that can never be changed, but can be reported.
	///
	/// This is not a complete list but just those that are device-independent.
	#[inline(always)]
	pub fn NETIF_F_NEVER_CHANGE() -> &'static Self
	{
		lazy_static!
		{
			static ref Static: FeatureGroup = FeatureGroup
			(
				hashset!
				{
					NETIF_F_VLAN_CHALLENGED_BIT,
					NETIF_F_LLTX_BIT,
					NETIF_F_NETNS_LOCAL_BIT,
				}
			);
		}
		&Static
	}
	
	/// Features that can be changed depending on the device.
	#[inline(always)]
	pub fn NETIF_F_ETHTOOL_BITS() -> &'static Self
	{
		lazy_static!
		{
			static ref Static: FeatureGroup =
			{
				let difference = FeatureGroup.all().difference(&FeatureGroup::NETIF_F_NEVER_CHANGE());
				FeatureGroup(difference.collect())
			};
		}
		&Static
	}
	
	/// Segmentation offload feature mask.
	pub fn NETIF_F_GSO_MASK() -> &'static Self
	{
		lazy_static!
		{
			static ref Static: FeatureGroup =
			{
				let inclusive_first_bit = NETIF_F::NETIF_F_GSO_SHIFT as u32;
				let inclusive_last_bit = NETIF_F::NETIF_F_GSO_LAST as u32;
				let mut hash_set = HashSet::with_capacity((inclusive_last_bit - inclusive_first_bit + 1) as usize);
				for gso_mask_bit in inclusive_first_bit ..= inclusive_last_bit
				{
					hash_set.add(unsafe { transmute(gso_mask_bit) })
				}
				FeatureGroup(hash_set)
			};
		}
		&Static
	}
	
	/// List of IP checksum features.
	///
	/// Note that `NETIF_F_HW_CSUM` should not be set in features when `NETIF_F_IP_CSUM` or `NETIF_F_IPV6_CSUM` are set; this would be contradictory.
	#[inline(always)]
	pub fn NETIF_F_CSUM_MASK() -> &'static Self
	{
		lazy_static!
		{
			static ref Static: FeatureGroup = FeatureGroup
			(
				hashset!
				{
					NETIF_F_IP_CSUM_BIT,
					NETIF_F_IPV6_CSUM_BIT,
					NETIF_F_HW_CSUM_BIT,
				}
			);
		}
		&Static
	}
	
	/// TCP segmentation offload.
	#[inline(always)]
	pub fn NETIF_F_ALL_TSO() -> &'static Self
	{
		lazy_static!
		{
			static ref Static: FeatureGroup = FeatureGroup
			(
				hashset!
				{
					NETIF_F_TSO_BIT,
					NETIF_F_TSO6_BIT,
					NETIF_F_TSO_ECN_BIT,
					NETIF_F_TSO_MANGLEID_BIT,
				}
			);
		}
		&Static
	}
	
	/// Fibre Channel over Ethernet (FCoE).
	#[inline(always)]
	pub fn NETIF_F_ALL_FCOE() -> &'static Self
	{
		lazy_static!
		{
			static ref Static: FeatureGroup = FeatureGroup
			(
				hashset!
				{
					NETIF_F_FCOE_CRC_BIT,
					NETIF_F_FCOE_MTU_BIT,
					NETIF_F_FSO_BIT,
				}
			);
		}
		&Static
	}
	
	/// List of features with software fallbacks.
	#[inline(always)]
	pub fn NETIF_F_GSO_SOFTWARE() -> &'static Self
	{
		lazy_static!
		{
			static ref Static: FeatureGroup = FeatureGroup::NETIF_F_ALL_TSO().merge_with_one(NETIF_F_GSO_SCTP_BIT);
		}
		&Static
	}
	
	/// If one device supports one of these features, then enable them for all in `netdev_increment_features`.
	#[inline(always)]
	pub fn NETIF_F_ONE_FOR_ALL() -> &'static Self
	{
		lazy_static!
		{
			static ref Static: FeatureGroup = FeatureGroup::NETIF_F_GSO_SOFTWARE().merge_with_many
			(
				&[
					NETIF_F_GSO_ROBUST_BIT,
					NETIF_F_SG_BIT,
					NETIF_F_HIGHDMA_BIT,
					NETIF_F_FRAGLIST_BIT,
					NETIF_F_VLAN_CHALLENGED_BIT,
				]
			);
		}
		&Static
	}
	
	/// If one device doesn't support one of these features, then disable it for all in `netdev_increment_features.`
	#[inline(always)]
	pub fn NETIF_F_ALL_FOR_ALL() -> &'static Self
	{
		lazy_static!
		{
			static ref Static: FeatureGroup = FeatureGroup
			(
				hashset!
				{
					NETIF_F_NOCACHE_COPY_BIT,
					NETIF_F_FSO_BIT,
				}
			);
		}
		&Static
	}
	
	/// If upper (master) device has these features disabled, they must be disabled on all lower (slave) devices as well.
	#[inline(always)]
	pub fn NETIF_F_UPPER_DISABLES() -> &'static Self
	{
		lazy_static!
		{
			static ref Static: FeatureGroup = FeatureGroup
			(
				hashset!
				{
					NETIF_F_LRO_BIT,
				}
			);
		}
		&Static
	}
	
	/// Changeable features with no special hardware requirements.
	#[inline(always)]
	pub fn NETIF_F_SOFT_FEATURES() -> &'static Self
	{
		lazy_static!
		{
			static ref Static: FeatureGroup = FeatureGroup
			(
				hashset!
				{
					NETIF_F_GSO_BIT,
					NETIF_F_GRO_BIT,
				}
			);
		}
		&Static
	}
	
	/// Changeable features with no special hardware requirements that default to off.
	#[inline(always)]
	pub fn NETIF_F_SOFT_FEATURES_OFF() -> &'static Self
	{
		lazy_static!
		{
			static ref Static: FeatureGroup = FeatureGroup
			(
				hashset!
				{
					NETIF_F_GRO_FRAGLIST_BIT,
				}
			);
		}
		&Static
	}
	
	/// ?
	#[inline(always)]
	pub fn NETIF_F_VLAN_FEATURES() -> &'static Self
	{
		lazy_static!
		{
			static ref Static: FeatureGroup = FeatureGroup
			(
				hashset!
				{
					NETIF_F_HW_VLAN_CTAG_FILTER_BIT,
					NETIF_F_HW_VLAN_CTAG_RX_BIT,
					NETIF_F_HW_VLAN_CTAG_TX_BIT,
					NETIF_F_HW_VLAN_STAG_FILTER_BIT,
					NETIF_F_HW_VLAN_STAG_RX_BIT,
					NETIF_F_HW_VLAN_STAG_TX_BIT,
				}
			);
		}
		&Static
	}
	
	/// ?
	#[inline(always)]
	pub fn NETIF_F_GSO_ENCAP_ALL() -> &'static Self
	{
		lazy_static!
		{
			static ref Static: FeatureGroup = FeatureGroup
			(
				hashset!
				{
					NETIF_F_GSO_GRE_BIT,
					NETIF_F_GSO_GRE_CSUM_BIT,
					NETIF_F_GSO_IPXIP4_BIT,
					NETIF_F_GSO_IPXIP6_BIT,
					NETIF_F_GSO_UDP_TUNNEL_BIT,
					NETIF_F_GSO_UDP_TUNNEL_CSUM_BIT,
				}
			);
		}
		&Static
	}
	
	/// Add one `feature`.
	#[inline(always)]
	pub fn merge_with_one(&self, feature: NETIF_F) -> Self
	{
		let mut feature_group = self.clone();
		feature_group.add(feature);
		feature_group
	}
	
	/// Add one `feature`.
	#[inline(always)]
	pub fn merge_with_many(&self, features: &[NETIF_F]) -> Self
	{
		let mut feature_group = self.clone();
		for feature in features
		{
			feature_group.add(*feature);
		}
		feature_group
	}
}
