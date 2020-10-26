// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub struct Cache<'cache>
{
	recent_message_identifiers: RecentMessageIdentifiers,
	
	a_query_type_cache: QueryTypeCache<Ipv4Addr>,
	
	aaaa_query_type_cache: QueryTypeCache<Ipv4Addr>,
	
	mx_query_type_cache: QueryTypeCache<CaseFoldedName<'cache>>,
	
	cname_query_type_cache: QueryTypeCache<CaseFoldedName<'cache>>,
}

impl<'cache> Cache<'cache>
{
	pub fn mx_enquire_over_tcp_and_cache<'yielder, 'cache, SD: SocketData>(&mut self, stream: &mut TlsClientStream<'yielder, SD>, query_name: CaseFoldedName<'cache>) -> Result<(), ProtocolError<QP::Error>>
	{
		let message_identifier = self.recent_message_identifiers.next();
		
		use self::AnswerOutcome::*;
		
		let (answer_existence, answer_outcome) = Query::enquire_over_tcp(stream, message_identifier, query_name, MXQueryProcessor::new(), &mut self.cname_query_type_cache, |query_processor, answer_existence, answer_outcome|
		{
			match answer_outcome
			{
				Answered =>
				{
					query_processor.finish(&mut self.mx_query_type_cache);
				},
				
				NameError((negative_cache_until, record)) =>
				{
					self.mx_query_type_cache.put_name_error(query_name, negative_cache_until, record)
				}
				
				NoData((negative_cache_until, record)) =>
				{
					self.mx_query_type_cache.put_no_data(query_name, negative_cache_until, record)
				}
				
				Referral =>
				{
					// TODO:Will have no records (checked for), might have a partial CNAME chain.
					// TODO: One or more NS records are present in the answer.
					// RFC 2308 Section 2.2 "REFERRAL RESPONSE". or // RFC 2308 Section 2.1 "REFERRAL RESPONSE".
					xxxxx
				}
			}
		})?;
	}
}
