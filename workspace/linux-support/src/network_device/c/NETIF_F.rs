// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Found in Linux source at `include/linux/netdev_features.h`.
///
/// Strings are in the `ethtool_stringset::ETH_SS_FEATURES` string set.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[derive(EnumCount, EnumIter)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename = "Feature")]
#[repr(u32)]
pub enum NETIF_F
{
	/// Scatter/gather IO.
	///
	/// String set value is `tx-scatter-gather`.
	///
	/// Ethtool setting is `sg` or `scatter-gather`.
	/// Historic commands are `ETHTOOL_GSG` and `ETHTOOL_SSG`.
	#[serde(rename = "tx-scatter-gather")]
	NETIF_F_SG_BIT = 0,
	
	/// Can checksum TCP/UDP over IPv4.
	///
	/// String set value is `tx-checksum-ipv4`.
	///
	/// Ethtool setting is `tx` or `tx-checksumming`.
	/// Historic commands are `ETHTOOL_GTXCSUM` and `ETHTOOL_STXCSUM`.
	#[serde(rename = "tx-checksum-ipv4")]
	NETIF_F_IP_CSUM_BIT = 1,
	
	#[serde(skip)]
	#[deprecated]
	__UNUSED_NETIF_F_1 = 2,
	
	/// Can checksum all the packets.
	///
	/// String set value is `tx-checksum-ip-generic`.
	///
	/// Ethtool setting is `tx` or `tx-checksumming`.
	/// Historic commands are `ETHTOOL_GTXCSUM` and `ETHTOOL_STXCSUM`.
	#[serde(rename = "tx-checksum-ip-generic")]
	NETIF_F_HW_CSUM_BIT = 3,
	
	/// Can checksum TCP/UDP over IPV6.
	///
	/// String set value is `tx-checksum-ipv6`.
	///
	/// Ethtool setting is `tx` or `tx-checksumming`.
	/// Historic commands are `ETHTOOL_GTXCSUM` and `ETHTOOL_STXCSUM`.
	#[serde(rename = "tx-checksum-ipv6")]
	NETIF_F_IPV6_CSUM_BIT = 4,
	
	/// Can DMA to high memory.
	///
	/// String set value is `highdma`.
	#[serde(skip)]
	NETIF_F_HIGHDMA_BIT = 5,
	
	/// Scatter/gather IO.
	///
	/// String set value is `tx-scatter-gather-fraglist`.
	///
	/// Ethtool setting is `sg` or `scatter-gather`.
	/// Historic commands are `ETHTOOL_GSG` and `ETHTOOL_SSG`.
	#[serde(rename = "tx-scatter-gather-fraglist")]
	NETIF_F_FRAGLIST_BIT = 6,
	
	/// Transmit VLAN CTAG HW acceleration.
	///
	/// String set value is `tx-vlan-hw-insert`.
	///
	/// Ethtool setting is `txvlan` or `tx-vlan-offload`.
	/// There are no historic commands.
	#[serde(rename = "tx-vlan-hw-insert")]
	NETIF_F_HW_VLAN_CTAG_TX_BIT = 7,
	
	/// Receive VLAN CTAG HW acceleration.
	///
	/// String set value is `rx-vlan-hw-parse`.
	///
	/// Ethtool setting is `rxvlan` or `rx-vlan-offload`.
	/// There are no historic commands.
	#[serde(rename = "rx-vlan-hw-parse")]
	NETIF_F_HW_VLAN_CTAG_RX_BIT = 8,
	
	/// Receive filtering on VLAN CTAGs.
	///
	/// String set value is `rx-vlan-filter`.
	#[serde(rename = "rx-vlan-filter")]
	NETIF_F_HW_VLAN_CTAG_FILTER_BIT = 9,
	
	/// Device cannot handle VLAN packets.
	///
	/// String set value is `vlan-challenged`.
	#[serde(skip)]
	NETIF_F_VLAN_CHALLENGED_BIT = 10,
	
