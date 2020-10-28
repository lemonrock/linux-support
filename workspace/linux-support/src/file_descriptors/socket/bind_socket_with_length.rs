// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[inline(always)]
fn bind_socket_with_length<SA>(socket_file_descriptor: &impl FileDescriptor, socket_address: &SA, length: usize) -> Result<(), SocketBindError>
{
	use self::SocketBindError::*;
	use self::FilePathInvalidReason::*;

	let result = unsafe { bind(socket_file_descriptor.as_raw_fd(), socket_address as *const SA as *const sockaddr_storage, length as socklen_t) };
	if likely!(result == 0)
	{
		Ok(())
	}
	else if likely!(result == -1)
	{
		Err
		(
			match errno().0
			{
				EACCES => PermissionDenied,
				EADDRINUSE => AddressInUse,
				ENOMEM => KernelWouldBeOutOfMemory,
				EBADF => panic!("`sockfd` is not a valid descriptor"),
				EINVAL => panic!("already bound, or the `addrlen` is wrong, or the socket was not in the `AF_UNIX` family"),
				ENOTSOCK => panic!("`sockfd` is not a socket file descriptor"),

				EADDRNOTAVAIL => FilePathInvalid(AddressUnavailable),
				EFAULT => panic!("`addr` points outside the user's accessible address space"),
				ELOOP => FilePathInvalid(TooManySymbolicLinksInFilePath),
				ENOENT => FilePathInvalid(DoesNotExist),
				ENOTDIR => FilePathInvalid(FilePathPrefixComponentIsNotADirectory),
				EROFS => FilePathInvalid(FilePathIsReadOnly),

				EAFNOSUPPORT => panic!("Invalid `sa_family_t` value"),

				_ => unreachable_code(format_args!("")),
			}
		)
	}
	else
	{
		unreachable_code(format_args!(""))
	}
}
