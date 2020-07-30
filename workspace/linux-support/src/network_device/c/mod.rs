// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use crate::file_descriptors::socket::c::*;


include!("cisco_proto.rs");
include!("DOWNSHIFT_DEV_.rs");
include!("ETH_GSTRING_LEN.rs");
include!("ETHTOOL_.rs");
include!("ethtool_channels.rs");
include!("ethtool_coalesce.rs");
include!("ethtool_drvinfo.rs");
include!("ethtool_gstrings.rs");
include!("ETHTOOL_PHY_EDPD_.rs");
include!("ETHTOOL_PHY_FAST_LINK_DOWN_.rs");
include!("ethtool_ringparam.rs");
include!("ethtool_sset_info.rs");
include!("ethtool_stringset.rs");
include!("ethtool_tunable.rs");
include!("ethtool_value.rs");
include!("ethtool_wolinfo.rs");
include!("EthtoolCommand.rs");
include!("ETHTOOL_X_LEN.rs");
include!("fr_proto.rs");
include!("fr_proto_pvc.rs");
include!("fr_proto_pvc_info.rs");
include!("hwtstamp_rx_filters.rs");
include!("hwtstamp_tx_types.rs");
include!("if_settings.rs");
include!("if_settings_ifsu.rs");
include!("IFALIASZ.rs");
include!("ifmap.rs");
include!("ifreq.rs");
include!("ifreq_ifrn.rs");
include!("ifreq_ifru.rs");
include!("MAX_ADDR_LEN.rs");
include!("MAX_PHYS_ITEM_ID_LEN.rs");
include!("net_device_flags.rs");
include!("PFC_STORM_PREVENTION_.rs");
include!("phy_tunable_id.rs");
include!("raw_hdlc_proto.rs");
include!("SOF_TIMESTAMPING.rs");
include!("sync_serial_settings.rs");
include!("te1_settings.rs");
include!("tunable_id.rs");
include!("tunable_type_id.rs");
include!("VariablySizedEthtoolCommand.rs");
include!("VariablySizedEthtoolCommandWrapper.rs");
include!("WAKE.rs");
include!("x25_hdlc_proto.rs");


