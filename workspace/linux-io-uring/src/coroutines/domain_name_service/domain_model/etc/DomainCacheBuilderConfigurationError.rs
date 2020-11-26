// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Errors when building a `DomainCache`.
#[derive(Debug)]
pub enum DomainCacheBuilderConfigurationError
{
	#[allow(missing_docs)]
	ParseHostsFileInFolderCouldNotReadDirectory
	{
		#[allow(missing_docs)]
		error: io::Error,
		
		#[allow(missing_docs)]
		folder_path: PathBuf,
	},

	#[allow(missing_docs)]
	ParseHostsFileInFolderCouldNotReadEntry
	{
		#[allow(missing_docs)]
		error: io::Error,
		
		#[allow(missing_docs)]
		folder_path: PathBuf,
	},

	#[allow(missing_docs)]
	ParseHostsFileInFolderCouldNotReadEntryFileType
	{
		#[allow(missing_docs)]
		error: io::Error,
		
		#[allow(missing_docs)]
		file_path: PathBuf,
	},

	#[allow(missing_docs)]
	ParseHosts
	{
		#[allow(missing_docs)]
		error: ParseHostsError,
		
		#[allow(missing_docs)]
		file_path: PathBuf,
	},

	#[allow(missing_docs)]
	CouldNotParseHostAliasesFromEnvironment
	{
		#[allow(missing_docs)]
		error: ParseHostsError,
	},

	#[allow(missing_docs)]
	CouldNotParseHostAliasesFromFile
	{
		#[allow(missing_docs)]
		error: ParseHostsError,
		
		#[allow(missing_docs)]
		file_path: PathBuf,
	},
	
	#[allow(missing_docs)]
	CouldNotParseResolvConf
	{
		#[allow(missing_docs)]
		error: ParseEtcResolvConfError,
	}
}

impl Display for DomainCacheBuilderConfigurationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for DomainCacheBuilderConfigurationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::DomainCacheBuilderConfigurationError::*;
		
		match self
		{
			ParseHostsFileInFolderCouldNotReadDirectory { ref error, .. } => Some(error),
			
			ParseHostsFileInFolderCouldNotReadEntry { ref error, .. } => Some(error),
			
			ParseHostsFileInFolderCouldNotReadEntryFileType { ref error, .. } => Some(error),
			
			ParseHosts { ref error, .. } => Some(error),
			
			CouldNotParseHostAliasesFromEnvironment { ref error, .. } => Some(error),
			
			CouldNotParseHostAliasesFromFile { ref error, .. } => Some(error),
			
			CouldNotParseResolvConf { ref error } => Some(error),
			
			_ => None,
		}
	}
}

impl From<ParseEtcResolvConfError> for DomainCacheBuilderConfigurationError
{
	#[inline(always)]
	fn from(error: ParseEtcResolvConfError) -> Self
	{
		DomainCacheBuilderConfigurationError::CouldNotParseResolvConf { error }
	}
}
