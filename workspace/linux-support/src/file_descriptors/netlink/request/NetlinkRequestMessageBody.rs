// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A netlink request message body.
///
/// All Netlink request message bodies must contain a first initial field of one unsigned char byte, the `family`.
///
/// The layout must be compatible with the struct `rtgenmsg`:-
///
/// ```
/// #[repr(C)]
/// struct rtgenmsg
/// {
/// 	rtgen_family: libc::c_uchar,
/// }
/// ```
pub trait NetlinkRequestMessageBody: Sized
{
	/// Deliberately exists to force correct implementation.
	fn family(&self) -> c_uchar;
}
