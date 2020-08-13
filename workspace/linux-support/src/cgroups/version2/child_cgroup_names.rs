// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Child cgroup names.
fn child_cgroup_names(folder_path: &Path) -> io::Result<impl Iterator<Item=CgroupName>>
{
	fn filter_and_map(result: io::Result<DirEntry>) -> Option<CgroupName>
	{
		match result
		{
			Ok(dir_entry) => match dir_entry.metadata()
			{
				Ok(metadata) => if metadata.file_type().is_dir()
				{
					let path = dir_entry.path();
					let file_name = path.file_name().unwrap();
					let name = CgroupName::try_from(file_name.to_os_string()).unwrap();
					Some(name)
				}
				else
				{
					None
				},
				
				Err(_) => None,
			},
			
			Err(_) => None,
		}
	}
	
	let read_directory = folder_path.read_dir()?;
	Ok(read_directory.filter_map(filter_and_map))
}
