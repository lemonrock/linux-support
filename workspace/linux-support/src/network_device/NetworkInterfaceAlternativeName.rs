// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a network interface alternative name.
///
/// Size is 128 bytes (`ALTIFNAMSIZ`).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct NetworkInterfaceAlternativeName(ObjectName128);

impl Display for NetworkInterfaceAlternativeName
{
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "{}", self.0)
	}
}

impl FromBytes for NetworkInterfaceAlternativeName
{
	type Error = ObjectNameFromBytesError;
	
	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
	{
		ObjectName128::from_bytes(bytes).map(|object_name| Self(object_name))
	}
}

impl From<ObjectName128> for NetworkInterfaceAlternativeName
{
	#[inline(always)]
	fn from(value: ObjectName128) -> Self
	{
		Self(value)
	}
}

impl Into<ObjectName128> for NetworkInterfaceAlternativeName
{
	#[inline(always)]
	fn into(self) -> ObjectName128
	{
		self.0
	}
}

impl<'a> Into<[c_char; ObjectName128::MaximumLengthIncludingAsciiNull]> for &'a NetworkInterfaceAlternativeName
{
	#[inline(always)]
	fn into(self) -> [c_char; ObjectName128::MaximumLengthIncludingAsciiNull]
	{
		self.to_object_name()
	}
}

impl Into<[c_char; ObjectName128::MaximumLengthIncludingAsciiNull]> for NetworkInterfaceAlternativeName
{
	#[inline(always)]
	fn into(self) -> [c_char; ObjectName128::MaximumLengthIncludingAsciiNull]
	{
		self.0.into_object_name()
	}
}

impl Deref for NetworkInterfaceAlternativeName
{
	type Target = ObjectName128;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}
