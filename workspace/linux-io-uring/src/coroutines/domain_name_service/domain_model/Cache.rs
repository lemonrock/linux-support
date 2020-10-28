// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub struct Cache<'cache>
{
	recent_message_identifiers: RecentMessageIdentifiers,
	
	//no_domain_cache: HashMap<CaseFoldedName<'cache>, NegativeCacheUntil>,
	
	cname_query_type_cache: QueryTypeCache<'cache, CaseFoldedName<'cache>>,
	
	soa_query_type_cache: QueryTypeCache<'cache, StartOfAuthority<'cache, CaseFoldedName<'cache>>>,
	
	ns_query_type_cache: QueryTypeCache<'cache, CaseFoldedName<'cache>>,
	
	a_query_type_cache: QueryTypeCache<'cache, Ipv4Addr>,
	
	aaaa_query_type_cache: QueryTypeCache<'cache, Ipv6Addr>,
	
	mx_query_type_cache: QueryTypeCache<'cache, CaseFoldedName<'cache>>,
}

impl<'cache> Cache<'cache>
{
	// TODO: https://tools.ietf.org/html/rfc8914 Extended DNS errors.
	
	// RFC 1034, Section 3.6.2 Aliases and canonical names, Page 15, Paragraph 7: "Domain names in RRs which point at another name should always point at the primary name and not the alias".
	// RFC 2181, Section 10.3, MX and NS records: "The domain name used as the value of a NS resource record, or part of the value of a MX resource record must not be an alias".
	// RFC 2782, Page 4, "Target": "… the name MUST NOT be an alias …"
	// RFC 2782, Page 4, "Target": "A Target of "." means that the service is decidedly not available at this domain".
	
	// TODO: https://tools.ietf.org/html/rfc2672 - non-terminal DNS name redirection (DNAME but includes CNAME rules) and https://tools.ietf.org/html/rfc6672#section-10.3
	
	// TODO: Cache NXDOMAIN against all QTYPE, not just the one requested, and use that knowledge for child domains (eg if NXDOMAIN for example.com,  abc.def.example.com won't exist, either).
	// TODO: Cache SERVFAIL for 5 minutes.
	// TODO: Cache TLS / TCP / IP errors for 5 minutes.
	
	// TODO: Cache CNAME chain, SOA and NS records (all are regular cache records).
	// TODO: Handle NoData, Referral and NoDomain responses
	// TODO: Optionally chase referrals, including implicit referrals.
	// TODO: Handle NXDOMAIN for CNAME chains.
	
	// TODO: Remove caching of do-not-cache stuff.
	
	// TODO: DNS wildcards - https://en.wikipedia.org/wiki/Wildcard_DNS_record - probably best avoided.
	
	// RFC 2308, Section 7.1 Server Failure (OPTIONAL):-
	// "\[A\] resolver MAY cache a server failure response.
	// If it does so it MUST NOT cache it for longer than five (5) minutes, and it MUST be cached against the specific query tuple `<query name, type, class, server IP address>`".
	//
	// RFC 2308, Section 7.2 Dead / Unreachable Server (OPTIONAL):-
	// "Dead / Unreachable servers are servers that fail to respond in any way to a query or where the transport layer has provided an indication that the server does not exist or is unreachable
	// …
	// A server MAY cache a dead server indication.
	// If it does so it MUST NOT cache it for longer than five (5) minutes, and it MUST be cached against the specific query tuple `<query name, type, class, server IP address>` unless there was a transport layer indication that the server does not exist, in which case it applies to all queries to that specific IP address".
	//
	
	pub fn mx_enquire_over_tcp_and_cache<'yielder, SD: SocketData>(&mut self, stream: &mut TlsClientStream<'yielder, SD>, query_name: CaseFoldedName<'cache>) -> Result<(), ProtocolError<Infallible>>
	{
		self.enquire_over_tcp_and_cache::<SD, MXQueryProcessor>(stream, query_name)
	}
	
	fn enquire_over_tcp_and_cache<'yielder, 'message, SD: SocketData, QP: QueryProcessor<'message, 'cache, Error=Infallible>>(&mut self, stream: &mut TlsClientStream<'yielder, SD>, query_name: CaseFoldedName<'cache>) -> Result<(), ProtocolError<Infallible>>
	where 'cache: 'message
	{
		let message_identifier = self.recent_message_identifiers.next();
		
		Query::enquire_over_tcp::<QP, SD>(stream, message_identifier, query_name, self)
	}
}
