// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
pub enum CacheStoreError
{
	/// Can not be an alias because it is never a valid domain name or the domain name must always exist.
	DomainNameCanNotBeAnAlias(Alias),
	
	/// Domain name must exist because it is never a valid domain name or the domain name must always exist.
	DomainNameCanNotNotExist(DomainTarget),
	
	/// Domain name is never valid.
	DomainNameCanNotNotHaveRecords(AliasOrDomainTarget),
	
	/// Domain name is fixed.
	DomainNameIsFixed(AliasOrDomainTarget),
}

impl Display for CacheStoreError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for CacheStoreError
{
}
