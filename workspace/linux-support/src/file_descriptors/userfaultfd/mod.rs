// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use self::c::*;
use self::events_reader_and_dispatcher::*;
use crate::memory::AbsoluteMemoryRange;
use crate::memory::FastAbsoluteMemoryRange;
use crate::memory::VirtualAddress;
use crate::poll::PollRequestFlags;
use crate::poll::PollResponseFlags;
use crate::thread::ThreadIdentifier;


mod c;


/// Supporting logic for reading and dispatching events.
pub mod events_reader_and_dispatcher;


include!("BlockingUserFaultFileDescriptor.rs");
include!("BlockingUserFaultFileDescriptorCreationError.rs");
include!("CopyError.rs");
include!("CopyMode.rs");
include!("Feature.rs");
include!("Features.rs");
include!("InputOutputControlRequest.rs");
include!("NonBlockingUserFaultFileDescriptor.rs");
include!("PageFaultEventType.rs");
include!("PageFaultEventNotificationSetting.rs");
include!("SupportedInputOutputControlRequests.rs");
include!("UserFaultEvent.rs");
include!("UserFaultEventHandler.rs");
include!("UserFaultFileDescriptor.rs");
include!("WriteProtectMode.rs");
include!("ZeroPageMode.rs");
