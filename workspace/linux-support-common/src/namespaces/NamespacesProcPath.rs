// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents namespace files in `/proc`.
///
/// This is a zero-cost abstraction because it is implemented as a new-type wrapper around `ProcPath`.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NamespacesProcPath<'a>(pub &'a ProcPath);

/// `setgroups` permission.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum SetGroupsPermission
{
	/// Allow.
	Allow,

	/// Deny.
	Deny,
}

impl<'a> IntoLineFeedTerminatedByteString<'a> for SetGroupsPermission
{
	/// Converts data to a byte string terminated with a new line (`\n`).
	#[inline(always)]
	fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>
	{
		use self::SetGroupsPermission::*;

		match self
		{
			Allow => Cow::from(b"allow\n" as &[u8]),
			Deny => Cow::from(b"deny\n" as &[u8]),
		}
	}
}

#[allow(missing_docs)]
impl<'a> NamespacesProcPath<'a>
{
	#[inline(always)]
	pub fn write_setgroups_permission(&self, child_process_identifier: NonZeroI32, set_groups_permission: SetGroupsPermission) -> io::Result<()>
	{
		self.0.process_file_path(child_process_identifier, "setgroups").write_value(set_groups_permission)
	}

	#[inline(always)]
	pub fn write_user_or_group_identifiers_map<U: UserOrGroupIdentifier>(&self, child_process_identifier: NonZeroI32, user_or_group_identifiers_map: &UserOrGroupIdentifierMap<U>) -> io::Result<()>
	{
		self.0.process_file_path(child_process_identifier, U::FileName).write_to_map_file(&file_path)
	}

	#[inline(always)]
	pub fn cgroup_namespace_inode(&self, process_identifier: NonZeroI32) -> Result<Inode, NamespaceInodeParseError>
	{
		self.namespace_inode(process_identifier, "cgroup")
	}

	#[inline(always)]
	pub fn cgroup_namespace_file_descriptor(&self, process_identifier: NonZeroI32) -> io::Result<RawFd>
	{
		self.namespace_file_descriptor(process_identifier, "cgroup")
	}

	#[inline(always)]
	pub fn inter_process_communication_namespace_inode(&self, process_identifier: NonZeroI32) -> Result<Inode, NamespaceInodeParseError>
	{
		self.namespace_inode(process_identifier, "ipc")
	}

	#[inline(always)]
	pub fn inter_process_communication_namespace_file_descriptor(&self, process_identifier: NonZeroI32) -> io::Result<RawFd>
	{
		self.namespace_file_descriptor(process_identifier, "ipc")
	}

	#[inline(always)]
	pub fn mount_namespace_inode(&self, process_identifier: NonZeroI32) -> Result<Inode, NamespaceInodeParseError>
	{
		self.namespace_inode(process_identifier, "mnt")
	}

	#[inline(always)]
	pub fn mount_namespace_file_descriptor(&self, process_identifier: NonZeroI32) -> io::Result<RawFd>
	{
		self.namespace_file_descriptor(process_identifier, "mnt")
	}

	#[inline(always)]
	pub fn net_namespace_inode(&self, process_identifier: NonZeroI32) -> Result<Inode, NamespaceInodeParseError>
	{
		self.namespace_inode(process_identifier, "net")
	}

	#[inline(always)]
	pub fn net_namespace_file_descriptor(&self, process_identifier: NonZeroI32) -> io::Result<RawFd>
	{
		self.namespace_file_descriptor(process_identifier, "net")
	}

	#[inline(always)]
	pub fn process_identifier_namespace_inode(&self, process_identifier: NonZeroI32) -> Result<Inode, NamespaceInodeParseError>
	{
		self.namespace_inode(process_identifier, "pid")
	}

	#[inline(always)]
	pub fn process_identifier_namespace_file_descriptor(&self, process_identifier: NonZeroI32) -> io::Result<RawFd>
	{
		self.namespace_file_descriptor(process_identifier, "pid")
	}

	#[inline(always)]
	pub fn process_identifier_for_children_namespace_inode(&self, process_identifier: NonZeroI32) -> Result<Inode, NamespaceInodeParseError>
	{
		self.namespace_inode(process_identifier, "pid_for_children")
	}

	#[inline(always)]
	pub fn process_identifier_for_children_namespace_file_descriptor(&self, process_identifier: NonZeroI32) -> io::Result<RawFd>
	{
		self.namespace_file_descriptor(process_identifier, "pid_for_children")
	}

	#[inline(always)]
	pub fn user_namespace_inode(&self, process_identifier: NonZeroI32) -> Result<Inode, NamespaceInodeParseError>
	{
		self.namespace_inode(process_identifier, "user")
	}

	#[inline(always)]
	pub fn user_namespace_file_descriptor(&self, process_identifier: NonZeroI32) -> io::Result<RawFd>
	{
		self.namespace_file_descriptor(process_identifier, "user")
	}

	#[inline(always)]
	pub fn uts_namespace_inode(&self, process_identifier: NonZeroI32) -> Result<Inode, NamespaceInodeParseError>
	{
		self.namespace_inode(process_identifier, "uts")
	}

	#[inline(always)]
	pub fn uts_namespace_file_descriptor(&self, process_identifier: NonZeroI32) -> io::Result<RawFd>
	{
		self.namespace_file_descriptor(process_identifier, "uts")
	}

