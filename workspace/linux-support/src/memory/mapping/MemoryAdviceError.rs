// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Memory advice error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum MemoryAdviceError
{
	/// There is no clear explanation of what causes this in Linux man pages.
	MemoryMapsSomethingWhichIsNotAFile,
	
	/// This only occurs if `advice` was `MemoryAdvice::Remove`.
	NotASharedWritableMapping,
	
	/// This can only occur if `advice` was `MemoryAdvice::DontNeed` or `MemoryAdvice::Remove`.
	///
	/// `PFN` pages are marked with the bit `VM_PFNMAP` in Linux, and are typically special memory areas unmanaged by the Linux virtual memory subsystem; they are usually physical pages directly mapped by device drivers.
	LockedPagesOrHugePagesOrPfnPagesAreNotSupportedForDontNeedOrRemove,
	
	/// This only occurs if `advice` was `MemoryAdvice::Mergeable` or `MemoryAdvice::Unmergeable` and the Linux kernel has not been compiled with `CONFIG_KSM`.
	MergeablePagesAreUnsupported,
	
	/// This can only occur `advice` was `MemoryAdvice::Free` or `MemoryAdvice::WipeOnFork`.
	///
	/// `PFN` pages are marked with the bit `VM_PFNMAP` in Linux, and are typically special memory areas unmanaged by the Linux virtual memory subsystem; they are usually physical pages directly mapped by device drivers.
	FileBackedPagesOrHugePagesOrSharedPagesOrPfnPagesAreNotSupportedForFreeOrWipeOnFork,
	
	/// This can only occur `advice` was `MemoryAdvice::WillNeed`.
	///
	/// Paging in this memory would cause the process' resident set size (RSS) to be exceeded,
	ProcessMaximumResidentSetSizeWouldBeExceededForWillNeed,
	
	/// This can only occur `advice` was `MemoryAdvice::WillNeed`.
	///
	/// Paging in failed.
	NotEnoughMemoryForWillNeed,
	
	/// This can only occur `advice` was `MemoryAdvice::WillNeed`.
	///
	/// The process does not have the capability `CAP_SYS_ADMIN`.
	PermissionDeniedForHardwarePoison,

	// EGAIN
	// EBADF
	// EINVAL addr is not page-aligned or length is negative. advice is not a valid.
	// ENOMEM Addresses in the specified range are not currently mapped, or are outside the address space of the process.
}

impl Display for MemoryAdviceError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for MemoryAdviceError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		None
	}
}