/*



const char netdev_features_strings[NETDEV_FEATURE_COUNT][ETH_GSTRING_LEN] = {
	[NETIF_F_SG_BIT] =               "tx-scatter-gather",
	[NETIF_F_IP_CSUM_BIT] =          "tx-checksum-ipv4",
	[NETIF_F_HW_CSUM_BIT] =          "tx-checksum-ip-generic",
	[NETIF_F_IPV6_CSUM_BIT] =        "tx-checksum-ipv6",
	[NETIF_F_HIGHDMA_BIT] =          "highdma",
	[NETIF_F_FRAGLIST_BIT] =         "tx-scatter-gather-fraglist",
	[NETIF_F_HW_VLAN_CTAG_TX_BIT] =  "tx-vlan-hw-insert",

	[NETIF_F_HW_VLAN_CTAG_RX_BIT] =  "rx-vlan-hw-parse",
	[NETIF_F_HW_VLAN_CTAG_FILTER_BIT] = "rx-vlan-filter",
	[NETIF_F_HW_VLAN_STAG_TX_BIT] =  "tx-vlan-stag-hw-insert",
	[NETIF_F_HW_VLAN_STAG_RX_BIT] =  "rx-vlan-stag-hw-parse",
	[NETIF_F_HW_VLAN_STAG_FILTER_BIT] = "rx-vlan-stag-filter",
	[NETIF_F_VLAN_CHALLENGED_BIT] =  "vlan-challenged",
	[NETIF_F_GSO_BIT] =              "tx-generic-segmentation",
	[NETIF_F_LLTX_BIT] =             "tx-lockless",
	[NETIF_F_NETNS_LOCAL_BIT] =      "netns-local",
	[NETIF_F_GRO_BIT] =              "rx-gro",
	[NETIF_F_GRO_HW_BIT] =           "rx-gro-hw",
	[NETIF_F_LRO_BIT] =              "rx-lro",

	[NETIF_F_TSO_BIT] =              "tx-tcp-segmentation",
	[NETIF_F_GSO_ROBUST_BIT] =       "tx-gso-robust",
	[NETIF_F_TSO_ECN_BIT] =          "tx-tcp-ecn-segmentation",
	[NETIF_F_TSO_MANGLEID_BIT] =	 "tx-tcp-mangleid-segmentation",
	[NETIF_F_TSO6_BIT] =             "tx-tcp6-segmentation",
	[NETIF_F_FSO_BIT] =              "tx-fcoe-segmentation",
	[NETIF_F_GSO_GRE_BIT] =		 "tx-gre-segmentation",
	[NETIF_F_GSO_GRE_CSUM_BIT] =	 "tx-gre-csum-segmentation",
	[NETIF_F_GSO_IPXIP4_BIT] =	 "tx-ipxip4-segmentation",
	[NETIF_F_GSO_IPXIP6_BIT] =	 "tx-ipxip6-segmentation",
	[NETIF_F_GSO_UDP_TUNNEL_BIT] =	 "tx-udp_tnl-segmentation",
	[NETIF_F_GSO_UDP_TUNNEL_CSUM_BIT] = "tx-udp_tnl-csum-segmentation",
	[NETIF_F_GSO_PARTIAL_BIT] =	 "tx-gso-partial",
	[NETIF_F_GSO_SCTP_BIT] =	 "tx-sctp-segmentation",
	[NETIF_F_GSO_ESP_BIT] =		 "tx-esp-segmentation",
	[NETIF_F_GSO_UDP_L4_BIT] =	 "tx-udp-segmentation",

	[NETIF_F_FCOE_CRC_BIT] =         "tx-checksum-fcoe-crc",
	[NETIF_F_SCTP_CRC_BIT] =        "tx-checksum-sctp",
	[NETIF_F_FCOE_MTU_BIT] =         "fcoe-mtu",
	[NETIF_F_NTUPLE_BIT] =           "rx-ntuple-filter",
	[NETIF_F_RXHASH_BIT] =           "rx-hashing",
	[NETIF_F_RXCSUM_BIT] =           "rx-checksum",
	[NETIF_F_NOCACHE_COPY_BIT] =     "tx-nocache-copy",
	[NETIF_F_LOOPBACK_BIT] =         "loopback",
	[NETIF_F_RXFCS_BIT] =            "rx-fcs",
	[NETIF_F_RXALL_BIT] =            "rx-all",
	[NETIF_F_HW_L2FW_DOFFLOAD_BIT] = "l2-fwd-offload",
	[NETIF_F_HW_TC_BIT] =		 "hw-tc-offload",
	[NETIF_F_HW_ESP_BIT] =		 "esp-hw-offload",
	[NETIF_F_HW_ESP_TX_CSUM_BIT] =	 "esp-tx-csum-hw-offload",
	[NETIF_F_RX_UDP_TUNNEL_PORT_BIT] =	 "rx-udp_tunnel-port-offload",
	[NETIF_F_HW_TLS_RECORD_BIT] =	"tls-hw-record",
	[NETIF_F_HW_TLS_TX_BIT] =	 "tls-hw-tx-offload",
	[NETIF_F_HW_TLS_RX_BIT] =	 "tls-hw-rx-offload",
	[NETIF_F_GRO_FRAGLIST_BIT] =	 "rx-gro-list",
	[NETIF_F_HW_MACSEC_BIT] =	 "macsec-hw-offload",
};

const char
rss_hash_func_strings[ETH_RSS_HASH_FUNCS_COUNT][ETH_GSTRING_LEN] = {
	[ETH_RSS_HASH_TOP_BIT] =	"toeplitz",
	[ETH_RSS_HASH_XOR_BIT] =	"xor",
	[ETH_RSS_HASH_CRC32_BIT] =	"crc32",
};

#define __LINK_MODE_NAME(speed, type, duplex) \
	#speed "base" #type "/" #duplex
#define __DEFINE_LINK_MODE_NAME(speed, type, duplex) \
	[ETHTOOL_LINK_MODE(speed, type, duplex)] = \
	__LINK_MODE_NAME(speed, type, duplex)
#define __DEFINE_SPECIAL_MODE_NAME(_mode, _name) \
	[ETHTOOL_LINK_MODE_ ## _mode ## _BIT] = _name

const char link_mode_names[][ETH_GSTRING_LEN] = {
	__DEFINE_LINK_MODE_NAME(10, T, Half),
	__DEFINE_LINK_MODE_NAME(10, T, Full),
	__DEFINE_LINK_MODE_NAME(100, T, Half),
	__DEFINE_LINK_MODE_NAME(100, T, Full),
	__DEFINE_LINK_MODE_NAME(1000, T, Half),
	__DEFINE_LINK_MODE_NAME(1000, T, Full),
	__DEFINE_SPECIAL_MODE_NAME(Autoneg, "Autoneg"),
	__DEFINE_SPECIAL_MODE_NAME(TP, "TP"),
	__DEFINE_SPECIAL_MODE_NAME(AUI, "AUI"),
	__DEFINE_SPECIAL_MODE_NAME(MII, "MII"),
	__DEFINE_SPECIAL_MODE_NAME(FIBRE, "FIBRE"),
	__DEFINE_SPECIAL_MODE_NAME(BNC, "BNC"),
	__DEFINE_LINK_MODE_NAME(10000, T, Full),
	__DEFINE_SPECIAL_MODE_NAME(Pause, "Pause"),
	__DEFINE_SPECIAL_MODE_NAME(Asym_Pause, "Asym_Pause"),
	__DEFINE_LINK_MODE_NAME(2500, X, Full),
	__DEFINE_SPECIAL_MODE_NAME(Backplane, "Backplane"),
	__DEFINE_LINK_MODE_NAME(1000, KX, Full),
	__DEFINE_LINK_MODE_NAME(10000, KX4, Full),
	__DEFINE_LINK_MODE_NAME(10000, KR, Full),
	__DEFINE_SPECIAL_MODE_NAME(10000baseR_FEC, "10000baseR_FEC"),
	__DEFINE_LINK_MODE_NAME(20000, MLD2, Full),
	__DEFINE_LINK_MODE_NAME(20000, KR2, Full),
	__DEFINE_LINK_MODE_NAME(40000, KR4, Full),
	__DEFINE_LINK_MODE_NAME(40000, CR4, Full),
	__DEFINE_LINK_MODE_NAME(40000, SR4, Full),
	__DEFINE_LINK_MODE_NAME(40000, LR4, Full),
	__DEFINE_LINK_MODE_NAME(56000, KR4, Full),
	__DEFINE_LINK_MODE_NAME(56000, CR4, Full),
	__DEFINE_LINK_MODE_NAME(56000, SR4, Full),
	__DEFINE_LINK_MODE_NAME(56000, LR4, Full),
	__DEFINE_LINK_MODE_NAME(25000, CR, Full),
	__DEFINE_LINK_MODE_NAME(25000, KR, Full),
	__DEFINE_LINK_MODE_NAME(25000, SR, Full),
	__DEFINE_LINK_MODE_NAME(50000, CR2, Full),
	__DEFINE_LINK_MODE_NAME(50000, KR2, Full),
	__DEFINE_LINK_MODE_NAME(100000, KR4, Full),
	__DEFINE_LINK_MODE_NAME(100000, SR4, Full),
	__DEFINE_LINK_MODE_NAME(100000, CR4, Full),
	__DEFINE_LINK_MODE_NAME(100000, LR4_ER4, Full),
	__DEFINE_LINK_MODE_NAME(50000, SR2, Full),
	__DEFINE_LINK_MODE_NAME(1000, X, Full),
	__DEFINE_LINK_MODE_NAME(10000, CR, Full),
	__DEFINE_LINK_MODE_NAME(10000, SR, Full),
	__DEFINE_LINK_MODE_NAME(10000, LR, Full),
	__DEFINE_LINK_MODE_NAME(10000, LRM, Full),
	__DEFINE_LINK_MODE_NAME(10000, ER, Full),
	__DEFINE_LINK_MODE_NAME(2500, T, Full),
	__DEFINE_LINK_MODE_NAME(5000, T, Full),
	__DEFINE_SPECIAL_MODE_NAME(FEC_NONE, "None"),
	__DEFINE_SPECIAL_MODE_NAME(FEC_RS, "RS"),
	__DEFINE_SPECIAL_MODE_NAME(FEC_BASER, "BASER"),
	__DEFINE_LINK_MODE_NAME(50000, KR, Full),
	__DEFINE_LINK_MODE_NAME(50000, SR, Full),
	__DEFINE_LINK_MODE_NAME(50000, CR, Full),
	__DEFINE_LINK_MODE_NAME(50000, LR_ER_FR, Full),
	__DEFINE_LINK_MODE_NAME(50000, DR, Full),
	__DEFINE_LINK_MODE_NAME(100000, KR2, Full),
	__DEFINE_LINK_MODE_NAME(100000, SR2, Full),
	__DEFINE_LINK_MODE_NAME(100000, CR2, Full),
	__DEFINE_LINK_MODE_NAME(100000, LR2_ER2_FR2, Full),
	__DEFINE_LINK_MODE_NAME(100000, DR2, Full),
	__DEFINE_LINK_MODE_NAME(200000, KR4, Full),
	__DEFINE_LINK_MODE_NAME(200000, SR4, Full),
	__DEFINE_LINK_MODE_NAME(200000, LR4_ER4_FR4, Full),
	__DEFINE_LINK_MODE_NAME(200000, DR4, Full),
	__DEFINE_LINK_MODE_NAME(200000, CR4, Full),
	__DEFINE_LINK_MODE_NAME(100, T1, Full),
	__DEFINE_LINK_MODE_NAME(1000, T1, Full),
	__DEFINE_LINK_MODE_NAME(400000, KR8, Full),
	__DEFINE_LINK_MODE_NAME(400000, SR8, Full),
	__DEFINE_LINK_MODE_NAME(400000, LR8_ER8_FR8, Full),
	__DEFINE_LINK_MODE_NAME(400000, DR8, Full),
	__DEFINE_LINK_MODE_NAME(400000, CR8, Full),
	__DEFINE_SPECIAL_MODE_NAME(FEC_LLRS, "LLRS"),
};
const char netif_msg_class_names[][ETH_GSTRING_LEN] = {
	[NETIF_MSG_DRV_BIT]		= "drv",
	[NETIF_MSG_PROBE_BIT]		= "probe",
	[NETIF_MSG_LINK_BIT]		= "link",
	[NETIF_MSG_TIMER_BIT]		= "timer",
	[NETIF_MSG_IFDOWN_BIT]		= "ifdown",
	[NETIF_MSG_IFUP_BIT]		= "ifup",
	[NETIF_MSG_RX_ERR_BIT]		= "rx_err",
	[NETIF_MSG_TX_ERR_BIT]		= "tx_err",
	[NETIF_MSG_TX_QUEUED_BIT]	= "tx_queued",
	[NETIF_MSG_INTR_BIT]		= "intr",
	[NETIF_MSG_TX_DONE_BIT]		= "tx_done",
	[NETIF_MSG_RX_STATUS_BIT]	= "rx_status",
	[NETIF_MSG_PKTDATA_BIT]		= "pktdata",
	[NETIF_MSG_HW_BIT]		= "hw",
	[NETIF_MSG_WOL_BIT]		= "wol",
};

 */
