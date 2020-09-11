// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FileSystemsDiagnostics
{
	pub file_systems: DiagnosticUnobtainableResult<FileSystemTypeList>,
	
	pub mounts: DiagnosticUnobtainableResult<HashMap<PathBuf, MountDiagnostic>>,
}

impl FileSystemsDiagnostics
{
	#[inline(always)]
	fn gather(proc_path: &ProcPath, process_identifier: ProcessIdentifierChoice) -> Self
	{
		Self
		{
			file_systems: FileSystemTypeList::parse(proc_path).map_err(DiagnosticUnobtainable::from),
			
			mounts: match Mounts::parse(proc_path, process_identifier)
			{
				Err(error) => Err(DiagnosticUnobtainable::from(error)),
				Ok(Mounts(mounts)) =>
				{
					let mut mount_diagnostics = HashMap::with_capacity(mounts.len());
					for (mount_point, mount) in mounts.into_iter()
					{
						mount_diagnostics.insert(mount_point, MountDiagnostic::gather(mount));
					}
					Ok(mount_diagnostics)
				}
			},
		}
	}
	
	#[inline(always)]
	fn bpf_mount_path(&self) -> Option<&Path>
	{
		self.first_known_pseudo_mount_path(FileSystemType::bpf)
	}
	
	#[inline(always)]
	fn cgroup_version2_mount_path(&self) -> Option<&Path>
	{
		self.first_known_pseudo_mount_path(FileSystemType::cgroup2)
	}
	
	#[inline(always)]
	fn first_known_pseudo_mount_path(&self, file_system_type: FileSystemType) -> Option<&Path>
	{
		if self.file_systems.as_ref().map(|list| list.verify_pseudo_file_system_is_supported(&file_system_type)).is_err()
		{
			return None
		}
		
		match self.mounts
		{
			Err(_) => None,
			
			Ok(ref mounts) =>
			{
				for (mount_path, mount_diagnostic) in mounts.iter()
				{
					if mount_diagnostic.file_system_type == file_system_type
					{
						return Some(mount_path.as_path())
					}
				}
				None
			}
		}
	}
}
