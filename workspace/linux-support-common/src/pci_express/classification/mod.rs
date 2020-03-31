// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use self::subclasses::*;
use crate::strings::FromBytes;
use crate::strings::parse_number::*;
use const_fn_assert::cfn_assert_ne;
use either::Either;
use likely::unlikely;
use serde::Deserialize;
use serde::Serialize;
use std::mem::size_of;


/// Subclass definitions.
pub mod subclasses;


include!("PciDeviceClass.rs");
include!("PciDeviceIdentifier.rs");
include!("PciDeviceType.rs");
include!("PciVendorAndDevice.rs");
include!("PciVendorIdentifier.rs");
include!("Revision.rs");
