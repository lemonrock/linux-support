// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default)]
pub(crate) struct MXQueryProcessor<'message>
{
	records: Records<'message, CaseFoldedName<'static>>,
}

impl<'message> ResourceRecordVisitor<'message> for MXQueryProcessor
{
	type Error = Infallible;
	
	#[inline(always)]
	fn MX(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, record: MailExchange<'message>) -> Result<(), Self::Error>
	{
		self.records.store_unweighted::<'message>(name, cache_until, record.preference, CaseFoldedName::from(record.mail_server_name));
		Ok(())
	}
}

impl<'message> QueryProcessor<'message> for MXQueryProcessor
{
	const DT: DataType = DataType::MX;
	
	type Record = CaseFoldedName<'static>;
	
	#[inline(always)]
	fn new() -> Self
	{
		Self::default()
	}
	
	#[inline(always)]
	fn finish(self, cache: &mut QueryTypeCache<Self::Record>)
	{
		cache.put_present::<'message>(self.records)
	}
}
