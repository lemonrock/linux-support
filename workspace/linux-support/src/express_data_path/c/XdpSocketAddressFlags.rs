// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Flags for `struct sockaddr_xdp`.
	pub(super) struct XdpSocketAddressFlags: u16
	{
		const SharedUserMemory = XDP_SHARED_UMEM;
		
		/// Force copy-mode.
		const ForceCopy = XDP_COPY;
		
		/// Force zero-copy mode.
		const ForceZeroCopy = XDP_ZEROCOPY;
		
		/// If this option is set, the driver might go sleep and in that case the `XDP_RING_NEED_WAKEUP` flag in the fill and/or Tx rings will be set.
		/// If it is set, the application need to explicitly wake up the driver with a `poll()` (Rx and Tx) or `sendto()` (Tx only).
		/// If you are running the driver and the application on the same core, you should use this option so that the kernel will yield to the user space application.
		const UseNeedsWakeUp = XDP_USE_NEED_WAKEUP;
	}
}

impl Default for XdpSocketAddressFlags
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::empty()
	}
}

impl XdpSocketAddressFlags
{
	#[inline(always)]
	pub(crate) fn sxdp_flags(mut self, force_copy: bool, force_zero_copy: bool, needs_wake_up: bool) -> Self
	{
		if force_copy
		{
			self |= XdpSocketAddressFlags::ForceCopy;
		}
		if force_zero_copy
		{
			self |= XdpSocketAddressFlags::ForceZeroCopy;
		}
		if needs_wake_up
		{
			self |= XdpSocketAddressFlags::UseNeedsWakeUp;
		}
		self
	}
}
