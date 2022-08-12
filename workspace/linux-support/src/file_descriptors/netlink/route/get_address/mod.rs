// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use crate::network_device::{NetworkInterfaceName, NetNamespaceIdentifier, InternetProtocolAddressLifetime};


include!("CacheTimestampInHundrethsOfSeconds.rs");
include!("ExtendedInterfaceFlags.rs");
include!("GetInternetProtocolVersion6AddressMessageData.rs");
include!("GetAddressMessageDataCommon.rs");
include!("GetAddressMessageDataUnicastCommon.rs");
include!("GetAddressProcessingMessageState.rs");
include!("GetAddressProcessingMessageStateCommon.rs");
include!("GetAddressProcessingMessageStateOtherCastInternetProtocolVersion6.rs");
include!("GetAddressProcessingMessageStateUnicastCommon.rs");
include!("GetInternetProtocolVersion4AddressMessageData.rs");
include!("GetInternetProtocolVersion6OtherCastAddressMessageData.rs");
include!("InterfaceFlags.rs");
