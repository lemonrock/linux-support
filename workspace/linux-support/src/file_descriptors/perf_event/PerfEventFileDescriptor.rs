// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a perf event file descriptor.
///
/// Can have `ExtendedBpfProgramFileDescriptor` attached using an `ioctl(PERF_EVENT_IOC_SET_BPF)`.
/// Can have `ExtendedBpfProgramFileDescriptor` queried using an `ioctl(PERF_EVENT_IOC_QUERY_BPF)`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PerfEventFileDescriptor(RawFd);

impl Drop for PerfEventFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.as_raw_fd().close()
	}
}

impl AsRawFd for PerfEventFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl IntoRawFd for PerfEventFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.as_raw_fd()
	}
}

impl FromRawFd for PerfEventFileDescriptor
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(fd)
	}
}

impl FileDescriptor for PerfEventFileDescriptor
{
}

impl UsedAsValueInArrayMapDescriptor for PerfEventFileDescriptor
{
	#[inline(always)]
	fn transmute_from_file_descriptor_copies(values: &[Self]) -> &[RawFd]
	{
		unsafe { transmute(values) }
	}
}

impl ProcessQueryableFileDescriptor for PerfEventFileDescriptor
{
}

impl PerfEventFileDescriptor
{
	/// Opens a new instance using `perf_event_open()`.
	///
	/// If `event_group_leader` this event is created as an event group leader in a new event group.
	/// To attach an event into an existing event group, pass the event group leader of the event group.
	///
	/// Events are created with the close-on-exec flag set.
	#[allow(dead_code)]
	pub(crate) fn open(mut attr: perf_event_attr, event_attachment: EventAttachment, event_group_leader: Option<&Self>, output: bool) -> Result<Self, io::Error>
	{
		let (pid, cpu, flags) = event_attachment.to_values();
		
		let group_fd = match event_group_leader
		{
			None => -1,
			Some(event_group_leader) => event_group_leader.as_raw_fd(),
		};
		
		let flags = flags | if output
		{
			PERF_FLAG_FD_OUTPUT
		}
		else
		{
			0
		};
		
		let result = perf_event_open(&mut attr, pid, cpu, group_fd, flags);
		if likely!(result >= 0)
		{
			Ok(Self(result))
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable_code(format_args!("Unexpected result `{}` from perf_event_open()", result))
		}
	}
}
