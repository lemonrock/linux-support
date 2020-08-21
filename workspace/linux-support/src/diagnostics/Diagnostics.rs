// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Diagnostics.
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct Diagnostics
{
	/// File system layout used.
	pub file_system_layout: FileSystemLayout,
	
	/// Users and groups.
	pub users_and_groups: UsersAndGroupsDiagnostics,

	/// Thread.
	pub thread: ThreadsDiagnostics,

	/// Swap.
	pub swap: SwapDiagnostics,

	/// Miscellaneous process control.
	pub miscellaneous_process_control: MiscellaneousProcessControlDiagnostics,
}

impl Diagnostics
{
	fn gather(file_system_layout: &FileSystemLayout) -> Self
	{
		let (sys_path, proc_path, dev_path, etc_path) = file_system_layout.paths();
		
		Self
		{
			file_system_layout: file_system_layout.clone(),
			users_and_groups: UsersAndGroupsDiagnostics::gather(proc_path, etc_path, ProcessIdentifierChoice::Current),
			thread: ThreadsDiagnostics::gather(proc_path),
			swap: SwapDiagnostics::gather(proc_path),
			miscellaneous_process_control: MiscellaneousProcessControlDiagnostics::gather(),
		}
	}
}
