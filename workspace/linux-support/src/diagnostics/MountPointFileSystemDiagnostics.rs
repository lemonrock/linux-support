// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MountPointFileSystemDiagnostics
{
	/// Only supported for `btrfs`, `f2fs` and `xfs`.
	///
	/// Errors shouldn't be treated too harshly by the caller.
	/// Like many such Linux APIs, everything is poorly documented and file systems pretty much interpret what's wanted as they wish.
	///
	/// Since Linux 4.18.
	pub file_system_label: DiagnosticUnobtainableResult<CString>,
	
	/// `vstatfs`.
	pub metadata: DiagnosticUnobtainableResult<FileSystemMetadataDiagnostics>,
}

impl MountPointFileSystemDiagnostics
{
	fn gather(mount_point: &Path) -> DiagnosticUnobtainableResult<Self>
	{
		let mount_point_file_descriptor = DirectoryFileDescriptor::new(mount_point.to_c_string().as_c_str()).map_err(DiagnosticUnobtainable::from)?;
		Ok
		(
			Self
			{
				file_system_label: mount_point_file_descriptor.get_file_system_label().map_err(DiagnosticUnobtainable::from),
				metadata: FileSystemMetadataDiagnostics::gather(&mount_point_file_descriptor),
			}
		)
	}
}
