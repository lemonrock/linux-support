// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Write the UID and GID maps.
#[inline(always)]
pub fn write_uid_and_gid_maps(proc_path: &ProcPath, child_process_identifier: NonZeroI32) -> io::Result<()>
{
	let namespaces_proc_path = NamespacesProcPath(proc_path);
	namespaces_proc_path.write_setgroups_permission(child_process_identifier, SetGroupsPermission::Deny)?;

	let user_identifiers_map = UserOrGroupIdentifierMap::<UserIdentifier>::unitary_for_current_user_identifier();
	namespaces_proc_path.write_user_or_group_identifiers_map(child_process_identifier, &user_identifiers_map)?;

	let group_identifiers_map = UserOrGroupIdentifierMap::<GroupIdentifier>::unitary_for_current_user_identifier();
	namespaces_proc_path.write_user_or_group_identifiers_map(child_process_identifier, &group_identifiers_map)
}
