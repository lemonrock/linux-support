// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a network interface alias.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct NetworkInterfaceAlias(ObjectName256);

impl Display for NetworkInterfaceAlias
{
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "{}", self.0)
	}
}

impl FromBytes for NetworkInterfaceAlias
{
	type Error = ObjectNameFromBytesError;
	
	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
	{
		ObjectName256::from_bytes(bytes).map(|object_name| Self(object_name))
	}
}

impl From<ObjectName256> for NetworkInterfaceAlias
{
	#[inline(always)]
	fn from(value: ObjectName256) -> Self
	{
		Self(value)
	}
}

impl Into<ObjectName256> for NetworkInterfaceAlias
{
	#[inline(always)]
	fn into(self) -> ObjectName256
	{
		self.0
	}
}

impl<'a> Into<[c_char; ObjectName256::MaximumLengthIncludingAsciiNull]> for &'a NetworkInterfaceAlias
{
	#[inline(always)]
	fn into(self) -> [c_char; ObjectName256::MaximumLengthIncludingAsciiNull]
	{
		self.to_object_name()
	}
}

impl Into<[c_char; ObjectName256::MaximumLengthIncludingAsciiNull]> for NetworkInterfaceAlias
{
	#[inline(always)]
	fn into(self) -> [c_char; ObjectName256::MaximumLengthIncludingAsciiNull]
	{
		self.0.into_object_name()
	}
}

impl Deref for NetworkInterfaceAlias
{
	type Target = ObjectName256;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}
