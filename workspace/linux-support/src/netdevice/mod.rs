// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use self::c::*;
use super::*;
use crate::file_descriptors::socket::c::sockaddr;
use crate::file_descriptors::{FileDescriptor, CreationError};
use crate::file_descriptors::socket::new_socket;


pub(crate) mod c;


include!("BusDeviceAddress.rs");
include!("EthToolString.rs");
include!("EthToolStringFromBytesError.rs");
include!("NetworkDeviceInputOutputControlError.rs");
include!("NetworkDeviceSocketFileDescriptor.rs");
include!("NetworkInterfaceIndex.rs");
include!("NetworkInterfaceIndexToNetworkInterfaceNameError.rs");
include!("NetworkInterfaceName.rs");
include!("NetworkInterfaceNameToSomethingError.rs");
