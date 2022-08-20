// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Common to BPF file descriptors returned from the `bpf()` syscall.
pub trait BpfFileDescriptor: FileDescriptor
{
	/// Identifier.
	type Identifier: Identifier;
	
	/// Information.
	type Information: Information<Identifier=Self::Identifier>;
	
	/// Type of access permissions, if used.
	type Access;
	
	#[doc(hidden)]
	const GetFileDescriptor: bpf_cmd;
	
	#[doc(hidden)]
	const DefaultAccess: Self::Access;
	
	#[doc(hidden)]
	fn access_permissions_to_open_flags(access: Self::Access) -> u32;
	
	/// Get information.
	#[inline(always)]
	fn get_information(&self) -> Result<Self::Information, SystemCallErrorNumber>
	{
		let mut information: Self::Information = unsafe_uninitialized();
		
		let mut attr = bpf_attr::default();
		attr.info = BpfCommandObjectGetInformationByFileDescriptor
		{
			bpf_fd: self.as_raw_fd(),
			info_len: size_of::<Self::Information>() as u32,
			info: AlignedU64::from(&mut information),
		};
		
		let result = attr.syscall(bpf_cmd::BPF_OBJ_GET_INFO_BY_FD);
		if likely!(result == 0)
		{
			// let length = (unsafe { attr.info.info_len }) as usize;
			// unsafe { information.set_len(length) }
			Ok(information)
		}
		else if likely!(result == -1)
		{
			Err(SystemCallErrorNumber::from_errno())
		}
		else
		{
			unreachable_code(format_args!("Unexpected result `{}` from bpf(BPF_OBJ_GET_INFO_BY_FD)", result))
		}
	}
	
	/// Pin to a path so other processes can use this file descriptor.
	///
	/// Path must be below `BpfMountPoint`.
	///
	/// cf `bpf_obj_pin()`.
	#[inline(always)]
	fn pin_to_path(&self, mount_point: &BpfMountPoint, relative_path: &OsStr) -> Result<(), SystemCallErrorNumber>
	{
		let path = mount_point.with_relative_path(relative_path);
		let mut path = path_bytes_without_trailing_nul(&path).to_vec();
		path.push(b'\0');
		
		let mut attr = bpf_attr::default();
		attr.object = BpfCommandObject
		{
			pathname: AlignedU64::from(path.as_mut_ptr()),
			bpf_fd: self.as_raw_fd(),
			file_flags: 0,
		};
		
		let result = attr.syscall(bpf_cmd::BPF_OBJ_PIN);
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			Err(SystemCallErrorNumber::from_errno())
		}
		else
		{
			unreachable_code(format_args!("Unexpected result `{}` from bpf(BPF_OBJ_PIN)", result))
		}
	}
	
	/// Get a file descriptor pinned to a path.
	///
	/// Path must be below `BpfMountPoint`.
	///
	/// cf `bpf_obj_get()`.
	#[inline(always)]
	fn get_pinned_relative_path(mount_point: &BpfMountPoint, relative_path: &OsStr, access_permissions: KernelOnlyAccessPermissions) -> Result<Self, SystemCallErrorNumber>
	{
		let absolute_path = mount_point.with_relative_path(relative_path);
		Self::get_pinned_absolute_path(&absolute_path, access_permissions)
	}
	
	/// Get a file descriptor pinned to a path.
	///
	/// cf `bpf_obj_get()`.
	#[inline(always)]
	fn get_pinned_absolute_path(absolute_path: &impl AsRef<Path>, access_permissions: KernelOnlyAccessPermissions) -> Result<Self, SystemCallErrorNumber>
	{
		let mut path = path_bytes_without_trailing_nul(&absolute_path).to_vec();
		path.push(b'\0');
		
		let mut attr = bpf_attr::default();
		attr.object = BpfCommandObject
		{
			pathname: AlignedU64::from(path.as_mut_ptr()),
			bpf_fd: 0,
			file_flags: access_permissions.to_map_flags().bits(),
		};
		
		let result = attr.syscall(bpf_cmd::BPF_OBJ_GET);
		if likely!(result == 0)
		{
			Ok(unsafe { Self::from_raw_fd(attr.object.bpf_fd) })
		}
		else if likely!(result == -1)
		{
			Err(SystemCallErrorNumber::from_errno())
		}
		else
		{
			unreachable_code(format_args!("Unexpected result `{}` from bpf(BPF_OBJ_GET)", result))
		}
	}
	/// To file descriptor.
	///
	/// `MapIdentifier` usage requires the capability `CAP_SYS_ADMIN`.
	#[inline(always)]
	fn from_identifier_with_access_defaults(identifier: Self::Identifier) -> Result<Option<Self>, SystemCallErrorNumber>
	{
		Self::from_identifier(identifier, Self::DefaultAccess)
	}
	
	/// To file descriptor.
	///
	/// `access_permissions` are only validated for `MapIdentifier`; otherwise it must be `()`.
	/// `MapIdentifier` usage requires the capability `CAP_SYS_ADMIN`.
	#[inline(always)]
	fn from_identifier(identifier: Self::Identifier, access_permissions: Self::Access) -> Result<Option<Self>, SystemCallErrorNumber>
	{
		let mut attr = bpf_attr::default();
		attr.get_identifier = BpfCommandGetIdentifier
		{
			value_of_identifier: identifier.into(),
			next_id: 0,
			open_flags: Self::access_permissions_to_open_flags(access_permissions),
		};
		
		let result = attr.syscall(Self::GetFileDescriptor);
		if likely!(result > 0)
		{
			Ok(Some(unsafe { Self::from_raw_fd(result) }))
		}
		else if likely!(result == -1)
		{
			match SystemCallErrorNumber::from_errno()
			{
				ENOENT => Ok(None),
				
				other @ _ => Err(other)
			}
		}
		else
		{
			unreachable_code(format_args!("Unexpected result `{}` from bpf({:?})", result, Self::GetFileDescriptor))
		}
	}
}
