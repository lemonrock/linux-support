// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// It is invalid in Linux to combine both `DoNotWakeUp` and `WriteProtect` in the Linux source file `fs/userfaultfd.c`, function `userfaultfd_writeprotect()`.
///
/// "Write protecting a region (`WP=1`) is unrelated to page faults, therefore `DONTWAKE` flag is meaningless with WP=1.
/// Removing write protection (`WP=0`) in response to a page fault wakes the faulting task unless `DONTWAKE` is set".
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[repr(u64)]
enum WriteProtectMode
{
	/// This can only be used if the memory range it is applied to has previously been write-protected with with `EnableWriteProtectionAndTheRaisingOfWriteProtectionEventsAndThenWakeUp`.
	///
	/// This is normally used when handling a page fault event.
	DisableWriteProtectionAndTheSettingOfThWriteProtectionFaultInPageFaultEventFlagsAndThenWakeUp = 0,
	
	/// This can only be used if the memory range it is applied to has previously been write-protected with with `EnableWriteProtectionAndTheRaisingOfWriteProtectionEventsAndThenWakeUp`.
	///
	/// This is normally used when handling a page fault event; a separate ioctl call to `UserFaultFileDescriptor.wake_up_memory_range()` will need to be made for the memory range.
	DisableWriteProtectionAndTheSettingOfThWriteProtectionFaultInPageFaultEventFlagsAndThenDoNotWakeUp = UFFDIO_WRITEPROTECT_MODE_DONTWAKE,
	
	/// Enable write protection and write protection events.
	///
	/// Enabling write protection is unrelated to page faults.
	EnableWriteProtectionAndTheRaisingOfWriteProtectionEventsAndThenWakeUp = UFFDIO_WRITEPROTECT_MODE_WP,
}

impl WriteProtectMode
{
	#[inline(always)]
	fn disable(wake_up_after_disabling_write_protection: bool) -> Self
	{
		if wake_up_after_disabling_write_protection
		{
			WriteProtectMode::DisableWriteProtectionAndTheSettingOfThWriteProtectionFaultInPageFaultEventFlagsAndThenWakeUp
		}
		else
		{
			WriteProtectMode::DisableWriteProtectionAndTheSettingOfThWriteProtectionFaultInPageFaultEventFlagsAndThenDoNotWakeUp
		}
	}
}
