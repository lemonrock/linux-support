// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Semaphores configuration.
#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct SemaphoresConfiguration
{
	/// `SEMMSL`.
	pub maximum_number_of_semaphores_per_array: u32,
	
	/// `SEMMNS`.
	pub maximum_number_of_semaphores: u32,
	
	/// `SEMOPM`.
	pub maximum_operations_per_semop_call: u32,
	
	/// `SEMMNI`.
	pub maximum_number_of_arrays: u32,
}

impl FromBytes for SemaphoresConfiguration
{
	type Error = ParseNumberError;

	#[inline(always)]
	fn from_bytes(value: &[u8]) -> Result<Self, Self::Error>
	{
		let mut iterator = value.split_bytes_n(4, b'\t');
		
		#[inline(always)]
		fn parse_number<'a>(iterator: &mut impl Iterator<Item=&'a [u8]>) -> Result<u32, ParseNumberError>
		{
			u32::parse_decimal_number(iterator.next().ok_or(ParseNumberError::TooShort)?)
		}
		
		Ok
		(
			Self
			{
				maximum_number_of_semaphores_per_array: parse_number(&mut iterator)?,
				maximum_number_of_semaphores: parse_number(&mut iterator)?,
				maximum_operations_per_semop_call: parse_number(&mut iterator)?,
				maximum_number_of_arrays: parse_number(&mut iterator)?,
			}
		)
	}
}
impl SemaphoresConfiguration
{
	/// Read from `/proc/sys/kernel/sem`.
	#[inline(always)]
	pub fn read(proc_path: &ProcPath) -> io::Result<Self>
	{
		Self::file_path(proc_path).read_value()
	}
	
	/// Read from `/proc/sys/kernel/sem`.
	#[inline(always)]
	pub fn write(&self, proc_path: &ProcPath) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write to `/proc/sys/kernel/sem`");
		
		let file_path = Self::file_path(proc_path);
		if file_path.exists()
		{
			file_path.write_value(self.to_string())
		}
		else
		{
			Ok(())
		}
	}
	
	#[inline(always)]
	fn to_string(&self) -> String
	{
		format!("{}\t{}\t{}\t{}\n", self.maximum_number_of_semaphores_per_array, self.maximum_number_of_semaphores, self.maximum_operations_per_semop_call, self.maximum_number_of_arrays)
	}
	
	#[inline(always)]
	fn file_path(proc_path: &ProcPath) -> PathBuf
	{
		proc_path.sys_kernel_file_path("sem")
	}
}
