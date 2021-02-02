// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// It is invalid in Linux to combine both `DoNotWakeUp` and `WriteProtect` in the Linux source file `fs/userfaultfd.c`, function `userfaultfd_writeprotect()`.
///
/// "Write protecting a region (`WP=1`) is unrelated to page faults, therefore `DONTWAKE` flag is meaningless with WP=1.
/// Removing write protection (`WP=0`) in response to a page fault wakes the faulting task unless `DONTWAKE` is set".
#[repr(u64)]
pub enum WriteProtectMode
{
	WakeUpWithoutWriteProtect = 0,
	
	/// "set the flag to avoid waking up any wait thread after the operation succeeds".
	DoNotWakeUpWithoutWriteProtect = UFFDIO_WRITEPROTECT_MODE_DONTWAKE,
	
	/// "set the flag to write protect a range, unset the flag to undo protection of a range which was previously write protected".
	WakeUpWithWriteProtect = UFFDIO_WRITEPROTECT_MODE_WP,
}
