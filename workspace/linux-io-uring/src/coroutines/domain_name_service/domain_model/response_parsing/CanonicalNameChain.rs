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
pub(crate) struct CanonicalNameChain<'message, 'cache: 'message>
{
	query_name: ParsedName<'message>,
	
	chain: IndexSet<ParsedName<'message>>,
	
	records: Records<'cache, CaseFoldedName<'cache>>,
}

impl<'message, 'cache: 'message> Into<Records<'cache, CaseFoldedName<'cache>>> for CanonicalNameChain<'message, 'cache>
{
	#[inline(always)]
	fn into(self) -> Records<'cache, CaseFoldedName<'cache>>
	{
		self.records
	}
}

impl<'message, 'cache: 'message> CanonicalNameChain<'message, 'cache>
{
	/// There is no standard limit for the number of links in a chain; the BIND software caps the number at 16.
	///
	/// We choose 6 as otherwise our design becomes a bit bloated.
	const MaximumChainLength: usize = 6;
	
	#[inline(always)]
	pub(crate) fn new(query_name: ParsedName<'message>) -> Self
	{
		Self
		{
			query_name,
			chain: IndexSet::with_capacity(Self::MaximumChainLength),
			records: Records::with_capacity(3),
		}
	}
	
	#[inline(always)]
	pub(crate) fn most_canonical_name(&self) -> &ParsedName<'message>
	{
		let chain_length = self.chain.len();
		
		if unlikely!(chain_length == 0)
		{
			&self.query_name
		}
		else
		{
			self.chain.get_index(chain_length - 1).unwrap()
		}
	}
	
	#[inline(always)]
	pub(crate) fn insert_link(&mut self, from: ParsedName<'message>, to: ParsedName<'message>, cache_until: CacheUntil) -> Result<(), CanonicalChainError>
	{
		use self::CanonicalChainError::*;
		
		if self.chain.len() == Self::MaximumChainLength
		{
			return Err(TooManyCanonicalNamesInChain(Self::MaximumChainLength))
		}
		
		if self.most_canonical_name().ne(&from)
		{
			return Err(CanonicalNamesNotSorted)
		}
		
		if to == self.query_name
		{
			return Err(CanonicalNameChainCanNotIncludeQueryNameAsItCreatesALoop)
		}
		
		let ok = self.chain.insert(to.clone());
		if unlikely!(!ok)
		{
			return Err(AddingNameToCanonicalNameChainCreatesALoop)
		}
		
		self.records.store_unprioritized_and_unweighted(from, cache_until, CaseFoldedName::map(to));
		Ok(())
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
