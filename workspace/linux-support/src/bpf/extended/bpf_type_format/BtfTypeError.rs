// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// BTF type error.
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BtfTypeError
{
	IntegerSizeCanNotExceed31Bytes,
	
	IdentifierIsEmpty,
	
	IdentifierContainsAsciiNul(NulError),

	IdentifierIsMoreThan127BytesLong,
	
	StringTableOffsetIsTooLarge,
	
	IdentifierIsTooLarge,
	
	FunctionHasTooManyParameters,
	
	StructHasTooManyNamedFields,
	
	StructHasTooManyUnnamedFields,
	
	UnionHasTooManyFields,
	
	EnumHasTooManyVariants,
	
	EnumVariantWithNamedFieldsIsUnsupported,
	
	EnumVariantWithUnnamedFieldsIsUnsupported,
	
	EnumUnitVariantIsNotRepresentableInAnI32,

	IntegerSizeExceeds128,

	IntegerBitsGreaterThanIntegerSize,

	TooManyBtfTypes,
	
	MoreThanU32MaxArrayElements,
	
	FieldOffsetTooLarge,
	
	SizeMustBeZeroIfThereAreNoFields,
}

impl Display for BtfTypeError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for BtfTypeError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(dyn error::Error + 'static)>
	{
		use self::BtfTypeError::*;
		
		match self
		{
			&IdentifierContainsAsciiNul(ref error) => Some(error),
			
			_ => None,
		}
	}
}
