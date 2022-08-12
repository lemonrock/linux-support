// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub(super) struct flock
{
	pub(super) l_type: c_short,
	pub(super) l_whence: c_short,
	pub(super) l_start: off_t,
	pub(super) l_len: off_t,
	pub(super) l_pid: pid_t,
}

impl flock
{
	#[inline(always)]
	pub(super) fn process_test(&self) -> Result<(), (AdvisoryFileRecordLock, ExtendedSeekFrom, i64, Option<ProcessIdentifierChoice>)>
	{
		use self::AdvisoryFileRecordLock::*;
		let advisory_record_lock = match self.l_type as i32
		{
			F_UNLCK => return Ok(()),
			
			F_RDLCK => Read,
			
			F_WRLCK => Write,
			
			unknown @ _ => panic!("Unknown l_type {}", unknown),
		};

		let seek = ExtendedSeekFrom::from_whence_and_start(self);

		let length = self.l_len;

		let process_identifier = match self.l_pid
		{
			-1 => None,
			0 => Some(ProcessIdentifierChoice::Current),
			pid if pid > 0 => Some(ProcessIdentifierChoice::Other(ProcessIdentifier::from(new_non_zero_i32(pid)))),
			unexpected @ _ => panic!("Unexpected pid {}", unexpected),
		};

		Err((advisory_record_lock, seek, length, process_identifier))
	}

	#[inline(always)]
	pub(super) fn new_for_per_process(advisory_file_record_lock: AdvisoryFileRecordLock, start: ExtendedSeekFrom, length: i64) -> Self
	{
		Self::new_for_per_process_lock_type(advisory_file_record_lock as i32, start, length)
	}

	#[allow(deprecated)]
	#[inline(always)]
	pub(super) fn new_for_per_process_lock_type(lock_type: i32, start: ExtendedSeekFrom, length: i64) -> Self
	{
		Self::new(lock_type, start, length, unsafe_uninitialized())
	}

	#[inline(always)]
	pub(super) fn new_for_open_file_description(advisory_file_record_lock: AdvisoryFileRecordLock, start: ExtendedSeekFrom, length: i64) -> Self
	{
		Self::new_for_open_file_description_lock_type(advisory_file_record_lock as i32, start, length)
	}

	#[inline(always)]
	pub(super) fn new_for_open_file_description_lock_type(lock_type: i32, start: ExtendedSeekFrom, length: i64) -> Self
	{
		Self::new(lock_type, start, length, 0)
	}

	#[inline(always)]
	fn new(lock_type: i32, start: ExtendedSeekFrom, length: i64, pid: pid_t) -> Self
	{
		let (whence, start) = start.to_whence_and_start();

		Self
		{
			l_type: lock_type as c_short,
			l_whence: whence,
			l_start: start,
			l_len: length,
			l_pid: pid,
		}
	}
}
