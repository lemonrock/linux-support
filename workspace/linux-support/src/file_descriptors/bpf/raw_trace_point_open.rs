// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Despite its function name, this function (a) attaches and (b) attaches to things other than raw trace points.
/// Good ol'Linux.
#[inline(always)]
fn raw_trace_point_open(extended_bpf_program_file_descriptor: &ExtendedBpfProgramFileDescriptor, name: AlignedU64) -> Result<RawFd, i32>
{
	let mut attr = bpf_attr::default();
	attr.raw_tracepoint = BpfCommandRawTracePointOpen
	{
		name,
		prog_fd: extended_bpf_program_file_descriptor.as_raw_fd(),
	};
	
	let result = attr.syscall(bpf_cmd::BPF_RAW_TRACEPOINT_OPEN);
	if likely!(result >= 0)
	{
		Ok(result)
	}
	else if likely!(result == -1)
	{
		match errno().0
		{
			ENOENT => Err(ENOENT),
			ENOMEM => Err(ENOMEM),
			
			EINVAL => panic!("Invalid attr or invalid attach type"),
			EPERM => panic!("Permission denied"),
			EFAULT => panic!("Fault copying to / from userspace"),
			
			errno @ _ => panic!("Unexpected error `{}` from bpf(BPF_RAW_TRACEPOINT_OPEN)", errno),
		}
	}
	else
	{
		unreachable_code(format_args!("Unexpected result `{}` from bpf(BPF_RAW_TRACEPOINT_OPEN)", result))
	}
}
