// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A resource entry.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResourceEntry
{
	/// Inclusive start.
	pub inclusive_start: NonNull<u8>,

	/// Inclusive end.
	pub inclusive_end: NonNull<u8>,

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
	pub(crate) fn memory_map<'a>(pci_device: &PciDevice<'a>, resource_index: u8) -> Result<MemoryMappedResource, io::Error>
	{
		pci_device.device_file_or_folder_path(&format!("resource{:?}", resource_index)).memory_map().map(|memory_mapped_file| MemoryMappedResource(memory_mapped_file))
	}

	/// A typical line might be `0x0000000000008200 0x000000000000823f 0x0000000000040101`.
	fn parse_line(line: &[u8]) -> Result<Option<Self>, &'static str>
	{
		#[inline(always)]
		fn parse_u64_hexadecimal_value<'a>(iterator: &mut impl Iterator<Item=&'a [u8]>) -> Result<u64, &'static str>
		{
			let next = iterator.next().ok_or("Missing expected field")?;
			u64::parse_hexadecimal_number_lower_case_with_0x_prefix_fixed_width(next, size_of::<u64>()).map_err(|_| "Invalid hexadecimal string")
		}

		let mut iterator = line.splitn(3, |byte | *byte == b' ');
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
					inclusive_start: unsafe { NonNull::new_unchecked(start as *mut u8) },
					inclusive_end: unsafe { NonNull::new_unchecked(end as *mut u8) },
					flags,
				}
			)
		)
	}
}
