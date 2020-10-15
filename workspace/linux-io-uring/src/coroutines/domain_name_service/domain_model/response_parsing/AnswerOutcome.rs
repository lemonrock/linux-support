// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) enum AnswerOutcome
{
	Answered,

	/// There is no answer, although a complete `CNAME` chain or `DNAME` record might be present.
	///
	/// RFC 2308 Section 2.1: "Where no CNAME records appear, the NXDOMAIN response refers to the name in the label of the RR in the question section".
	///
	/// RFC 2308, Section 5: "A negative answer that resulted from a name error (NXDOMAIN) should be cached such that it can be retrieved and returned in response to another query for the same <QNAME, QCLASS> that resulted in the cached negative response.
	///
	/// For us, this means caching a negative response against the `QNAME` unary tuple for the last resolved name in the `CNAME` chain.
	NameError(NegativeCachingTimeToLiveInSeconds),

	/// There is no answer, although a complete `CNAME` chain or `DNAME` record might be present.
	///
	/// RFC 2308 Section 2.2: " ... in which case it would be the value of the last CNAME (the QNAME) for which NODATA would be concluded".
	///
	/// RFC 2308, Section 5: "A negative answer that resulted from a no data error (NODATA) should be cached such that it can be retrieved and returned in response to another query for the same <QNAME, QTYPE, QCLASS> that resulted in the cached negative response".
	///
	/// For us, this means caching a negative response against the `QNAME, QTYPE` binary tuple for the last resolved name in the `CNAME` chain.
	NoData(NegativeCachingTimeToLiveInSeconds),

	/// There is no answer, although a partial `CNAME` chain or `DNAME` record might be present.
	///
	/// We can opt to chase this, or we can just give up.
	Referral,
}

