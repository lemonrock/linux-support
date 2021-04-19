// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// The constructor of the body of a while loop which checks that the thread should continue running.
pub trait ThreadFunction: std::marker::Send + std::marker::Sync + 'static
{
	/// Configured type.
	type TLBF: ThreadLoopBodyFunction;

	/// Initialize.
	///
	/// Runs on the thread itself.
	///
	/// At this point the thread will have been bound to a specific `HyperThread`(s) and so per-thread optimizations can take place, eg creating a thread-specific memory allocator, using code that needs per-hyper-thread lightweight locks, etc.
	/// At this point the thread will still be running as the original invoking user and may have some capabilities (eg to open raw sockets).
	/// It will not yet have a final seccomp profile applied.
	fn initialize(self) -> Result<Self::TLBF, Box<dyn error::Error + Send + Sync + 'static>>;
}
