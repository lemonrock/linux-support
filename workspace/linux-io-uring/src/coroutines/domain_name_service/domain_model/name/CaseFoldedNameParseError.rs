// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A parse erorr.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CaseFoldedNameParseError
{
	#[allow(missing_docs)]
	RootLabelWasNotEmpty,
	
	#[allow(missing_docs)]
	NonRootLabelWasEmpty,
	
	#[allow(missing_docs)]
	TotalNameLengthExceed255Bytes,
	
	#[allow(missing_docs)]
	NumberOfLabelsExceed127,
	
	#[allow(missing_docs)]
	CaseFolderLabelParse(CaseFoldedLabelParseError),
}

impl Display for CaseFoldedNameParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for CaseFoldedNameParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::CaseFoldedNameParseError::*;
		
		match self
		{
			&CaseFolderLabelParse(ref error) => Some(error),
			
			_ => None,
		}
	}
}

impl From<CaseFoldedLabelParseError> for CaseFoldedNameParseError
{
	#[inline(always)]
	fn from(value: CaseFoldedLabelParseError) -> Self
	{
		CaseFoldedNameParseError::CaseFolderLabelParse(value)
	}
}
