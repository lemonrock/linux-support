// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Why was a `NAPTR` service field ignored?
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IgnoredServiceFieldReason
{
	/// No known matching pattern.
	NoMatchingPattern,

	#[allow(missing_docs)]
	NonTerminalServiceFieldMustNotHaveARegularExpression(ServiceFieldKind),

	#[allow(missing_docs)]
	TerminalServiceFieldMustNotHaveARegularExpression(ServiceFieldKind, NamingAuthorityMutuallyExclusiveFlag),

	#[allow(missing_docs)]
	DFlagFieldMustNotHaveARegularExpression(ServiceFieldKind),
	
	#[allow(missing_docs)]
	DoesNotSupportMutuallyExclusiveFlag(ServiceFieldKind, NamingAuthorityMutuallyExclusiveFlag),
	
	/// Requires a U-NAPTR regular expression.
	RequiresAnUNaptrRegularExpression(ServiceFieldKind),
	
	#[allow(missing_docs)]
	ExpectedAnUNaptrRegularExpression(ServiceFieldKind),
	
	#[allow(missing_docs)]
	UNaptrRegularExpressionDoesNotStartWithCorrectPrefix(ServiceFieldKind),
	
	#[allow(missing_docs)]
	UNaptrRegularExpressionDoesNotEndWithCorrectSuffix(ServiceFieldKind),
	
	#[allow(missing_docs)]
	UNaptrRegularExpressionUriIsNotHttpOrHttps(ServiceFieldKind, Scheme<'static>),
	
	#[allow(missing_docs)]
	UNaptrRegularExpressionUriIsNotHttp(ServiceFieldKind, Scheme<'static>),
	
	#[allow(missing_docs)]
	UNaptrRegularExpressionUriIsNotHttps(ServiceFieldKind, Scheme<'static>),
	
	#[allow(missing_docs)]
	NoSolicitRegularExpressionUriIsNotHttpOrHttpsOrFtp(Scheme<'static>),
	
	/// Invalid target URI.
	InvalidTargetUri(URIError, ServiceFieldKind),
	
	#[allow(missing_docs)]
	ExpectedANoSolicitRegularExpression,
	
	#[allow(missing_docs)]
	ExpectedANoSolicitRegularExpressionToHaveTheSameDelimiterCharacter
	{
		first_delimiter_character: u8,
		
		second_delimiter_character: u8,
		
		last_delimiter_character: u8,
	},

	#[allow(missing_docs)]
	NoSolicitRegularExpressionHasAnInvalidDelimiterCharacter(u8),
}

impl Display for IgnoredServiceFieldReason
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for IgnoredServiceFieldReason
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::IgnoredServiceFieldReason::*;
		
		match self
		{
			&InvalidTargetUri(ref error, ..) => Some(error),
			
			_ => None,
		}
	}
}
