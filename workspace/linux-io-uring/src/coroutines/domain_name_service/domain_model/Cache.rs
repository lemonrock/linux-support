// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Cache.
pub struct Cache
{
	recent_message_identifiers: RecentMessageIdentifiers,

	domain_cache: DomainCache,
}

impl Cache
{
	/// MX.
	pub fn MX<'yielder, SD: SocketData>(&mut self, stream: &mut TlsClientStream<'yielder, SD>, query_name: &FullyQualifiedDomainName) -> Result<(), ProtocolError<Infallible>>
	{
		self.enquire_over_tcp_and_cache::<SD, MXQueryProcessor>(stream, query_name)
	}
	
	fn enquire_over_tcp_and_cache<'yielder, SD: SocketData, QP: QueryProcessor>(&mut self, stream: &mut TlsClientStream<'yielder, SD>, query_name: &FullyQualifiedDomainName) -> Result<(), ProtocolError<Infallible>>
	{
		let now = NanosecondsSinceUnixEpoch::now();
		
		if self.no_domain_cache.recursive_existence(&query_name, now)
		{
			return Ok(())
		}
		
		let message_identifier = self.recent_message_identifiers.next();
		
		Query::enquire_over_tcp::<QP, SD>(stream, message_identifier, now, query_name, self)
	}
	
	// TODO: https://tools.ietf.org/html/rfc8914 Extended DNS errors.
	
	// TODO: Cache SERVFAIL for 5 minutes.
	// TODO: Cache TLS / TCP / IP errors for 5 minutes.
	
	// TODO: Optionally chase referrals, including implicit referrals.
	
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
	
}
