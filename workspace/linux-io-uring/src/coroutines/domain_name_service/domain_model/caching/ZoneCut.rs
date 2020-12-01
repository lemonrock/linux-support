// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


struct ZoneCut
{
	domain_cache_entry: DomainCacheEntry,

	subdomains: HashMap<Box<[u8]>, ZoneCut>
}

struct ExpiryEvents
{
	multi_map: BTreeMap<NanosecondsSinceUnixEpoch, HashMap<DomainTarget, ExpiryEvent>>
}

impl ExpiryEvents
{
	fn process(&mut self, now: NanosecondsSinceUnixEpoch, root: &mut ZoneCut)
	{
		let keep = self.multi_map.split_off(&now);
		let process = replace(&mut self.multi_map, keep);
		for events_at_a_point_in_time in process.into_values()
		{
			for (domain_target, expiry_event) in events_at_a_point_in_time
			{
				root.expired(domain_target, expiry_event)
			}
		}
	}
	
	fn remove(&mut self, was: NanosecondsSinceUnixEpoch, domain_target: &DomainTarget)
	{
		use std::collections::btree_map::Entry::*;
		
		match self.multi_map.entry(was)
		{
			Vacant(_) =>
			{
				panic!();
			}
			
			Occupied(ref mut occupied) =>
			{
				let remove_entry =
				{
					let value = occupied.get_mut();
					let removed = value.remove(domain_target);
					removed.expect("Missing");
					value.is_empty()
				};
				if remove_entry
				{
					occupied.remove();
				}
			}
		}
	}
}



impl ZoneCut
{
	pub(crate) fn expired(&mut self, domain_target: DomainTarget, expiry_event: ExpiryEvent)
	{
		debug_assert!(!domain_target.is_root());
		
		let remove = self.expired_recursively(domain_target, expiry_event, new_non_zero_u8(1));
		debug_assert_eq!(remove, false, "Should never remove root as it is DomainCacheEntry::Fixed");
	}
	
	fn expired_recursively(&mut self, domain_target: DomainTarget, expiry_event: ExpiryEvent, number_of_non_root_labels: NonZeroU8) -> bool
	{
		let label_index = number_of_non_root_labels.get();
		
		let is_leaf = label_index + 1 == domain_target.number_of_labels_including_root().get();
		
		if is_leaf
		{
			self.expired_leaf(expiry_event)
		}
		else
		{
			let key = domain_target.label(label_index);
			
			use self::FastSecureRawMutHashMapEntry::*;
			match self.subdomains.raw_entry_mut().from_key(key.deref())
			{
				// This seems possible because we removed when is_a_now_an_expired_leaf_placeholder() was true
				// BUT: Since the node was a placeholder, it will only be empty because of former expiry events being processed.
				Vacant(_) => unreachable_code(format_args!("Vacant entry for non-leaf for ExpiryEvent {:?} for domain {} at number_of_non_root_labels {}", expiry_event, domain_target, number_of_non_root_labels)),
				
				Occupied(occupied) =>
				{
					let remove = occupied.get_mut().expired_recursively(domain_target,  expiry_event, new_non_zero_u8(label_index + 1));
					if remove
					{
						occupied.remove();
					}
				}
			};
			
			self.is_a_now_an_expired_leaf_placeholder()
		}
	}
	
	/// A placeholder is an entry added simply to give a subdomain a point in the tree.
	#[inline(always)]
	fn is_a_now_an_expired_leaf_placeholder(&self) -> bool
	{
		use self::DomainCacheEntry::*;
		
		self.subdomains.is_empty() && match self.domain_cache_entry
		{
			NeverValid => false,
			
			Valid { always_valid: false, ref query_types_cache, .. } => query_types_cache.is_empty(),
			
			_ => false,
		}
	}
	
