// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a link file descriptor with operations `bpf_tracing_link_lops`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TracingFileDescriptor(RawFd);

impl Drop for TracingFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.as_raw_fd().close()
	}
}

impl AsRawFd for TracingFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl IntoRawFd for TracingFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.as_raw_fd()
	}
}

impl FromRawFd for TracingFileDescriptor
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(fd)
	}
}

impl FileDescriptor for TracingFileDescriptor
{
}

impl ProcessQueryableFileDescriptor for TracingFileDescriptor
{
}

impl LinkFileDescriptor for TracingFileDescriptor
{
}

impl TracingFileDescriptor
{
	/// `extended_bpf_program_file_descriptor` must have a suitable program type.
	/// Only three program types are supported and only a subset of their expected attach type:-
	///
	/// * `BPF_PROG_TYPE_LSM`.
	///   * `BPF_LSM_MAC`.
	/// * `BPF_PROG_TYPE_EXT`.
	///   * (ignored).
	/// * `BPF_PROG_TYPE_TRACING`.
	///   * `BPF_TRACE_FENTRY`.
	///   * `BPF_TRACE_FEXIT`.
	///   * `BPF_TRACE_RETURN`.
	///   * But ***NOT*** `BPF_TRACE_RAW_TP`.
	#[inline(always)]
	pub fn attach(extended_bpf_program_file_descriptor: &ExtendedBpfProgramFileDescriptor) -> Result<Self, ()>
	{
		match raw_trace_point_open(extended_bpf_program_file_descriptor, AlignedU64::Null)
		{
			Ok(raw_fd) => Ok(Self(raw_fd)),
			Err(errno) => match errno
			{
				ENOENT => unreachable_code(format_args!("")),
				ENOMEM => Err(()),
				_ => unreachable_code(format_args!("")),
			}
		}
	}
	
}
