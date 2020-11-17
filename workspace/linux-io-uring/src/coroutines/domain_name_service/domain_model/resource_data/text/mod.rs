// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;


// // TODO: Will need to case-folded
// #[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
// pub struct Key<OOPB: OwnedOrParsedBytes>(OOPB);
//
//
// /// As defined by RFC 6763.
// pub struct DnsServiceDiscoveryAttribute

/*
	"global underscored node name"

	TODO: Undescrore domain names
		https://tools.ietf.org/html/rfc8553
		https://tools.ietf.org/html/rfc8552
		
		TXT
		SRV
			_Service._Proto. ...
			
			_Proto is now `protocol`.
			Is registered at IANA.
			Is case-insensitive
			Starts with an underscore.
			
		URI
			_Service._Proto. ...
			_C._A._B. ...  this is an Enumservice A:B:C (see RFC 6117).
			
		

	TODO: TXT - needs sorting, are there standard key formats?
	    * DNS-SD
	        * <https://tools.ietf.org/html/rfc6763#section-6>
	        * Section 6.7.  Version Tag, defines a version key that *should* occur first in a record.
	            * txtvers
	            * protovers
	        * Multiple strings are multiple key-values or just boolean present keys.
	        * Uses _service._tcp.example.com for the owner name
	        * Multiple TXT records are possible but discouraged
	        * There is a registry.
	        * See <https://www.iana.org/assignments/service-names-port-numbers/service-names-port-numbers.xml>
	    * DMARC
	        * Starts "v=DMARC1;"
	        * only for owner names that start "_dmarc.example.com"
	    * SPF
	        * RFC 7298, Section 3.3 Multiple Strings in a Single DNS Record.
	        * Solitary
	        * first string must start "v=spf1 "
	    * DKIM
	        * RFC 6376, <https://tools.ietf.org/html/rfc6376#page-53> Section 7.5 for a list of valid tags.
	        * Only for owner names containing `_domainkey` eg "selector._domainkey.example.net" or "_domainkey.example.net"
	            * Can selectors have dots?
	        * Starts "v=DKIM1;"
	    * Site verification, eg
	        "google-site-verification=6P08Ow5E-8Q0m6vQ7FMAqAYIDprkVV8fUf_7hZ4Qvc8"
	        "facebook-domain-verification=somethingbase64"
	        Would seem to be key-value
	    * Amazon SES
	        * Has an owner name of `_amazonses`
	    * RFC 1464 defines a structure format, heavily abused, but essentially key=value in which keys are supposed to printable ASCII and case-insensitive; any '=' sign in an attribute name is supposed to be escaped with a preceeding backquote (`); backquotes are escaped likewise as ``. Empty attribute names are not permitted.
	    * See examples at https://en.wikipedia.org/wiki/TXT_record
	
 */
