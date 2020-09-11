// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A RDMA configuration or data file.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct RdmaFile(HashMap<RdmaDeviceName, RdmaDeviceValues>);

impl Deref for RdmaFile
{
	type Target = HashMap<RdmaDeviceName, RdmaDeviceValues>;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl DerefMut for RdmaFile
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.0
	}
}

impl RdmaFile
{
	pub(crate) fn from_file(file_path: &Path) -> Result<Self, RdmaParseError>
	{
		use self::RdmaParseError::*;
		
		let reader = file_path.read_raw()?;
		
		let mut this = Self::default();
		
		for line in reader.split_bytes(b'\n')
		{
			let mut fields = line.split_bytes_n(2, b' ');
			let device_name = RdmaDeviceName(fields.next().expect("Split always should produce at least one item").to_vec());
			
			let key_value_fields = fields.next().ok_or(MissingKeyValueFields { device_name: device_name.clone() })?;
			let device_values = RdmaDeviceValues::from_bytes(key_value_fields)?;
			
			let was_a_duplicate = this.insert(device_name.clone(), device_values).is_some();
			if unlikely!(was_a_duplicate)
			{
				return Err(DuplicateDevice { device_name })
			}
		}
		
		Ok(this)
	}
	
	pub(crate) fn to_file(&self, file_path: &Path) -> io::Result<()>
	{
		#[inline(always)]
		fn write_maximum_number_to_bytes(maximum_number: MaximumNumber<i32>, writer: &mut impl Write) -> io::Result<()>
		{
			let bytes_with_line_feed = maximum_number.into_line_feed_terminated_byte_string();
			let bytes = &bytes_with_line_feed[.. bytes_with_line_feed.len() - 1];
			writer.write_all(bytes)
		}
		
		let mut writer = BufWriter::new(OpenOptions::new().write(true).open(file_path)?);
		
		for (device_name, device_values) in self.iter()
		{
			writer.write_all(&device_name[..])?;
			
			writer.write_all(b" hca_handle=")?;
			write_maximum_number_to_bytes(device_values.handles, &mut writer)?;
			
			writer.write_all(b" hca_object=")?;
			write_maximum_number_to_bytes(device_values.objects, &mut writer)?;
			
			writer.write_all(b"\n")?;
		}
		
		writer.flush()?;
		
		Ok(())
	}
}
