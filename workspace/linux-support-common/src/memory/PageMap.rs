// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A convenience.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct PageMap;

impl PageMap
{
	/// Read the current process' pagemap file.
	#[inline(always)]
	pub fn read_our_pagemap<HVA: HasVirtualAddress>(proc_path: &ProcPath, have_virtual_addresses: impl Iterator<Item=HVA>, page_map_entry_user: impl FnMut(HVA, VirtualAddress, PageMapEntry)) -> io::Result<()>
	{
		Self::read_process_pagemap(proc_path, 0, have_virtual_addresses, page_map_entry_user)
	}

	/// Read a process' pagemap file.
	#[inline(always)]
	pub fn read_process_pagemap<HVA: HasVirtualAddress>(proc_path: &ProcPath, process: pid_t, have_virtual_addresses: impl Iterator<Item=HVA>, page_map_entry_user: impl FnMut(HVA, VirtualAddress, PageMapEntry)) -> io::Result<()>
	{
		Self::read_pagemap_file(have_virtual_addresses, page_map_entry_user, &proc_path.process_file_path(process, "pagemap"))
	}

	#[inline(always)]
	fn read_pagemap_file<HVA: HasVirtualAddress>(have_virtual_addresses: impl Iterator<Item=HVA>, mut page_map_entry_user: impl FnMut(HVA, VirtualAddress, PageMapEntry), page_map_file_path: impl AsRef<Path>) -> io::Result<()>
	{
		let mut file = File::open(page_map_file_path)?;

		for has_virtual_address in have_virtual_addresses
		{
			let virtual_address = has_virtual_address.virtual_address();
			let page_map_entry = PageMapEntry::read_from_pagemap_file(&mut file, virtual_address)?;
			page_map_entry_user(has_virtual_address, virtual_address, page_map_entry)
		}

		Ok(())
	}
}
