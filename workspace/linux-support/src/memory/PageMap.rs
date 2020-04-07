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
		Self::read_process_pagemap(proc_path, ProcessIdentifierChoice::Current, have_virtual_addresses, page_map_entry_user)
	}

	/// Read a process' pagemap file.
	#[inline(always)]
	pub fn read_process_pagemap<HVA: HasVirtualAddress>(proc_path: &ProcPath, process_identifier: ProcessIdentifierChoice, have_virtual_addresses: impl Iterator<Item=HVA>, page_map_entry_user: impl FnMut(HVA, VirtualAddress, PageMapEntry)) -> io::Result<()>
	{
		Self::read_pagemap_file(have_virtual_addresses, page_map_entry_user, &proc_path.process_file_path(process_identifier, "pagemap"))
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


/*

/proc/pid/numa_map
55c0f5fd4000 default file=/bin/busybox mapped=12 mapmax=10 N0=12 kernelpagesize_kB=4
55c0f5fe0000 default file=/bin/busybox mapped=90 mapmax=14 N0=90 kernelpagesize_kB=4
55c0f607c000 default file=/bin/busybox mapped=33 mapmax=14 N0=33 kernelpagesize_kB=4
55c0f609e000 default file=/bin/busybox anon=4 dirty=4 N0=4 kernelpagesize_kB=4
55c0f60a2000 default file=/bin/busybox anon=1 dirty=1 N0=1 kernelpagesize_kB=4
55c0f6864000 default heap anon=35 dirty=35 N0=35 kernelpagesize_kB=4
7f0951b20000 default file=/lib/ld-musl-x86_64.so.1 mapped=20 mapmax=11 N0=20 kernelpagesize_kB=4
7f0951b35000 default file=/lib/ld-musl-x86_64.so.1 mapped=60 mapmax=18 N0=60 kernelpagesize_kB=4
7f0951b7c000 default file=/lib/ld-musl-x86_64.so.1 mapped=32 mapmax=18 N0=32 kernelpagesize_kB=4
7f0951bb1000 default file=/lib/ld-musl-x86_64.so.1 anon=1 dirty=1 N0=1 kernelpagesize_kB=4
7f0951bb2000 default file=/lib/ld-musl-x86_64.so.1 anon=1 dirty=1 N0=1 kernelpagesize_kB=4
7f0951bb3000 default anon=3 dirty=3 N0=3 kernelpagesize_kB=4
7ffc4c759000 default stack anon=17 dirty=17 N0=17 kernelpagesize_kB=4
7ffc4c796000 default
7ffc4c799000 default

// The /proc/[pid]/pagemap file is present only if the CON‐
//              FIG_PROC_PAGE_MONITOR kernel configuration option is enabled.
//
//              Permission to access this file is governed by a ptrace access
//              mode PTRACE_MODE_READ_FSCREDS check; see ptrace(2).
/proc/pid/pagemap


*/
