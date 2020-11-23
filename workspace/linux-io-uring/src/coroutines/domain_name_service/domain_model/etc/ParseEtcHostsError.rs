// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Error when parsing `/etc/hosts`.
#[derive(Debug)]
pub enum ParseEtcHostsError
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
	InvalidCanonicalOrAliasName
	{
		/// Which line? (zero-based).
		line_index: usize,
		
		#[allow(missing_docs)]
		error: EfficientCaseFoldedNameParseError,
	},
	
	#[allow(missing_docs)]
	CanonicalNameWasPreviouslyDefinedAsAnAlias
	{
		/// Which line? (zero-based).
		line_index: usize,
		
		#[allow(missing_docs)]
		canonical_name: EfficientCaseFoldedName,
		
		#[allow(missing_docs)]
		previously_defined_alias: EfficientCaseFoldedName,
	},
	
	#[allow(missing_docs)]
	AliasNameWasPreviouslyDefinedAsACanonicalName
	{
		/// Which line? (zero-based).
		line_index: usize,
		
		#[allow(missing_docs)]
		alias_name: EfficientCaseFoldedName,
	},
	
	#[allow(missing_docs)]
	AliasNameWasPreviouslyDefinedWithADifferentTargetName
	{
		/// Which line? (zero-based).
		line_index: usize,
		
		#[allow(missing_docs)]
		alias_name: EfficientCaseFoldedName,
		
		#[allow(missing_docs)]
		previously_defined_alias: EfficientCaseFoldedName,
	},
}

impl Display for ParseEtcHostsError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for ParseEtcHostsError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::ParseEtcHostsError::*;
		
		match self
		{
			CouldNotRead(ref error) => Some(error),
			
			IsNotUtf8(ref error) => Some(error),
			
			InvalidInternetProtocolAddress { ref error, .. } => Some(error),
			
			InvalidCanonicalOrAliasName { ref error, .. } => Some(error),
			
			_ => None,
		}
	}
}

impl From<io::Error> for ParseEtcHostsError
{
	#[inline(always)]
	fn from(value: io::Error) -> Self
	{
		ParseEtcHostsError::CouldNotRead(value)
	}
}

impl From<FromUtf8Error> for ParseEtcHostsError
{
	#[inline(always)]
	fn from(value: FromUtf8Error) -> Self
	{
		ParseEtcHostsError::IsNotUtf8(value)
	}
}
