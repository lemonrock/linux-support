// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Section error.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum SectionError<E: 'static + error::Error>
{
	/// Query section.
	QuerySection(QuerySectionError),
	
	/// Answer section.
	AnswerSection(AnswerSectionError<WrappingCanonicalChainError<E>>),
	
	/// Authority section.
	AuthoritySection(AuthoritySectionError<AuthorityError>),
	
	/// Additional section.
	AdditionalSection(AdditionalSectionError<Infallible>),
}

impl<E: error::Error> Display for SectionError<E>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl<E: 'static + error::Error> error::Error for SectionError<E>
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::SectionError::*;
		
		match self
		{
			&QuerySection(ref error) => Some(error),
			
			&AnswerSection(ref error) => Some(error),
			
			&AuthoritySection(ref error) => Some(error),
			
			&AdditionalSection(ref error) => Some(error),
			
			_ => None,
		}
	}
}

impl<E: error::Error> From<QuerySectionError> for SectionError<E>
{
	#[inline(always)]
	fn from(value: QuerySectionError) -> Self
	{
		SectionError::QuerySection(value)
	}
}

impl<E: 'static + error::Error> From<AnswerSectionError<WrappingCanonicalChainError<E>>> for SectionError<E>
{
	#[inline(always)]
	fn from(value: AnswerSectionError<WrappingCanonicalChainError<E>>) -> Self
	{
		SectionError::AnswerSection(value)
	}
}

impl<E: error::Error> From<AuthoritySectionError<AuthorityError>> for SectionError<E>
{
	#[inline(always)]
	fn from(value: AuthoritySectionError<AuthorityError>) -> Self
	{
		SectionError::AuthoritySection(value)
	}
}

impl<E: error::Error> From<AdditionalSectionError<Infallible>> for SectionError<E>
{
	#[inline(always)]
	fn from(value: AdditionalSectionError<Infallible>) -> Self
	{
		SectionError::AdditionalSection(value)
	}
}
