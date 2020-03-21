/// Write the UID and GID maps.
#[inline(always)]
pub fn write_uid_and_gid_maps(proc_path: &ProcPath, child_process_identifier: NonZeroU32) -> io::Result<()>
{
	let namespaces_proc_path = NamespacesProcPath(proc_path);
	namespaces_proc_path.write_setgroups_permission(child_process_identifier, SetGroupsPermission::Deny)?;

	let user_identifiers_map = UserOrGroupIdentifierMap::<UserIdentifier>::unitary_for_current_user_identifier();
	namespaces_proc_path.write_user_or_group_identifiers_map(child_process_identifier, &user_identifiers_map)?;

	let group_identifiers_map = UserOrGroupIdentifierMap::<GroupIdentifier>::unitary_for_current_user_identifier();
	namespaces_proc_path.write_user_or_group_identifiers_map(child_process_identifier, &group_identifiers_map)
}
