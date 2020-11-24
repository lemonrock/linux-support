// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A host name can not be empty.
///
/// It is, effectively, an owned Label.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HostNameLabel(Box<[u8]>);

impl Deref for HostNameLabel
{
	type Target = [u8];
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0[..]
	}
}

impl Display for HostNameLabel
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		self.display(f)
	}
}

impl<'a > PartialEq<EfficientCaseFoldedLabel<'a>> for HostNameLabel
{
	#[inline(always)]
	fn eq(&self, rhs: &EfficientCaseFoldedLabel<'a>) -> bool
	{
		self.equals(rhs)
	}
}

impl HasTypeEquality for HostNameLabel
{
	type TypeEquality = OwnedTypeEquality;
}

impl<'a> Label<'a> for HostNameLabel
{
	#[inline(always)]
	fn bytes_pointer(&self) -> *const u8
	{
		self.0.as_ptr()
	}
	
	#[inline(always)]
	fn len(&self) -> u8
	{
		self.0.len() as u8
	}
	
	#[inline(always)]
	fn get_unchecked_safe_case_folded_byte(&self, index: u8) -> u8
	{
		self.get_unchecked_value_safe(index)
	}
	
	#[inline(always)]
	fn get_unchecked_safe(&self, index: u8) -> &u8
	{
		self.0.get_unchecked_safe(index)
	}
}

impl HostNameLabel
{
	/// Localhost.
	#[inline(always)]
	pub fn localhost() -> Self
	{
		Self(b"localhost".to_vec().into_boxed_slice())
	}
	
	/// As an label.
	#[inline(always)]
	pub fn as_label<'a>(&'a self) -> EfficientCaseFoldedLabel<'a>
	{
		EfficientCaseFoldedLabel(&self.0[..])
	}
}
