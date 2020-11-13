// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Task query information.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ProcessQueryInformation
{
	/// Pointer to data of various kinds:-
	///
	/// * tp_name for tracepoint
	/// * symbol for kprobe
	/// * filename for uprobe
	pub program_identifier: ExtendedBpfProgramIdentifier,
	
	/// Used for output.
	pub file_descriptor_type: FileDescriptorType,
}

impl ProcessQueryInformation
{
	/// Needs the capability `CAP_SYS_ADMIN`.
	///
	/// `file_descriptor` can be one that represents a raw trace point or a perf event.
	///
	/// Only a subset of instances of `RawTracePointFileDescriptor` are valid.
	///
	/// Fails if permission denied.
	pub fn query(process_identifier: ProcessIdentifierChoice, file_descriptor: &impl ProcessQueryableFileDescriptor) -> Result<Option<Self>, ()>
	{
		const SizeIncrement: usize = 256;
		
		let mut buffer = Vec::with_capacity(SizeIncrement);
		unsafe { buffer.set_len(buffer.capacity()) };
		
		let mut attr = bpf_attr::default();
		attr.task_fd_query = BpfCommandTaskFileDescriptorQuery
		{
			pid: process_identifier.into(),
			fd: file_descriptor.as_raw_fd(),
			flags: 0,
			buf_len: buffer.len() as u32,
			buf: AlignedU64::from(buffer.as_mut_ptr()),
			prog_id: unsafe_zeroed(),
			fd_type: unsafe_zeroed(),
			probe_offset: 0,
			probe_addr: 0
		};
		
		loop
		{
			let result = attr.syscall(bpf_cmd::BPF_TASK_FD_QUERY);
			if likely!(result == 0)
			{
				return Ok(Some(Self::construct(buffer, unsafe { &attr.task_fd_query })))
			}
			else if likely!(result == -1)
			{
				match errno().0
				{
					ENOSPC =>
					{
						buffer.reserve(SizeIncrement);
						unsafe { buffer.set_len(buffer.capacity()) };
						attr.task_fd_query.buf_len = buffer.capacity() as u32;
						continue
					}
					EINVAL => panic!("Invalid attr"),
					EPERM => return Err(()),
					ENOENT | EBADF | ENOTSUPP | EOPNOTSUPP => return Ok(None),
					EFAULT => panic!("Could not access buffer"),
					errno @ _ => panic!("Unexpected error `{}`", errno),
				}
			}
			else
			{
				unreachable_code(format_args!("Unexpected result `{}` from bpf(BPF_TASK_FD_QUERY)", result))
			}
		}
	}
	
	#[inline(always)]
	fn construct(buffer: Vec<c_char>, task_fd_query: &BpfCommandTaskFileDescriptorQuery) -> Self
	{
		let string = CString::from_fixed_length_buffer_ascii_nul_terminated(buffer);
		
		use self::bpf_task_fd_type::*;
		use self::FileDescriptorType::*;
		
		Self
		{
			program_identifier: task_fd_query.prog_id,
			
			file_descriptor_type: match task_fd_query.fd_type
			{
				BPF_FD_TYPE_RAW_TRACEPOINT => RawTracePoint(TracePointDetails::new(string).unwrap()),
				
				BPF_FD_TYPE_TRACEPOINT => TracePoint(TracePointDetails::new(string).unwrap()),
				
				BPF_FD_TYPE_KPROBE => KProbe(KProbeDetails::construct(string,  task_fd_query)),
				
				BPF_FD_TYPE_KRETPROBE => KRetProbe(KProbeDetails::construct(string,  task_fd_query)),
				
				BPF_FD_TYPE_UPROBE => UProbe(UProbeDetails::construct(string, task_fd_query)),
				
				BPF_FD_TYPE_URETPROBE => URetProbe(UProbeDetails::construct(string, task_fd_query)),
			},
		}
	}
}
