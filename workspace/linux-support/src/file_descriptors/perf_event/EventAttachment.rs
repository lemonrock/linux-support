// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Event attachment.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EventAttachment<'file_descriptor>
{
	/// Attach event to all processes on a specific HyperThread.
	///
	/// The user must have the capability `CAP_SYS_ADMIN` or `/proc/sys/kernel/perf_event_paranoid` must be less than `1`.
	AllProcessesSpecificHyperThread(HyperThread),
	
	/// Attach event to a specific process on all HyperThreads.
	///
	/// This is a proces event; the event follows whatever CPUs the process gets scheduled on.
	SpecificProcessAllHyperThreads(ProcessIdentifierChoice),
	
	/// Attach event to a specific process on a specific HyperThread.
	SpecificProcessAndSpecificHyperThread(ProcessIdentifierChoice, HyperThread),
	
	/// Attach event to a specific process on all HyperThreads.
	SpecificCgroupAllHyperThreads(&'file_descriptor CgroupFileDescriptor),
	
	/// Attach event to a specific cgroup on a specific HyperThread.
	SpecificCgroupAndSpecificHyperThread(&'file_descriptor CgroupFileDescriptor, HyperThread),
}

impl<'file_descriptor> EventAttachment<'file_descriptor>
{
	#[inline(always)]
	fn to_values(self) -> (i32, i32, u32)
	{
		use self::EventAttachment::*;
		
		match self
		{
			AllProcessesSpecificHyperThread(hyper_thread) => (-1, hyper_thread.into(), PERF_FLAG_FD_CLOEXEC),
			SpecificProcessAllHyperThreads(process_identifier) => (process_identifier.into(), -1, PERF_FLAG_FD_CLOEXEC),
			SpecificProcessAndSpecificHyperThread(process_identifier, hyper_thread) => (process_identifier.into(), hyper_thread.into(), PERF_FLAG_FD_CLOEXEC),
			SpecificCgroupAllHyperThreads(cgroup_file_descriptor) => (cgroup_file_descriptor.as_raw_fd(), -1, PERF_FLAG_FD_CLOEXEC | PERF_FLAG_PID_CGROUP),
			SpecificProcessAndSpecificHyperThread(cgroup_file_descriptor, hyper_thread) => (cgroup_file_descriptor.as_raw_fd(), hyper_thread.into(), PERF_FLAG_FD_CLOEXEC | PERF_FLAG_PID_CGROUP),
		}
	}
}
