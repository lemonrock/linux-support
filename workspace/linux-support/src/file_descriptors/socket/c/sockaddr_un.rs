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
	pub sun_path: [u8; Self::PathLength]
}

impl Default for sockaddr_un
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			sun_family: AF_UNIX as sa_family_t,
			sun_path: unsafe_zeroed(),
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
	type Address = [u8; Self::PathLength];
	
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
	fn display_format(&self, f: &mut Formatter, address_length: usize) -> fmt::Result
	{
		#[inline(always)]
		fn format_path_bytes(f: &mut Formatter, path_bytes_excluding_terminating_ascii_null: &[u8]) -> fmt::Result
		{
			write!(f, "unix:path:")?;
			format_escaped_ascii_string(path_bytes_excluding_terminating_ascii_null, f)
		}
		
		let borrow_check_hack_as_we_know_only_one_closure_is_executed_with_mutably_borrowed_formatter = f as *mut Formatter;
		
		self.use_bytes
		(
			address_length,
			
			||
			{
				let f = unsafe { &mut * borrow_check_hack_as_we_know_only_one_closure_is_executed_with_mutably_borrowed_formatter };
				write!(f, "unix:unnamed")
			},
			
			|abstract_name|
			{
				let f = unsafe { &mut * borrow_check_hack_as_we_know_only_one_closure_is_executed_with_mutably_borrowed_formatter };
				write!(f, "unix:abstract:")?;
				format_escaped_ascii_string(abstract_name, f)
			},
			
			|path_bytes_excluding_terminating_ascii_null|
			{
				let f = unsafe { &mut * borrow_check_hack_as_we_know_only_one_closure_is_executed_with_mutably_borrowed_formatter };
				format_path_bytes(f, path_bytes_excluding_terminating_ascii_null)
			},
			
			|path_bytes_excluding_terminating_ascii_null|
			{
				let f = unsafe { &mut * borrow_check_hack_as_we_know_only_one_closure_is_executed_with_mutably_borrowed_formatter };
				format_path_bytes(f, &path_bytes_excluding_terminating_ascii_null[..])
			}
		)
	}
	
	#[inline(always)]
	fn specialized_drop(socket_file_descriptor: &mut SocketFileDescriptor<Self>)
	{
		let address_and_length = socket_file_descriptor.local_address();

		socket_file_descriptor.0.close();

		if let Ok((address, address_length)) = address_and_length
		{
			let _ignored_as_nothing_to_be_done_in_drop = address.unlink_socket_file_path(address_length);
		}
	}
}

impl sockaddr_un
{
	/// Length of the `sun_path` entry.
	pub const PathLength: usize = 108;
	
	const PathLengthWithTerminatingAsciiNull: usize = sockaddr_un::PathLength + Self::SizeOfAsciiNull;
	
	const AsciiNull: u8 = 0;
	
	const SizeOfAsciiNull: usize = 1;
	
	const SizeOfFamily: usize = size_of::<sa_family_t>();
	
	#[inline(always)]
	fn unlink_socket_file_path(&self, address_length: usize) -> io::Result<()>
	{
		self.use_bytes
		(
			address_length,
			
			||
			{
				Ok(())
			},
			
			|_abstract_name|
			{
				Ok(())
			},
			
			|path_bytes_excluding_terminating_ascii_null|
			{
				let result = unsafe { unlink(path_bytes_excluding_terminating_ascii_null.as_ptr() as *const i8) };
				if likely!(result == 0)
				{
					Ok(())
				}
				else if likely!(result == -1)
				{
					Err(io::Error::last_os_error())
				}
				else
				{
					unexpected_result!(unlink, result)
				}
			},
			
			|path_bytes_excluding_terminating_ascii_null|
			{
				let copy = unsafe
				{
					let mut copy: [u8; Self::PathLengthWithTerminatingAsciiNull] = unsafe_uninitialized();
					copy.as_mut_ptr().copy_from_nonoverlapping(path_bytes_excluding_terminating_ascii_null.as_ptr(), sockaddr_un::PathLength);
					copy.set_unchecked_mut_safe(sockaddr_un::PathLength, Self::AsciiNull);
					copy
				};
				
				let result = unsafe { unlink(copy.as_ptr() as *const i8) };
				if likely!(result == 0)
				{
					Ok(())
				}
				else if likely!(result == -1)
				{
					Err(io::Error::last_os_error())
				}
				else
				{
					unexpected_result!(unlink, result)
				}
			}
		)
	}
	
