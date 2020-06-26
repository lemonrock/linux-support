// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Whilst this is present in libc, it does not support useful derives.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct in_addr
{
	/// Must a 32-bit integer in Network Endian form, not Native Endian form.
	pub s_addr: in_addr_t,
}

impl<'de> Deserialize<'de> for in_addr
{
	#[inline(always)]
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
	{
		Ok(unsafe { transmute(Ipv4Addr::deserialize(deserializer)?) })
	}
}

impl Serialize for in_addr
{
	#[inline(always)]
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
	{
		let outer: &Ipv4Addr = self.into();
		outer.serialize(serializer)
	}
}

impl Default for in_addr
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::LocalHost
	}
}

impl Display for in_addr
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		let ipv4: &Ipv4Addr = unsafe { transmute(self) };
		Display::fmt(ipv4, f)
	}
}

impl From<Ipv4Addr> for in_addr
{
	#[inline(always)]
	fn from(value: Ipv4Addr) -> Self
	{
		unsafe { transmute(value) }
	}
}

impl TryFrom<IpAddr> for in_addr
{
	type Error = ();
	
	#[inline(always)]
	fn try_from(value: IpAddr) -> Result<Self, Self::Error>
	{
		use self::IpAddr::*;
		
		match value
		{
			V4(address) => Ok(Self::from(address)),
			V6(_) => Err(()),
		}
	}
}

impl Into<Ipv4Addr> for in_addr
{
	#[inline(always)]
	fn into(self) -> Ipv4Addr
	{
		unsafe { transmute(self) }
	}
}

impl<'a> Into<&'a Ipv4Addr> for &'a in_addr
{
	#[inline(always)]
	fn into(self) -> &'a Ipv4Addr
	{
		unsafe { transmute(self) }
	}
}

impl Into<IpAddr> for in_addr
{
	#[inline(always)]
	fn into(self) -> IpAddr
	{
		IpAddr::V4(self.into())
	}
}

impl InternetProtocolAddress for in_addr
{
	const InclusiveMaximumPrefixLength: u8 = 32;
	
	const AddressFamily: u8 = AF_INET as u8;
	
	const LocalHost: Self = unsafe { transmute(Ipv4Addr::LOCALHOST) };
	
	#[inline(always)]
	fn bytes(&self) -> &[u8]
	{
		let bytes: &[u8; Self::InclusiveMaximumPrefixLength as usize / 8] = unsafe { transmute(self) };
		&bytes[..]
	}
	
	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, TryFromSliceError>
	{
		let bytes: [u8; Self::InclusiveMaximumPrefixLength as usize / 8] = bytes.try_into()?;
		Ok(unsafe { transmute(bytes) })
	}
}
