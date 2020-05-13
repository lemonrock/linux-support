// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// File open kind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileOpenKind<'a>
{
	/// Fail if it does not exist.
	FailIfDoesNotExist
	{
		/// Path.
		path: &'a CStr,

		/// Access.
		access: Access,

		/// `O_EXCL`.
		fail_if_path_name_is_a_block_device_and_is_in_use: bool,

		/// `O_NOFOLLOW`.
		no_follow: bool,
	},

	/// Create if it does not exist.
	CreateIfDoesNotExist
	{
		/// Path.
		path: &'a CStr,

		/// Access.
		access: Access,

		/// If this is true and the file already exists, then creating the file will fail with an error.
		exclusive: bool,

		/// `O_NOFOLLOW`.
		no_follow: bool,

		/// Access permissions.
		access_permissions: AccessPermissions,
	},

	/// Requires the file system to support this (most do).
	MakeATemporaryFile
	{
		/// Access.
		access: TemporaryFileAccess,

		/// Specifying this prevents a temporary file from being linked into the filesystem and so makes it more secure.
		exclusive: bool,

		/// Access permissions.
		access_permissions: AccessPermissions,
	}
}

impl<'a> FileOpenKind<'a>
{
	#[inline(always)]
	fn o_flags_mode_and_path(&self) -> (i32, mode_t, *const c_char)
	{
		use self::FileOpenKind::*;

		match self
		{
			&FailIfDoesNotExist { path, access, fail_if_path_name_is_a_block_device_and_is_in_use, no_follow } =>
			{
				let o_flags = if fail_if_path_name_is_a_block_device_and_is_in_use
				{
					access.to_oflag() | O_NOCTTY | O_EXCL
				}
				else
				{
					access.to_oflag() | O_NOCTTY
				};
				let o_flags = if no_follow
				{
					o_flags | O_NOFOLLOW
				}
				else
				{
					o_flags
				};

				(o_flags, 0, path.as_ptr())
			}

			&CreateIfDoesNotExist { path, access, exclusive, no_follow, access_permissions } =>
			{
				let o_flags = if exclusive
				{
					access.to_oflag() | O_NOCTTY | O_CREAT | O_EXCL
				}
				else
				{
					access.to_oflag() | O_NOCTTY | O_CREAT
				};
				let o_flags = if no_follow
				{
					o_flags | O_NOFOLLOW
				}
				else
				{
					o_flags
				};
				(o_flags, DirectoryFileDescriptor::mask_mode(access_permissions), path.as_ptr())
			}

			&MakeATemporaryFile { access, exclusive, access_permissions } =>
			{
				let o_flags = if exclusive
				{
					access.to_oflag() | O_TMPFILE | O_EXCL
				}
				else
				{
					access.to_oflag() | O_TMPFILE
				};
				(o_flags, DirectoryFileDescriptor::mask_mode(access_permissions), null())
			}
		}
	}
}
