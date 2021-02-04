// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A supported `ioctl` request.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u64)]
pub enum InputOutputControlRequest
{
	/// API.
	///
	/// Always supported.
	ApplicationProgrammerInterface = 1 << _UFFDIO_API,
	
	/// Register memory.
	///
	/// Always supported.
	Register = 1 << _UFFDIO_REGISTER,
	
	/// Unregister memory.
	///
	/// Always supported.
	///
	/// Only useful for registered memory.
	Unregister = 1 << _UFFDIO_UNREGISTER,
	
	/// Wake.
	///
	/// Only useful for registered memory.
	Wake = 1 << _UFFDIO_WAKE,
	
	/// Copy.
	///
	/// Only useful for registered memory.
	Copy = 1 << _UFFDIO_COPY,
	
	/// Zero page copy.
	///
	/// Only useful for registered memory which does not use huge pages.
	ZeroPageCopy = 1 << _UFFDIO_ZEROPAGE,
	
	/// Write Protect.
	///
	/// Only useful for registered memory which does not use huge pages and if `PageFaultEventNotificationSetting::RaisePageFaultEventIfWriteProtectedPageAccessed` was specified on registration.
	WriteProtect = 1 << _UFFDIO_WRITEPROTECT,
}
