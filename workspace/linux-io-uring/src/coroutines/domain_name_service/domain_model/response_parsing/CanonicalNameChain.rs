// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A chain of CNAMEs, that, unbroken, eventually goes from `query_name` to a final name, which is resolved as a `A` (or `AAAA`, etc) record.
///
/// A typical example for `dig www.microsoft.com`:-
///
/// ```bash
///;; ANSWER SECTION:
/// www.microsoft.com.	1168	IN	CNAME	www.microsoft.com-c-3.edgekey.net.
/// www.microsoft.com-c-3.edgekey.net. 2865	IN CNAME www.microsoft.com-c-3.edgekey.net.globalredir.akadns.net.
/// www.microsoft.com-c-3.edgekey.net.globalredir.akadns.net. 853 IN CNAME e13678.dspb.akamaiedge.net.
/// e13678.dspb.akamaiedge.net. 15	IN	A	104.75.174.50
/// ```
///
/// In this example, the `most_canonical_name` is `e13678.dspb.akamaiedge.net.` and its `parent` is `dspb.akamaiedge.net.`.
/// The `query_name` was `www.microsoft.com.`.
///
/// An example where a SOA authority is returned because the record type does not exist, eg for `dig MX www.microsofot.com`:-
///
/// ```bash
/// ;; QUESTION SECTION:
/// ;www.microsoft.com.		IN	MX
///
/// ;; ANSWER SECTION:
/// www.microsoft.com.	3515	IN	CNAME	www.microsoft.com-c-3.edgekey.net.
/// www.microsoft.com-c-3.edgekey.net. 19598 IN CNAME www.microsoft.com-c-3.edgekey.net.globalredir.akadns.net.
/// www.microsoft.com-c-3.edgekey.net.globalredir.akadns.net. 528 IN CNAME e13678.dspb.akamaiedge.net.
///
/// ;; AUTHORITY SECTION:
/// dspb.akamaiedge.net.	90	IN	SOA	n0dspb.akamaiedge.net. hostmaster.akamai.com. 1602858165 1000 1000 1000 1800
/// ```
///
/// In this example, the `most_canonical_name` is `e13678.dspb.akamaiedge.net.` and its `parent` is `dspb.akamaiedge.net.`.
/// The `query_name` was `www.microsoft.com.`.
///
/// The function `validate_authority_section_name()` below will then validate that the `start_of_authority_name`, `dspb.akamaiedge.net.`, is the same as `parent`.
#[derive(Debug)]
pub(crate) struct CanonicalNameChain<'message>
{
	query_name: &'message EfficientCaseFoldedName,
	
	chain: CanonicalNameChainRecords,
	
	/// RFC 6672, Section 3.2 Server Algorithm Step 3.C. implies in the final paragraph that multiple `DNAME` records can be included in an answer.
	delegation_name_records: DelegationNameRecords,
}

impl<'message> CanonicalNameChain<'message>
{
	/// There is no standard limit for the number of links in a chain; the BIND software caps the number at 16.
	///
	/// We choose 6 as otherwise our design becomes a bit bloated.
	const MaximumChainLength: usize = 6;
	
	#[inline(always)]
	pub(crate) fn new(query_name: &'message EfficientCaseFoldedName) -> Self
	{
		Self
		{
			query_name,
			chain: CanonicalNameChainRecords::with_capacity(Self::MaximumChainLength),
			delegation_name_records: Records::with_capacity(1),
		}
	}
	
	pub(crate) fn validate_records(records) -> Result<(), X>
	{
	
	}
	
	#[inline(always)]
	pub(crate) fn most_canonical_name<'a>(&'a self) -> &'a EfficientCaseFoldedName
	where 'message: 'a
	{
		Self::most_canonical_name_static::<'a>(self.query_name, &self.chain)
	}
	
	#[inline(always)]
	pub(crate) fn most_canonical_name_static<'a>(query_name: &'a EfficientCaseFoldedName, canonical_name_chain_records: &'a CanonicalNameChainRecords) -> &'a EfficientCaseFoldedName
	{
		let chain_length = canonical_name_chain_records.len();
		
		if unlikely!(chain_length == 0)
		{
			query_name
		}
		else
		{
			let (_key, (_, ref last_name)) = canonical_name_chain_records.get_index(length - 1).unwrap();
			last_name
		}
	}
	
	#[inline(always)]
	pub(crate) fn insert_link(&mut self, from: &ParsedName<'message>, cache_until: CacheUntil, to: &ParsedName<'message>) -> Result<(), CanonicalChainError>
	{
		use self::CanonicalChainError::*;
		
		let length = self.chain.len();
		
		if length == Self::MaximumChainLength
		{
			return Err(TooManyCanonicalNamesInChain(Self::MaximumChainLength))
		}
		
		if to.eq(self.query_name)
		{
			return Err(CanonicalNameChainCanNotIncludeQueryNameAsItCreatesALoop)
		}
		
		let most_canonical_name = self.most_canonical_name();
		if most_canonical_name.ne(from)
		{
			return Err(CanonicalNamesNotSorted)
		}
		
		use self::IndexMapEntry::*;
		match self.chain.entry(most_canonical_name.clone())
		{
			Occupied(_) => return Err(AddingNameToCanonicalNameChainCreatesALoop),
			
			Vacant(vacant) =>
			{
				let value = PresentSolitary::new(cache_until, EfficientCaseFoldedName::from(to));
				vacant.insert(value);
			},
		};
		
		Ok(())
	}
	
	#[inline(always)]
	pub(crate) fn record_delegation_name(&mut self, from: &ParsedName<'message>, cache_until: CacheUntil, to: &ParsedName<'message>)
	{
		self.delegation_name_records.store_unprioritized_and_unweighted(from, cache_until, EfficientCaseFoldedName::from(to));
	}
	
	#[inline(always)]
	fn validate_authority_section_name(&self, authority_section_name: &ParsedName<'message>) -> bool
	{
		match self.most_canonical_name().parent()
		{
			Some(ref most_canonical_name_parent) => authority_section_name.eq(most_canonical_name_parent),
			
			// Very rare; only if we queried for root or the CNAME chain points to root (why)?
			None => authority_section_name.is_root(),
		}
	}
}
