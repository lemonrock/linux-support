// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


use super::*;
use self::c::*;
use crate::paths::*;
use crate::user_and_groups::assert_effective_user_id_is_root;


mod c;


include!("default_maximum_message_size.rs");
include!("default_maximum_number_of_messages_in_a_queue.rs");
include!("maximum_maximum_message_size.rs");
include!("maximum_maximum_number_of_messages_in_a_queue.rs");
include!("maximum_number_of_queues.rs");
include!("PosixMessagePriority.rs");
include!("PosixMessageQueue.rs");
include!("PosixMessageQueueConstraints.rs");
include!("PosixMessageQueueCreateSendOrReceive.rs");
include!("PosixMessageQueueCreateSettings.rs");
include!("PosixMessageQueueFileDescriptor.rs");
include!("PosixMessageQueueUnlinkError.rs");
include!("OpenOrCreatePosixMessageQueue.rs");
include!("OptionalPosixMessageQueueCreateSettings.rs");
include!("Receive.rs");
include!("ReceivePosixMessageQueueFileDescriptor.rs");
include!("Send.rs");
include!("SendAndReceivePosixMessageQueueFileDescriptor.rs");
include!("SendPosixMessageQueueFileDescriptor.rs");
include!("set_default_maximum_message_size.rs");
include!("set_maximum_maximum_message_size.rs");
include!("set_default_maximum_number_of_messages_in_a_queue.rs");
include!("set_maximum_maximum_number_of_messages_in_a_queue.rs");
include!("set_maximum_number_of_queues.rs");
