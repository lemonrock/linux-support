// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use self::IORING_OP::*;
use self::c::*;
use self::non_null::*;
use self::offsets::*;
use super::LoadNonAtomically;
use crate::cpu::HyperThread;
use crate::file_descriptors::*;
use crate::file_descriptors::block_device::BlockDeviceFileDescriptor;
use crate::file_descriptors::character_device::CharacterDeviceFileDescriptor;
use crate::file_descriptors::directory::*;
use crate::file_descriptors::directory::c::*;
use crate::file_descriptors::epoll::*;
use crate::file_descriptors::epoll::c::*;
use crate::file_descriptors::eventfd::EventFileDescriptor;
use crate::file_descriptors::file::*;
use crate::file_descriptors::memfd::MemoryFileDescriptor;
use crate::file_descriptors::path::PathFileDescriptor;
use crate::file_descriptors::pipes_and_fifos::*;
use crate::file_descriptors::socket::*;
use crate::file_descriptors::socket::c::*;
use crate::file_descriptors::terminal::TerminalFileDescriptor;
use crate::io_priority::CompressedIoPriority;
use crate::memory::huge_pages::DefaultPageSizeAndHugePageSizes;
use crate::memory::huge_pages::PageSizeOrHugePageSizeSettings;
use crate::memory::mapping::*;
use crate::poll::*;


mod c;


mod non_null;


/// Offsets.
pub mod offsets;


include!("BufferGroup.rs");
include!("CompletionResponse.rs");
include!("CompletionQueueEntry.rs");
include!("CompletionQueueRing.rs");
include!("CompletionQueueRingIterator.rs");
include!("FileDescriptorKind.rs");
include!("FileDescriptorOrigin.rs");
include!("FileSynchronize.rs");
include!("IORING_OP.rs");
include!("IoUring.rs");
include!("IoUringCreationError.rs");
include!("IoUringFileDescriptor.rs");
include!("LinuxKernelSubmissionQueuePollingThreadConfiguration.rs");
include!("OpenOnDisk.rs");
include!("PersonalityCredentialsIdentifier.rs");
include!("RegisteredBufferIndex.rs");
include!("RegisteredFileDescriptorIndex.rs");
include!("RelativeOrAbsoluteTimeout.rs");
include!("SendOrReceiveFlags.rs");
include!("SubmissionQueueRing.rs");
include!("SubmissionQueueEntry.rs");
include!("SubmissionQueueEntryFlags.rs");
include!("SubmissionQueueEntryOptions.rs");
include!("SubmitError.rs");
include!("SupportedFileDescriptor.rs");
include!("UserData.rs");
