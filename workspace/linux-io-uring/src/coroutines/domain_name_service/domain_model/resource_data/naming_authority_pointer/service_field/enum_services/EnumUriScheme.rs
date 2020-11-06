// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An ENUM compatible URI scheme.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Hash)]
#[derive(EnumCount, EnumIter, IntoStaticStr)]
pub enum EnumUriScheme
{
	#[allow(missing_docs)]
	acct,
	
	#[allow(missing_docs)]
	mailto,
	
	#[allow(missing_docs)]
	tel,
	
	#[allow(missing_docs)]
	ftp,
	
	#[allow(missing_docs)]
	h323,
	
	#[allow(missing_docs)]
	iax,
	
	#[allow(missing_docs)]
	http,
	
	#[allow(missing_docs)]
	https,
	
	#[allow(missing_docs)]
	im,
	
	#[allow(missing_docs)]
	pres,
	
	#[allow(missing_docs)]
	ldap,
	
	#[allow(missing_docs)]
	sip,
	
	#[allow(missing_docs)]
	sips,
	
	#[allow(missing_docs)]
	xmpp,
}
