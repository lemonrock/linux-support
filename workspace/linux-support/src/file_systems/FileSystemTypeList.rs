// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// List of supported file systems.
#[derive(Default, Debug)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct FileSystemTypeList(HashMap<FileSystemType, HasNoAssociatedDevice>);

impl FileSystemTypeList
{
	/// Verifies a nodev file system is supported, eg `sysfs` will be `Ok()`; `ext3` will be `Err(HasAssociatedDevices)`.
	#[inline(always)]
	pub fn verify_pseudo_file_system_is_supported(&self, file_system_type: &FileSystemType) -> Result<(), FileSystemSupportedError>
	{
		use self::FileSystemSupportedError::*;

		match self.0.get(&file_system_type)
		{
			None => Err(Unsupported(file_system_type.clone())),

			Some(has_no_associated_device) => if *has_no_associated_device
			{
				Ok(())
			}
			else
			{
				Err(HasAssociatedDevices(file_system_type.clone()))
			},
		}
	}
	/// File systems (from `/proc/filesystems`).
	#[inline(always)]
	pub fn parse(proc_path: &ProcPath) -> io::Result<FileSystemTypeList>
	{
		let file_path = proc_path.file_path("filesystems");
		let reader = file_path.read_raw()?;

		let mut file_systems_map = HashMap::new();
		let mut line_number = 0;

		for line in reader.split_bytes(b'\n')
		{
			{
				let mut split = line.split_bytes_n(2, b'\t');

				let has_no_associated_device = match split.next().unwrap()
				{
					b"" => false,
					b"nodev" => true,

					unrecognised @ _ => return Err(io_error_invalid_data(format!("Zero-based line number '{}' has a first column value of '{:?}' which isn't recognised", line_number, unrecognised))),
				};

				let file_system_type = match split.next()
				{
					None => return Err(io_error_invalid_data(format!("Zero-based line number '{}' does not have second column", line_number))),
					Some(value) => FileSystemType::from_byte_slice(value),
				};

				if let Some(_) = file_systems_map.insert(file_system_type, has_no_associated_device)
				{
					return Err(io_error_invalid_data(format!("Zero-based line number '{}' is a duplicate", line_number)));
				}
			}

			line_number += 1;
		}

		Ok(FileSystemTypeList(file_systems_map))
	}
}
