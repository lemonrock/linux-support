// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use self::bit_set::*;
use super::features::*;
use super::tunables::*;
use crate::file_descriptors::socket::c::*;


include!("AUTONEG.rs");
include!("cisco_proto.rs");
include!("DOWNSHIFT_DEV_.rs");
include!("DUPLEX.rs");
include!("ETH_GSTRING_LEN.rs");
include!("ETH_MDIO_SUPPORTS.rs");
include!("ETH_TP_MDI.rs");
include!("ETHTOOL_.rs");
include!("ethtool_channels.rs");
include!("ethtool_coalesce.rs");
include!("ethtool_drvinfo.rs");
include!("ethtool_eee.rs");
include!("ethtool_fec_config_bits.rs");
include!("ethtool_fecparam.rs");
include!("ethtool_get_features_block.rs");
include!("ethtool_gfeatures.rs");
include!("ethtool_gstrings.rs");
include!("ethtool_link_mode_bit_indices.rs");
include!("ethtool_link_mode_bit_indices_forward_error_correction.rs");
include!("ethtool_link_mode_bit_indices_pause.rs");
include!("ethtool_link_mode_bit_indices_ports.rs");
include!("ethtool_link_mode_bit_indices_speed.rs");
include!("ethtool_link_settings.rs");
include!("ethtool_link_settings_link_modes.rs");
include!("ethtool_pauseparam.rs");
include!("ETHTOOL_PHY_EDPD_.rs");
include!("ETHTOOL_PHY_FAST_LINK_DOWN_.rs");
include!("ethtool_ringparam.rs");
include!("ethtool_set_features_block.rs");
include!("ethtool_sfeatures.rs");
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
include!("ForwardErrorCorrectionCode.rs");
include!("hwtstamp_rx_filters.rs");
include!("hwtstamp_tx_types.rs");
include!("IF_PORT.rs");
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
include!("NET_ADDR.rs");
include!("NET_NAME.rs");
include!("NETIF_F.rs");
include!("NETIF_MSG.rs");
include!("PFC_STORM_PREVENTION_.rs");
include!("phy_tunable_id.rs");
include!("PORT.rs");
include!("raw_hdlc_proto.rs");
include!("SOF_TIMESTAMPING.rs");
include!("SPEED.rs");
include!("sync_serial_settings.rs");
include!("te1_settings.rs");
include!("tunable_id.rs");
include!("tunable_type_id.rs");
include!("VariablySizedEthtoolCommand.rs");
include!("VariablySizedEthtoolCommandWrapper.rs");
include!("WAKE.rs");
include!("x25_hdlc_proto.rs");
include!("XCVR.rs");


mod bit_set;
