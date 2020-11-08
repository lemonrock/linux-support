// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use std::collections::HashMap;
use std::collections::HashSet;
use std::ffi::OsString;
use std::io;
use maplit::hashmap;
use maplit::hashset;


include!("AllPermutationsOfASet.rs");


pub fn main(manifest_dir: &OsString, out_dir: &OsString) -> io::Result<()>
{
	// TODO: Zero length.
	// TODO: Max length is 255 bytes.
	
	// RFC 3588, Section 11.6 NAPTR Service Fields.
	let legacy_diameter: HashMap<&'static str, &'static str> = hashmap!
	{
		"AAA+D2T" => "Some(LegacyDiameter(DiameterLegacyResolutionService::D2T))",
		"AAA+D2S" => "Some(LegacyDiameter(DiameterLegacyResolutionServiceD2S))",
	};
	
	// See <https://www.iana.org/assignments/sip-table/sip-table.xhtml>.
	let legacy_sip: HashMap<&'static str, &'static str> = hashmap!
	{
		// RFC 3263, Section 4.1 Selecting a Transport Protocol.
		"SIP+D2T" => "Some(LegacySip(SipLegacyResolutionService::D2T))",
		
		// RFC 3263, Section 4.1 Selecting a Transport Protocol.
		"SIPS+D2T" => "Some(LegacySipSecure(SipSecureLegacyResolutionService::D2T))",
		
		// RFC 3263, Section 4.1 Selecting a Transport Protocol.
		"SIP+D2U" => "Some(LegacySip(SipLegacyResolutionService::D2U))",
		
		// RFC 3263, Section 4.1 Selecting a Transport Protocol.
		"SIP+D2S" => "Some(LegacySip(SipLegacyResolutionService::D2S))",
		
		// RFC 3263, Section 4.1 Selecting a Transport Protocol.
		"SIPS+D2S" => "Some(LegacySipSecure(SipSecureLegacyResolutionService::D2S))",
		
		// RFC 7118.
		"SIP+D2W" => "Some(LegacySip(SipLegacyResolutionService::D2W))",
		
		// RFC 7118.
		"SIPS+D2W" => "Some(LegacySipSecure(SipSecureLegacyResolutionService::D2W))",
	};
	
	// 'Application Service' and 'Application Protocol' have meaning as defined as per RFC 3958.
	// 'Application Service' and 'Application Protocol' valid values, excluding those starting `x-`, are listed at <https://www.iana.org/assignments/s-naptr-parameters/s-naptr-parameters.xhtml>.
	
	// TODO: Consider unofficial 'Application Identifier' values, as these are 1-10 digits where the first digit does not start with `0`.
	// First defined in RFC 6408.
	// Subset of <https://www.iana.org/assignments/s-naptr-parameters/s-naptr-parameters.xhtml#s-naptr-parameters-1>.
	let modern_diameter_application_services: HashSet<&'static str> = hashset!
	{
		"aaa",
		"aaa+ap1",
		"aaa+ap2",
		"aaa+ap3",
		"aaa+ap4",
		"aaa+ap5",
		"aaa+ap6",
		"aaa+ap7",
		"aaa+ap8",
		"aaa+ap9",
		"aaa+ap9",
		"aaa+ap16777250",
		"aaa+ap16777251",
		"aaa+ap16777264",
		"aaa+ap16777267",
		"aaa+ap16777281",
		"aaa+ap16777282",
		"aaa+ap16777283",
		"aaa+ap16777284",
		"aaa+ap16777285",
		"aaa+ap16777286",
		"aaa+ap16777287",
		"aaa+ap16777288",
		"aaa+ap16777289",
		"aaa+ap16777290",
		"aaa+ap4294967295",
	};
	
	// First defined in RFC 6408.
	// Subset of <https://www.iana.org/assignments/s-naptr-parameters/s-naptr-parameters.xhtml#s-naptr-parameters-2>.
	let modern_diameter_application_protocols: HashSet<&'static str> = hashset!
	{
		// RFC 6733.
		"diameter.dtls.sctp",
		
		// RFC 6408.
		"diameter.sctp",
		
		// RFC 6408.
		"diameter.tcp",
		
		// RFC 6408.
		"diameter.tls.tcp",
	};
	
	
	
	
	fn combine_application_services_and_application_protocols(application_services: HashSet<impl AsRef<str>>, application_protocols: HashSet<&'static str>)
	{
		let application_protocols = into_vec(application_protocols);
		(&application_protocols[..]).permutations();
	}
	
	fn into_vec(application_protocols: HashSet<&'static str>) -> Vec<Box<[u8]>>
	{
		let length = application_protocols.len();
		let mut application_protocols_vec = Vec::with_capacity(length);
		for application_protocol in application_protocols
		{
			debug_assert_ne!(application_protocol.len(), 0);
			application_protocols_vec.push(application_protocol.as_bytes().to_owned().into_boxed_slice());
		}
		application_protocols_vec
	}
	
	
	
	// TODO: codegen
	/*
		For each possible string combination, generation all the permutations of upper and lower case
		Then sort them
		Then match byte-by-byte, with lots of duplicate code paths.
		
		eg "SIP+D2W" and "AAA+D2W" and "aaa+ap1:diameter_dtls_sctp" and "aaa+ap1:diameter_tls_tcp"
		- generate all permutations of string; put into a BTreeSet
		
		- also the "x-"
		
		- also all the enumservice palavers.
		
		
		Then do match parsing, but, instead of going byte-by-byte, consider going u64, u32, u16, u8 blocks at a time?
	
	
	 */
	Ok(())
}
