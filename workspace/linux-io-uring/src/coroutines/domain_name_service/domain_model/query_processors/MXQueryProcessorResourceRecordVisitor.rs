// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) struct MXQueryProcessorResourceRecordVisitor<'message>
{
	query_name: &'message FullyQualifiedDomainName,
	
	records: OwnerNameToRecords<MailServerName<ParsedName<'message>>, Priority>,
}

impl<'message> ResourceRecordVisitor<'message> for MXQueryProcessorResourceRecordVisitor<'message>
{
	type Error = Infallible;
	
	type Finished = OwnerNameToRecords<MailServerName<ParsedName<'message>>>;
	
	#[inline(always)]
	fn finished(self) -> Self::Finished
	{
		self.records
	}
	
	#[inline(always)]
	fn MX(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, preference: Priority, mail_server_name: MailServerName<ParsedName<'message>>) -> Result<(), Self::Error>
	{
		self.records.add(name, cache_until, mail_server_name, preference);
		Ok(())
	}
}
