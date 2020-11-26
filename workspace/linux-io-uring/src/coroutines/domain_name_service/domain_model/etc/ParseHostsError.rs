// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Error when parsing `/etc/hosts` or similar.
#[derive(Debug)]
pub enum ParseHostsError
{
	#[allow(missing_docs)]
	CouldNotRead(io::Error),
	
	/// The `/etc/hosts` file is not valid UTF-8 (and thus, by implication, US-ASCII).
	IsNotUtf8(FromUtf8Error),
	
	#[allow(missing_docs)]
	InvalidInternetProtocolAddress
	{
		/// Which line? (zero-based).
		line_index: usize,
		
		#[allow(missing_docs)]
		error: AddrParseError,
	},
	
	#[allow(missing_docs)]
	MissingCanonicalNameField
	{
		/// Which line? (zero-based).
		line_index: usize,
	},
	
	#[allow(missing_docs)]
	InvalidCanonicalName
	{
		/// Which line? (zero-based).
		line_index: usize,
		
		#[allow(missing_docs)]
		error: EfficientCaseFoldedNameParseError,
	},
	
	#[allow(missing_docs)]
	InvalidAliasName
	{
		/// Which line? (zero-based).
		line_index: usize,
		
		/// Which alias? (zero-based).
		alias_index: usize,
		
		#[allow(missing_docs)]
		error: EfficientCaseFoldedNameParseError,
	},
	
	#[allow(missing_docs)]
	CanonicalName
	{
		/// Which line? (zero-based).
		line_index: usize,
		
		#[allow(missing_docs)]
		error: DomainCacheBuilderError,
	},
	
	#[allow(missing_docs)]
	AliasName
	{
		/// Which line? (zero-based).
		line_index: usize,
		
		/// Which alias? (zero-based).
		alias_index: usize,
		
		#[allow(missing_docs)]
		error: DomainCacheBuilderError,
	},
	
	#[allow(missing_docs)]
	MoreThanOneCanonicalNameInHostAliasesFile
	{
		/// Which line? (zero-based).
		line_index: usize,
	}
}

impl Display for ParseHostsError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for ParseHostsError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::ParseHostsError::*;
		
		match self
		{
			CouldNotRead(ref error) => Some(error),
			
			IsNotUtf8(ref error) => Some(error),
			
			InvalidInternetProtocolAddress { ref error, .. } => Some(error),
			
			InvalidCanonicalName { ref error, .. } => Some(error),
			
			InvalidAliasName { ref error, .. } => Some(error),
			
			CanonicalName { ref error, .. } => Some(error),
			
			AliasName { ref error, .. } => Some(error),
			
			_ => None,
		}
	}
}

impl From<io::Error> for ParseHostsError
{
	#[inline(always)]
	fn from(value: io::Error) -> Self
	{
		ParseHostsError::CouldNotRead(value)
	}
}

impl From<FromUtf8Error> for ParseHostsError
{
	#[inline(always)]
	fn from(value: FromUtf8Error) -> Self
	{
		ParseHostsError::IsNotUtf8(value)
	}
}
