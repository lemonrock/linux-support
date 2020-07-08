// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A named field.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NamedField
{
	/// Type and offset..
	pub unnamed: UnnamedField,
	
	/// Name.
	pub ident: &'static str,
}

impl Field for NamedField
{
	#[inline(always)]
	fn to_btf_member(&self, type_identifiers: &mut BtfTypeIdentifiers, _index: u16) -> Result<btf_member, BtfTypeError>
	{
		Ok
		(
			btf_member
			{
				name_off: unsafe { transmute(type_identifiers.push_c_identifier(self.ident, BtfKind::Function)?) },
				type_identifier: type_identifiers.get_or_create_type_identifier(self.unnamed.type_)?,
				offset: self.unnamed.offset_in_bits()?,
			}
		)
	}
}

impl NamedField
{
	/// New instance.
	#[inline(always)]
	pub const fn new<T: 'static + HasReflectionInformation>(offset_in_bytes: u32, ident: &'static str) -> Self
	{
		Self
		{
			unnamed: UnnamedField::new::<T>(offset_in_bytes),
			ident,
		}
	}
}
