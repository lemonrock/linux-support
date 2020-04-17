// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[link(name = "c")]
extern "C"
{
	/// Given a `pathname` for a file, `open()` returns a file descriptor.
	///
	/// The argument flags must include one of the following access modes: `O_RDONLY`, `O_WRONLY`, or `O_RDWR`.
	/// These request opening the file read-only, write-only, or read/write, respectively.
	///
	/// In addition, zero or more file creation `flags` and file status `flags` can be bitwise-or'd in flags.
	///
	/// `open()` returns the new file descriptor, or `-1` if an error occurred (in which case, `errno` is set appropriately).
	///
	/// * `EACCES`: The requested access to the file is not allowed, or search permission is denied for one of the directories in the path prefix of pathname, or the file did not exist yet and write access to the parent directory is not allowed.
	/// * `EDQUOT`: Where `O_CREAT `is specified, the file does not exist, and the user's quota of disk blocks or inodes on the file system has been exhausted.
	/// * `EEXIST`: `pathname` already exists and `O_CREAT` and `O_EXCL` were used.
	/// * `EFAULT`: `pathname` points outside your accessible address space.
	/// * `EFBIG`: See `EOVERFLOW`.
	/// * `EINTR`: While blocked waiting to complete an open of a slow device (eg, a FIFO), the call was interrupted by a signal handler.
	/// * `EISDIR`: `pathname` refers to a directory and the access requested involved writing (that is, `O_WRONLY` or `O_RDWR` is set).
	/// * `ELOOP`: Too many symbolic links were encountered in resolving pathname, or `O_NOFOLLOW` was specified but pathname was a symbolic link.
	/// * `EMFILE`: The process already has the maximum number of files open.
	/// * `ENAMETOOLONG`: `pathname` was too long.
	/// * `ENFILE`: The system limit on the total number of open files has been reached.
	/// * `ENODEV`: `pathname` refers to a device special file and no corresponding device exists. (This is a Linux kernel bug; in this situation `ENXIO` must be returned).
	/// * `ENOENT`: `O_CREAT` is not set and the named file does not exist. Or, a directory component in pathname does not exist or is a dangling symbolic link.
	/// * `ENOMEM`: Insufficient kernel memory was available.
	/// * `ENOSPC`: `pathname` was to be created but the device containing `pathname` has no room for the new file.
	/// * `ENOTDIR`: A component used as a directory in `pathname` is not, in fact, a directory, or `O_DIRECTORY` was specified and `pathname` was not a directory.
	/// * `ENXIO`: O_NONBLOCK | O_WRONLY is set, the named file is a FIFO and no process has the file open for reading. Or, the file is a device special file and no corresponding device exists.
	/// * `EOVERFLOW`: `pathname` refers to a regular file that is too large to be opened. The usual scenario here is that an application compiled on a 32-bit platform without `-D_FILE_OFFSET_BITS=64` tried to open a file whose size exceeds `(2<<31)-1` bits; see also `O_LARGEFILE` above. This is the error specified by POSIX.1-2001; in kernels before 2.6.24, Linux gave the error `EFBIG` for this case.
	/// * `EPERM`: The `O_NOATIME` flag was specified, but the effective user ID of the caller did not match the owner of the file and the caller was not privileged (`CAP_FOWNER`).
	/// * `EROFS`: `pathname` refers to a file on a read-only file system and write access was requested.
	/// * `ETXTBSY`: `pathname` refers to an executable image which is currently being executed and write access was requested.
	/// * `EWOULDBLOCK`: The `O_NONBLOCK` flag was specified, and an incompatible lease was held on the file (see `man 2 fcntl`).
	pub(crate) fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
}
