// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MountDiagnostic
{
	/// eg eg `/dev/sda1`, `proc`, etc; not really that useful.
	pub source: CString,
	
	/// File system type.
	///
	/// eg `proc`, `sysfs`, `hugetlbs`, `ext4`; listed in second column of `/proc/filesystems`.
	pub file_system_type: FileSystemType,
	
	/// Mount options.
	///
	/// eg `nodev mode=0177`
	pub mount_options: HashMap<Box<[u8]>, Option<Box<[u8]>>>,
	
	/// Typically `0` (zero).
	pub dump_frequency_in_days: i32,
	
	/// Typically `0` (zero).
	pub pass_number_on_parallel_filesystem_type: i32,
	
	pub mount_point_file_system_diagnostics: DiagnosticUnobtainableResult<MountPointFileSystemDiagnostics>,
}

impl MountDiagnostic
{
	#[inline(always)]
	fn gather(mount: Mount<'static>) -> Self
	{
		Self
		{
			source: mount.source,
			file_system_type: mount.file_system_type,
			mount_options:
			{
				let from_mount_options = mount.mount_options;
				let mut mount_options = HashMap::with_capacity(from_mount_options.len())
				for (name, option) in from_mount_options
				{
					let name = name.into_owned().into_boxed_slice();
					let option = option.map(|value| value.into_owned().into_boxed_slice());
					mount_options.insert(name, option)
				}
				mount_options
			}
			dump_frequency_in_days: mount.dump_frequency_in_days,
			pass_number_on_parallel_filesystem_type: mount.pass_number_on_parallel_filesystem_type,
			mount_point_file_system_diagnostics: MountPointFileSystemDiagnostics::gather(&mount.mount_point),
		}
	}
}
