// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a raw trace point file descriptor.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RawTracePointFileDescriptor(RawFd);

impl Drop for RawTracePointFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.as_raw_fd().close()
	}
}

impl AsRawFd for RawTracePointFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl IntoRawFd for RawTracePointFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.as_raw_fd()
	}
}

impl FromRawFd for RawTracePointFileDescriptor
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(fd)
	}
}

impl FileDescriptor for RawTracePointFileDescriptor
{
}

impl RawTracePointFileDescriptor
{
	// `raw_trace_point_type` MUST match the program type of `extended_bpf_program_file_descriptor`.
	// returns a trace point file descriptor.
	pub fn attach_raw_trace_point(extended_bpf_program_file_descriptor: &ExtendedBpfProgramFileDescriptor, raw_trace_point_type: &RawTracePointType) -> Result<RawFd, ()>
	{
		use self::RawTracePointType::*;
		
		let mut attr: bpf_attr = bpf_attr::default();
		attr.raw_tracepoint = BpfCommandRawTracePointOpen
		{
			name: match raw_trace_point_type
			{
				RawTracePoint(TracePointDetails { ref trace_point_name }) => AlignedU64::from(name.as_ptr()),
				RawTracePointWritable(TracePointDetails { ref trace_point_name }) => AlignedU64::from(name.as_ptr()),
				_ => AlignedU64::Null,
			},
			prog_fd: extended_bpf_program_file_descriptor.as_raw_fd(),
		};
		
		let result = attr.syscall(bpf_cmd::BPF_RAW_TRACEPOINT_OPEN);
		if likely!(result == 0)
		{
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EINVAL => panic!("Invalid attr or invalid attach type"),
				// Should only occur for:-
				// * `RawTracePoint` and `RawTracePointWritable`: the `trace_point_name` does not exist.
				// * `BPF_PROG_TYPE_TRACING` with expected attach type `BPF_TRACE_RAW_TP`: `prog->aux->attach_func_name` does not exist.
				ENOENT => Err(()),
				ENOMEM => (),
				EPERM => panic!("Permission denied"),
				EFAULT => panic!("Fault copying to / from userspace"),
				
				errno @ _ => panic!("Unexpected error `{}`", errno),
			}
		}
		else
		{
			unreachable!("Unexpected result `{}` from bpf(BPF_PROG_QUERY)", result)
		}
	}
}
