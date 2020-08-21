// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Swap line.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SwapLine
{
	/// File path.
	pub file_path: PathBuf,
	
	/// Swap type.
	pub swap_type: SwapType,
	
	/// Size in bytes.
	pub size: usize,
	
	/// Used in bytes.
	pub used: usize,
	
	/// Priority (can be negative eg `-2`).
	pub priority: i32,
}

impl FromBytes for SwapLine
{
	type Error = io::Error;
	
	fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
	{
		#[inline(always)]
		fn file_path_field_and_remaining_fields(bytes: &[u8]) -> io::Result<(&[u8], &[u8])>
		{
			let index = memchr(b' ', bytes).ok_or(io::Error::new(ErrorKind::InvalidData, "Path field not terminated by spaces"))?;
			let file_path_field = &bytes[.. index];
			
			let last_index_of_file_path_field = memrchr(b' ', bytes).unwrap();
			let start_of_tab_separated_fields = last_index_of_file_path_field + 1;
			let remaining_fields = &bytes[start_of_tab_separated_fields .. ];
			
			Ok((file_path_field, remaining_fields))
		}
		
		let (file_path, remaining_fields) = file_path_field_and_remaining_fields(bytes)?;
		
		// There is inconsistent tab-separation between fields; if the `Type` field is `file`, an additional tab is also inserted.
		let mut fields = remaining_fields.split_bytes(b'\t').filter(|potential_field| !potential_field.is_empty());
		
		let priority = fields.next().ok_or(io::Error::new(ErrorKind::InvalidData, "No `Priority` field"))?;
		
		use self::SwapType::*;
		
		Ok
		(
			Self
			{
				file_path: PathBuf::from(OsString::from_vec(file_path.to_vec())),
				swap_type:
				{
					match fields.next().ok_or(io::Error::new(ErrorKind::InvalidData, "No `Type` field"))?
					{
						b"partition" => Partition,
						b"file" => File,
						_ => return Err(io::Error::new(ErrorKind::InvalidData, "Invalid `Type` field"))
					}
				},
				size:
				{
					let size = fields.next().ok_or(io::Error::new(ErrorKind::InvalidData, "No `Size` field"))?;
					usize::from_bytes(size).map_err(|error| io::Error::new(ErrorKind::InvalidData, error))?
				},
				used:
				{
					let used = fields.next().ok_or(io::Error::new(ErrorKind::InvalidData, "No `Used` field"))?;
					usize::from_bytes(used).map_err(|error| io::Error::new(ErrorKind::InvalidData, error))?
				},
				priority:
				{
					let used = fields.next().ok_or(io::Error::new(ErrorKind::InvalidData, "No `Priority` field"))?;
					i32::from_bytes(used).map_err(|error| io::Error::new(ErrorKind::InvalidData, error))?
				},
			}
		)
	}
}

impl SwapLine
{
	#[inline(always)]
	pub fn swap_off(&self) -> io::Result<()>
	{
		let nul_terminated_c_string = self.file_path.to_c_string();
		swap_off(nul_terminated_c_string.as_bytes_with_nul())
	}
}
