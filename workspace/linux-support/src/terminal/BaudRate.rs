// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Baud rate.
#[derive(EnumIter, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum BaudRate
{
	/// This is used to terminate the connection.
	///
	/// If specified, the modem control lines shall no longer be asserted.
	/// Normally, this will disconnect the line.
	HangUpModem = B0,

	/// Baud rate of 50 bits per second (bps).
	B50 = B50,

	/// Baud rate of 75 bits per second (bps).
	B75 = B75,

	/// Baud rate of 110 bits per second (bps).
	B110 = B110,

	/// Baud rate of 134 bits per second (bps).
	B134 = B134,

	/// Baud rate of 150 bits per second (bps).
	B150 = B150,

	/// Baud rate of 200 bits per second (bps).
	B200 = B200,

	/// Baud rate of 300 bits per second (bps).
	B300 = B300,

	/// Baud rate of 600 bits per second (bps).
	B600 = B600,

	/// Baud rate of 1,200 bits per second (bps).
	B1200 = B1200,

	/// Baud rate of 1,800 bits per second (bps).
	B1800 = B1800,

	/// Baud rate of 2,400 bits per second (bps).
	B2400 = B2400,

	/// Baud rate of 4,800 bits per second (bps).
	B4800 = B4800,

	/// Baud rate of 9,600 bits per second (bps).
	B9600 = B9600,

	/// Baud rate of 19,200 bits per second (bps).
	B19200 = B19200,

	/// Baud rate of 38,400 bits per second (bps).
	///
	/// Actually, this can be almost any bit rate on linux; see `man 8 setserial`.
	B38400 = B38400,

	/// Baud rate of 57,600 bits per second (bps).
	B57600 = B57600,

	/// Baud rate of 115,200 bits per second (bps).
	///
	/// This is the default.
	B115200 = B115200,

	/// Baud rate of 230,400 bits per second (bps).
	B230400 = B230400,

	/// Baud rate of 500,000 bits per second (bps).
	///
	/// Only supported on Android, ?Fuschia and Linux.
	#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))]
	B500000 = B500000,

	/// Baud rate of 576,600 bits per second (bps).
	///
	/// Only supported on Android, ?Fuschia and Linux.
	#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))]
	B576000 = B576000,

	/// Baud rate of 460,800 bits per second (bps).
	///
	/// Only supported on Android, FreeBSD, ?Fuschia and Linux.
	#[cfg(any(target_os = "android", taget_os = "freebsd", target_os = "fuschia", target_os = "linux"))]
	B460800 = B460800,

	/// Baud rate of 921,600 bits per second (bps).
	///
	/// Only supported on Android, FreeBSD, ?Fuschia and Linux.
	#[cfg(any(target_os = "android", taget_os = "freebsd", target_os = "fuschia", target_os = "linux"))]
	B921600 = B921600,

	/// Baud rate of 1,000,000 bits per second (bps).
	///
	/// Only supported on Android, ?Fuschia and Linux.
	#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))]
	B1000000 = B1000000,

	/// Baud rate of 1,152,000 bits per second (bps).
	///
	/// Only supported on Android, ?Fuschia and Linux.
	#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))]
	B1152000 = B1152000,

	/// Baud rate of 1,500,000 bits per second (bps).
	///
	/// Only supported on Android, ?Fuschia and Linux.
	#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))]
	B1500000 = B1500000,

	/// Baud rate of 2,000,000 bits per second (bps).
	///
	/// Only supported on Android, ?Fuschia and Linux.
	#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))]
	B2000000 = B2000000,

	/// Baud rate of 2,500,000 bits per second (bps).
	///
	/// Only supported on Android, ?Fuschia and Linux.
	#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))]
	B2500000 = B2500000,

	/// Baud rate of 3,000,000 bits per second (bps).
	///
	/// Only supported on Android, ?Fuschia and Linux.
	#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))]
	B3000000 = B3000000,

	/// Baud rate of 3,500,000 bits per second (bps).
	///
	/// Only supported on Android, ?Fuschia and Linux.
	#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))]
	B3500000 = B3500000,

	/// Baud rate of 4,000,000 bits per second (bps).
	///
	/// Only supported on Android, ?Fuschia and Linux.
	#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))]
	B4000000 = B4000000,
}

impl Default for BaudRate
{
	#[inline(always)]
	fn default() -> Self
	{
		BaudRate::B115200
	}
}

impl BaudRate
{
	/// `EXTA` baud rate; 19,200 bits per second.
	pub const ExtendedA: Self = BaudRate::B19200;

	/// `EXTB` baud rate; 38,400 bits per second.
	pub const ExtendedB: Self = BaudRate::B38400;

	/// On Linux, the terminal input speed and terminal output speed can not differ.
	#[inline(always)]
	pub(crate) fn set_terminal_input_and_output_speed(self, terminal_options: &mut termios)
	{
		let result = unsafe { cfsetspeed(terminal_options, self as u32) };
		if likely!(result == 0)
		{
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EINVAL => panic!("Baud was not valid (`& !CBAUD`)"),

				_ => unreachable!(),
			}
		}
		else
		{
			unreachable!()
		}
	}
}
