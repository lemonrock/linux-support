// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Assumes `/etc/hosts` is UTF-8.
///
/// NOTE: This is not an efficient parser.
/// NOTE: This parser will fail on 'valid' `/etc/hosts` files that do not order canonical names and aliases; the Alpine Linux default will fail, eg:-
///
/// ```bash
/// 127.0.0.1	alpine.localdomain alpine localhost.localdomain localhost
/// ::1		localhost localhost.localdomain
/// ```
///
/// This needs to be re-worked as:-
///
/// ```bash
/// 127.0.0.1	alpine.localdomain alpine localhost.localdomain localhost
/// ::1		alpine.localdomain alpine localhost.localdomain localhost
/// ```
fn parse_etc_hosts(etc_path: &EtcPath, default_domain_name: &EfficientCaseFoldedName) -> Result<HashMap<EfficientCaseFoldedName, Either<QueryTypesFixed, Alias>>, ParseEtcHostsError>
{
	#[inline(always)]
	fn line_without_comment(line: &str) -> &str
	{
		let line_without_leading_whitespace = line.trim_start();
		
		// Safe, because `#` is always a valid UTF-8 sequence.
		unsafe { from_utf8_unchecked(line_without_leading_whitespace.split_bytes_n(2, b'#').next().unwrap()) }
	}
	
	use self::ParseEtcHostsError::*;
	
	let mut outcome: HashMap<EfficientCaseFoldedName, Either<QueryTypesFixed, DomainTarget>> = HashMap::new();
	
	let raw = etc_path.hosts().read_raw()?;
	let string: String = String::from_utf8(raw.into_vec())?;
	
	for (line_index, line) in string.split_terminator(b'\n').enumerate()
	{
		let line_without_comment = line_without_comment(line);
		
		let mut fields = line_without_comment.split_ascii_whitespace();
		
		let internet_protocol_address = IpAddr::from_str(fields.next().unwrap()).map_err(|error| InvalidInternetProtocolAddress { line_index, error })?;
		let canonical_name = fields.next().ok_or(MissingCanonicalNameField { line_index })?;
		
		let canonical_name = default_domain_name.convert_etc_hosts_name(canonical_name.as_bytes()).map_err(|error| InvalidCanonicalOrAliasName { line_index, error })?;
		
		use self::FastSecureHashMapEntry::*;
		
		match outcome.entry(canonical_name.clone())
		{
			Vacant(vacant) =>
			{
				vacant.insert(Left(QueryTypesFixed::new(internet_protocol_address)))
			}
			
			Occupied(occupied) =>
			{
				match occupied.get_mut()
				{
					Left(query_types_fixed) => query_types_fixed.add_internet_protocol_address(internet_protocol_address),
					
					Right(value) => return Err(CanonicalNameWasPreviouslyDefinedAsAnAlias { line_index, canonical_name: occupied.replace_key(), previously_defined_alias: value.clone() }),
				}
			}
		}
		
		for alias_name in fields
		{
			let alias_name = default_domain_name.convert_etc_hosts_name(alias_name.as_bytes())?;
			
			match outcome.entry(alias_name)
			{
				Vacant(vacant) =>
				{
					vacant.insert(Right(canonical_name.clone()))
				}
				
				Occupied(occupied) =>
				{
					match occupied.get()
					{
						Left(_) => return Err(AliasNameWasPreviouslyDefinedAsACanonicalName { line_index, alias_name: occupied.replace_key() }),
						
						Right(existing) =>
						{
							if existing.ne(&canonical_name)
							{
								return Err(AliasNameWasPreviouslyDefinedWithADifferentTargetName { line_index, alias_name: occupied.replace_key(), previously_defined_alias: value.clone() })
							}
						},
					}
				}
			}
		}
	}
	
	Ok(outcome)
}