	/// Enable software GSO.
	///
	/// String set value is `tx-generic-segmentation`.
	///
	/// Ethtool setting is `gso` or `generic-segmentation-offload`.
	/// Historic commands are `ETHTOOL_GGSO` and `ETHTOOL_SGSO`.
	#[serde(rename = "tx-generic-segmentation")]
	NETIF_F_GSO_BIT = 11,
	
	/// LockLess TX.
	///
	/// String set value is `tx-lockless`.
	#[serde(skip)]
	#[deprecated]
	NETIF_F_LLTX_BIT = 12,
	
	/// Does not change network namespaces.
	///
	/// String set value is `netns-local`.
	#[serde(skip)]
	NETIF_F_NETNS_LOCAL_BIT = 13,
	
	/// Generic receive offload.
	///
	/// String set value is `rx-gro`.
	///
	/// Ethtool setting is `gro` or `generic-receive-offload`.
	/// Historic commands are `ETHTOOL_GGRO` and `ETHTOOL_SGRO`.
	#[serde(rename = "rx-gro")]
	NETIF_F_GRO_BIT = 14,
	
	/// Large receive offload.
	///
	/// String set value is `rx-lro`.
	///
	/// Ethtool setting is `lro` or `large-receive-offload`.
	/// There are no historic commands.
	#[serde(rename = "rx-lro")]
	NETIF_F_LRO_BIT = 15,
	
	/// A `SKB_GSO_*` bit; see `FeatureGroup::NETIF_F_GSO_MASK`.
	///
	/// Keep the order of `SKB_GSO_*` bits.
	///
	/// String set value is `tx-tcp-segmentation`.
	///
	/// Ethtool setting is `tso` or `tcp-segmentation-offload`.
	/// Historic commands are `ETHTOOL_GTSO` and `ETHTOOL_STSO`.
	#[serde(rename = "tx-tcp-segmentation")]
	NETIF_F_TSO_BIT = 16,
	
	/// A `SKB_GSO_*` bit; see `FeatureGroup::NETIF_F_GSO_MASK`.
	///
	/// String set value is `tx-gso-robust`.
	#[serde(skip)]
	NETIF_F_GSO_ROBUST_BIT = 17,
	
	/// A `SKB_GSO_*` bit; see `FeatureGroup::NETIF_F_GSO_MASK`.
	///
	/// TCP ECN support.
	///
	/// String set value is `tx-tcp-ecn-segmentation`.
	///
	/// Ethtool setting is `tso` or `tcp-segmentation-offload`.
	/// Historic commands are `ETHTOOL_GTSO` and `ETHTOOL_STSO`.
	#[serde(rename = "tx-tcp-ecn-segmentation")]
	NETIF_F_TSO_ECN_BIT = 18,
	
	/// A `SKB_GSO_*` bit; see `FeatureGroup::NETIF_F_GSO_MASK`.
	///
	/// IPV4 ID mangling allowed.
	///
	/// String set value is `tx-tcp-mangleid-segmentation`.
	///
	/// Ethtool setting is `tso` or `tcp-segmentation-offload`.
	/// Historic commands are `ETHTOOL_GTSO` and `ETHTOOL_STSO`.
	#[serde(rename = "tx-tcp-mangleid-segmentation")]
	NETIF_F_TSO_MANGLEID_BIT = 19,
	
	/// A `SKB_GSO_*` bit; see `FeatureGroup::NETIF_F_GSO_MASK`.
	///
	/// TCPv6 segmentation.
	///
	/// String set value is `tx-tcp6-segmentation`.
	///
	/// Ethtool setting is `tso` or `tcp-segmentation-offload`.
	/// Historic commands are `ETHTOOL_GTSO` and `ETHTOOL_STSO`.
	#[serde(rename = "tx-tcp6-segmentation")]
	NETIF_F_TSO6_BIT = 20,
	
	/// A `SKB_GSO_*` bit; see `FeatureGroup::NETIF_F_GSO_MASK`.
	///
	/// FCoE segmentation.
	///
	/// String set value is `tx-fcoe-segmentation`.
	#[serde(rename = "tx-fcoe-segmentation")]
	NETIF_F_FSO_BIT = 21,
	
