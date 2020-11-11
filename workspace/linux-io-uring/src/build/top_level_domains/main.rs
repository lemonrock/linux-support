// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// The following top-level domains (TLDs) are generated from the database at <https://data.iana.org/TLD/tlds-alpha-by-domain.txt>
///
/// This was consulted on 29th October 2020 and `Version 2020102900, Last Updated Thu Oct 29 07:07:01 2020 UTC` was used; the root zone was last updated on 22nd October 2020 (with serial number `2020102201`).
///
/// Other sources are:-
///
/// * <https://www.iana.org/domains/root/files> which contains
/// 		* The root zone file from <https://www.internic.net/domain/root.zone> which includes records but requires writing a master file parser;
/// 		* The hints file at <https://www.internic.net/domain/named.root>;
///		* The root trust anchor <https://data.iana.org/root-anchors/>;
///		* The top-level domain list at <https://data.iana.org/TLD/tlds-alpha-by-domain.txt>.
/// * <https://www.iana.org/domains/root/db>, which includes test domains and clarifies domains by type (and is a super set):-
/// 		* `country-code`.
/// 		* `generic`.
/// 		* `generic-restricted`.
///	 	* `infrastructure`.
/// 		* `sponsored`.
/// 		* `test`. (not present in the root zone file).
/// * The daily reports at <http://stats.research.icann.org/dns/tld_report/index.html> were consulted, which gives insight into insecure domains.
pub fn main(manifest_dir: &OsString, out_dir: &OsString) -> io::Result<()>
{
	println!("cargo:rerun-if-changed={}", TopLevelDomainsRelativeFilePath);
	
	let (version_and_updated_at, top_level_domains) = top_level_domains(&manifest_dir)?;
	
	write_case_folded_labels(&out_dir, &top_level_domains)?;
	write_case_folded_names(&out_dir, &top_level_domains, &version_and_updated_at)?;
	
	Ok(())
}