	#[inline(always)]
	fn expired_leaf(&mut self, expiry_event: ExpiryEvent) -> bool
	{
		use self::DomainCacheEntry::*;
		use self::ExpiryEvent::*;
		
		match (self.domain_cache_entry, expiry_event)
		{
			(NoDomain(_), RemoveNoDomain) => Expired,
			
			(Valid { always_valid: false, ref mut query_types_cache, .. }, RemoveQueryType(query_types_cache_field)) =>
			{
				query_types_cache.expired(query_types_cache_field);
				query_types_cache.is_empty()
			}
			
			_ => unreachable_code(format_args!("Invalid combination of DomainCacheEntry {:?} and ExpiryEvent {:?} for domain {}", self.domain_cache_entry, expiry_event, domain_target)),
		}
	}
	
	pub(crate) fn get<'a, OR: OwnedRecord>(&'a self, domain_target: &DomainTarget, now: NanosecondsSinceUnixEpoch) -> Option<DomainCacheGet<'a, OR::OwnedRecords, OR>>
	{
		if unlikely!(domain_target.is_root())
		{
			if let DomainCacheEntry::Fixed { subdomains_implicitly_resolve_to_the_same_record_as_this_one: false, ref fixed_domain_cache_entry  } = self.domain_cache_entry
			{
				DomainCacheGet::fixed(fixed_domain_cache_entry)
			}
			else
			{
				unreachable_code(format_args!("Not DomainCacheEntry::Fixed for root!"))
			}
		}
		else
		{
			self.get_recursively(domain_target, now, new_non_zero_u8(1))
		}
	}
	
	fn get_recursively<'a, OR: OwnedRecord>(&'a self, domain_target: &DomainTarget, now: NanosecondsSinceUnixEpoch, number_of_non_root_labels: NonZeroU8) -> Option<DomainCacheGet<'a, OR::OwnedRecords, OR>>
	{
		let label_index = number_of_non_root_labels.get();
		
		let is_leaf = label_index + 1 == domain_target.number_of_labels_including_root().get();
		if is_leaf
		{
			self.get_leaf(now)
		}
		else
		{
			let key = domain_target.label(label_index);
			
			match self.subdomains.get(key.deref())
			{
				None => None,
				
				Some(subdomain) => subdomain.get_recursively(domain_target, now, new_non_zero_u8(label_index + 1))
			}
		}
	}
	
	#[inline(always)]
	fn get_leaf<'a, OR: OwnedRecord>(&'a self,  now: NanosecondsSinceUnixEpoch) -> Option<DomainCacheGet<'a, OR::OwnedRecords, OR>>
	{
		use self::DomainCacheEntry::*;
		
		match self.domain_cache_entry
		{
			NeverValid => Some(DomainCacheGet::NoDomain),
			
			Fixed { ref fixed_domain_cache_entry, .. } => DomainCacheGet::fixed(fixed_domain_cache_entry),
			
			Valid { always_valid: true, ref query_types_cache, .. } => DomainCacheGet::valid(query_types_cache, now),
			
			Valid { always_valid: false, ref query_types_cache, .. } => DomainCacheGet::valid(query_types_cache, now),
			
			Alias { ref flattened_target, .. } => DomainCacheGet::alias(flattened_target, now),
			
			NoDomain(ref no_domain_cache_entry) => DomainCacheGet::no_domain(no_domain_cache_entry, now),
		}
	}
	
	pub(crate) fn get_domain_cache_entry<'a, OR: OwnedRecord>(&'a self, domain_target: &DomainTarget) -> Option<&DomainCacheEntry>
	{
		if unlikely!(domain_target.is_root())
		{
			Some(&self.domain_cache_entry)
		}
		else
		{
			self.get_domain_cache_entry_recursively(domain_target, new_non_zero_u8(1))
		}
	}
	
	#[inline(always)]
	fn get_domain_cache_entry_recursively(&self, domain_target: &DomainTarget, number_of_non_root_labels: NonZeroU8) -> Option<&DomainCacheEntry>
	{
		let label_index = number_of_non_root_labels.get();
		
		let is_leaf = label_index + 1 == domain_target.number_of_labels_including_root().get();
		if is_leaf
		{
			Some(&self.domain_cache_entry)
		}
		else
		{
			let key = domain_target.label(label_index);
			
			match self.subdomains.get(key.deref())
			{
				None => None,
				
				Some(subdomain) => subdomain.get_domain_cache_entry_recursively(domain_target, new_non_zero_u8(label_index + 1))
			}
		}
	}
}
