// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Number of file handles.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct NumberOfFileHandles(pub usize);

impl From<usize> for NumberOfFileHandles
{
	#[inline(always)]
	fn from(value: usize) -> Self
	{
		Self(value)
	}
}

impl Into<usize> for NumberOfFileHandles
{
	#[inline(always)]
	fn into(self) -> usize
	{
		self.0
	}
}

impl NumberOfFileHandles
{
	/// Allocated, free and maximum as reported by `/proc/sys/fs/file-nr`.
	///
	/// The maximum should always be the same as `Self::maximum()`.
	///
	/// Allocated + free should equal maximum.
	#[inline(always)]
	pub fn allocated_free_and_maximum(proc_path: &ProcPath) -> io::Result<AllocatedFreeAndMaximumNumberOfFileHandles>
	{
		let bytes = proc_path.sys_fs_file_path("file-nr").read_raw_without_line_feed()?;
		let mut fields = bytes.split_bytes_n(3, b'\t');

		#[inline(always)]
		fn next<'a>(fields: &mut impl Iterator<Item=&'a [u8]>, field_name: &'static str) -> io::Result<NumberOfFileHandles>
		{
			let next = fields.next().ok_or(io_error_invalid_data(format!("Missing field {}", field_name)))?;
			let parsed = usize::parse_decimal_number(next).map_err(io_error_invalid_data)?;
			Ok(NumberOfFileHandles(parsed))
		}
		
		Ok
		(
			AllocatedFreeAndMaximumNumberOfFileHandles
			{
				allocated: next(&mut fields, "allocated")?,
				free: next(&mut fields, "free")?,
				maximum: next(&mut fields, "maximum")?,
			}
		)
	}

	/// Maximum as reported by `/proc/sys/fs/file-max`.
	///
	/// Default varies; might be 99,642 on a system with 1Gb.
	#[inline(always)]
	pub fn maximum(proc_path: &ProcPath) -> io::Result<Self>
	{
		let value: usize = Self::file_max_file_path(proc_path).read_value()?;
		Ok(Self(value))
	}

	/// Set maximum in `/proc/sys/fs/file-max`.
	#[inline(always)]
	pub fn set_maximum(&self, proc_path: &ProcPath) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write /proc/sys/fs/file-max");
		let file_path = Self::file_max_file_path(proc_path);
		if file_path.exists()
		{
			file_path.write_value(UnpaddedDecimalInteger(self.0))
		}
		else
		{
			Ok(())
		}
	}

	#[inline(always)]
	fn file_max_file_path(proc_path: &ProcPath) -> PathBuf
	{
		proc_path.sys_fs_file_path("file-max")
	}
}
