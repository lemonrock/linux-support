// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Errors when building a `DomainCache`.
#[derive(Debug)]
pub enum DomainCacheBuilderError
{
	#[allow(missing_docs)]
	CanonicalNameWasPreviouslyBlockedButNowIsDefined
	{
		#[allow(missing_docs)]
		canonical_name: FullyQualifiedDomainName,
	},
	
	#[allow(missing_docs)]
	CanonicalNameWasPreviouslyDefinedButNowIsBlocked
	{
		#[allow(missing_docs)]
		canonical_name: FullyQualifiedDomainName,
	},
	
	#[allow(missing_docs)]
	InternetProtocolAddressPointerNameWasPreviouslyDefinedAsSomethingElse
	{
		#[allow(missing_docs)]
		internet_protocol_address_pointer_name: FullyQualifiedDomainName,
	},
	
	#[allow(missing_docs)]
	CanonicalNameWasPreviouslyDefinedAsAnAlias
	{
		#[allow(missing_docs)]
		canonical_name: FullyQualifiedDomainName,
		
		#[allow(missing_docs)]
		previously_defined_alias: FullyQualifiedDomainName,
	},
	
	#[allow(missing_docs)]
	AliasNameWasPreviouslyDefinedAsACanonicalName
	{
		#[allow(missing_docs)]
		alias_name: FullyQualifiedDomainName,
	},
	
	#[allow(missing_docs)]
	AliasNameWasPreviouslyDefinedWithADifferentTargetName
	{
		#[allow(missing_docs)]
		alias_name: FullyQualifiedDomainName,
		
		#[allow(missing_docs)]
		previously_defined_alias: FullyQualifiedDomainName,
	},
}

impl Display for DomainCacheBuilderError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for DomainCacheBuilderError
{
}
