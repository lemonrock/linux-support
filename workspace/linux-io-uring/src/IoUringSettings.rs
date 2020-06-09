// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// io_uring settings.
#[derive(Debug)]
pub struct IoUringSettings
{
	ideal_maximum_number_of_coroutines: NonZeroU64,
	number_of_submission_queue_entries: NonZeroU16,
	number_of_completion_queue_entries: Option<NonZeroU32>,
	kernel_submission_queue_thread_configuration: Option<LinuxKernelSubmissionQueuePollingThreadConfiguration>,
	registered_buffer_settings: RegisteredBufferSettings,
}

impl IoUringSettings
{
	fn setup(&self, defaults: &DefaultPageSizeAndHugePageSizes) -> Result<(Rc<IoUring>, RegisteredBuffers), IoUringSetupError>
	{
		let io_uring = IoUring::new(&defaults, self.number_of_submission_queue_entries, self.number_of_completion_queue_entries, self.kernel_submission_queue_thread_configuration.as_ref(), None)?;
		let registered_buffers = RegisteredBuffers::new(&self.registered_buffer_settings, defaults)?;
		registered_buffers.register(&io_uring).map_err(IoUringSetupError::RegisteringBuffers);
		Ok((Rc::new(io_uring), registered_buffers))
	}
}
