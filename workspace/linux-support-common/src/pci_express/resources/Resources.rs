// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Resources.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Resources<'a>
{
	pci_device: &'a PciDevice<'a>,
	resources: BTreeMap<u8, ResourceEntry>,
}

impl<'a> Deref for Resources<'a>
{
	type Target = BTreeMap<u8, ResourceEntry>;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.resources
	}
}

impl<'a> Resources<'a>
{
	/// Memory map resource.
	#[inline(always)]
	pub fn memory_map(&self, resource_index: u8) -> Result<Option<MemoryMappedResource>, io::Error>
	{
		if !self.resources.contains_key(&resource_index)
		{
			return Ok(None)
		}

		ResourceEntry::memory_map(self.pci_device, resource_index).map(|resource| Some(resource))
	}

	/*
		A typical file might be
		0x0000000000008200 0x000000000000823f 0x0000000000040101
		0x00000000ee000000 0x00000000ee000fff 0x0000000000040200
		0x0000000000000000 0x0000000000000000 0x0000000000000000
		0x0000000000000000 0x0000000000000000 0x0000000000000000
		0x0000000000000000 0x0000000000000000 0x0000000000000000
		0x0000000000000000 0x0000000000000000 0x0000000000000000
		0x0000000000000000 0x0000000000000000 0x0000000000000000
		0x0000000000000000 0x0000000000000000 0x0000000000000000
		0x0000000000000000 0x0000000000000000 0x0000000000000000
		0x0000000000000000 0x0000000000000000 0x0000000000000000
		0x0000000000000000 0x0000000000000000 0x0000000000000000
		0x0000000000000000 0x0000000000000000 0x0000000000000000
		0x0000000000000000 0x0000000000000000 0x0000000000000000
	*/
	pub(crate) fn parse_lines(pci_device: &'a PciDevice<'a>) -> io::Result<Self>
	{
		let path = pci_device.device_file_or_folder_path("resource");
		let file = File::open(path)?;

		let mut resources = BTreeMap::default();
		let mut index = 0;
		// `split()` preferred over `lines()` as the later also converts CRLF, which wouldn't be a valid sequence in this file.
		for line in BufReader::new(file).split(b'\n')
		{
			let vec = line?;
			let line_string = from_utf8(vec.as_slice()).map_err(|error| io::Error::new(ErrorKind::Other, error))?;
			if let Some(resource_entry) = ResourceEntry::parse_line(line_string).map_err(|message| io::Error::new(ErrorKind::Other, message))?
			{
				resources.insert(index, resource_entry);
			}
			index += 1;
		}
		Ok
		(
			Self
			{
				pci_device,
				resources,
			}
		)
	}
}
