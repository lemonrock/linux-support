// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Owned arbitrary bytes.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct OwnedBytes(Box<[u8]>);

impl HasTypeEquality for OwnedBytes
{
	type TypeEquality = OwnedTypeEquality;
}

impl OwnedOrParsedBytes for OwnedBytes
{
}

impl Deref for OwnedBytes
{
	type Target = [u8];
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0[..]
	}
}

impl<'message, 'a> From<&'a ParsedBytes<'message>> for OwnedBytes
{
	#[inline(always)]
	fn from(value: &'a ParsedBytes<'message>) -> Self
	{
		Self(value.0.to_vec().into_boxed_slice())
	}
}

impl<'message> From<ParsedBytes<'message>> for OwnedBytes
{
	#[inline(always)]
	fn from(value: ParsedBytes<'message>) -> Self
	{
		Self::from(&value)
	}
}
