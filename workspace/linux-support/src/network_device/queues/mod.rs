// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use crate::cpu::HyperThreads;
use crate::configuration::Milliseconds;


include!("CommonTransmitQueueConfiguration.rs");
include!("ExpressDataPathQueueIdentifier.rs");
include!("GlobalNetworkDeviceReceiveQueueConfiguration.rs");
include!("GlobalNetworkDeviceTransmitQueueConfiguration.rs");
include!("QueueCount.rs");
include!("QueueIdentifier.rs");
include!("QueueIdentifiers.rs");
include!("ReceiveSysfsQueue.rs");
include!("SysfsQueue.rs");
include!("TransmitQueueTrafficClass.rs");
include!("TransmitSysfsQueue.rs");
