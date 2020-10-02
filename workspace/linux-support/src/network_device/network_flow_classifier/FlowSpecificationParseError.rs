// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Flow specification parse error.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum FlowSpecificationParseError
{
	#[allow(missing_docs)]
	SpecialRuleLocationInvalid(ParseNumberError),
	
	#[allow(missing_docs)]
	BasicFlowParse(BasicFlowParseError),
	
	#[allow(missing_docs)]
	RingCookieQueueIdentifierOutOfRange(ParseNumberError),
}

impl Display for FlowSpecificationParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for FlowSpecificationParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn Error + 'static)>
	{
		use self::FlowSpecificationParseError::*;
		
		match self
		{
			&SpecialRuleLocationInvalid(ref error) => Some(error),
			
			&BasicFlowParse(ref error) => Some(error),
			
			&RingCookieQueueIdentifierOutOfRange(ref error) => Some(error),
		}
	}
}

impl From<BasicFlowParseError> for FlowSpecificationParseError
{
	#[inline(always)]
	fn from(value: BasicFlowParseError) -> Self
	{
		FlowSpecificationParseError::BasicFlowParse(value)
	}
}
