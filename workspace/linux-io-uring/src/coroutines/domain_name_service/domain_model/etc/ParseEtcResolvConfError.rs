// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Error when parsing `/etc/resolv.conf`.
#[derive(Debug)]
pub enum ParseEtcResolvConfError
{
	#[allow(missing_docs)]
	CouldNotRead(io::Error),
	
	/// The `/etc/resolv.conf` file is not valid UTF-8 (and thus, by implication, US-ASCII).
	IsNotUtf8(FromUtf8Error),
	
	#[allow(missing_docs)]
	EnvironmentVariableIsNotUtf8
	{
		name: &'static str,
	},
	
	#[allow(missing_docs)]
	MissingNameserverInternetProtocolAddress
	{
		/// Which line? (zero-based).
		line_index: usize,
	},
	
	#[allow(missing_docs)]
	InvalidNameserverInternetProtocolAddress
	{
		/// Which line? (zero-based).
		line_index: usize,
		
		#[allow(missing_docs)]
		error: AddrParseError,
	},
	
	#[allow(missing_docs)]
	InvalidSearchName
	{
		/// Which line? (zero-based).
		line_index: usize,
		
		/// Which search domain index (zero-based).
		search_domain_index: usize,
		
		#[allow(missing_docs)]
		error: EfficientCaseFoldedNameParseError,
	},
	
	#[allow(missing_docs)]
	InvalidDomainName
	{
		/// Which line? (zero-based).
		line_index: usize,
		
		#[allow(missing_docs)]
		error: EfficientCaseFoldedNameParseError,
	},
	
	#[allow(missing_docs)]
	MissingSortList
	{
		/// Which line? (zero-based).
		line_index: usize,
	},
	
	#[allow(missing_docs)]
	MissingSearchList
	{
		/// Which line? (zero-based).
		line_index: usize,
	},
	
	#[allow(missing_docs)]
	MissingDomainName
	{
		/// Which line? (zero-based).
		line_index: usize,
	},
	
	#[allow(missing_docs)]
	InvalidSortListInternetProtocolAddress
	{
		/// Which line? (zero-based).
		line_index: usize,
		
		/// Which field? (zero-based).
		sort_list_pair_index: usize,
		
		#[allow(missing_docs)]
		error: AddrParseError,
	},
	
	#[allow(missing_docs)]
	InvalidSortListInternetProtocolMask
	{
		/// Which line? (zero-based).
		line_index: usize,
		
		/// Which field? (zero-based).
		sort_list_pair_index: usize,
		
		#[allow(missing_docs)]
		error: AddrParseError,
	},
	
	#[allow(missing_docs)]
	InvalidSortListInternetProtocolMaskSetBits
	{
		/// Which line? (zero-based).
		line_index: usize,
		
		/// Which field? (zero-based).
		sort_list_pair_index: usize,
	},
	
	#[allow(missing_docs)]
	OptionShouldNotHaveAValue
	{
		/// Which line? (zero-based).
		line_index: usize,
		
		/// Option name.
		option_name: Box<[u8]>,
	},
	
	#[allow(missing_docs)]
	OptionShouldHaveAValue
	{
		/// Which line? (zero-based).
		line_index: usize,
		
		/// Option name.
		option_name: Box<[u8]>,
	},
	
	#[allow(missing_docs)]
	OptionValueNotADecimalNumber
	{
		/// Which line? (zero-based).
		line_index: usize,
		
		/// Option name.
		option_name: Box<[u8]>,
		
		#[allow(missing_docs)]
		error: ParseNumberError,
	}
}

impl Display for ParseEtcResolvConfError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for ParseEtcResolvConfError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::ParseEtcResolvConfError::*;
		
		match self
		{
			CouldNotRead(ref error) => Some(error),
			
			IsNotUtf8(ref error) => Some(error),
			
			InvalidNameserverInternetProtocolAddress { ref error, .. } => Some(error),
			
			InvalidSearchName { ref error, .. } => Some(error),
			
			InvalidDomainName { ref error, .. } => Some(error),
			
			InvalidSortListInternetProtocolAddress { ref error, .. } => Some(error),
			
			InvalidSortListInternetProtocolMask { ref error, .. } => Some(error),
			
			OptionValueNotADecimalNumber { ref error, .. } => Some(error),
			
			_ => None,
		}
	}
}

impl From<io::Error> for ParseEtcResolvConfError
{
	#[inline(always)]
	fn from(value: io::Error) -> Self
	{
		ParseEtcResolvConfError::CouldNotRead(value)
	}
}

impl From<FromUtf8Error> for ParseEtcResolvConfError
{
	#[inline(always)]
	fn from(value: FromUtf8Error) -> Self
	{
		ParseEtcResolvConfError::IsNotUtf8(value)
	}
}
