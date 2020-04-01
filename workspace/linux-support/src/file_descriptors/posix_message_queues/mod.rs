// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


use super::*;
use self::syscall::*;
use libc::c_uint;
use libc::ENAMETOOLONG;
use libc::EEXIST;
use libc::EMSGSIZE;
use libc::ETIMEDOUT;
use libc::mode_t;
use libc::O_CREAT;
use libc::O_EXCL;
use libc::S_IRUSR;
use libc::S_IWUSR;
use std::convert::TryFrom;


mod syscall;


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
