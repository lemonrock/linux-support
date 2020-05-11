// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Poll response flags.
	///
	/// Distinguishes between High-Priority, Priority and Normal data.
	///
	/// Linux does not distinguish between Priority and Normal data.
	///
	/// High-Priority data includes:-
	///
	/// * Out-of-band data on a TCP socket (see `tcp(7)`).
	/// * A pseudoterminal master in packet mode has seen a state change on the slave (see `ioctl_tty(2)`).
	/// * A `cgroup.events` file has been modified (see `cgroups(7)`).
	pub struct PollResponseFlags: u16
	{
		/// Data other than High-Priority data may be read without blocking.
		///
		/// This is equivalent to `ReadNormal | ReadPriority`.
		const In = POLLIN as u16;

		/// High-Priority data may be read without blocking.
		const ReadHighPriority = POLLPRI as u16;

		/// Normal data may be written without blocking.
		///
		/// This is equivalent to `WriteNormal`.
		///
		/// Mutually exclusive with `HangUp`.
		const Out = POLLOUT as u16;

		/// An exceptional condition has occurred on the device or socket.
		///
		/// This flag is output only.
		const Error = POLLERR as u16;

		/// The device or socket has been disconnected.
		///
		/// Note that when reading from a channel such as a pipe or a stream socket, this event merely indicates that the peer closed its end of the channel.
		///
		/// This flag is output only.
		///
		/// Mutually exclusive with `Out`.
		const HangUp = POLLHUP as u16;

		/// The file descriptor is not open.
		///
		/// This flag is output only.
		const FileDescriptorIsNotOpen = POLLNVAL as u16;

		/// Normal data may be read without blocking.
		const ReadNormal = POLLRDNORM as u16;

		/// Priority data may be read without blocking.
		const ReadPriority = POLLRDBAND as u16;

		/// Normal data may be written without blocking.
		const WriteNormal = POLLWRNORM;

		/// Priority data may be written without blocking.
		const WritePriority = POLLWRBAND;

		/// Unused on Linux.
		const Message = POLLMSG;

		/// Stream socket peer closed connection, or shut down writing half of connection.
		///
		/// ?This flag is output only.
		const ReadHangUp = POLLRDHUP;
	}
}

impl From<PollRequestFlags> for PollResponseFlags
{
	#[inline(always)]
	fn from(value: PollRequestFlags) -> Self
	{
		unsafe { transmute(value )}
	}
}
