// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Supported `ioctl` requests.
	#[derive(Deserialize, Serialize)]
	#[serde(deny_unknown_fields)]
	pub struct SupportedInputOutputControlRequests: u64
	{
		/// Equivalent to `UFFD_API_IOCTLS` (which has not been defined in this module).
		const ApplicationProgrammerInterfaces = (InputOutputControlRequest::ApplicationProgrammerInterface as u64) | (InputOutputControlRequest::Register as u64) | (InputOutputControlRequest::Unregister as u64);
		
		/// Equivalent to `UFFD_API_RANGE_IOCTLS_BASIC` (which has not been defined in this module).
		///
		/// All registered memory can use these ioctls.
		const HugePages = (InputOutputControlRequest::Wake as u64) | (InputOutputControlRequest::Copy as u64);
		
		/// Only registed memory that is not using huge pages can use these ioctls.
		const RegularPages = Self::HugePages.bits | (InputOutputControlRequest::ZeroPageCopy as u64);
		
		/// Equivalent to `UFFD_API_RANGE_IOCTLS` (which has not been defined in this module).
		///
		/// Only registed memory that is not using huge pages can use these ioctls and was registered with `register_mode` containing `PageFaultEventNotificationSetting::RaisePageFaultEventIfWriteProtectedPageAccessed`.
		const RegularPagesWithWriteProtectOnCopy = Self::RegularPages.bits | (InputOutputControlRequest::WriteProtect as u64);
	}
}

impl Default for SupportedInputOutputControlRequests
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::empty()
	}
}

impl SupportedInputOutputControlRequests
{
	/// Supports an `ioctl` request?
	#[inline(always)]
	pub fn supports(self, input_output_control_request: InputOutputControlRequest) -> bool
	{
		self.contains(unsafe { transmute(input_output_control_request) })
	}
}
