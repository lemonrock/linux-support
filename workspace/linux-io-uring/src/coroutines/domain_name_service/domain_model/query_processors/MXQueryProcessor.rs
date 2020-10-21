// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default)]
struct MXQueryProcessor<'message>
{
	by_priority: BTreeMap<u16, Vec<WeightedRecord<CaseFoldedName>>>
}

impl<'message, A: Alloctor> ResourceRecordVisitor<'message> for MXQueryProcessor<A>
{
	type Error = Infallible;
	
	#[inline(always)]
	fn MX(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, record: MailExchange<'message>) -> Result<(), Self::Error>
	{
		let priority = record.preference;
		let weighted_records = self.by_priority.entry(priority).or_insert_with(|| Vec::with_capacity(4));
		weighted_records.push
		(
			WeightedRecord
			{
				weight: u16::MAX,
				
				record: Record
				{
					// TODO: ACTUALLY USE THE QUERY PROCESSOR! with query class. Also do for SRV and URI
					
					// TODO: this is a key - it belongs outside of this structure.
					xxx;
					name: CaseFoldedName::from(record.name),
					time_to_live,
					data: CaseFoldedName::from(record.mail_server_name)
				}
			}
		);
		
		Ok(())
	}
}

impl<'message> QueryProcessor<'message, Ipv4Addr> for MXQueryProcessor
{
	const DT: DataType = DataType::MX;
	
	type R = PrioritizedAndWeightedRecords<CaseFoldedName>;
	
	#[inline(always)]
	fn finish(self) -> Self::R
	{
		PrioritizedAndWeightedRecords
		{
		}
	}
}
