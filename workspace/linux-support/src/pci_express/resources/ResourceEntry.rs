// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A resource entry.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResourceEntry
{
	/// Inclusive start.
	#[serde(deserialize_with = "ResourceEntry::non_null_deserialize", serialize_with = "ResourceEntry::non_null_serialize")] pub inclusive_start: NonNull<u8>,

	/// Inclusive end.
	#[serde(deserialize_with = "ResourceEntry::non_null_deserialize", serialize_with = "ResourceEntry::non_null_serialize")] pub inclusive_end: NonNull<u8>,

	/// Flags.
	pub flags: u64,
}

impl ResourceEntry
{
	/// Length.
	#[inline(always)]
	pub fn length(&self) -> usize
	{
		self.inclusive_end.as_ptr() as usize + 1 - self.inclusive_start.as_ptr() as usize
	}

	#[inline(always)]
	pub(crate) fn memory_map<'a>(pci_device: &PciDevice<'a>, resource_index: u8, default_page_size: PageSize) -> Result<MemoryMappedResource, io::Error>
	{
		let page_size_or_huge_page_size_settings = PageSizeOrHugePageSizeSettings::for_default_page_size(default_page_size);
		pci_device.device_file_or_folder_path(&format!("resource{:?}", resource_index)).memory_map_read_write(0, AddressHint::any(), Sharing::Private, false, false, &page_size_or_huge_page_size_settings).map(|memory_mapped_file| MemoryMappedResource(memory_mapped_file))
	}

	/// A typical line might be `0x0000000000008200 0x000000000000823f 0x0000000000040101`.
	fn parse_line(line: &[u8]) -> Result<Option<Self>, &'static str>
	{
		#[inline(always)]
		fn parse_u64_hexadecimal_value<'a>(iterator: &mut impl Iterator<Item=&'a [u8]>) -> Result<u64, &'static str>
		{
			let next = iterator.next().ok_or("Missing expected field")?;
			u64::parse_hexadecimal_number_lower_case_with_0x_prefix_fixed_width(next, size_of::<u64>() * 2).map_err(|_| "Invalid hexadecimal string")
		}

		let mut iterator = line.split_bytes_n(3, b' ');
		let start = parse_u64_hexadecimal_value(&mut iterator)?;
		let end = parse_u64_hexadecimal_value(&mut iterator)?;
		let flags = parse_u64_hexadecimal_value(&mut iterator)?;

		if start == 0 && end == 0 && flags == 0
		{
			return Ok(None)
		}

		if unlikely!(start == 0)
		{
			return Err("start is zero");
		}

		if unlikely!(start >= end)
		{
			return Err("start is greater than or equal to end");
		}

		Ok
		(
			Some
			(
				Self
				{
					inclusive_start: new_non_null(start as *mut u8),
					inclusive_end: new_non_null(end as *mut u8),
					flags,
				}
			)
		)
	}
	
	#[inline(always)]
	fn non_null_deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<NonNull<u8>, D::Error>
	{
		let value = usize::deserialize(deserializer)?;
		if value == 0
		{
			use serde::de::Error as SerdeDeserializeError;
			Err(<D::Error as SerdeDeserializeError>::custom("Not NonNull"))
		}
		else
		{
			Ok(new_non_null(value as *mut u8))
		}
	}
	
	#[inline(always)]
	fn non_null_serialize<S: Serializer>(t: &NonNull<u8>, serializer: S) -> Result<S::Ok, S::Error>
	{
		 (t.as_ptr() as usize).serialize(serializer)
	}
}
