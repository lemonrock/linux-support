// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Represents an event that occurs after waiting on an epoll file descriptor.
#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(target_arch = "x86_64", repr(C, packed))]
#[cfg_attr(not(target_arch = "x86_64"), repr(C))]
pub struct epoll_event
{
	/// The events member is a bit mask composed by ORing together zero or more of the following available event types (`EPOLL*` constants).
	pub(crate) events: u32,

	/// An union containing the data associated when epoll_ctl was called.
	pub(crate) data: epoll_data_t,
}

impl epoll_event
{
	/// Returns readiness flags.
	#[inline(always)]
	pub fn flags(&self) -> EPollEventFlags
	{
		unsafe { transmute(self.events) }
	}

	/// Token.
	#[inline(always)]
	pub fn token(&self) -> u64
	{
		unsafe { self.data.u64 }
	}
}
