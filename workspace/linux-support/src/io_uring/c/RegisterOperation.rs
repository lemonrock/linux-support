// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub(crate) enum RegisterOperation
{
	RegisterPersonality = IORING_REGISTER_PERSONALITY,

	UnregisterPersonality = IORING_UNREGISTER_PERSONALITY,

	RegisterBuffers = IORING_REGISTER_BUFFERS,

	UnregisterBuffers = IORING_UNREGISTER_BUFFERS,

	RegisterFiles = IORING_REGISTER_FILES,

	RegisterFilesUpdate = IORING_REGISTER_FILES_UPDATE,

	UnregisterFiles = IORING_UNREGISTER_FILES,

	RegisterEventFileDescriptor = IORING_REGISTER_EVENTFD,

	RegisterEventFileDescriptorAsynchronous = IORING_REGISTER_EVENTFD_ASYNC,

	UnregisterEventFileDescriptor = IORING_UNREGISTER_EVENTFD,

	RegisterProbe = IORING_REGISTER_PROBE,
}
