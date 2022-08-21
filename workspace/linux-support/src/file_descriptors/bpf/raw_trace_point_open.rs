// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Despite its function name, this function (a) attaches and (b) attaches to things other than raw trace points.
/// Good ol'Linux.
#[inline(always)]
fn raw_trace_point_open<O, E>(extended_bpf_program_file_descriptor: &ExtendedBpfProgramFileDescriptor, name: AlignedU64, constructor: impl FnOnce(RawFd) -> O, enoent: impl FnOnce() -> E, enomem: impl FnOnce() -> E) -> Result<O, E>
{
	let mut attr = bpf_attr::default();
	attr.raw_tracepoint = BpfCommandRawTracePointOpen
	{
		name,
		prog_fd: extended_bpf_program_file_descriptor.as_raw_fd(),
	};
	
	match attr.syscall(bpf_cmd::BPF_RAW_TRACEPOINT_OPEN).as_usize()
	{
		raw_file_descriptor @ SystemCallResult::InclusiveMinimumRawFileDescriptor_usize ..= SystemCallResult::InclusiveMaximumRawFileDescriptor_usize => Ok(constructor(raw_file_descriptor as RawFd)),
		
		SystemCallResult::ENOENT_usize => Err(enoent()),
		SystemCallResult::ENOMEM_usize => Err(enomem()),
		SystemCallResult::EINVAL_usize => panic!("Invalid attr or invalid attach type"),
		SystemCallResult::EPERM_usize => panic!("Permission denied"),
		SystemCallResult::EFAULT_usize => panic!("Fault copying to / from userspace"),
		unexpected_error @ _ => unexpected_error!(bpf, BPF_RAW_TRACEPOINT_OPEN, SystemCallResult::usize_to_system_call_error_number(unexpected_error)),
		
		unexpected @ _ => unexpected_result!(bpf, BPF_RAW_TRACEPOINT_OPEN, unexpected),
	}
}
