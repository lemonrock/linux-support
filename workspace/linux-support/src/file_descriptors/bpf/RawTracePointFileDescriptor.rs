// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a link file descriptor with operations `bpf_raw_tp_lops`.
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

impl ProcessQueryableFileDescriptor for RawTracePointFileDescriptor
{
}

impl LinkFileDescriptor for RawTracePointFileDescriptor
{
}

impl RawTracePointFileDescriptor
{
	/// `extended_bpf_program_file_descriptor` must have a suitable program type.
	#[inline(always)]
	pub fn attach(extended_bpf_program_file_descriptor: &ExtendedBpfProgramFileDescriptor, raw_trace_point_type: &RawTracePointType) -> Result<Self, RawTracePointAttachError>
	{
		use self::RawTracePointType::*;
		
		let name = match raw_trace_point_type
		{
			TracingOfRawTracePoint => AlignedU64::Null,
			
			RawTracePoint(TracePointDetails { ref trace_point_name }) => AlignedU64::from(trace_point_name.as_ptr()),
			
			RawTracePointWritable(TracePointDetails { ref trace_point_name }) => AlignedU64::from(trace_point_name.as_ptr()),
		};
		
		use self::RawTracePointAttachError::*;
		match raw_trace_point_open(extended_bpf_program_file_descriptor, name)
		{
			Ok(raw_fd) => Ok(Self(raw_fd)),
			Err(errno) => match errno
			{
				ENOENT => Err(TracePointNameNotFound),
				ENOMEM => Err(OutOfMemory),
				_ => unreachable_code(format_args!("")),
			}
		}
	}
	
}
