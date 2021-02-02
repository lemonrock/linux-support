// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Ioctls supported.
	#[derive(Deserialize, Serialize)]
	#[serde(deny_unknown_fields)]
	pub struct Ioctls: u64
	{
		/// Register memory.
		const Register = _UFFDIO_REGISTER;
		
		/// Unregister memory.
		///
		/// Only available for registered memory.
		const Unregister = _UFFDIO_UNREGISTER;
		
		/// API.
		const ApplicationProgrammerInterface = _UFFDIO_API;
		
		/// Wake.
		///
		/// Only available for registered memory.
		const Wake = _UFFDIO_WAKE;
		
		/// Copy.
		///
		/// Only available for registered memory.
		const Copy = _UFFDIO_COPY;
		
		/// Zero page copy.
		///
		/// Only available for registered memory which does not use huge pages.
		const ZeroPageCopy = _UFFDIO_ZEROPAGE;
		
		/// Write Protect.
		///
		/// Only available for registered memory which does not use huge pages and if `RegisterMode::AllowWriteProtectedCopying` was specified on registration.
		const WriteProtectOnCopy = _UFFDIO_WRITEPROTECT;
		
		/// Equivalent to `UFFD_API_IOCTLS` (which has not been defined in this module).
		const ApplicationProgrammerInterfaces = Self::ApplicationProgrammerInterface.bits | Self::Register.bits | Self::Unregister.bits;
		
		/// Equivalent to `UFFD_API_RANGE_IOCTLS_BASIC` (which has not been defined in this module).
		///
		/// All registered memory can use these ioctls.
		const HugePages = Self::Wake.bits | Self::Copy.bits;
		
		/// Only registed memory that is not using huge pages can use these ioctls.
		const RegularPages = Self::HugePages.bits | Self::ZeroPage.bits;
		
		/// Equivalent to `UFFD_API_RANGE_IOCTLS` (which has not been defined in this module).
		///
		/// Only registed memory that is not using huge pages can use these ioctls and was registered with `register_mode` containing `RegisterMode::AllowWriteProtectedCopying`.
		const RegularPagesWithWriteProtectOnCopy = Self::RegularPages.bits | Self::WriteProtect.bits;
	}
}
