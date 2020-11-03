// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A parse erorr.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EfficientCaseFoldedNameParseError
{
	#[allow(missing_docs)]
	NonRootLabelWasEmpty,
	
	#[allow(missing_docs)]
	TotalNameLengthExceed255Bytes,
	
	#[allow(missing_docs)]
	NumberOfLabelsExceed127,
	
	#[allow(missing_docs)]
	CaseFoldedLabelParse(EfficientCaseFoldedLabelParseError),
}

impl Display for EfficientCaseFoldedNameParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for EfficientCaseFoldedNameParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::EfficientCaseFoldedNameParseError::*;
		
		match self
		{
			&CaseFoldedLabelParse(ref error) => Some(error),
			
			_ => None,
		}
	}
}

impl From<EfficientCaseFoldedLabelParseError> for EfficientCaseFoldedNameParseError
{
	#[inline(always)]
	fn from(value: EfficientCaseFoldedLabelParseError) -> Self
	{
		EfficientCaseFoldedNameParseError::CaseFoldedLabelParse(value)
	}
}
