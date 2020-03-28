// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Move pages to another NUMA node.
///
/// See also `NumaNode::status_of_pages()`.
#[inline(always)]
pub fn move_pages(process_identifier: pid_t, pages_to_move: &[(NonNull<u8>, NumaNode)], move_all: bool) -> Result<Box<[PageMoveStatus]>, PageMoveError>
{
	let count = pages_to_move.len();
	if unlikely!(count == 0)
	{
		return Ok(Vec::new().into_boxed_slice())
	}

	let mut status: Vec<PageMoveStatus> = Vec::with_capacity(count);

	let mut pages: Vec<*const c_void> = Vec::with_capacity(count);
	let mut nodes: Vec<i32> = Vec::with_capacity(count);
	for &(non_null, numa_node) in pages_to_move
	{
		pages.push(non_null.as_ptr() as *const c_void);
		nodes.push(numa_node.into());
	}

	let flags = if move_all
	{
		MemoryBindFlags::MPOL_MF_MOVE
	}
	else
	{
		MemoryBindFlags::MPOL_MF_MOVE_ALL
	};
	let result = syscall::move_pages(process_identifier, count, pages.as_ptr() as *const *const c_void, nodes.as_ptr(), status.as_mut_ptr() as *mut i32, flags);

	if likely!(result == 0)
	{
		unsafe { status.set_len(count) }
		Ok(status.into_boxed_slice())
	}
	else if likely!(result == -1)
	{
		use self::PageMoveError::*;

		match errno().0
		{
			EACCES => Err(TargetNodeNotAllowed),
			ENODEV => Err(OneOrMoreTargetNodesIsNotOnline),
			ESRCH => Err(ProcessDoesNotExist(process_identifier)),
			EPERM => match move_all
			{
				true => Err(CallerNeedsToHaveSysNiceCapabilityForMoveAll),
				false => if process_identifier == 0
				{
					panic!("We need to have CAP_SYS_NICE for ourselves?!")
				}
				else
				{
					Err(CallerNeedsToHaveSysNiceCapabilityToMoveAnotherPagesOfAnotherProcess(process_identifier))
				},
			},

			EINVAL => panic!("Flags other than MPOL_MF_MOVE and MPOL_MF_MOVE_ALL was specified or an attempt was made to migrate pages of a kernel thread"),
			E2BIG => panic!("Kernel should not generate E2BIG"),

			unexpected @ _ => panic!("Unexpected error number '{}'", unexpected),
		}
	}
	else
	{
		panic!("Unknown result '{}'", result)
	}
}
