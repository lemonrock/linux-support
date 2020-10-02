// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use self::bit_set::*;
use self::c::flow_specifications::ethtool_rx_flow_spec;
use self::network_flow_classifier::RuleAction;
use self::receive_side_scaling::ContextIdentifier;
use self::wake_on_lan::WakeOnLanWhen;
use crate::file_descriptors::socket::c::*;


include!("ALTIFNAMSIZ.rs");
include!("AUTONEG.rs");
include!("cisco_proto.rs");
include!("DOWNSHIFT_DEV_.rs");
include!("DUPLEX.rs");
include!("ETH_FW_DUMP_DISABLE.rs");
include!("ETH_GSTRING_LEN.rs");
include!("ETH_MDIO_SUPPORTS.rs");
include!("ETH_MODULE_SFF_.rs");
include!("ETH_MODULE_SFF_x_LEN.rs");
include!("ETH_MODULE_SFF_x_MAX_LEN.rs");
include!("ETH_RSS_HASH.rs");
include!("ETH_RX_FLOW_SPEC_.rs");
include!("ETH_RXFH_.rs");
include!("ETH_TP_MDI.rs");
include!("ETH_x_LEN.rs");
include!("ETH_x_MTU.rs");
include!("ETH_xLEN.rs");
include!("ethtool_fec_config_bits.rs");
include!("ethtool_get_features_block.rs");
include!("ethtool_link_mode_bit_indices.rs");
include!("ethtool_link_mode_bit_indices_forward_error_correction.rs");
include!("ethtool_link_mode_bit_indices_pause.rs");
include!("ethtool_link_mode_bit_indices_ports.rs");
include!("ethtool_link_mode_bit_indices_speed.rs");
include!("ETHTOOL_PHY_EDPD_.rs");
include!("ETHTOOL_PHY_FAST_LINK_DOWN_.rs");
include!("ethtool_rxnfc_rule_count_or_rss_context.rs");
include!("ethtool_set_features_block.rs");
include!("ethtool_stringset.rs");
include!("ETHTOOL_x_LEN.rs");
include!("fr_proto.rs");
include!("fr_proto_pvc.rs");
include!("fr_proto_pvc_info.rs");
include!("ForwardErrorCorrectionCode.rs");
include!("hwtstamp_rx_filters.rs");
include!("hwtstamp_tx_types.rs");
include!("IF_PORT.rs");
include!("if_settings.rs");
include!("if_settings_ifsu.rs");
include!("IFALIASZ.rs");
include!("IFF_.rs");
include!("ifmap.rs");
include!("ifreq.rs");
include!("ifreq_ifrn.rs");
include!("ifreq_ifru.rs");
include!("in6_addr_gen_mode.rs");
include!("MAX_ADDR_LEN.rs");
include!("MAX_NUM_QUEUE.rs");
include!("MAX_PHYS_ITEM_ID_LEN.rs");
include!("net_device_flags.rs");
include!("NET_ADDR.rs");
include!("NET_NAME.rs");
include!("NETIF_F.rs");
include!("NETIF_MSG.rs");
include!("PFC_STORM_PREVENTION_.rs");
include!("phy_tunable_id.rs");
include!("PORT.rs");
include!("raw_hdlc_proto.rs");
include!("SIOC.rs");
include!("SOF_TIMESTAMPING.rs");
include!("SPEED.rs");
include!("sync_serial_settings.rs");
include!("te1_settings.rs");
include!("tunable_id.rs");
include!("tunable_type_id.rs");
include!("WAKE.rs");
include!("x25_hdlc_proto.rs");
include!("XCVR.rs");


pub(crate) mod bit_set;


/// Ethtool commands.
pub mod commands;


pub(crate) mod flow_specifications;
