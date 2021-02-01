// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use crate::memory::PageSize;


include!("CompletionQueue.rs");
include!("ConsumerXskRingQueueKind.rs");
include!("FillQueue.rs");
include!("ProducerXskRingQueueKind.rs");
include!("ReceiveQueue.rs");
include!("RingQueueDepth.rs");
include!("RingQueueEntryIndex.rs");
include!("RingQueueIndex.rs");
include!("TransmitQueue.rs");
include!("XskRingQueue.rs");
include!("XskRingQueueKind.rs");
