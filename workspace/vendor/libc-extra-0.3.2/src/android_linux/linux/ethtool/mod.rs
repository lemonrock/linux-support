// This file is part of libc-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT. No part of libc-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of libc-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT.


#![allow(non_camel_case_types)] 

use ::std::default::Default;
use ::std::mem::transmute;
use ::std::mem::zeroed;
use ::libc::c_char;
use ::libc::c_uchar;
use ::libc::c_uint;
use ::libc::c_ulonglong;
use ::libc::c_ushort;
use ::libc::c_void;
use ::libc::size_t;
use ::libc::ssize_t;
use ::android_linux::netinet::if_ether::ethhdr;


include!("constants/ADVERTISED.rs");
include!("constants/ETH_MDIO_SUPPORTS.rs");
include!("constants/ETH_MODULE_SFF.rs");
include!("constants/ETH_TP_MDI.rs");
include!("constants/FLOW.rs");
include!("constants/lengthsAndMaxima.rs");
include!("constants/miscellany.rs");
include!("constants/PORT.rs");
include!("constants/RXH.rs");
include!("constants/RX_CLS.rs");
include!("constants/SPEED.rs");
include!("constants/SUPPORTED.rs");
include!("constants/WAKE.rs");
include!("constants/XCVR.rs");

include!("ethtool_cmd.rs");
include!("ethtool_coalesce.rs");
include!("ethtool_drvinfo.rs");
include!("ethtool_flow_union.rs");
include!("ethtool_ringparam.rs");
include!("ethtool_rx_ntuple.rs");
include!("ethtool_rx_ntuple_flow_spec.rs");
include!("ethtool_value.rs");
include!("ethtool_wolinfo.rs");


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum tunable_id
{
	ETHTOOL_ID_UNSPEC = 0,
	ETHTOOL_RX_COPYBREAK = 1,
	ETHTOOL_TX_COPYBREAK = 2,
	__ETHTOOL_TUNABLE_COUNT = 3,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum tunable_type_id
{
	ETHTOOL_TUNABLE_UNSPEC = 0,
	ETHTOOL_TUNABLE_U8 = 1,
	ETHTOOL_TUNABLE_U16 = 2,
	ETHTOOL_TUNABLE_U32 = 3,
	ETHTOOL_TUNABLE_U64 = 4,
	ETHTOOL_TUNABLE_STRING = 5,
	ETHTOOL_TUNABLE_S8 = 6,
	ETHTOOL_TUNABLE_S16 = 7,
	ETHTOOL_TUNABLE_S32 = 8,
	ETHTOOL_TUNABLE_S64 = 9,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ethtool_tunable
{
	pub cmd: u32,
	pub id: u32,
	pub type_id: u32,
	pub len: u32,
	pub data: [*mut c_void; 0],
}

impl Default for ethtool_tunable
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ethtool_regs
{
	pub cmd: u32,
	pub version: u32,
	pub len: u32,
	pub data: [u8; 0],
}

impl Default for ethtool_regs
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ethtool_eeprom
{
	pub cmd: u32,
	pub magic: u32,
	pub offset: u32,
	pub len: u32,
	pub data: [u8; 0],
}

impl Default for ethtool_eeprom
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ethtool_eee
{
	pub cmd: u32,
	pub supported: u32,
	pub advertised: u32,
	pub lp_advertised: u32,
	pub eee_active: u32,
	pub eee_enabled: u32,
	pub tx_lpi_enabled: u32,
	pub tx_lpi_timer: u32,
	pub reserved: [u32; 2],
}

impl Default for ethtool_eee
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ethtool_modinfo
{
	pub cmd: u32,
	pub type_: u32,
	pub eeprom_len: u32,
	pub reserved: [u32; 8],
}

impl Default for ethtool_modinfo
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ethtool_channels
{
	pub cmd: u32,
	pub max_rx: u32,
	pub max_tx: u32,
	pub max_other: u32,
	pub max_combined: u32,
	pub rx_count: u32,
	pub tx_count: u32,
	pub other_count: u32,
	pub combined_count: u32,
}

impl Default for ethtool_channels
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ethtool_pauseparam
{
	pub cmd: u32,
	pub autoneg: u32,
	pub rx_pause: u32,
	pub tx_pause: u32,
}

impl Default for ethtool_pauseparam
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ethtool_stringset
{
	ETH_SS_TEST = 0,
	ETH_SS_STATS = 1,
	ETH_SS_PRIV_FLAGS = 2,
	ETH_SS_NTUPLE_FILTERS = 3,
	ETH_SS_FEATURES = 4,
	ETH_SS_RSS_HASH_FUNCS = 5,
	ETH_SS_TUNABLES = 6,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ethtool_gstrings
{
	pub cmd: u32,
	pub string_set: u32,
	pub len: u32,
	pub data: [u8; 0],
}

impl Default for ethtool_gstrings
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ethtool_sset_info
{
	pub cmd: u32,
	pub reserved: u32,
	pub sset_mask: u64,
	pub data: [u32; 0],
}

impl Default for ethtool_sset_info
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ethtool_test_flags
{
	ETH_TEST_FL_OFFLINE = 1,
	ETH_TEST_FL_FAILED = 2,
	ETH_TEST_FL_EXTERNAL_LB = 4,
	ETH_TEST_FL_EXTERNAL_LB_DONE = 8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ethtool_test
{
	pub cmd: u32,
	pub flags: u32,
	pub reserved: u32,
	pub len: u32,
	pub data: [u64; 0],
}

impl Default for ethtool_test
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ethtool_stats
{
	pub cmd: u32,
	pub n_stats: u32,
	pub data: [u64; 0],
}

impl Default for ethtool_stats
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ethtool_perm_addr
{
	pub cmd: u32,
	pub size: u32,
	pub data: [u8; 0],
}

impl Default for ethtool_perm_addr
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ethtool_flags
{
	ETH_FLAG_TXVLAN = 128,
	ETH_FLAG_RXVLAN = 256,
	ETH_FLAG_LRO = 32768,
	ETH_FLAG_NTUPLE = 134217728,
	ETH_FLAG_RXHASH = 268435456,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ethtool_tcpip4_spec
{
	pub ip4src: u32, // Big Endian Always
	pub ip4dst: u32, // Big Endian Always
	pub psrc: u16, // Big Endian Always
	pub pdst: u16, // Big Endian Always
	pub tos: u8,
}

impl Default for ethtool_tcpip4_spec
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ethtool_ah_espip4_spec
{
	pub ip4src: u32, // Big Endian Always
	pub ip4dst: u32, // Big Endian Always
	pub spi: u32, // Big Endian Always
	pub tos: u8,
}

impl Default for ethtool_ah_espip4_spec
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ethtool_usrip4_spec
{
	pub ip4src: u32, // Big Endian Always
	pub ip4dst: u32, // Big Endian Always
	pub l4_4_bytes: u32, // Big Endian Always
	pub tos: u8,
	pub ip_ver: u8,
	pub proto: u8,
}

impl Default for ethtool_usrip4_spec
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ethtool_flow_ext
{
	pub padding: [u8; 2],
	pub h_dest: [c_uchar; 6],
	pub vlan_etype: u16, // Big Endian Always
	pub vlan_tci: u16, // Big Endian Always
	pub data: [u32; 2], // Big Endian Always
}

impl Default for ethtool_flow_ext
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

#[repr(C)]
#[derive(Copy)]
#[allow(missing_debug_implementations)]
pub struct ethtool_rx_flow_spec
{
	pub flow_type: u32,
	pub h_u: ethtool_flow_union,
	pub h_ext: ethtool_flow_ext,
	pub m_u: ethtool_flow_union,
	pub m_ext: ethtool_flow_ext,
	pub ring_cookie: u64,
	pub location: u32,
}

impl Clone for ethtool_rx_flow_spec
{
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for ethtool_rx_flow_spec
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

#[repr(C)]
#[derive(Copy)]
#[allow(missing_debug_implementations)]
pub struct ethtool_rxnfc
{
	pub cmd: u32,
	pub flow_type: u32,
	pub data: u64,
	pub fs: ethtool_rx_flow_spec,
	pub rule_cnt: u32,
	pub rule_locs: [u32; 0],
}

impl Clone for ethtool_rxnfc
{
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for ethtool_rxnfc
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ethtool_rxfh_indir
{
	pub cmd: u32,
	pub size: u32,
	pub ring_index: [u32; 0],
}

impl Default for ethtool_rxfh_indir
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ethtool_rxfh
{
	pub cmd: u32,
	pub rss_context: u32,
	pub indir_size: u32,
	pub key_size: u32,
	pub hfunc: u8,
	pub rsvd8: [u8; 3],
	pub rsvd32: u32,
	pub rss_config: [u32; 0],
}

impl Default for ethtool_rxfh
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ethtool_flash_op_type
{
	ETHTOOL_FLASH_ALL_REGIONS = 0,
}

#[repr(C)]
#[derive(Copy)]
#[allow(missing_debug_implementations)]
pub struct ethtool_flash
{
	pub cmd: u32,
	pub region: u32,
	pub data: [c_char; 128],
}

impl Clone for ethtool_flash
{
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for ethtool_flash
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ethtool_dump
{
	pub cmd: u32,
	pub version: u32,
	pub flag: u32,
	pub len: u32,
	pub data: [u8; 0],
}

impl Default for ethtool_dump
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ethtool_get_features_block
{
	pub available: u32,
	pub requested: u32,
	pub active: u32,
	pub never_changed: u32,
}

impl Default for ethtool_get_features_block
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ethtool_gfeatures
{
	pub cmd: u32,
	pub size: u32,
	pub features: [ethtool_get_features_block; 0],
}

impl Default for ethtool_gfeatures
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ethtool_set_features_block
{
	pub valid: u32,
	pub requested: u32,
}

impl Default for ethtool_set_features_block
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ethtool_sfeatures
{
	pub cmd: u32,
	pub size: u32,
	pub features: [ethtool_set_features_block; 0],
}

impl Default for ethtool_sfeatures
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ethtool_ts_info
{
	pub cmd: u32,
	pub so_timestamping: u32,
	pub phc_index: i32,
	pub tx_types: u32,
	pub tx_reserved: [u32; 3],
	pub rx_filters: u32,
	pub rx_reserved: [u32; 3],
}

impl Default for ethtool_ts_info
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ethtool_sfeatures_retval_bits
{
	ETHTOOL_F_UNSUPPORTED__BIT = 0,
	ETHTOOL_F_WISH__BIT = 1,
	ETHTOOL_F_COMPAT__BIT = 2,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ethtool_reset_flags
{
	ETH_RESET_MGMT = 1,
	ETH_RESET_IRQ = 2,
	ETH_RESET_DMA = 4,
	ETH_RESET_FILTER = 8,
	ETH_RESET_OFFLOAD = 16,
	ETH_RESET_MAC = 32,
	ETH_RESET_PHY = 64,
	ETH_RESET_RAM = 128,
	ETH_RESET_DEDICATED = 65535,
	ETH_RESET_ALL = 4294967295,
}
