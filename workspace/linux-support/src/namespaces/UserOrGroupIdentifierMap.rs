// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// NOTE: This requires a custom Serde implementation.
#[derive(Debug)]
pub struct UserOrGroupIdentifierMap<U: UserOrGroupIdentifier>
{
	inside_namespace_to_outside_namespace_user_or_group_identifiers: Intervals<U>,
	reverse_mapping_does_not_overlap_check: Intervals<()>,
}

impl<U: UserOrGroupIdentifier> UserOrGroupIdentifierMap<U>
{
	/// Add a mapping to the map for the current user identifier.
	#[inline(always)]
	pub fn unitary_for_current_user_identifier() -> Self
	{
		let mut this = Self
		{
			inside_namespace_to_outside_namespace_user_or_group_identifiers: Intervals::default(),
			reverse_mapping_does_not_overlap_check: Intervals::default(),
		};
		this.add_mapping(U::Zero, U::current_real(), new_non_zero_u32(1));
		this
	}

	/// Add a mapping to the map.
	#[inline(always)]
	pub fn add_mapping(&mut self, inside_namespace_user_or_group_identifier: U, outside_namespace_user_or_group_identifier: U, length: NonZeroU32)
	{
		let inside_namespace_user_or_group_identifier: u32 = inside_namespace_user_or_group_identifier.into();
		self.inside_namespace_to_outside_namespace_user_or_group_identifiers.add_interval(inside_namespace_user_or_group_identifier, length, outside_namespace_user_or_group_identifier);

		let outside_namespace_user_or_group_identifier: u32 = outside_namespace_user_or_group_identifier.into();
		self.reverse_mapping_does_not_overlap_check.add_interval(outside_namespace_user_or_group_identifier, length, ());
	}

	/// For writing `U: UserIdentifier`, the writing process must have the capability `CAP_SETUID` in the user namespace of the process's map file and in the outside_namespace namespace.
	/// For writing `U: GroupIdentifier`, the writing process must have the capability `CAP_SETGID` in the user namespace of the process's map file and in the outside_namespace namespace (the later in the outside_namespace namespace is not required if `deny` has been written to `/proc/[pid]/setgroups`.
	pub fn write_to_map_file(&self, path: &Path) -> io::Result<()>
	{
		let file = path.open_file_for_writing()?;
		let mut buf_writer = BufWriter::new(file);
		for (&inside_namespace_user_or_group_identifier, &(length, outside_namespace_user_or_group_identifier)) in self.inside_namespace_to_outside_namespace_user_or_group_identifiers.iter()
		{
			let outside_namespace_user_or_group_identifier: u32 = outside_namespace_user_or_group_identifier.into();
			writeln!(buf_writer, "{} {} {}", inside_namespace_user_or_group_identifier, outside_namespace_user_or_group_identifier, length)?;
		}
		
		buf_writer.flush()?;
		
		Ok(())
	}
}
