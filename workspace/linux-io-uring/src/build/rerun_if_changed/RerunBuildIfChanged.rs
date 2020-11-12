// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub struct RerunBuildIfChanged
{
	build_folder_path: PathBuf,
}

impl RerunBuildIfChanged
{
	const PathPrefix: &'static str = "src/build";
	
	pub fn new(manifest_dir: &OsString) -> Self
	{
		Self
		{
			build_folder_path: PathBuf::from(manifest_dir.to_owned()).join(Self::PathPrefix),
		}
	}
	
	pub fn process(self) -> io::Result<()>
	{
		self.process_directory(&self.build_folder_path)
	}
	
	fn process_directory(&self, directory: &Path) -> io::Result<()>
	{
		self.cargo_rerun_if_build_changed(&directory);
		for entry in read_dir(directory)?
		{
			let entry = entry?;
			let rerun_item_path = entry.path();
			
			let metadata = rerun_item_path.symlink_metadata()?;
			if metadata.file_type().is_dir()
			{
				self.process_directory(&rerun_item_path)?;
			}
			else
			{
				self.cargo_rerun_if_build_changed(&rerun_item_path);
			}
		}
		
		Ok(())
	}
	
	fn cargo_rerun_if_build_changed(&self, rerun_item_path: &Path)
	{
		let relative_path_below_build = rerun_item_path.strip_prefix(&self.build_folder_path).unwrap();
		
		let (separator, path_below_src_build) = if relative_path_below_build.as_os_str().is_empty()
		{
			("", "")
		}
		else
		{
			("/", relative_path_below_build.to_str().expect("Relative path is valid but not representable in UTF-8; fix our source code so this isn't the case"))
		};
		
		println!("cargo:rerun-if-changed={}{}{}", Self::PathPrefix, separator, path_below_src_build)
	}
}
