// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Groups.
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct Groups(BTreeSet<GroupIdentifier>);

impl Deref for Groups
{
	type Target = BTreeSet<GroupIdentifier>;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl FromBytes for Groups
{
	type Error = StatusStatisticParseError;

	#[inline(always)]
	fn from_bytes(value: &[u8]) -> Result<Self, Self::Error>
	{
		let mut groups = BTreeSet::new();
		for value in value.split_bytes(b' ')
		{
			let was_added_for_the_first_time = groups.insert(GroupIdentifier::from_bytes(value)?);
			if unlikely!(!was_added_for_the_first_time)
			{
				return Err(StatusStatisticParseError::DuplicatedStatisticValue)
			}
		}
		Ok(Self(groups))
	}
}

impl Groups
{
	// Was 32 before Linux 2.6.4.
	const NGROUPS_MAX: usize = 65536;
	
	/// Current supplementary.
	///
	/// *SLOW*; makes two syscalls and two mallocs.
	pub fn current_supplementary() -> Self
	{
		let supplementary_groups = Self::current_supplementary_group_identifiers();
		Self(supplementary_groups.into_iter().collect())
	}
	
	pub(crate) fn current_supplementary_group_identifiers() -> Vec<GroupIdentifier>
	{
		let length = Self::get_groups(0, null_mut());
		let mut supplementary_groups: Vec<GroupIdentifier> = Vec::with_capacity(length);
		unsafe { supplementary_groups.set_len(length) };
		Self::get_groups(length, supplementary_groups.as_mut_ptr() as *mut gid_t);
		supplementary_groups
	}
	
	fn get_groups(length: usize, list: *mut gid_t) -> usize
	{
		debug_assert!(length < Self::NGROUPS_MAX);
		
		let result = unsafe { getgroups(length as i32, list) };
		if likely!(result >= 0)
		{
			result as usize
		}
		else if result == -1
		{
			match errno().0
			{
				EFAULT => panic!("list has an invalid address"),
				
				EINVAL => panic!("size is less than the number of supplementary group IDs, but is not zero"),
				
				_ => unreachable!("Unexpected error {}", errno()),
			}
		}
		else
		{
			unreachable!("Unexpected result {} from `getgroups()`", result)
		}
	}
	
	#[inline(always)]
	fn drop_all_supplementary_groups()
	{
		Self::set_groups(0, null())
	}
	
	#[inline(always)]
	fn set_groups(length: usize, list: *const gid_t)
	{
		debug_assert!(length < Self::NGROUPS_MAX);
		
		let result = unsafe { setgroups(length, list) };
		
		if likely!(result == 0)
		{
			return
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EFAULT => panic!("list has an invalid address"),
				
				EINVAL => panic!("list has an invalid address, or size is greater than NGROUPS_MAX (32 before Linux 2.6.4; 65536 since Linux 2.6.4)"),
				
				ENOMEM => panic!("Out of memory"),
				
				EPERM => panic!("Permission denied (either setgroups is denied in this user namespace or the process lacks the `CAP_SETGID` capability"),

				unexpected @ _ => panic!("Unexpected error {} from setgroups()", unexpected)
			}
		}
		else
		{
			unreachable!("setgroups() returned an unexpected result of {}", result)
		}
	}
}
