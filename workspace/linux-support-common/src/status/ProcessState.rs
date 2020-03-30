// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Process state.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProcessState
{
	/// Running.
	Running,

	/// Sleeping in an interruptible wait.
	Sleeping,

	/// Waiting in uninterruptible disk sleep.
	SleepingInAnUninterruptibleWait,

	/// Stopped.
	///
	/// Before Linux 2.6.33, this also covered `TracingStop` below.
	Stopped,

	/// Stopped while Traching.
	///
	/// Linux 2.6.33 onward.
	TracingStop,

	/// Dead.
	Dead,

	/// Zombie.
	Zombie,

	/// Idle.
	///
	/// Not documented in manpage for `/proc/<N>/stat`.
	Idle,

	/// WakeKill.
	///
	/// Linux 3.9 to 3.13 only.
	#[deprecated]
	WakeKill,

	/// Parked.
	///
	/// Linux 3.9 to 3.13 only.
	#[deprecated]
	Parked,

	/// Paging before Linux 2.6.0.
	///
	/// Waking for Linux 2.6.33 to 3.13 only.
	#[deprecated]
	PagingOrWaking,
}
