// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a link file descriptor with operations `bpf_cgroup_link_lops`.
///
/// Created using `attach_link()` on `CgroupFileDescriptor`.
#[derive(Debug)]
pub struct CgroupLinkFileDescriptor(RawFd);

impl Drop for CgroupLinkFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.as_raw_fd().close()
	}
}

impl AsRawFd for CgroupLinkFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl IntoRawFd for CgroupLinkFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.as_raw_fd()
	}
}

impl FromRawFd for CgroupLinkFileDescriptor
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(fd)
	}
}

impl FileDescriptor for CgroupLinkFileDescriptor
{
}

impl LinkFileDescriptor for CgroupLinkFileDescriptor
{
}

impl CgroupLinkFileDescriptor
{
	/// Requires the capability `CAP_NET_ADMIN`.
	///
	/// Replacement program must have same type and attachment type as that used in original call to `CgroupFileDescriptor::attach_link()`.
	///
	/// If `currently_attached_program` is not currently attached, an error is returned.
	pub fn update(&self, currently_attached_program: Option<&ExtendedBpfProgramFileDescriptor>, replacement_program: &ExtendedBpfProgramFileDescriptor) -> Result<(), ()>
	{
		let (flags, old_prog_fd) = match currently_attached_program
		{
			None => (0, 0),
			Some(currently_attached_program) => (BPF_PROG_ATTACH_flags::BPF_F_REPLACE.bits(), currently_attached_program.as_raw_fd()),
		};
		
		let mut attr = bpf_attr::default();
		attr.link_update = BpfCommandLinkUpdate
		{
			link_fd: self.as_raw_fd(),
			new_prog_fd: replacement_program.as_raw_fd(),
			flags,
			old_prog_fd,
		};
		
		let result = attr.syscall(bpf_cmd::BPF_LINK_UPDATE);
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EINVAL => panic!("Invalid attr or invalid attach type"),
				EPERM => if currently_attached_program.is_none()
				{
					panic!("Permission denied")
				}
				else
				{
					Err(())
				},
				
				errno @ _ => panic!("Unexpected error `{}` from bpf(BPF_LINK_UPDATE)", errno),
			}
		}
		else
		{
			unreachable!("Unexpected result `{}` from bpf(BPF_LINK_UPDATE)", result)
		}
	}
}
