// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Common to BPF file descriptors returned from the `bpf()` syscall.
pub trait BpfFileDescriptor: FileDescriptor
{
	/// Pin to a path so other processes can use this file descriptor.
	#[inline(always)]
	fn pin_to_path(&self, path: &impl AsRef<Path>) -> Result<(), Errno>
	{
		let mut path = path_bytes_without_trailing_nul(path).to_vec();
		path.push(b'\0');
		
		let mut attr = bpf_attr
		{
			object: BpfCommandObject
			{
				pathname: AlignedU64::from(path.as_mut_ptr()),
				bpf_fd: self.as_raw_fd(),
				file_flags: 0,
			},
		};
		
		let result = attr.syscall(bpf_cmd::BPF_OBJ_PIN);
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			Err(errno())
		}
		else
		{
			unreachable!("Unexpected result `{}` from bpf(BPF_OBJ_PIN)", result)
		}
	}
	
	/// Get a file descriptor pinned to a path.
	#[inline(always)]
	fn get_pinned(&self, path: &impl AsRef<Path>, access_permissions: KernelOnlyAccessPermissions) -> Result<Self, Errno>
	{
		let mut path = path_bytes_without_trailing_nul(path).to_vec();
		path.push(b'\0');
		
		let mut attr = bpf_attr
		{
			object: BpfCommandObject
			{
				pathname: AlignedU64::from(path.as_mut_ptr()),
				bpf_fd: 0,
				file_flags: access_permissions.to_map_flags().bits(),
			},
		};
		
		let result = attr.syscall(bpf_cmd::BPF_OBJ_GET);
		if likely!(result == 0)
		{
			Ok(unsafe { Self::from_raw_fd(attr.object.bpf_fd) })
		}
		else if likely!(result == -1)
		{
			Err(errno())
		}
		else
		{
			unreachable!("Unexpected result `{}` from bpf(BPF_OBJ_GET)", result)
		}
	}
}
