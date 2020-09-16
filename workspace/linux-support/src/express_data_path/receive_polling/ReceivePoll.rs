// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A receive poll.
///
/// In practice, this could suspend the thread or coroutine, or make a callback via `io_uring`.
///
/// When an implementor of this trait is dropped, it should unregister the `express_data_path_socket_file_descriptor` passed in `register()` below.
pub trait ReceivePoll
{
	/// A 'blocking' receive poll.
	///
	/// If using the `poll()` system call, it is sufficient to check for only the `POLLIN` event.
	///
	/// Timing out is also supported; there is no need to report this and hence no return value is provided for.
	fn blocking_poll(&mut self);
}
