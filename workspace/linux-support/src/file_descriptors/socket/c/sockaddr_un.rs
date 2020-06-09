// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Whilst this is present in libc, it does not support useful derives.
#[derive(Clone)]
#[repr(C)]
pub struct sockaddr_un
{
	/// Socket address family.
	sun_family: sa_family_t,

	/// Zero-terminated C String.
	///
	/// ***Caution!***
	///
	/// If the string is exactly `Self::PathLength` bytes, it is not ASCII NUL terminated.
	pub sun_path: [c_char; Self::PathLength]
}

impl Default for sockaddr_un
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			sun_family: AF_UNIX as sa_family_t,
			sun_path: unsafe { zeroed() },
		}
	}
}

impl Debug for sockaddr_un
{
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "sockaddr_un {{ sun_family: {}, sun_path: {:?} }}", self.sun_family, &self.sun_path[..])
	}
}

impl PartialEq for sockaddr_un
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		self.sun_family == other.sun_family && (&self.sun_path[..]) == (&other.sun_path[..])
	}
}

impl Eq for sockaddr_un
{
}

impl PartialOrd for sockaddr_un
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		Some(self.cmp(other))
	}
}

impl Ord for sockaddr_un
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		self.sun_family.cmp(&other.sun_family).then_with(|| (&self.sun_path[..]).cmp(&other.sun_path[..]))
	}
}

impl Hash for sockaddr_un
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, hasher: &mut H)
	{
		self.sun_family.hash(hasher);
		(&self.sun_path[..]).hash(hasher);
	}
}

impl SocketData for sockaddr_un
{
	type Address = [c_char; Self::PathLength];
	
	#[inline(always)]
	fn family(&self) -> sa_family_t
	{
		self.sun_family
	}
	
	#[inline(always)]
	fn address(&self) -> &Self::Address
	{
		&self.sun_path
	}
	
	#[inline(always)]
	fn specialized_drop(socket_file_descriptor: &mut SocketFileDescriptor<Self>)
	{
		let local_address = socket_file_descriptor.local_address();

		socket_file_descriptor.0.close();

		fn unlink_socket_file_path(local_address: &sockaddr_un, length: usize)
		{
			let is_unnamed = length <= size_of::<sa_family_t>();
			if unlikely!(is_unnamed)
			{
				return
			}

			const AsciiNull: i8 = 0;

			let first_byte = unsafe { *local_address.sun_path.get_unchecked(0) };
			let is_abstract = first_byte == AsciiNull;
			if unlikely!(is_abstract)
			{
				return
			}

			let last_byte = unsafe { *local_address.sun_path.get_unchecked(sockaddr_un::PathLength - 1) };
			let last_byte_is_zero_terminated = last_byte == AsciiNull;
			if likely!(last_byte_is_zero_terminated)
			{
				unsafe
				{
					// NOTE: Result ignored; nothing we can do about it.
					unlink(local_address.sun_path.as_ptr());
				}
			}
			else
			{
				unsafe
				{
					#[allow(deprecated)]
					let mut copy: [c_char; sockaddr_un::PathLength + 1] = uninitialized();
					copy.as_mut_ptr().copy_from_nonoverlapping(local_address.sun_path.as_ptr(), sockaddr_un::PathLength);
					*copy.get_unchecked_mut(sockaddr_un::PathLength) = AsciiNull;

					// NOTE: Result ignored; nothing we can do about it.
					unlink(copy.as_ptr());
				}
			}
		}

		if let Ok((local_address, length)) = local_address
		{
			unlink_socket_file_path(&local_address, length)
		}
	}
}

impl sockaddr_un
{
	/// Length of the `sun_path` entry.
	pub const PathLength: usize = 108;
}
