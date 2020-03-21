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

	/// Parse the file system types.
	pub fn parse(file_path: &Path) -> Result<Self, io::Error>
	{
		use self::ErrorKind::InvalidData;

		let reader = BufReader::with_capacity(4096, File::open(file_path)?);

		let mut file_systems_map = HashMap::new();
		let mut line_number = 0;

		for line in reader.split(b'\n')
		{
			{
				let line = line?;
				let mut split = splitn(&line, 2, b'\t');

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
