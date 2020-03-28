// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A failure caused when moving a page.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PageMoveError
{
	/// One of the target nodes is not allowed by the current cpuset.
	TargetNodeNotAllowed,

	/// One of the target nodes is not online.
	OneOrMoreTargetNodesIsNotOnline,

	/// Process does not exist.
	ProcessDoesNotExist(pid_t),

	/// The caller specified MPOL_MF_MOVE_ALL without sufficient privileges (CAP_SYS_NICE).
	CallerNeedsToHaveSysNiceCapabilityForMoveAll,

	/// The caller attempted to move pages of a process belonging to another user but did not have privilege to do so (CAP_SYS_NICE).
	CallerNeedsToHaveSysNiceCapabilityToMoveAnotherPagesOfAnotherProcess(pid_t),
}

impl Display for PageMoveError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<PageMoveError as Debug>::fmt(self, f)
	}
}

impl error::Error for PageMoveError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		None
	}
}
