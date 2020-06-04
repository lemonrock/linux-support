// This file is part of libc-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT. No part of libc-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of libc-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT.



pub const ETH_RX_NFC_IP4: c_uchar = 1;

pub const ETHTOOL_RX_FLOW_SPEC_RING: c_uint = 4294967295;
pub const ETHTOOL_RX_FLOW_SPEC_RING_VF: c_ulonglong = 1095216660480;
pub const ETHTOOL_RX_FLOW_SPEC_RING_VF_OFF: c_uchar = 32;

pub const ETH_RXFH_INDIR_NO_CHANGE: c_uint = 4294967295;

pub const ETHTOOL_RXNTUPLE_ACTION_DROP: c_char = -1;
pub const ETHTOOL_RXNTUPLE_ACTION_CLEAR: c_char = -2;

pub const ETH_FW_DUMP_DISABLE: c_uchar = 0;

pub const ETHTOOL_GSET: u32 = 1;
pub const ETHTOOL_SSET: u32 = 2;
pub const ETHTOOL_GDRVINFO: u32 = 3;
pub const ETHTOOL_GREGS: u32 = 4;
pub const ETHTOOL_GWOL: u32 = 5;
pub const ETHTOOL_SWOL: u32 = 6;
pub const ETHTOOL_GMSGLVL: u32 = 7;
pub const ETHTOOL_SMSGLVL: u32 = 8;
pub const ETHTOOL_NWAY_RST: u32 = 9;
pub const ETHTOOL_GLINK: u32 = 10;
pub const ETHTOOL_GEEPROM: u32 = 11;
pub const ETHTOOL_SEEPROM: u32 = 12;
pub const ETHTOOL_GCOALESCE: u32 = 14;
pub const ETHTOOL_SCOALESCE: u32 = 15;
pub const ETHTOOL_GRINGPARAM: u32 = 16;
pub const ETHTOOL_SRINGPARAM: u32 = 17;
pub const ETHTOOL_GPAUSEPARAM: u32 = 18;
pub const ETHTOOL_SPAUSEPARAM: u32 = 19;
pub const ETHTOOL_GRXCSUM: u32 = 20;
pub const ETHTOOL_SRXCSUM: u32 = 21;
pub const ETHTOOL_GTXCSUM: u32 = 22;
pub const ETHTOOL_STXCSUM: u32 = 23;
pub const ETHTOOL_GSG: u32 = 24;
pub const ETHTOOL_SSG: u32 = 25;
pub const ETHTOOL_TEST: u32 = 26;
pub const ETHTOOL_GSTRINGS: u32 = 27;
pub const ETHTOOL_PHYS_ID: u32 = 28;
pub const ETHTOOL_GSTATS: u32 = 29;
pub const ETHTOOL_GTSO: u32 = 30;
pub const ETHTOOL_STSO: u32 = 31;
pub const ETHTOOL_GPERMADDR: u32 = 32;
pub const ETHTOOL_GUFO: u32 = 33;
pub const ETHTOOL_SUFO: u32 = 34;
pub const ETHTOOL_GGSO: u32 = 35;
pub const ETHTOOL_SGSO: u32 = 36;
pub const ETHTOOL_GFLAGS: u32 = 37;
pub const ETHTOOL_SFLAGS: u32 = 38;
pub const ETHTOOL_GPFLAGS: u32 = 39;
pub const ETHTOOL_SPFLAGS: u32 = 40;
pub const ETHTOOL_GRXFH: u32 = 41;
pub const ETHTOOL_SRXFH: u32 = 42;
pub const ETHTOOL_GGRO: u32 = 43;
pub const ETHTOOL_SGRO: u32 = 44;
pub const ETHTOOL_GRXRINGS: u32 = 45;
pub const ETHTOOL_GRXCLSRLCNT: u32 = 46;
pub const ETHTOOL_GRXCLSRULE: u32 = 47;
pub const ETHTOOL_GRXCLSRLALL: u32 = 48;
pub const ETHTOOL_SRXCLSRLDEL: u32 = 49;
pub const ETHTOOL_SRXCLSRLINS: u32 = 50;
pub const ETHTOOL_FLASHDEV: u32 = 51;
pub const ETHTOOL_RESET: u32 = 52;
pub const ETHTOOL_SRXNTUPLE: u32 = 53;
pub const ETHTOOL_GRXNTUPLE: u32 = 54;
pub const ETHTOOL_GSSET_INFO: u32 = 55;
pub const ETHTOOL_GRXFHINDIR: u32 = 56;
pub const ETHTOOL_SRXFHINDIR: u32 = 57;
pub const ETHTOOL_GFEATURES: u32 = 58;
pub const ETHTOOL_SFEATURES: u32 = 59;
pub const ETHTOOL_GCHANNELS: u32 = 60;
pub const ETHTOOL_SCHANNELS: u32 = 61;
pub const ETHTOOL_SET_DUMP: u32 = 62;
pub const ETHTOOL_GET_DUMP_FLAG: u32 = 63;
pub const ETHTOOL_GET_DUMP_DATA: u32 = 64;
pub const ETHTOOL_GET_TS_INFO: u32 = 65;
pub const ETHTOOL_GMODULEINFO: u32 = 66;
pub const ETHTOOL_GMODULEEEPROM: u32 = 67;
pub const ETHTOOL_GEEE: u32 = 68;
pub const ETHTOOL_SEEE: u32 = 69;
pub const ETHTOOL_GRSSH: u32 = 70;
pub const ETHTOOL_SRSSH: u32 = 71;
pub const ETHTOOL_GTUNABLE: u32 = 72;
pub const ETHTOOL_STUNABLE: u32 = 73;
pub const SPARC_ETH_GSET: u32 = 1;
pub const SPARC_ETH_SSET: u32 = 2;

pub const DUPLEX_HALF: c_uchar = 0;
pub const DUPLEX_FULL: c_uchar = 1;
pub const DUPLEX_UNKNOWN: c_uchar = 255;

pub const AUTONEG_DISABLE: c_uchar = 0;
pub const AUTONEG_ENABLE: c_uchar = 1;

pub const FLOW_EXT: c_uint = 2147483648;
pub const FLOW_MAC_EXT: c_uint = 1073741824;

pub const ETH_RESET_SHARED_SHIFT: c_uchar = 16;
