// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Whilst this is present in libc, it does not support useful derives.
#[derive(Copy, Clone, Eq)]
#[repr(C)]
pub union in6_addr
{
	/// 16 bytes.
	pub s6_addr: [u8; 16],

	/// 8 network endian 16-bit integers.
	pub s6_addr16: [u16; 8],

	/// 4 network endian 32-bit integers.
	pub s6_addr32: [u32; 4],
}

impl<'de> Deserialize<'de> for in6_addr
{
	#[inline(always)]
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
	{
		Ok(unsafe { transmute(Ipv6Addr::deserialize(deserializer)?) })
	}
}

impl Serialize for in6_addr
{
	#[inline(always)]
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
	{
		let outer: &Ipv6Addr = self.into();
		outer.serialize(serializer)
	}
}

impl Default for in6_addr
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::Default
	}
}

impl Display for in6_addr
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		let ipv6: &Ipv6Addr = unsafe { transmute(self) };
		Display::fmt(ipv6, f)
	}
}

impl Debug for in6_addr
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{:?}", unsafe { self.s6_addr })
	}
}

impl PartialEq for in6_addr
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		unsafe { self.s6_addr == other.s6_addr }
	}
}

impl PartialOrd for in6_addr
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		unsafe { self.s6_addr.partial_cmp(&other.s6_addr) }
	}
}

impl Ord for in6_addr
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		unsafe { self.s6_addr.cmp(&other.s6_addr) }
	}
}

impl Hash for in6_addr
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, hasher: &mut H)
	{
		unsafe { self.s6_addr.hash(hasher) }
	}
}

impl From<Ipv6Addr> for in6_addr
{
	#[inline(always)]
	fn from(value: Ipv6Addr) -> Self
	{
		unsafe { transmute(value) }
	}
}

impl TryFrom<IpAddr> for in6_addr
{
	type Error = ();
	
	#[inline(always)]
	fn try_from(value: IpAddr) -> Result<Self, Self::Error>
	{
		use self::IpAddr::*;
		
		match value
		{
			V4(_) => Err(()),
			V6(address) => Ok(Self::from(address)),
		}
	}
}

impl Into<Ipv6Addr> for in6_addr
{
	#[inline(always)]
	fn into(self) -> Ipv6Addr
	{
		unsafe { transmute(self) }
	}
}

impl<'a> Into<&'a Ipv6Addr> for &'a in6_addr
{
	#[inline(always)]
	fn into(self) -> &'a Ipv6Addr
	{
		unsafe { transmute(self) }
	}
}

impl Into<IpAddr> for in6_addr
{
	#[inline(always)]
	fn into(self) -> IpAddr
	{
		IpAddr::V6(self.into())
	}
}

impl InternetProtocolAddress for in6_addr
{
	const InclusiveMaximumPrefixLength: u8 = 128;
	
	const AddressFamily: u8 = AF_INET6 as u8;
	
	const Default: Self = unsafe { transmute(Ipv6Addr::LOCALHOST) };
	
	#[inline(always)]
	fn bytes(&self) -> &[u8]
	{
		let bytes: &[u8; (Self::InclusiveMaximumPrefixLength as usize / 8)] = unsafe { transmute(self) };
		&bytes[..]
	}
	
	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, TryFromSliceError>
	{
		let bytes: [u8; (Self::InclusiveMaximumPrefixLength as usize / 8)] = bytes.try_into()?;
		Ok(unsafe { transmute(bytes) })
	}
}
