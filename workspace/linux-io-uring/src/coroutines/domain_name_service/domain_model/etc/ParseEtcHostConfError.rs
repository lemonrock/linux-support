// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Error when parsing `/etc/resolv.conf`.
#[derive(Debug)]
pub enum ParseEtcHostConfError
{
	#[allow(missing_docs)]
	CouldNotRead(io::Error),
	
	/// The `/etc/resolv.conf` file is not valid UTF-8 (and thus, by implication, US-ASCII).
	IsNotUtf8(FromUtf8Error),
	
	#[allow(missing_docs)]
	InvalidTrimName
	{
		/// Which line? (zero-based).
		line_index: usize,
		
		#[allow(missing_docs)]
		error: EfficientCaseFoldedNameParseError,
	},
	
	#[allow(missing_docs)]
	MissingValue
	{
		/// Keyword.
		keyword: &'static str,
		
		/// Which line? (zero-based).
		line_index: usize,
	},
	
	#[allow(missing_docs)]
	DuplicateKeyword
	{
		/// Keyword.
		keyword: &'static str,
		
		/// Which line? (zero-based).
		line_index: usize,
	},
	
	#[allow(missing_docs)]
	ValueNotBoolean
	{
		keyword: &'static str,
		
		line_index: usize,
		
		value: String,
	},
	
	#[allow(missing_docs)]
	InvalidEnvironmentVariable
	{
		#[allow(missing_docs)]
		name: &'static str,
		
		#[allow(missing_docs)]
		value: OsString,
	},
	
	#[allow(missing_docs)]
	EnvironmentVariableInvalidTrimName
	{
		#[allow(missing_docs)]
		name: &'static str,
		
		#[allow(missing_docs)]
		error: EfficientCaseFoldedNameParseError,
	},
}

impl Display for ParseEtcHostConfError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for ParseEtcHostConfError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::ParseEtcHostConfError::*;
		
		match self
		{
			CouldNotRead(ref error) => Some(error),
			
			IsNotUtf8(ref error) => Some(error),
			
			InvalidTrimName { ref error, .. } => Some(error),
			
			EnvironmentVariableInvalidTrimName { ref error, .. } => Some(error),
			
			_ => None,
		}
	}
}

impl From<io::Error> for ParseEtcHostConfError
{
	#[inline(always)]
	fn from(value: io::Error) -> Self
	{
		ParseEtcHostConfError::CouldNotRead(value)
	}
}

impl From<FromUtf8Error> for ParseEtcHostConfError
{
	#[inline(always)]
	fn from(value: FromUtf8Error) -> Self
	{
		ParseEtcHostConfError::IsNotUtf8(value)
	}
}
