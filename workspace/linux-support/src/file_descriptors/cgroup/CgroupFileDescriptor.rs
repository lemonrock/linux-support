// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a Cgroup file descriptor which is backed by a `File`.
#[derive(Debug)]
pub struct CgroupFileDescriptor(RawFd);

impl Into<File> for CgroupFileDescriptor
{
	#[inline(always)]
	fn into(self) -> File
	{
		unsafe { File::from_raw_fd(self.into_raw_fd()) }
	}
}

impl Drop for CgroupFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.as_raw_fd().close()
	}
}

impl AsRawFd for CgroupFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl IntoRawFd for CgroupFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.as_raw_fd()
	}
}

impl FromRawFd for CgroupFileDescriptor
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(fd)
	}
}

impl FileDescriptor for CgroupFileDescriptor
{
}

impl UsedAsValueInArrayMapDescriptor for CgroupFileDescriptor
{
	#[inline(always)]
	fn transmute_from_file_descriptor_copies(values: &[Self]) -> &[RawFd]
	{
		unsafe { transmute(values) }
	}
}

impl ExtendedBpfProgramCanBeAttachedFileDescriptor for CgroupFileDescriptor
{
	type ProgramAttachmentType = CgroupProgramAttachmentType;
	
	type ProgramQueryFlags = CgroupProgramQueryFlags;
	
	type ProgramAttachmentFlags = CgroupProgramAttachmentFlags;
	
	type ProgramAttachmentOptions = CgroupProgramAttachmentOptions;
	
	/// `BPF_CGROUP_MAX_PROGS`.
	const InitialProgramCountGuess: usize = 64;
}

impl CgroupFileDescriptor
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new<'name>(mount_point: &CgroupMountPoint, cgroup: &Rc<impl Cgroup<'name>>) -> io::Result<Self>
	{
		let path = cgroup.to_path(mount_point);
		let file = File::open(path)?;
		Ok(Self::from_raw_fd(file.into_raw_fd()))
	}
	
	/// Requires the capability `CAP_NET_ADMIN`.
	///
	/// `extended_bpf_program_file_descriptor` must have a suitable program type with one of the expected attachment types in `CgroupProgramAttachmentType`, ie be one of:-
	///
	/// * `BPF_PROG_TYPE_CGROUP_SKB`.
	/// * `BPF_PROG_TYPE_CGROUP_SOCK`.
	/// * `BPF_PROG_TYPE_CGROUP_DEVICE`.
	/// * `BPF_PROG_TYPE_SOCK_OPS`.
	/// * `BPF_PROG_TYPE_CGROUP_SOCK_ADDR`.
	/// * `BPF_PROG_TYPE_CGROUP_SOCKOPT`.
	/// * `BPF_PROG_TYPE_CGROUP_SYSCTL`.
	pub fn attach_link(cgroup: &CgroupFileDescriptor, program_attachment_type: CgroupProgramAttachmentType, attach_program: &ExtendedBpfProgramFileDescriptor) -> Result<CgroupLinkFileDescriptor, ()>
	{
		let mut attr = bpf_attr::default();
		attr.link_create = BpfCommandLinkCreate
		{
			prog_fd: attach_program.as_raw_fd(),
			target_fd: cgroup.as_raw_fd(),
			attach_type: program_attachment_type.to_bpf_attach_type(),
			flags: 0,
		};
		
		let result = attr.syscall(bpf_cmd::BPF_LINK_CREATE);
		if likely!(result >= 0)
		{
			Ok(CgroupLinkFileDescriptor(result))
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EINVAL => panic!("Invalid attr or invalid attach type"),
				EPERM => panic!("Permission denied"),
				
				errno @ _ => panic!("Unexpected error `{}` from bpf(BPF_LINK_CREATE)", errno),
			}
		}
		else
		{
			unreachable!("Unexpected result `{}` from bpf(BPF_LINK_CREATE)", result)
		}
	}
}
