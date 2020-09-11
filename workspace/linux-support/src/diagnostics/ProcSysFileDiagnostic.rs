// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProcSysFileDiagnostic(ByteBuf);

impl ProcSysFileDiagnostic
{
	fn gather(proc_path: &ProcPath) -> BTreeMap<PathBuf, Self>
	{
		let mut all = BTreeMap::new();
		let root_folder_path = proc_path.sys_folder_path();
		let mut folder_paths = vec![root_folder_path];
		
		while let Some(folder_path) = folder_paths.pop()
		{
			if let Ok(read_dir) = (&folder_path).read_dir()
			{
				for dir_entry in read_dir
				{
					if let Ok(dir_entry) = dir_entry
					{
						if let Ok(metadata) = dir_entry.metadata()
						{
							if metadata.is_dir()
							{
								folder_paths.push(dir_entry.path())
							}
							else if metadata.is_file()
							{
								let file_path = dir_entry.path();
								if let Ok(mut file) = File::open(&file_path)
								{
									let mut buffer = Vec::with_capacity(64);
									if let Ok(_number_of_bytes_read) = file.read_to_end(&mut buffer)
									{
										buffer.shrink_to_fit();
										all.insert(file_path, Self(ByteBuf::from(buffer)));
									}
								}
							}
						}
					}
				}
			}
		}
		
		all
	}
}
