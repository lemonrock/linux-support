// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) struct MXQueryProcessorResourceRecordVisitor<'message>
{
	query_name: &'message EfficientCaseFoldedName,
	records: Records<DomainTarget>,
}

impl<'message> ResourceRecordVisitor<'message> for MXQueryProcessorResourceRecordVisitor<'message>
{
	type Error = Infallible;
	
	type Finished = Records<DomainTarget>;
	
	#[inline(always)]
	fn finished(self) -> Self::Finished
	{
		self.records
	}
	
	#[inline(always)]
	fn MX(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, preference: Priority, mail_server_name: ParsedName<'message>) -> Result<(), Self::Error>
	{
		self.records.store_unweighted(&name, cache_until, preference, EfficientCaseFoldedName::from(mail_server_name));
		Ok(())
	}
}