	#[inline(always)]
	fn use_bytes<R>(&self, address_length: usize, unnamed: impl FnOnce() -> R, abstract_: impl FnOnce(&[u8]) -> R, path_is_ascii_null_terminated: impl FnOnce(&[u8]) -> R, path_is_not_ascii_null_terminated: impl FnOnce(&[u8; Self::PathLength]) -> R) -> R
	{
		if Self::is_unnamed(address_length)
		{
			unnamed()
		}
		else if self.is_abstract()
		{
			abstract_(self.abstract_name_excluding_leading_ascii_null(address_length))
		}
		else
		{
			self.if_path_bytes_less_than_maximum
			(
				address_length,
				path_is_ascii_null_terminated,
				path_is_not_ascii_null_terminated,
			)
		}
	}
	
	#[inline(always)]
	fn if_path_bytes_less_than_maximum<R>(&self, address_length: usize, path_is_ascii_null_terminated: impl FnOnce(&[u8]) -> R, path_is_not_ascii_null_terminated: impl FnOnce(&[u8; Self::PathLength]) -> R) -> R
	{
		let actual_length_of_path_bytes_excluding_terminating_ascii_nul = Self::actual_length_of_path_bytes_excluding_terminating_ascii_nul(address_length);
		if likely!(Self::path_bytes_less_than_maximum(actual_length_of_path_bytes_excluding_terminating_ascii_nul))
		{
			debug_assert_eq!(self.sun_path[actual_length_of_path_bytes_excluding_terminating_ascii_nul], Self::AsciiNull);
			let path_bytes_excluding_terminating_ascii_null = &self.sun_path[0 .. actual_length_of_path_bytes_excluding_terminating_ascii_nul];
			path_is_ascii_null_terminated(path_bytes_excluding_terminating_ascii_null)
		}
		else
		{
			let path_bytes_excluding_terminating_ascii_null = &self.sun_path;
			path_is_not_ascii_null_terminated(path_bytes_excluding_terminating_ascii_null)
		}
	}
	
	#[inline(always)]
	const fn is_unnamed(length: usize) -> bool
	{
		length <= Self::SizeOfFamily
	}
	
	#[inline(always)]
	fn is_abstract(&self) -> bool
	{
		let first_byte = self.sun_path.get_unchecked_value_safe(0);
		first_byte == Self::AsciiNull
	}
	
	#[inline(always)]
	fn abstract_name_excluding_leading_ascii_null(&self, address_length: usize) -> &[u8]
	{
		&self.sun_path[Self::SizeOfAsciiNull .. (Self::SizeOfAsciiNull + Self::actual_length_of_abstract_name_bytes_excluding_leading_ascii_nul(address_length))]
	}
	
	#[inline(always)]
	const fn actual_length_of_abstract_name_bytes_excluding_leading_ascii_nul(address_length: usize) -> usize
	{
		address_length - (Self::SizeOfFamily + Self::SizeOfAsciiNull)
	}
	
	#[inline(always)]
	const fn actual_length_of_path_bytes_excluding_terminating_ascii_nul(address_length: usize) -> usize
	{
		address_length - (Self::SizeOfFamily + Self::SizeOfAsciiNull)
	}
	
	#[inline(always)]
	const fn path_bytes_less_than_maximum(actual_length_of_path_bytes_excluding_terminating_ascii_nul: usize) -> bool
	{
		actual_length_of_path_bytes_excluding_terminating_ascii_nul < Self::PathLength
	}
}
