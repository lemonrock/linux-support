// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This uses the musl libc logic: "queries with fewer dots than the ndots configuration variable are processed with search first then tried literally (just like glibc), but those with at least as many dots as ndots are only tried in the global namespace (never falling back to search, which glibc would do if the name is not found in the global DNS namespace).
/// This difference comes from a consistency requirement not to return different results subject to transient failures or to global DNS namespace changes outside of one’s control (addition of new TLDs)".
pub struct SearchListIterator<'resolv_conf, 'search_name: 'next, 'next>
{
	search_domains: &'resolv_conf ArrayVec<[FullyQualifiedDomainName; Self::MaximumSearchDomains]>,
	
	next_search_domain_index: usize,
	
	search_name: &'search_name SearchName,
	
	harden_using_iana_ssac_advisory_on_search_list_processing: bool,
	
	marker: PhantomData<&'next ()>,
}

impl<'resolv_conf, 'search_name: 'next, 'next> SearchListIterator<'resolv_conf, 'search_name, 'next>
{
	/// See the [SSAC Advisory on Search List Processing (SSAC064)](https://www.icann.org/en/system/files/files/sac-064-en.pdf).
	///
	/// Note that if using `Self::Host()` and `harden_using_iana_ssac_advisory_on_search_list_processing` is true, the iterator will not produce any domain names.
	#[inline(always)]
	pub(crate) fn new(resolv_conf: &'resolv_conf ResolvConf, search_name: &'search_name SearchName, harden_using_iana_ssac_advisory_on_search_list_processing: bool) -> Self
	{
		use self::SearchName::*;
		
		let use_search_list = match search_name
		{
			FullyQualified(_) => false,
			
			Relative(relative_domain_name) => if harden_using_iana_ssac_advisory_on_search_list_processing
			{
				// As per SSAC064, Section 4.1.3 Unqualified Multi-label Domain Names Never Use Search Lists.
				false
			}
			else
			{
				Self::use_search_list(relative_domain_name.number_of_periods(), resolv_conf)
			},
			
			Host(host_name) => Self::use_search_list(0, resolv_conf),
		};
		
		Self
		{
			search_domains: &resolv_conf.search_domains,
		
			next_search_domain_index: if use_search_list
			{
				0
			}
			else
			{
				resolv_conf.search_domains.len()
			},
		
			search_name,
		
			marker: PhantomData,
		}
	}
	
	#[inline(always)]
	fn use_search_list(query_number_of_periods: u8, resolv_conf: &'resolv_conf ResolvConf) -> bool
	{
		query_number_of_periods < resolv_conf.number_of_periods()
	}
}

impl<'resolv_conf, 'search_name: 'next, 'next> Iterator for SearchListIterator<'resolv_conf, 'search_name, 'next>
{
	type Item = Result<Cow<'next, FullyQualifiedDomainName>, EfficientCaseFoldedNameParseError>;
	
	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		use self::SearchName::*;
		
		let length = self.search_domains.len();
		if self.next_search_domain_index == length + 1
		{
			None
		}
		else if self.next_search_domain_index == length
		{
			let cow = match self.search_name
			{
				FullyQualified(fully_qualified_domain_name) => Cow::Borrowed(fully_qualified_domain_name),
				
				Relative(relative_domain_name) => Cow::Borrowed(relative_domain_name.as_ref()),
				
				// As per SSAC064, Section 4.1.2 Unqualified Single-Label Domain Names Are Never Queried Directly.
				Host(host_name) => if self.harden_using_iana_ssac_advisory_on_search_list_processing
				{
					self.next_search_domain_index += 1;
					return None
				}
				else
				{
					Cow::Owned(host_name.as_fully_qualified_domain_name())
				}
			};
			
			self.next_search_domain_index += 1;
			
			Some(Ok(cow))
		}
		else
		{
			debug_assert!(self.next_search_domain_index < length);
			
			let search_domain: &FullyQualifiedDomainName = self.search_domains.get_unchecked_safe(self.next_search_domain_index);
			
			let owned = match self.search_name
			{
				FullyQualified(_) => unreachable_code(format_args!("This is checked at constrution time")),
				
				Relative(relative_domain_name) => search_domain.prepend_relative_name(relative_domain_name)?,
				
				Host(host_name) => search_domain.prepend_host_name(host_name)?,
			};
			
			self.next_search_domain_index += 1;
			
			Some(Ok(Cow::Owned(owned)))
		}
	}
}
