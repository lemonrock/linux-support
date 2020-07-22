// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(super) const XDP_SHARED_UMEM: u32 = 1 << 0;

/// Force copy-mode.
pub(super) const XDP_COPY: u32 = 1 << 1;

/// Force zero-copy mode.
pub(super) const XDP_ZEROCOPY: u32 = 1 << 2;

/// If this option is set, the driver might go sleep and in that case the `XDP_RING_NEED_WAKEUP` flag in the fill and/or Tx rings will be set.
/// If it is set, the application need to explicitly wake up the driver with a `poll()` (Rx and Tx) or `sendto()` (Tx only).
/// If you are running the driver and the application on the same core, you should use this option so that the kernel will yield to the user space application.
pub(super) const XDP_USE_NEED_WAKEUP: u32 = 1 << 3;
