// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A memory map entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryMapEntry
{
	/// Memory range.
	memory_range: Range<VirtualAddress>,

	/// Protection.
	pub protection: Protection,

	/// Sharing (can not differentiate persistent shares, sadly).
	pub sharing: Sharing,

	/// Kind of mapping.
	pub kind: MemoryMapEntryKind,

	/// Only `Some` if the kernel has been built with `CONFIG_NUMA`.
	pub numa_details: Option<MemoryMapEntryNumaDetails>
}

impl MemoryMapEntry
{
	/*
		Example of /proc/pid/maps:-
		55c0f5fd4000-55c0f5fe0000 r--p 00000000 08:03 1048666                    /bin/busybox
		55c0f5fe0000-55c0f607c000 r-xp 0000c000 08:03 1048666                    /bin/busybox
		55c0f607c000-55c0f609d000 r--p 000a8000 08:03 1048666                    /bin/busybox
		55c0f609e000-55c0f60a2000 r--p 000c9000 08:03 1048666                    /bin/busybox
		55c0f60a2000-55c0f60a3000 rw-p 000cd000 08:03 1048666                    /bin/busybox
		55c0f6864000-55c0f6887000 rw-p 00000000 00:00 0                          [heap]
		7f0951b20000-7f0951b35000 r--p 00000000 08:03 2097277                    /lib/ld-musl-x86_64.so.1
		7f0951b35000-7f0951b7c000 r-xp 00015000 08:03 2097277                    /lib/ld-musl-x86_64.so.1
		7f0951b7c000-7f0951bb0000 r--p 0005c000 08:03 2097277                    /lib/ld-musl-x86_64.so.1
		7f0951bb1000-7f0951bb2000 r--p 00090000 08:03 2097277                    /lib/ld-musl-x86_64.so.1
		7f0951bb2000-7f0951bb3000 rw-p 00091000 08:03 2097277                    /lib/ld-musl-x86_64.so.1
		7f0951bb3000-7f0951bb6000 rw-p 00000000 00:00 0
		7ffc4c759000-7ffc4c77a000 rw-p 00000000 00:00 0                          [stack]
		7ffc4c796000-7ffc4c799000 r--p 00000000 00:00 0                          [vvar]
		7ffc4c799000-7ffc4c79a000 r-xp 00000000 00:00 0                          [vdso]
		Note that the anonymous line (`7f0951bb3000-7f0951bb6000 rw-p 00000000 00:00 0 `) has one space (b' ') after the final, right-hand `0`; this may have been striped by the IDE.

		Example of /proc/pid/numa_maps:-
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
		Note that the anonymous lines (`7ffc4c796000` and `7ffc4c799000`) have no trailing space.
	*/
	/// This parse can be used for lines in`/proc/pid/maps`, `/proc/pids/smaps` and `/proc/pids/smaps_rollup` (although for the last case there is a special type of `[rollup]`).
	#[inline(always)]
	fn parse_maps_line(parse_state: &mut ParseState, (zero_based_line_number, map_line): (usize, Vec<u8>)) -> Result<Self, MemoryMapParseError>
	{
		parse_state.new_line(zero_based_line_number);
		let mut fields = ParseState::maps_line_split_fields(&map_line[..]);

		let memory_range =
		{
			let from = parse_state.next_number_field(&mut fields, "from", VirtualAddress::parse_hexadecimal_number_lower_case)?;
			let to = parse_state.next_number_field(&mut fields, "to", VirtualAddress::parse_hexadecimal_number_lower_case)?;
			parse_state.validate_from_and_to(from, to)?
		};
		let (protection, sharing) = parse_state.next_field(&mut fields, "permissions", |field_bytes, zero_based_line_number, zero_based_field_index| Self::parse_protection_and_sharing(field_bytes, zero_based_line_number, zero_based_field_index))?;
		let offset = parse_state.next_number_field(&mut fields, "offset", Self::parse_offset)?;
		let block_device = BlockDevice
		{
			major: parse_state.next_number_field(&mut fields, "block_device_major", Self::parse_major_or_minor)?,
			minor: parse_state.next_number_field(&mut fields, "block_device_minor", Self::parse_major_or_minor)?,
		};
		let inode = parse_state.next_number_field(&mut fields, "inode", Inode::parse_decimal_number)?;
		let kind =
		{
			let field_bytes = parse_state.last_field(fields, "file_name", |field_bytes, _zero_based_line_number, _zero_based_field_index| Ok(field_bytes))?;
			parse_state.parse_kind(field_bytes, offset, block_device, inode, protection, sharing)?
		};

		Ok
		(
			Self
			{
				memory_range,
				protection,
				sharing,
				kind,
				numa_details: None,
			}
		)
	}

	#[inline(always)]
	fn parse_protection_and_sharing(field_bytes: &[u8], zero_based_line_number: usize, zero_based_field_index: usize) -> Result<(Protection, Sharing), MemoryMapParseError>
	{
		if likely!(field_bytes.len() == 4)
		{
			Ok
			(
				(
					Self::parse_protection(field_bytes, zero_based_line_number, zero_based_field_index)?,
					Self::parse_sharing(field_bytes, zero_based_line_number, zero_based_field_index)?
				)
			)
		}
		else
		{
			Err(PermissionsFieldIsWrongLength { zero_based_line_number, zero_based_field_index })
		}
	}

	#[inline(always)]
	fn parse_protection(field_bytes: &[u8], zero_based_line_number: usize, zero_based_field_index: usize) -> Result<Protection, MemoryMapParseError>
	{
		use self::Protection::*;

		let three_bytes: &[u8; 3] = &field_bytes[0 .. 3].try_into().unwrap();

		let protection = match three_bytes
		{
			b"---" => Unaccessible,
			b"r--" => Read,
			b"rw-" => ReadWrite,
			b"r-x" => ReadExecutable,
			b"rwx" => ReadWriteExecutable,
			_ => return Err(PermissionsFieldUnrecognised { zero_based_line_number, zero_based_field_index, unrecognised: three_bytes.clone() }),
		};
		Ok(protection)
	}

	#[inline(always)]
	fn parse_sharing(field_bytes: &[u8], zero_based_line_number: usize, zero_based_field_index: usize) -> Result<Sharing, MemoryMapParseError>
	{
		use self::Sharing::*;

		let sharing = match unsafe { *field_bytes.get_unchecked(3) }
		{
			b'p' => Private,
			b's' => Shared,
			unrecognised @ _ => return Err(SharingFieldUnrecognised { zero_based_line_number, zero_based_field_index, unrecognised }),
		};
		Ok(sharing)
	}

	#[inline(always)]
	fn parse_offset(field_bytes: &[u8]) -> Result<u32, ParseNumberError>
	{
		u32::parse_hexadecimal_number_lower_case_fixed_width(field_bytes, size_of::<u32>() * 2)
	}

	#[inline(always)]
	fn parse_major_or_minor(field_bytes: &[u8]) -> Result<u8, ParseNumberError>
	{
		u8::parse_hexadecimal_number_lower_case_fixed_width(field_bytes, size_of::<u8>() * 2)
	}
}