	#[inline(always)]
	pub fn maximum_cgroup_namespaces(&self) -> io::Result<u64>
	{
		self.maximum_namespaces_file_path("max_cgroup_namespaces").read_value()
	}

	#[inline(always)]
	pub fn write_maximum_cgroup_namespaces(&self, maximum: u64) -> io::Result<()>
	{
		self.maximum_namespaces_file_path("max_cgroup_namespaces").write_value(maximum)
	}

	#[inline(always)]
	pub fn maximum_inter_process_communication_namespaces(&self) -> io::Result<u64>
	{
		self.maximum_namespaces_file_path("max_ipc_namespaces").read_value()
	}

	#[inline(always)]
	pub fn write_maximum_inter_process_communication_namespaces(&self, maximum: u64) -> io::Result<()>
	{
		self.maximum_namespaces_file_path("max_ipc_namespaces").write_value(maximum)
	}

	#[inline(always)]
	pub fn maximum_mount_namespaces(&self) -> io::Result<u64>
	{
		self.maximum_namespaces_file_path("max_mnt_namespaces").read_value()
	}

	#[inline(always)]
	pub fn write_maximum_mount_namespaces(&self, maximum: u64) -> io::Result<()>
	{
		self.maximum_namespaces_file_path("max_mnt_namespaces").write_value(maximum)
	}

	#[inline(always)]
	pub fn maximum_net_namespaces(&self) -> io::Result<u64>
	{
		self.maximum_namespaces_file_path("max_net_namespaces").read_value()
	}

	#[inline(always)]
	pub fn write_maximum_net_namespaces(&self, maximum: u64) -> io::Result<()>
	{
		self.maximum_namespaces_file_path("max_net_namespaces").write_value(maximum)
	}

	#[inline(always)]
	pub fn maximum_process_identifier_namespaces(&self) -> io::Result<u64>
	{
		self.maximum_namespaces_file_path("max_pid_namespaces").read_value()
	}

	#[inline(always)]
	pub fn write_maximum_process_identifier_namespaces(&self, maximum: u64) -> io::Result<()>
	{
		self.maximum_namespaces_file_path("max_pid_namespaces").write_value(maximum)
	}

	#[inline(always)]
	pub fn maximum_user_namespaces(&self) -> io::Result<u64>
	{
		self.maximum_namespaces_file_path("max_user_namespaces").read_value()
	}

	#[inline(always)]
	pub fn write_maximum_user_namespaces(&self, maximum: u64) -> io::Result<()>
	{
		self.maximum_namespaces_file_path("max_user_namespaces").write_value(maximum)
	}

	#[inline(always)]
	pub fn maximum_uts_namespaces(&self) -> io::Result<u64>
	{
		self.maximum_namespaces_file_path("max_uts_namespaces").read_value()
	}

	#[inline(always)]
	pub fn write_maximum_uts_namespaces(&self, maximum: u64) -> io::Result<()>
	{
		self.maximum_namespaces_file_path("max_uts_namespaces").write_value(maximum)
	}

	#[inline(always)]
	fn namespace_inode(&self, process_identifier: NonZeroI32, namespace: &str) -> Result<Inode, NamespaceInodeParseError>
	{
		use self::NamespaceInodeParseError::*;

		let file_path = self.namespace_file_path(process_identifier, namespace);

		// eg `cgroup:[4026531835]`.
		let link = match file_path.read_link()?.into_os_string().into_string()
		{
			Ok(link) => link,
			Err(_) => return Err(NamespaceLinkIsNotUtf8)
		};

		let namespace_length = namespace.len();
		let namespace_with_colon_open_square_bracket_length = namespace_length + 2;
		let namespace_with_colon_open_square_bracket_close_bracket_length = namespace_with_colon_open_square_bracket_length + 1;

		if unlikely!(!link.len() < namespace_with_colon_open_square_bracket_close_bracket_length)
		{
			return Err(NamespaceLinkIsTooShort)
		}
		if unlikely!(!link.starts_with(namespace))
		{
			return Err(NamespaceLinkDoesNotStartWithNamespace)
		}
		if unlikely!(&link[namespace_length .. namespace_with_colon_open_square_bracket_length] != ":[")
		{
			return Err(NamespaceLinkDoesNotStartWithNamespace)
		}
		if unlikely!(!link.ends_with(']'))
		{
			return Err(NamespaceLinkDoesNotEndWithSquareBracket)
		}

		let inode = &link[namespace_with_colon_open_square_bracket_length .. (namespace_length - 1)];
		let inode: u64 = inode.parse()?;
		Ok(Inode::from(inode))
	}

	#[inline(always)]
	fn namespace_file_descriptor(&self, process_identifier: NonZeroI32, namespace: &str) -> io::Result<RawFd>
	{
		Ok(File::open(self.namespace_file_path(process_identifier, namespace))?.into_raw_fd())
	}

	#[inline(always)]
	fn maximum_namespaces_file_path(&self, name: &str) -> PathBuf
	{
		self.0.sys_user_file_path(name)
	}

	#[inline(always)]
	fn namespace_file_path(&self, process_identifier: NonZeroI32, namespace: &str) -> PathBuf
	{
		self.0.process_file_path(process_identifier, "ns").append(namespace)
	}
}
