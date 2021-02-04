// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This error can only happen if using user fault file descriptors to monitor separated processes to that owning the user fault file descriptor and calling copy.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum CopyError
{
	#[allow(missing_docs)]
	FaultingProcessHasChangedItsMemoryLayout,
	
	#[allow(missing_docs)]
	FaultingProcessHasExited,
	
	#[allow(missing_docs)]
	LinuxKernelIsOutOfMemory,
	
	/// Happens due to `EEXIST` occuring inside various functions in the Linux source file `mm/userfault.c`:-
	///
	/// * No destination page table entry;
	/// * Destination page is or becomes Transparent Huge Page (THP).
	DestinationMemoryStructuresSuchAsVmaDoNotExist,
}

impl Display for CopyError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for CopyError
{
}
