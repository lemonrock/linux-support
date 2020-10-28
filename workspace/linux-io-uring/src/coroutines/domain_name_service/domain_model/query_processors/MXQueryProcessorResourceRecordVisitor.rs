// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) struct MXQueryProcessorResourceRecordVisitor<'cache>
{
	records: Records<'cache, CaseFoldedName<'cache>>,
}

impl<'cache: 'message, 'message> ResourceRecordVisitor<'message> for MXQueryProcessorResourceRecordVisitor<'cache>
{
	type Error = Infallible;
	
	type Finished = Records<'cache, CaseFoldedName<'cache>>;
	
	#[inline(always)]
	fn finished(self) -> Self::Finished
	{
		self.records
	}
	
	#[inline(always)]
	fn MX(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, record: MailExchange<'message>) -> Result<(), Self::Error>
	{
		self.records.store_unweighted(&name, cache_until, record.preference, CaseFoldedName::map::<'message>(record.mail_server_name));
		Ok(())
	}
}