	/// A `SKB_GSO_*` bit; see `FeatureGroup::NETIF_F_GSO_MASK`.
	///
	/// GRE with TSO.
	///
	/// String set value is `tx-gre-segmentation`.
	#[serde(rename = "tx-gre-segmentation")]
	NETIF_F_GSO_GRE_BIT = 22,
	
	/// A `SKB_GSO_*` bit; see `FeatureGroup::NETIF_F_GSO_MASK`.
	///
	/// GRE with checksum with TSO.
	///
	/// String set value is `tx-gre-csum-segmentation`.
	#[serde(rename = "tx-gre-csum-segmentation")]
	NETIF_F_GSO_GRE_CSUM_BIT = 23,
	
	/// A `SKB_GSO_*` bit; see `FeatureGroup::NETIF_F_GSO_MASK`.
	///
	/// IP4 or IP6 over IP4 with TSO.
	///
	/// String set value is `tx-ipxip4-segmentation`.
	#[serde(rename = "tx-ipxip4-segmentation")]
	NETIF_F_GSO_IPXIP4_BIT = 24,
	
	/// A `SKB_GSO_*` bit; see `FeatureGroup::NETIF_F_GSO_MASK`.
	///
	/// IP4 or IP6 over IP6 with TSO.
	///
	/// String set value is `tx-ipxip6-segmentation`.
	#[serde(rename = "tx-ipxip6-segmentation")]
	NETIF_F_GSO_IPXIP6_BIT = 25,
	
	/// A `SKB_GSO_*` bit; see `FeatureGroup::NETIF_F_GSO_MASK`.
	///
	/// UDP tunnel with TSO.
	///
	/// String set value is `tx-udp_tnl-segmentation`.
	#[serde(rename = "tx-udp_tnl-segmentation")]
	NETIF_F_GSO_UDP_TUNNEL_BIT = 26,
	
	/// A `SKB_GSO_*` bit; see `FeatureGroup::NETIF_F_GSO_MASK`.
	///
	/// UDP tunnel and checksum with TSO.
	///
	/// String set value is `tx-udp_tnl-csum-segmentation`.
	#[serde(rename = "tx-udp_tnl-csum-segmentation")]
	NETIF_F_GSO_UDP_TUNNEL_CSUM_BIT = 27,
	
	/// A `SKB_GSO_*` bit; see `FeatureGroup::NETIF_F_GSO_MASK`.
	///
	/// Only segment inner-most L4 in hardware and all other headers in software.
	///
	/// String set value is `tx-gso-partial`.
	#[serde(rename = "tx-gso-partial")]
	NETIF_F_GSO_PARTIAL_BIT = 28,
	
	/// A `SKB_GSO_*` bit; see `FeatureGroup::NETIF_F_GSO_MASK`.
	///
	/// Tunnel with TSO & REMCSUM.
	#[serde(skip)]
	NETIF_F_GSO_TUNNEL_REMCSUM_BIT = 29,
	
	/// A `SKB_GSO_*` bit; see `FeatureGroup::NETIF_F_GSO_MASK`.
	///
	/// SCTP fragmentation.
	///
	/// String set value is `tx-sctp-segmentation`.
    #[serde(rename = "tx-sctp-segmentation")]
	NETIF_F_GSO_SCTP_BIT = 30,
	
	/// A `SKB_GSO_*` bit; see `FeatureGroup::NETIF_F_GSO_MASK`.
	///
	/// ESP with TSO.
	///
	/// String set value is `tx-esp-segmentation`.
    #[serde(rename = "tx-esp-segmentation")]
	NETIF_F_GSO_ESP_BIT = 31,
	
	/// A `SKB_GSO_*` bit; see `FeatureGroup::NETIF_F_GSO_MASK`.
	///
	/// UFO, deprecated except tuntap.
	///
	/// String set value *WAS MAYBE* `tx-udp-fragmentation`.
	///
	/// Ethtool setting is `ufo` or `udp-fragmentation-offload`.
	/// Historic commands are `ETHTOOL_GUFO` and `ETHTOOL_SUFO`.
    #[serde(skip)]
	#[deprecated]
	NETIF_F_GSO_UDP_BIT = 32,
	
