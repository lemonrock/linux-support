// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use crate::file_descriptors::*;
use crate::file_descriptors::network_device::*;


pub(crate) mod c;


include!("BusDeviceAddress.rs");
include!("NetworkDeviceInputOutputControlError.rs");
include!("NetworkInterfaceIndex.rs");
include!("NetworkInterfaceIndexToNetworkInterfaceNameError.rs");
include!("NetworkInterfaceName.rs");
include!("NetworkInterfaceNameToSomethingError.rs");
