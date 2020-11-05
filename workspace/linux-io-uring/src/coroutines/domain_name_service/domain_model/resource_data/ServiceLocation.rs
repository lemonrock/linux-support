// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A `SRV` record.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ServiceLocation<'label, N: Name<'label>>
{
	/// TCP, UDP or SCTP port for the service.
	pub port: u16,

	/// Must not be an alias; should not use name compression; a value of '.' (ie Root) means the service is unavailable.
	pub target: N,
	
	pub(crate) marker: PhantomData<&'label ()>,
}

impl<'message> Into<ServiceLocation<'static, EfficientCaseFoldedName>> for ServiceLocation<'message, ParsedName<'message>>
{
	#[inline(always)]
	fn into(self) -> ServiceLocation<'static, EfficientCaseFoldedName>
	{
		ServiceLocation
		{
			port: self.port,
			target: EfficientCaseFoldedName::from(self.target),
			marker: PhantomData,
		}
	}
}
