// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A transmit send.
///
/// In practice, this could suspend the thread or coroutine, or make a callback via `io_uring`.
pub trait TransmitSend
{
	/// A 'non-blocking' transmit send.
	///
	/// Typically implemented using `sendto()`.
	///
	/// Timing out is also supported; there is no need to report this and hence no return value is provided for.
	fn send(&mut self);
}
