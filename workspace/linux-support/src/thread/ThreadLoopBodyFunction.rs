// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// The body of a while loop which checks that the thread should continue running.
pub trait ThreadLoopBodyFunction
{
	/// Invoke.
	///
	/// If a pause is desired to avoid busy loops, then the implementor is responsible for calling `libc::nanosleep()` or `thread::yield()`.
	///
	/// `busy_wait_spin_loop_hint()` is called by the runtime after invoke to support execution on HyperThreads.
	///
	/// Use `terminate` if implementing a signal handler, say.
	fn invoke<T: Terminate>(&mut self, terminate: &Arc<T>);
}
