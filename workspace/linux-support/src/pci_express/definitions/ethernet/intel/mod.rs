// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use crate::pci_express::PciDevicePhysicalOrVirtualFunction;


/// Intel Ethernet Controller 'mp' register definitions.
pub mod mp;

/// Intel Ethernet Controller 700 Series register definitions.
pub mod Intel700Series;

/// Intel® Ethernet Controller 200 and 300 Series device identifiers.
pub mod Intel200SeriesAnd300SeriesPciDeviceIdentifier;


include!("Intel200SeriesAnd300SeriesMediaAccessControlBlockType.rs");

// See e1000_hw.h in FreeBSD igb driver (which uses old e1000 naming to be confusing).
