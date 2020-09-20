// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Frame descriptor.
/// Used for `ReceiveQueue` and `TransmitQueue`.
///
/// Effectively, a fat pointer, containing a relative, bit-encoded pointer to a frame in memory and the length of the frame.
pub(crate) type FrameDescriptor = xdp_desc;

impl Descriptor for FrameDescriptor
{
}
