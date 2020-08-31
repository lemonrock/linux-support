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
}
