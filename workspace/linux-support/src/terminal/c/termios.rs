// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub(crate) struct termios
{
	/// Input mode flags.
	pub(crate) c_iflag: tcflag_t,

	/// Output mode flags.
	pub(crate) c_oflag: tcflag_t,

	/// Control mode flags.
	pub(crate) c_cflag: tcflag_t,

	/// Local mode flags.
	pub(crate) c_lflag: tcflag_t,

	/// Line discipline ("ldisc").
	///
	/// Seems to be unused in user space by libc.
	///
	/// Should normally be set to `N_TTY` (zero) for terminals; a list of constants (namespaced `N_`) is at <https://github.com/torvalds/linux/blob/6f0d349d922ba44e4348a17a78ea51b7135965b1/include/uapi/linux/tty.h>.
	c_line: cc_t,

	/// Control characters.
	pub(crate) c_cc: [cc_t; NCCS],

	/// Never modified in Linux user space, even by libc.
	///
	/// Input speed is always ignored.
	///
	/// Use `cfsetspeed()` or `cfsetispeed()`.
	__c_ispeed: speed_t,

	/// Never modified in user space, even by libc.
	///
	/// Instead modifies the `c_cflag` field via `cfsetospeed()`.
	///
	/// Use `cfsetspeed()` or `cfsetispeed()`.
	__c_ospeed: speed_t,
}

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct termios
{
	pub(crate) c_iflag: tcflag_t,
	pub(crate) c_oflag: tcflag_t,
	pub(crate) c_cflag: tcflag_t,
	pub(crate) c_lflag: tcflag_t,
	pub(crate) c_cc: [cc_t; NCCS],
	c_ispeed: speed_t,
	c_ospeed: speed_t
}
