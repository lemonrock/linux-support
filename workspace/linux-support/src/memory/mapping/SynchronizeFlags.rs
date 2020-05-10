// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Synchronize flags.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(i32)]
pub enum SynchronizeFlags
{
	/// Specifies that an update be scheduled, but the call returns immediately.
	Asynchronous = MS_ASYNC,

	/// Specifies that an update be scheduled, but the call returns immediately.
	///
	/// Also, asks to invalidate other mappings of the same file (so that they can be updated with the fresh values just written).
	AsynchronousAndInvalidate = MS_ASYNC | MS_INVALIDATE,

	/// Requests an update and waits (blocks) for it to complete.
	Synchronous = MS_SYNC,

	/// Requests an update and waits (blocks) for it to complete.
	///
	/// Also, asks to invalidate other mappings of the same file (so that they can be updated with the fresh values just written).
	SynchronousAndInvalidate = MS_SYNC | MS_INVALIDATE,
}
