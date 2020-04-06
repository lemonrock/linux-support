// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Sharing.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum Sharing
{
	/// Visible only to this process.
	Private = MAP_PRIVATE,

	/// Visible to other processes, either to those that are cloned (forked) or use the same file (if file backed).
	///
	/// Changes to memory backed by files are only g'teed to become visible once `msync()` has been called.
	///
	/// Internally, this uses `MAP_SHARED_VALIDATE` rather than `MAP_SHARED`, making it compatible only with Linux 4.15 and later.
	Shared = MAP_SHARED_VALIDATE,

	/// This is intended for Persistent RAM and other similar technologies that use Linux DAX (Direct Access).
	///
	/// Shared file mappings with this flag provide the guarantee that while some memory is writably mapped in the address space of the process, it will be visible in the same file at the same offset even after the system crashes or is rebooted.
	/// In conjunction with the use of appropriate CPU instructions, this provides users of such mappings with a more efficient way of making data modifications persistent.
	///
	/// There is no need to used `msync()` with this short of mapping but instead a variant of `CLWB`.
	///
	/// Internally, this uses `MAP_SHARED_VALIDATE` with `MAP_SYNC`.
	Persistent = MAP_SHARED_VALIDATE | MAP_SYNC,
}
