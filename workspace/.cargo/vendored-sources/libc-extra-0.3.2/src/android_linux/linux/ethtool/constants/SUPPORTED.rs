// This file is part of libc-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT. No part of libc-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of libc-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT.


pub const SUPPORTED_10baseT_Half: c_uchar = 1;
pub const SUPPORTED_10baseT_Full: c_uchar = 2;
pub const SUPPORTED_100baseT_Half: c_uchar = 4;
pub const SUPPORTED_100baseT_Full: c_uchar = 8;
pub const SUPPORTED_1000baseT_Half: c_uchar = 16;
pub const SUPPORTED_1000baseT_Full: c_uchar = 32;
pub const SUPPORTED_Autoneg: c_uchar = 64;
pub const SUPPORTED_TP: c_uchar = 128;
pub const SUPPORTED_AUI: c_ushort = 256;
pub const SUPPORTED_MII: c_ushort = 512;
pub const SUPPORTED_FIBRE: c_ushort = 1024;
pub const SUPPORTED_BNC: c_ushort = 2048;
pub const SUPPORTED_10000baseT_Full: c_ushort = 4096;
pub const SUPPORTED_Pause: c_ushort = 8192;
pub const SUPPORTED_Asym_Pause: c_ushort = 16384;
pub const SUPPORTED_2500baseX_Full: c_ushort = 32768;
pub const SUPPORTED_Backplane: c_uint = 65536;
pub const SUPPORTED_1000baseKX_Full: c_uint = 131072;
pub const SUPPORTED_10000baseKX4_Full: c_uint = 262144;
pub const SUPPORTED_10000baseKR_Full: c_uint = 524288;
pub const SUPPORTED_10000baseR_FEC: c_uint = 1048576;
pub const SUPPORTED_20000baseMLD2_Full: c_uint = 2097152;
pub const SUPPORTED_20000baseKR2_Full: c_uint = 4194304;
pub const SUPPORTED_40000baseKR4_Full: c_uint = 8388608;
pub const SUPPORTED_40000baseCR4_Full: c_uint = 16777216;
pub const SUPPORTED_40000baseSR4_Full: c_uint = 33554432;
pub const SUPPORTED_40000baseLR4_Full: c_uint = 67108864;
pub const SUPPORTED_56000baseKR4_Full: c_uint = 134217728;
pub const SUPPORTED_56000baseCR4_Full: c_uint = 268435456;
pub const SUPPORTED_56000baseSR4_Full: c_uint = 536870912;
pub const SUPPORTED_56000baseLR4_Full: c_uint = 1073741824;
