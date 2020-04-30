// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a seccomp user notification identifier.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SeccompUserNotificationFileDescriptor(RawFd);

impl Drop for SeccompUserNotificationFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.0.close()
	}
}

impl AsRawFd for SeccompUserNotificationFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl IntoRawFd for SeccompUserNotificationFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.0
	}
}

impl FromRawFd for SeccompUserNotificationFileDescriptor
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(fd)
	}
}

impl FileDescriptor for SeccompUserNotificationFileDescriptor
{

}

impl SeccompUserNotificationFileDescriptor
{
	/// Allocates on the heap.
	#[inline(always)]
	pub fn new_request() -> VariablySized<seccomp_notif>
	{
		seccomp_notif::allocate_zeroed()
	}

	/// Allocates on the heap.
	#[inline(always)]
	pub fn new_response() -> VariablySized<seccomp_notif_resp>
	{
		seccomp_notif_resp::allocate_zeroed()
	}

	/// Reads an user notification.
	///
	/// Blocks (it may be possible to use this file descriptor with epoll).
	///
	/// Returns `request` on success.
	/// A `request` requires a response to be sent using `RequestGuard.send_response()`.
	#[inline(always)]
	pub fn get_request(&self, mut request: VariablySized<seccomp_notif>) -> io::Result<VariablySized<seccomp_notif>>
	{
		request.zero();
		let result = unsafe { ioctl(self.as_raw_fd(), SECCOMP_IOCTL_NOTIF_RECV, request.deref_mut() as *mut seccomp_notif as *mut c_void) };
		if likely!(result == 0)
		{
			Ok(request)
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable!("ioctl() returned unexpected result {}", result);
		}
	}

	/// This exists to avoid a TOCTOU.
	///
	/// Call this check after any task-specific resources are opened (eg such as `/proc/<request.pid>/mem`), to make sure that the task has not died and we're not wrongly reading someone else's state in order to make decisions.
	///
	/// (This exists because of the wrap-around of PIDs - today one would use a UUID).
	#[inline(always)]
	pub fn request_is_still_valid(&self, request: &seccomp_notif) -> bool
	{
		let result = unsafe { ioctl(self.as_raw_fd(), SECCOMP_IOCTL_NOTIF_ID_VALID, &request.id as *const u64 as *mut u64 as *mut _) };
		if likely!(result == 0)
		{
			true
		}
		else if likely!(result == -1)
		{
			false
		}
		else
		{
			unreachable!("ioctl returned unexpected result {}", result)
		}
	}

	/// Sends a response.
	///
	/// The response will be re-used and all data in it lost.
	///
	/// `error` is something like `EPERM`; it can not exceed `4095` inclusive.
	/// If `error` is `0` the syscall succeeds.
	#[inline(always)]
	pub fn send_response(&self, request: &seccomp_notif, mut response: VariablySized<seccomp_notif_resp>, value: i64, error: u32, flags: UserNotificationFlags) -> io::Result<VariablySized<seccomp_notif_resp>>
	{
		debug_assert!(error < 4096);

		response.zero();
		response.id = request.id;
		response.val = value;
		response.error = -(error as i32);
		response.flags = flags;

		let result = unsafe { ioctl(self.as_raw_fd(), SECCOMP_IOCTL_NOTIF_SEND, response.deref_mut() as *mut seccomp_notif_resp as *mut c_void) };
		if likely!(result == 0)
		{
			Ok(response)
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable!("ioctl() returned unexpected result {}", result);
		}
	}
}
