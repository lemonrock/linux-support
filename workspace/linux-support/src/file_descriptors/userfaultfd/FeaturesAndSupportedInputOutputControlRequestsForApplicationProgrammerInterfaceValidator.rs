// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Used to guard `features` and `supported_input_output_control_requests_for_application_programmer_interface`.
pub trait FeaturesAndSupportedInputOutputControlRequestsForApplicationProgrammerInterfaceValidator
{
	/// Validation error type.
	type Error: error::Error;
	
	/// Validates.
	///
	/// * `features` is a list of supported features for the `UserFaultFileDescriptor`; it is always `Feature::all()` if running on Linux version 5.11.
	/// * `supported_input_output_control_requests_for_application_programmer_interface` is a list of supported input-output control requests (ioctl); it is always `SupportedInputOutputControlRequests::ApplicationProgrammerInterfaces`.
	fn validate(self, features: Features, supported_input_output_control_requests_for_application_programmer_interface: SupportedInputOutputControlRequests) -> Result<(), Self::Error>;
}

impl<F: FnOnce(Features, SupportedInputOutputControlRequests)> FeaturesAndSupportedInputOutputControlRequestsForApplicationProgrammerInterfaceValidator for F
{
	type Error = Infallible;
	
	#[inline(always)]
	fn validate(self, features: Features, supported_input_output_control_requests_for_application_programmer_interface: SupportedInputOutputControlRequests) -> Result<(), Self::Error>
	{
		(self)(features, supported_input_output_control_requests_for_application_programmer_interface);
		Ok(())
	}
}
