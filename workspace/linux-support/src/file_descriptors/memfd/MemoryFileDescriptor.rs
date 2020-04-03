// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A memfd; only works on files.
pub trait MemoryFileDescriptor: Sized + AsRawFd + AsRawFdExt + IntoRawFd + FromRawFd
{
	/// Opens a memfd.
	///
	/// There are two uses:-
	///
	/// * As an alternative to using `/tmp`, `tmpfs` or `O_TMPFILE` if there is no intention to link a file into the file system;
	/// * To use file sealing (see <http://man7.org/linux/man-pages/man2/fcntl.2.html>); particularly `F_SEAL_FUTURE_WRITE` for a shared memory buffer.
	///
	/// The file can be mmap'd and the file descriptor passed to other processes like any other.
	///
	/// The initial size of the file is set to 0.
	///
	/// Following the call, the file size should be set using `ftruncate()`, or writes made using `write()` or the like.
	///
	/// `non_unique_name_for_debugging_purposes` can be seen under `/proc/self/fd` prefixed with `memfd:`.
	/// It does not have to be unique.
	///
	/// `allow_sealing_operations`:  If true, then the `fcntl()` `F_ADD_SEALS` and `F_GET_SEALS` operations are supported; the initial set of seals is empty.
	/// If not specifed, then the initial set of seals will be `F_SEAL_SEAL`, meaning that no other seals can be set on the file.
	///
	/// `huge_page_size` supports the following:-
	///
	/// * `None`: No huge pages.
	/// * `Some(None)`: Use the system default huge page size.
	/// * `Some(Some(huge_page_size))`: Use the specific `huge_page_size` huge page size.
	///
	/// The resultant file descriptor will have the close-on-exec flag set (as do all file descriptors created in the super module, `file_descriptors`).
	///
	/// Supported since Linux 3.17.
	/// However, support for `allow_sealing_operations` with `huge_page_size` has only existed since Linux 4.16.
	fn open_anonymous_memory_as_file(non_unique_name_for_debugging_purposes: &CStr, allow_sealing_operations: bool, huge_page_size: Option<Option<HugePageSize>>) -> Result<Self, CreationError>
	{
		const MFD_CLOEXEC: u64 = 0x0001;
		const MFD_ALLOW_SEALING: u64 = 0x0002;
		const MFD_HUGETLB: u64 = 0x0004;

		extern "C"
		{
			fn memfd_create(name: *const c_char, flags: c_uint) -> c_int;
		}

		let flags: u64 = MFD_CLOEXEC
		| if allow_sealing_operations
		{
			MFD_ALLOW_SEALING
		}
		else
		{
			0
		}
		| match huge_page_size
		{
			None => 0,
			Some(None) => MFD_HUGETLB,
			Some(Some(huge_page_size)) => MFD_HUGETLB | huge_page_size.mmap_and_memfd_flags_bits(),
		};

		let result = unsafe { memfd_create(non_unique_name_for_debugging_purposes.as_ptr() as *const _, flags as u32) };

		if likely!(result == 0)
		{
			return Ok(unsafe { Self::from_raw_fd(result) })
		}
		else if likely!(result == -1)
		{
			use self::CreationError::*;

			match errno().0
			{
				EMFILE => Err(PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded),

				ENFILE => Err(SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded),

				ENOMEM => Err(KernelWouldBeOutOfMemory),

				ENOENT => panic!("No queue with this name exists"),

				EINVAL => panic!("The address in name points to invalid memory, or, flags included unknown bits, or, name was too long (The limit is 249 bytes, excluding the
  terminating null byte), or, both  MFD_HUGETLB and MFD_ALLOW_SEALING were specified in flags before Linux 4.16"),

				_ => unreachable!(),
			}
		}
		else
		{
			panic!("Unexpected result {}", result)
		}
	}
}
