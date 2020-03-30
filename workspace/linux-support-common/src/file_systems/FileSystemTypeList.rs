// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// List of supported file systems.
#[derive(Default, Debug)]
pub struct FileSystemTypeList(HashMap<FileSystemType, HasNoAssociatedDevice>);

impl FileSystemTypeList
{
	/// Verifies a file system is supported.
	#[inline(always)]
	pub fn verify_file_system_is_supported(&self, file_system_type: FileSystemType) -> Result<(), FileSystemSupportedError>
	{
		use self::FileSystemSupportedError::*;

		match self.0.get(&file_system_type)
		{
			None => Err(Unsupported(file_system_type)),

			Some(has_no_associated_device) => if *has_no_associated_device
			{
				Ok(())
			}
			else
			{
				Err(HasAssociatedDevices(file_system_type))
			},
		}
	}
	/// File systems (from `/proc/filesystems`).
	#[inline(always)]
	pub fn parse(proc_path: &ProcPath) -> Result<FileSystemTypeList, io::Error>
	{
		let file_path = proc_path.file_path("filesystems");
		use self::ErrorKind::InvalidData;

		let reader = BufReader::with_capacity(4096, File::open(file_path)?);

		let mut file_systems_map = HashMap::new();
		let mut line_number = 0;

		for line in reader.split(b'\n')
		{
			{
				let line = line?;
				let mut split = line.splitn(2, |byte| *byte == b'\t');

				let has_no_associated_device = match split.next().unwrap()
				{
					b"" => false,
					b"nodev" => true,

					unrecognised @ _ => return Err(io::Error::new(InvalidData, format!("Zero-based line number '{}' has a first column value of '{:?}' which isn't recognised", line_number, unrecognised))),
				};

				let file_system_type = match split.next()
				{
					None => return Err(io::Error::new(InvalidData, format!("Zero-based line number '{}' does not have second column", line_number))),
					Some(value) => FileSystemType::from_byte_slice(value),
				};

				if let Some(_) = file_systems_map.insert(file_system_type, has_no_associated_device)
				{
					return Err(io::Error::new(InvalidData, format!("Zero-based line number '{}' is a duplicate", line_number)));
				}
			}

			line_number += 1;
		}

		Ok(FileSystemTypeList(file_systems_map))
	}
}
