// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// io_uring settings.
#[allow(missing_docs)]
#[derive(Debug, Clone)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IoUringSettings
{
	/// Maximum value is `IORING_MAX_ENTRIES`, 32768.
	#[serde(default = "IoUringSettings::number_of_submission_queue_entries_default")] pub number_of_submission_queue_entries: NonZeroU16,
	
	/// Maximum value is `IORING_MAX_CQ_ENTRIES`, `2 * IORING_MAX_ENTRIES`.
	///
	/// Defaults to double `number_of_submission_queue_entries`.
	///
	/// Should usually be `None`.
	#[serde(default)] pub number_of_completion_queue_entries: Option<NonZeroU32>,
	
	/// Should nearly always be `None`.
	#[serde(default)] pub kernel_submission_queue_thread_configuration: Option<LinuxKernelSubmissionQueuePollingThreadConfiguration>,
	
	/// Sizings of registered buffers.
	pub registered_buffer_settings: RegisteredBufferSettings,
}

impl IoUringSettings
{
	fn setup(self, defaults: &DefaultPageSizeAndHugePageSizes) -> Result<(Rc<IoUring<'static>>, RegisteredBuffers), IoUringSetupError>
	{
		let io_uring = IoUring::new(defaults, self.number_of_submission_queue_entries, self.number_of_completion_queue_entries, self.kernel_submission_queue_thread_configuration.as_ref(), None)?;
		
		let registered_buffers = self.registered_buffer_settings.create_and_register(defaults, &io_uring)?;
		
		Ok((Rc::new(io_uring), registered_buffers))
	}
	
	/// Default-ish.
	#[inline(always)]
	pub const fn defaultish(registered_buffer_settings: RegisteredBufferSettings) -> Self
	{
		Self
		{
			number_of_submission_queue_entries: Self::number_of_submission_queue_entries_default(),
			number_of_completion_queue_entries: None,
			kernel_submission_queue_thread_configuration: None,
			registered_buffer_settings
		}
	}
	
	#[inline(always)]
	const fn number_of_submission_queue_entries_default() -> NonZeroU16
	{
		new_non_zero_u16(32_768)
	}
}
