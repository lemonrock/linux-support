// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Daemonizes the current process.
#[inline(always)]
pub fn daemonize(dev_path: &DevPath)
{
	#[inline(always)]
	fn fork_process()
	{
		const ForkedToChild: i32 = 0;

		match unsafe { fork() }
		{
			ForkedToChild => (),

			-1 => panic!("Fork failed with {}", io::Error::last_os_error()),

			_child_process_id_returned_to_parent @ _ => exit(0),
		}
	}

	#[inline(always)]
	fn create_a_new_process_group_and_session_detach_controlling_terminal() -> ProcessGroupIdentifier
	{
		let result = unsafe { setsid() };
		if likely!(result > 0)
		{
			return ProcessGroupIdentifier(new_non_zero_i32(result))
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EPERM => panic!("The process group ID of any process equals the PID of the calling process. Thus, in particular, setsid() fails if the calling process is already a process group leader."),

				unknown @ _ => panic!("Unknown error code `{}` fron `setsid()`", unknown),
			}
		}
		else
		{
			panic!("Unknown result `{}` from `setsid()`", result)
		}
	}

	stdin().redirect_to_dev_null(dev_path);
	stdout().redirect_to_dev_null(dev_path);
	stderr().redirect_to_dev_null(dev_path);

	// This first fork causes the process to be reparented to `init` (process 1).
	//
	// The first fork creates a child that is a session leader without a controlling terminal, so it's possible for it to acquire one by opening a terminal in the future.
	// Hence the need for the second fork below.
	fork_process();

	create_a_new_process_group_and_session_detach_controlling_terminal();

	// This second fork guarantees that the child is no longer a session leader, preventing the daemon from ever acquiring a controlling terminal.
	// A controlling terminal allows `Ctrl-C` et al to be sent as signals to the process.
	//
	// This matters so that a daemon doesn't need to specify `O_NOCTTY` to `open()` everywhere where it might be opening a console (a terminal character file device).
	fork_process();
}
