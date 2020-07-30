// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An identifier.
pub trait Identifier: From<u32> + Into<u32> + Into<BpfCommandGetIdentifierValueOfIdentifier> + Default + Debug + Copy + Clone + PartialEq + Eq + PartialOrd + Ord + Hash
{
	#[doc(hidden)]
	const Next: bpf_cmd;
	
	#[doc(hidden)]
	const GetFileDescriptor: bpf_cmd;
	
	#[doc(hidden)]
	fn access_permissions_to_open_flags(access: Self::Access) -> u32;
	
	/// File descriptor associated with identifier.
	type FD: BpfFileDescriptor;
	
	/// Type of access permissions, if used.
	type Access;
	
	/// Next identifier.
	#[inline(always)]
	fn next(self) -> Option<Self>
	{
		let mut attr = bpf_attr::default();
		attr.get_identifier = BpfCommandGetIdentifier
		{
			value_of_identifier: BpfCommandGetIdentifierValueOfIdentifier
			{
				start_id: self.into(),
			},
			next_id: 0,
			open_flags: 0,
		};
		
		let result = attr.syscall(Self::Next);
		if likely!(result == 0)
		{
			Some(Self::from(unsafe { attr.get_identifier.next_id }))
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				ENOENT => None,
				unexpected @ _ => panic!("Unexpected errror {}", unexpected),
			}
		}
		else
		{
			unreachable!("Unexpected result `{}` from bpf({:?})", result, Self::Next)
		}
	}
	
	/// To file descriptor.
	///
	/// `access_permissions` are only validated for `MapIdentifier`; otherwise it must be `()`.
	/// `MapIdentifier` usage requires the capability `CAP_SYS_ADMIN`.
	#[inline(always)]
	fn to_file_descriptor(self, access_permissions: Self::Access) -> Result<Option<Self::FD>, Errno>
	{
		let mut attr = bpf_attr::default();
		attr.get_identifier = BpfCommandGetIdentifier
		{
			value_of_identifier: self.into(),
			next_id: 0,
			open_flags: Self::access_permissions_to_open_flags(access_permissions),
		};
		
		let result = attr.syscall(Self::GetFileDescriptor);
		if likely!(result > 0)
		{
			Ok(Some(unsafe { Self::FD::from_raw_fd(result) }))
		}
		else if likely!(result == -1)
		{
			let errno = errno();
			match errno.0
			{
				ENOENT => Ok(None),
				_ => Err(errno)
			}
		}
		else
		{
			unreachable!("Unexpected result `{}` from bpf({:?})", result, Self::GetFileDescriptor)
		}
	}
	
	#[doc(hidden)]
	fn froms(values: Vec<u32>) -> Vec<Self>;
}