	/// A `SKB_GSO_*` bit; see `FeatureGroup::NETIF_F_GSO_MASK`.
	///
	/// UDP payload GSO (not UFO).
	///
	/// String set value is `tx-udp-segmentation`.
    #[serde(rename = "tx-udp-segmentation")]
	NETIF_F_GSO_UDP_L4_BIT = 33,
	
	/// A `SKB_GSO_*` bit; see `FeatureGroup::NETIF_F_GSO_MASK`.
	///
	/// Fraglist GSO.
    #[serde(skip)]
	NETIF_F_GSO_FRAGLIST_BIT = 34,
	
	/// FCoE CRC32.
	///
	/// String set value is `tx-checksum-fcoe-crc`.
	///
	/// Ethtool setting is `tx` or `tx-checksumming`.
	/// Historic commands are `ETHTOOL_GTXCSUM` and `ETHTOOL_STXCSUM`.
    #[serde(rename = "tx-checksum-fcoe-crc")]
	NETIF_F_FCOE_CRC_BIT = 35,
	
	/// SCTP checksum offload.
	///
	/// String set value is `tx-checksum-sctp`.
	///
	/// Ethtool setting is `tx` or `tx-checksumming`.
	/// Historic commands are `ETHTOOL_GTXCSUM` and `ETHTOOL_STXCSUM`.
    #[serde(rename = "tx-checksum-sctp")]
	NETIF_F_SCTP_CRC_BIT = 36,
	
	/// Supports max FCoE MTU, 2158 bytes.
	///
	/// String set value is `fcoe-mtu`.
    #[serde(rename = "fcoe-mtu")]
	NETIF_F_FCOE_MTU_BIT = 37,
	
	/// N-tuple filters supported.
	///
	/// String set value is `rx-ntuple-filter`.
	///
	/// Ethtool setting is `ntuple` or `ntuple-filters`.
	/// There are no historic commands.
    #[serde(rename = "rx-ntuple-filter")]
	NETIF_F_NTUPLE_BIT = 38,
	
	/// Receive hashing offload.
	///
	/// String set value is `rx-hashing`.
	///
	/// Ethtool setting is `rxhash` or `receive-hashing`.
	/// There are no historic commands.
    #[serde(rename = "rx-hashing")]
	NETIF_F_RXHASH_BIT = 39,
	
	/// Receive checksumming offload.
	///
	/// String set value is `rx-checksum`.
	///
	/// Ethtool setting is `rx` or `rx-checksumming`.
	/// Historic commands are `ETHTOOL_GRXCSUM` and `ETHTOOL_SRXCSUM`.
    #[serde(rename = "rx-checksum")]
	NETIF_F_RXCSUM_BIT = 40,
	
	/// Use no-cache copyfromuser.
	///
	/// String set value is `tx-nocache-copy`.
    #[serde(rename = "tx-nocache-copy")]
	NETIF_F_NOCACHE_COPY_BIT = 41,
	
	/// Enable loopback.
	///
	/// String set value is `loopback`.
    #[serde(rename = "loopback")]
	NETIF_F_LOOPBACK_BIT = 42,
	
	/// Append received Frame Check Sequence (FCS) to skb pkt data.
	///
	/// String set value is `rx-fcs`.
    #[serde(rename = "rx-fcs")]
	NETIF_F_RXFCS_BIT = 43,
	
	/// Receive errored frames too.
	///
	/// String set value is `rx-all`.
    #[serde(rename = "rx-all")]
	NETIF_F_RXALL_BIT = 44,
	
	/// Transmit VLAN STAG HW acceleration.
	///
	/// String set value is `tx-vlan-stag-hw-insert`.
    #[serde(rename = "tx-vlan-stag-hw-insert")]
	NETIF_F_HW_VLAN_STAG_TX_BIT = 45,
	
