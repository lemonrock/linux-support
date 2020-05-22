// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Is not permitted to be empty.
#[repr(transparent)]
pub struct StructuredDataName(ArrayVec<[PrintableAsciiCharacter; 32]>);

impl Deref for StructuredDataName
{
	type Target = [PrintableAsciiCharacter];
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0[..]
	}
}

impl StructuredDataName
{
	/// ***SLOW***.
	#[inline(always)]
	fn private(name: &[u8], private_enterprise_number: &PrivateEnterpriseNumber) -> Result<Self, PrintableAsciiCharacterPushError>
	{
		let mut inner = ArrayVec::new();
		
		Self::push_raw_slice(name, &mut inner)?;
		
		PrintableAsciiCharacter(b'@').push_into_array_vec(&mut inner)?;
		
		let a = private_enterprise_number.private_enterprise_number.unpadded_decimal();
		Self::push_raw_slice(&a[..], &mut inner)?;
		
		Ok(Self(inner))
	}
	
	/// ***SLOW***.
	#[inline(always)]
	fn iana_registered(name: &[u8]) -> Self
	{
		Self::name(name).expect("IANA names should not be invalid")
	}
	
	/// ***SLOW***.
	#[inline(always)]
	fn name(name: &[u8]) -> Result<Self, PrintableAsciiCharacterPushError>
	{
		let mut inner = ArrayVec::new();
		Self::push_raw_slice(name, &mut inner)?;
		Ok(Self(inner))
	}
	
	/// ***SLOW***.
	#[inline(always)]
	fn push_raw_slice(raw_slice: &[u8], inner: &mut ArrayVec<[PrintableAsciiCharacter; 32]>) -> Result<(), PrintableAsciiCharacterPushError>
	{
		lazy_static!
		{
    		static ref Denied: HashSet<u8> = hashset!
			{
				b'=',
				b']',
				b'"',
			};
    	}
		
		PrintableAsciiCharacter::push_raw_slice_into_array_vec_with_additional_restrictions(raw_slice, inner, &Denied)?;
		Ok(())
	}
}
