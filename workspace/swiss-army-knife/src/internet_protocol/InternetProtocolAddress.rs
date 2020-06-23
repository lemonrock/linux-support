// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An Internet Protocol (IP) version 4 or version 6 address.
pub trait InternetProtocolAddress: Clone + PartialEq + Eq + PartialOrd + Ord + Sized + Default
{
	/// Inclusive aximum prefix (netmask / subnet) length.
	const InclusiveMaximumPrefixLength: u8;
	
	/// Address family, eg `AF_INET`.
	const AddressFamily: u8;
	
	/// Default value.
	const LocalHost: Self;
	
	/// Bytes.
	///
	/// If const generics were operational in Rust, could return `&[u8; Size]`.
	fn bytes(&self) -> &[u8];
	
	/// From bytes.
	fn from_bytes(bytes: &[u8]) -> Result<Self, TryFromSliceError>;
}
