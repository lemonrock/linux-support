// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Metadata returned by read.
#[repr(C, align(8))]
pub struct fanotify_event_metadata
{
	/// This is the length of the data for the current event and the offset to the next event in the buffer.
	///
	/// In the current implementation, the value of `event_len` is always `FAN_EVENT_METADATA_LEN`.
	/// However, the API is designed to allow variable-length structures to be returned in the future.
	event_len: c_uint,

	/// This field holds a version number for the structure.
	///
	/// It must be compared to `FANOTIFY_METADATA_VERSION` to verify that the structures returned at run time match the structures defined at compile time.
	/// In case of a mismatch, the application should abandon trying to use the fanotify file descriptor.
	vers: c_uchar,

	/// This field is not used.
	reserved: c_uchar,

	/// This is the length of the structure.
	///
	/// The field was introduced to facilitate the implementation of optional headers per event type.
	///
	/// No such optional headers exist in the current implementation.
	metadata_len: c_ushort,

	/// This is a bit mask describing the event.
	mask: c_ulonglong,

	/// This is an open file descriptor for the object being accessed, or `FAN_NOFD` if a queue overflow occurred.
	///
	/// The file descriptor can be used to access the contents of the monitored file or directory.
	/// The reading application is responsible for closing this file descriptor.
	///
	/// When calling `fanotify_init()`, the caller may specify (via the `event_f_flags` argument) various file status flags that are to be set on the open file description that corresponds to this file descriptor.
	///
	/// In addition, the (kernel-internal) `FMODE_NONOTIFY` file status flag is set on the open file description.
	/// This flag suppresses fanotify event generation.
	/// Hence, when the receiver of the fanotify event accesses the notified file or directory using this file descriptor, no additional events will be created.
	fd: RawFd,

	/// This is the ID of the process that caused the event.
	///
	/// A program listening to fanotify events can compare this PID to the PID returned by `getpid()`, to determine whether the event is caused by the listener itself, or is due to a file access by another process.
	pid: pid_t,
}

impl fanotify_event_metadata
{
	/// Destroys the underlying data structure returned by the kernel.
	///
	/// Takes `self` by value because the lifetime of the `File` needs to be managed, making this method inconvenient to use.
	#[inline(always)]
	pub fn move_out(self) -> (Option<File>, EventFlags, pid_t)
	{
		debug_assert!(self.is_valid(), "Is not valid");

		let file = if self.fd == FAN_NOFD
		{
			None
		}
		else
		{
			Some(unsafe { File::from_raw_fd(self.fd) })
		};
		(file, unsafe { transmute(self.mask) }, self.pid)
	}

	#[inline(always)]
	fn is_valid(&self) -> bool
	{
		self.event_len == FAN_EVENT_METADATA_LEN || self.vers == FANOTIFY_METADATA_VERSION || self.metadata_len == 0
	}
}

/// Value of the `event_len` field in `fanotify_event_metadata`.
pub(crate) const FAN_EVENT_METADATA_LEN: c_uint = size_of::<fanotify_event_metadata>() as c_uint;

/// Value of the `vers` field in `fanotify_event_metadata`.
pub(crate) const FANOTIFY_METADATA_VERSION: c_uchar = 3;

/// Value of the `fd` field in `fanotify_event_metadata` if a queue overflow occurred.
pub(crate) const FAN_NOFD: RawFd = -1;
