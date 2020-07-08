// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct UsageHashMap<V>
{
	map: HashMap<String, (V, Cell<u64>)>,
	could_not_resolve_program_error: ProgramError,
	not_all_values_have_been_used_program_error: fn(Vec<String>) -> ProgramError,
}

impl<V> UsageHashMap<V>
{
	#[inline(always)]
	pub(crate) fn new(could_not_resolve_program_error: ProgramError, not_all_values_have_been_used_program_error: fn(Vec<String>) -> ProgramError) -> Self
	{
		Self
		{
			map: HashMap::default(),
			could_not_resolve_program_error,
			not_all_values_have_been_used_program_error,
		}
	}
	
	#[inline(always)]
	pub(crate) fn resolve(&self, key: &str) -> Result<&V, ProgramError>
	{
		match self.map.get(key)
		{
			Some((value, usage_count)) =>
			{
				usage_count.set(usage_count.get().saturating_add(1));
				Ok(value)
			}
			
			None => Err(self.could_not_resolve_program_error),
		}
	}
	
	#[inline(always)]
	pub(crate) fn guard_all_values_have_been_resolved_at_least_once(self) -> Result<(), ProgramError>
	{
		let mut unused = Vec::new();
		for (name, (_, usage_count)) in self.0
		{
			if unlikely!(usage_count.get() == 0)
			{
				unused.push(name)
			}
		}
		
		if likely!(unused.is_empty())
		{
			Ok(())
		}
		else
		{
			Err((self.not_all_values_have_been_used_program_error)(unused))
		}
	}
}
