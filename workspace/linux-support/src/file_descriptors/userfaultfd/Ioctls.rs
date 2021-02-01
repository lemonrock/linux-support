// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Ioctls supported.
	#[derive(Deserialize, Serialize)]
	#[serde(deny_unknown_fields)]
	pub struct Ioctls: u64
	{
		/// Register
		const Register = _UFFDIO_REGISTER;
		
		/// Register
		const Unregister = _UFFDIO_UNREGISTER;
		
		/// API.
		const ApplicationProgrammerInterface = _UFFDIO_API;
		
		/// Wake.
		const Wake = _UFFDIO_WAKE;
		
		/// Copy.
		const Copy = _UFFDIO_COPY;
		
		/// Zero page.
		const ZeroPage = _UFFDIO_ZEROPAGE;
		
		/// Write Protect.
		const WriteProtect = _UFFDIO_WRITEPROTECT;
		
		/// Equivalent to `UFFD_API_IOCTLS` (which has not been defined in this module).
		const ApplicationProgrammerInterfaces = Self::Register.bits | Self::Unregister.bits | Self::ApplicationProgrammerInterface.bits;
		
		/// Equivalent to `UFFD_API_RANGE_IOCTLS_BASIC` (which has not been defined in this module).
		const ApplicationProgrammerInterfacesRangeBasic = Self::Wake.bits | Self::Copy.bits;
		
		/// Equivalent to `UFFD_API_RANGE_IOCTLS` (which has not been defined in this module).
		const ApplicationProgrammerInterfacesRange = Self::ApplicationProgrammerInterfacesRangeBasic.bits | Self::ZeroPage.bits | Self::WriteProtect.bits;
	}
}
