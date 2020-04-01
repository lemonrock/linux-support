// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Whilst this is present in libc, it is not always consistently defined.
#[derive(Clone)]
#[repr(C)]
pub(crate) struct sockaddr_storage
{
	/// Socket address family.
	ss_family: sa_family_t,

	/// Alignment.
	__ss_align: size_t,

	/// Padding to 128 bytes.
	__ss_pad2: [u8; 128 - size_of::<sa_family_t>() - size_of::<size_t>()],
}

impl Debug for sockaddr_storage
{
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "sockaddr_storage {{ ss_family: {} }}", self.ss_family)
	}
}

impl PartialEq for sockaddr_storage
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		self.ss_family == other.ss_family && self.__ss_align == other.__ss_align && (&self.__ss_pad2[..]) == (&other.__ss_pad2[..])
	}
}

impl Eq for sockaddr_storage
{
}

impl PartialOrd for sockaddr_storage
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		Some(self.cmp(other))
	}
}

impl Ord for sockaddr_storage
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		self.ss_family.cmp(&other.ss_family).then_with(|| self.__ss_align.cmp(&other.__ss_align)).then_with(|| (&self.__ss_pad2[..]).cmp(&self.__ss_pad2[..]))
	}
}

impl Hash for sockaddr_storage
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, hasher: &mut H)
	{
		self.ss_family.hash(hasher);
		self.__ss_align.hash(hasher);
		(&self.__ss_pad2[..]).hash(hasher);
	}
}
