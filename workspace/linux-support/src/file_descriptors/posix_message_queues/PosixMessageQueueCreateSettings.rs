// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Settings for creating a queue.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct PosixMessageQueueCreateSettings
{
	/// File-like permissions to use.
	pub permissions: mode_t,

	/// Optional create settings.
	///
	/// If `None`, then Linux applies a default (see documentation of fields on `OptionalPosixMessageQueueCreateSettings`).
	pub optional_create_settings: Option<OptionalPosixMessageQueueCreateSettings>,
}

impl Default for PosixMessageQueueCreateSettings
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			permissions: S_IRUSR | S_IWUSR,
			optional_create_settings: None,
		}
	}
}

impl PosixMessageQueueCreateSettings
{
	#[inline(always)]
	pub(crate) fn invoke_mq_open(&self, name_pointer: *const c_char, oflag: i32) -> c_int
	{
		let mode = self.permissions;

		match self.optional_create_settings
		{
			None => unsafe { mq_open(name_pointer, oflag, mode, null_mut::<mq_attr>()) },

			Some(ref optional_create_settings) =>
			{
				let mut attributes = mq_attr::for_create(optional_create_settings);
				unsafe { mq_open(name_pointer, oflag, mode, &mut attributes) }
			}
		}
	}
}
