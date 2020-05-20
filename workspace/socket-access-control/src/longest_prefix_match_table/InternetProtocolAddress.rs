// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An Internet Protocol (IP) version 4 or version 6 address.
pub trait InternetProtocolAddress: PartialEq + Eq + PartialOrd + Ord
{
	/// Bytes.
	///
	/// If const generics were operational in Rust, could return `&[u8; Size]`.
	fn bytes(&self) -> &[u8];
}

impl InternetProtocolAddress for in_addr
{
	#[inline(always)]
	fn bytes(&self) -> &[u8]
	{
		let bytes: &[u8; 4] = unsafe { transmute(self) };
		&bytes[..]
	}
}

impl InternetProtocolAddress for in6_addr
{
	#[inline(always)]
	fn bytes(&self) -> &[u8]
	{
		let bytes: &[u8; 16] = unsafe { transmute(self) };
		&bytes[..]
	}
}
