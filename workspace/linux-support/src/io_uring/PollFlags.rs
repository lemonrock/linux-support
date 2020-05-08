// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Poll flags.
	pub struct PollFlags: u16
	{
		// .
		const In = POLLIN as u16;

		// .
		const Priority = POLLPRI as u16;

		// .
		const Out = POLLOUT as u16;

		// .
		const Error = POLLERR as u16;

		// .
		const HangUp = POLLHUP as u16;

		// .
		const NoValue = POLLNVAL as u16;

		// .
		const ReadNormal = POLLRDNORM as u16;

		// .
		const ReadBand = POLLRDBAND as u16;

		// .
		const WriteNormal = Self::POLLWRNORM;

		// .
		const WriteBand = Self::POLLWRBAND;

		// .
		const Message = Self::POLLMSG;

		// .
		const ReadHangUp = Self::POLLRDHUP;
	}
}

impl PollFlags
{
	const POLLWRNORM: u16 = 0x100;
	
	const POLLWRBAND: u16 = 0x200;
	
	const POLLMSG: u16 = 0x400;
	
	const POLLRDHUP: u16 = 0x2000;
}
