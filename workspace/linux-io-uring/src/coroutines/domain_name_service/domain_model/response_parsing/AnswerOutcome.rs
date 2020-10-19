// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) enum AnswerOutcome
{
	/// Even if this code is sent, they may be no matching records although a complete `CNAME` chain (?`DNAME`) may be present.
	Answered,

	/// This is only for authoritative answers.
	///
	/// There is no answer, although a complete `CNAME` chain or `DNAME` record might be present.
	///
	/// RFC 2308 Section 2.1: "Where no CNAME records appear, the NXDOMAIN response refers to the name in the label of the RR in the question section".
	///
	/// RFC 2308, Section 5: "A negative answer that resulted from a name error (NXDOMAIN) should be cached such that it can be retrieved and returned in response to another query for the same <QNAME, QCLASS> that resulted in the cached negative response.
	///
	/// For us, this means caching a negative response against the `QNAME` unary tuple for the last resolved name in the `CNAME` chain.
	NameError(NegativeCacheUntil),
	
	/// This is only for non-authoritative answers (this is quite common).
	///
	/// There is no answer, although a complete `CNAME` chain or `DNAME` record might be present.
	///
	/// RFC 2308 Section 2.2: " ... in which case it would be the value of the last CNAME (the QNAME) for which NODATA would be concluded".
	///
	/// RFC 2308, Section 5: "A negative answer that resulted from a no data error (NODATA) should be cached such that it can be retrieved and returned in response to another query for the same <QNAME, QTYPE, QCLASS> that resulted in the cached negative response".
	///
	/// For us, this means caching a negative response against the `QNAME, QTYPE` binary tuple for the last resolved name in the `CNAME` chain.
	///
	///
	/// This happens at the time of writing for `dig SRV www.microsoft.com`, a name which exists but does not have any `SRV` records; an entire `CNAME` chain is returned:-
	///
	/// ```bash
	/// ;; Got answer:
	/// ;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 38716
	/// ;; flags: qr rd ra; QUERY: 1, ANSWER: 3, AUTHORITY: 1, ADDITIONAL: 1
	///
	/// ;; OPT PSEUDOSECTION:
	/// ; EDNS: version: 0, flags:; udp: 1280
	/// ;; QUESTION SECTION:
	/// ;www.microsoft.com.		IN	SRV
	///
	/// ;; ANSWER SECTION:
	/// www.microsoft.com.	308	IN	CNAME	www.microsoft.com-c-3.edgekey.net.
	/// www.microsoft.com-c-3.edgekey.net. 8991	IN CNAME www.microsoft.com-c-3.edgekey.net.globalredir.akadns.net.
	/// www.microsoft.com-c-3.edgekey.net.globalredir.akadns.net. 636 IN CNAME e13678.dspb.akamaiedge.net.
	///
	/// ;; AUTHORITY SECTION:
	/// dspb.akamaiedge.net.	90	IN	SOA	n0dspb.akamaiedge.net. hostmaster.akamai.com. 1602933020 1000 1000 1000 1800
	/// ```
	///
	/// If the name does not exist and does not have any records, a `CNAME` chain will be absent.
	///
	/// This happens at the time of writing for `dig SRV mail.microsoft.com`; an entire `CNAME` chain is returned:-
	///
	/// ```bash
	/// ;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 44029
	/// ;; flags: qr rd ra; QUERY: 1, ANSWER: 0, AUTHORITY: 1, ADDITIONAL: 1
	///
	/// ;; OPT PSEUDOSECTION:
	/// ; EDNS: version: 0, flags:; udp: 1280
	/// ;; QUESTION SECTION:
	/// ;mail.microsoft.com.		IN	SRV
	///
	/// ;; AUTHORITY SECTION:
	/// microsoft.com.		90	IN	SOA	ns1-205.azure-dns.com. azuredns-hostmaster.microsoft.com. 1 3600 300 2419200 300
	/// ```
	NoData(NegativeCacheUntil),

	/// There is no answer, although a partial `CNAME` chain or `DNAME` record might be present.
	///
	/// If this outcome occurs then there was at least one `NS` (nameserver) record in the authority section.
	///
	/// We can opt to chase this, or we can just give up.
	Referral,
}

