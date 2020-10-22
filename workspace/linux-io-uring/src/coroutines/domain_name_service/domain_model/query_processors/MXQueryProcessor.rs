// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default)]
struct MXQueryProcessor<'message>
{
	records: HashMap<ParsedName<'message>, Present<ParsedName<'message>>>,
}

impl<'message> ResourceRecordVisitor<'message> for MXQueryProcessor
{
	type Error = Infallible;
	
	#[inline(always)]
	fn MX(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, record: MailExchange<'message>) -> Result<(), Self::Error>
	{
		
		Present::store_unprioritized_and_unweighted::<'message>(&mut self.records,name, cache_until, record);
		



// SRV, URI records also have 'service name' (eg _ftp) and transport protocol (eg _tcp)
// See "Service Name and
//    Transport Protocol Port Number Registry" [RFC6335] or "Enumservice
//
//
//
//
// Faltstrom & Kolkman           Informational                     [Page 4]
//
//
// RFC 7553                   URI Resource Record                 June 2015
//
//
//    Registrations [RFC6117].
// https://tools.ietf.org/html/rfc7553#section-4.1.


// TODO: SRV can use a target name of '.' to indicate that the service is not present.


// SRV: https://tools.ietf.org/html/rfc2782
		
		/*
		n the absence of a protocol whose specification calls for the
				use of other weighting information, a client arranges the SRV
				RRs of the same Priority in the order in which target hosts,
				specified by the SRV RRs, will be contacted. The following
				algorithm SHOULD be used to order the SRV RRs of the same
				priority:
		
		To select a target to be contacted next, arrange all SRV RRs
				(that have not been ordered yet) in any order, except that all
				those with weight 0 are placed at the beginning of the list.
		
				Compute the sum of the weights of those RRs, and with each RR
				associate the running sum in the selected order.
				
				Then choose a
				uniform random number between 0 and the sum computed
				(inclusive), and select the RR whose running sum value is the
				first in the selected order which is greater than or equal to
				the random number selected.
				
				The target host specified in the
				selected SRV RR is the next one to be contacted by the client.
				
				Remove this SRV RR from the set of the unordered SRV RRs and
				apply the described algorithm to the unordered SRV RRs to select
				the next target host.
				
				Continue the ordering process until there
				are no unordered SRV RRs.
				
				This process is repeated for each
				Priority.
		
		 */

		
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
	
	type Record = PrioritizedAndWeightedRecords<CaseFoldedName>;
	
	#[inline(always)]
	fn finish(self, cache: &mut QueryTypeCache<Self::Record>)
	{
	}
}