	/// Receive VLAN STAG HW acceleration.
	///
	/// String set value is `rx-vlan-stag-hw-parse`.
    #[serde(rename = "rx-vlan-stag-hw-parse")]
	NETIF_F_HW_VLAN_STAG_RX_BIT = 46,
	
	/// Receive filtering on VLAN STAGs.
	///
	/// String set value is `rx-vlan-stag-filter`.
    #[serde(rename = "rx-vlan-stag-filter")]
	NETIF_F_HW_VLAN_STAG_FILTER_BIT = 47,
	
	/// Allow L2 Forwarding in Hardware.
	///
	/// String set value is `l2-fwd-offload`.
    #[serde(rename = "l2-fwd-offload")]
	NETIF_F_HW_L2FW_DOFFLOAD_BIT = 48,
	
	/// Offload TC infrastructure.
	///
	/// String set value is `hw-tc-offload`.
    #[serde(rename = "hw-tc-offload")]
	NETIF_F_HW_TC_BIT = 49,
	
	/// Hardware ESP transformation offload.
	///
	/// String set value is `esp-hw-offload`.
    #[serde(rename = "esp-hw-offload")]
	NETIF_F_HW_ESP_BIT = 50,
	
	/// ESP with TX checksum offload.
	///
	/// String set value is `esp-tx-csum-hw-offload`.
    #[serde(rename = "esp-tx-csum-hw-offload")]
	NETIF_F_HW_ESP_TX_CSUM_BIT = 51,
	
	/// Offload of RX port for UDP tunnels.
	///
	/// String set value is `udp_tunnel`.
    #[serde(rename = "udp_tunnel")]
	NETIF_F_RX_UDP_TUNNEL_PORT_BIT = 52,
	
	/// Hardware TLS TX offload.
	///
	/// String set value is `tls-hw-tx-offload`.
    #[serde(rename = "tls-hw-tx-offload")]
	NETIF_F_HW_TLS_TX_BIT = 53,
	
	/// Hardware TLS RX offload.
	///
	/// String set value is `tls-hw-rx-offload`.
    #[serde(rename = "tls-hw-rx-offload")]
	NETIF_F_HW_TLS_RX_BIT = 54,
	
	/// Hardware Generic Receive Offload (GRO).
	///
	/// String set value is `rx-gro-hw`.
    #[serde(rename = "rx-gro-hw")]
	NETIF_F_GRO_HW_BIT = 55,
	
	/// Offload TLS record.
	///
	/// String set value is `tls-hw-record`.
    #[serde(rename = "tls-hw-record")]
	NETIF_F_HW_TLS_RECORD_BIT = 56,
	
	/// Fraglist GRO.
	///
	/// String set value is `rx-gro-list`.
    #[serde(rename = "rx-gro-list")]
	NETIF_F_GRO_FRAGLIST_BIT = 57,
	
	/// Offload MACsec operations.
	///
	/// String set value is `macsec-hw-offload`.
    #[serde(rename = "macsec-hw-offload")]
	NETIF_F_HW_MACSEC_BIT = 58,
}

impl Bit for NETIF_F
{
	#[inline(always)]
	fn to_u32(self) -> u32
	{
		unsafe { transmute(self) }
	}
}

impl NETIF_F
{
	/// First of the `SKB_GSO` GSO bits.
	const NETIF_F_GSO_SHIFT: Self = NETIF_F::NETIF_F_TSO_BIT;
	
	/// Last of the `SKB_GSO` GSO bits.
	pub const NETIF_F_GSO_LAST: Self = NETIF_F::NETIF_F_GSO_FRAGLIST_BIT;
	
	const NETDEV_FEATURE_COUNT: usize = Self::NETIF_F_COUNT;
	
	const ETHTOOL_DEV_FEATURE_WORDS: usize = BitSetHelper::divide_rounded_up_word(Self::NETDEV_FEATURE_COUNT);
	
	// There are, as of the time of coding, 59 entries, and Linux is likely to grow more.
	// This creates space for up to 256 entries as of the time of writing.
	const SafeSize: usize = Self::ETHTOOL_DEV_FEATURE_WORDS + 2;
}
