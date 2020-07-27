// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// If attaching a program fails, the Linux kernel will normally restore the previously attached program ***EXCEPT*** for the `Offloaded` mode.
///
/// The default is `GenericOrNative`.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum AttachMode
{
	/// Allows the Linux kernel to choose the `Native` (but not `Offloaded`) mode if supported.
	///
	/// Falls back to `Generic` mode otherwise.
	///
	/// This is the default.
	///
	/// Can cause the error `EEXIST` on program attach if there is already a program attached in the `Native` or `Offloaded` mode.
	GenericOrNative,
	
	/// Uses a soft in-kernel implementation.
	///
	/// Can cause the error `EEXIST` on program attach if there is already a program attached in the `Native` or `Offloaded` mode.
	/// Can cause the error `EINVAL` on program attach if the program being attached can only be used in `Offloaded` mode (it is a 'device-bound' program).
	Generic,
	
	/// Uses native driver support but code runs on regular CPU.
	///
	/// Can cause the error `EOPNOTSUPP` on program attach if the driver does not support this mode.
	/// Can cause the error `EINVAL` on program attach if the program being attached can only be used in `Offloaded` mode (it is a 'device-bound' program).
	Native,
	
	/// Uses native driver support with code running on a processor on the native hardware.
	///
	/// An expanded set of functionality over `Native`.
	///
	/// Rarely supported.
	/// It is known that the Netronome `nfp` driver can support this for some Netronome hardware.
	///
	/// Can cause the error `EOPNOTSUPP` on program attach if the driver does not support this mode.
	Offloaded,
}

impl Default for AttachMode
{
	#[inline(always)]
	fn default() -> Self
	{
		AttachMode::GenericOrNative
	}
}
