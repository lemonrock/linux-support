// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Extended attribute name.
///
/// Limited to 255 bytes (?exluding terminal NUL?) by Linux.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ExtendedAttributeName<'a>(&'a CStr);

impl<'a> Deref for ExtendedAttributeName<'a>
{
	type Target = CStr;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.0
	}
}

impl<'a> ExtendedAttributeName<'a>
{
	/// Namespace and relative name.
	#[inline(always)]
	pub fn namespace_and_relative_name(&self) -> (Option<ExtendedAttributeNamespace<'a>>, &[u8])
	{
		let bytes = self.0.to_bytes();
		memchr(b'.', bytes).map(|index| (Some(ExtendedAttributeNamespace(&bytes[0 .. index])), &bytes[index + 1 .. ])).unwrap_or((None, bytes))
	}
}
