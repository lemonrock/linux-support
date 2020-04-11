// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A memory map entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryMapEntry
{
	/// Memory range.
	pub memory_range: Range<VirtualAddress>,

	/// Protection.
	pub protection: Protection,

	/// Sharing (can not differentiate persistent shares, sadly).
	pub sharing: Sharing,

	/// Kind of mapping.
	pub kind: MemoryMapEntryKind,

	/// Only `Some` if the kernel has been built with `CONFIG_NUMA`.
	pub numa_memory_policy_details: Option<NumaMemoryPolicyDetails>,
}

impl MemoryMapEntry
{
	/*
		Example of /proc/<pid>/maps:-
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
	*/
	/// This parse can be used for lines in`/proc/<pid>/maps` and `/proc/<pid>/smaps` but not `/proc/<pid>/smaps_rollup` (as in the last case there is a special type of `[rollup]`).
	#[inline(always)]
	fn parse_maps_line(parse_state: &mut ParseState, (zero_based_line_number, map_line): (usize, Vec<u8>), memory_details: &mut impl FnMut(&Range<VirtualAddress>, &MemoryMapEntryKind) -> Result<Option<(NumaMemoryPolicyDetails, Option<PageCounts>)>, MemoryMapParseError>) -> Result<Self, MemoryMapParseError>
	{
		parse_state.new_line(zero_based_line_number);
		let mut fields = ParseState::map_line_split_fields(&map_line[..]);

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
		let mut kind =
		{
			let field_bytes = parse_state.last_field(fields, "file_name", |field_bytes, _zero_based_line_number, _zero_based_field_index| Ok(field_bytes))?;
			parse_state.parse_kind(field_bytes, offset, block_device, inode, protection, sharing)?
		};

		let numa_memory_policy_details = if let Some((numa_memory_policy_details, numa_page_counts)) = memory_details(&memory_range, &kind)?
		{
			use self::MemoryMapEntryKind::*;
			use self::MemoryMapEntryKindSpecial::*;

			match kind
			{
				File { ref mut page_counts, .. } => *page_counts = numa_page_counts,
				Anonymous { ref mut page_counts, .. } => *page_counts = numa_page_counts,
				Special(Heap { ref mut page_counts }) => *page_counts = numa_page_counts,
				Special(Stack { ref mut page_counts }) => *page_counts = numa_page_counts,
				Special(vDSO) => debug_assert!(numa_page_counts.is_none()),
				Special(VVAR) => debug_assert!(numa_page_counts.is_none()),
			}

			Some(numa_memory_policy_details)
		}
		else
		{
			None
		};
		Ok
		(
			Self
			{
				memory_range,
				protection,
				sharing,
				kind,
				numa_memory_policy_details,
			}
		)
	}

	/*
		Example of /proc/<pid>/numa_maps:-
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
	/// Returns `Ok(None)` if it appears that the `numa_maps` line does not match its associated `smaps` (or `maps`) line.
	///
	/// Very rarely returns `Ok(Some((None, None)))` or `Ok(Some((None, Some())))` if Linux has a bug and returns an `unknown` memory policy.
	fn parse_numa_maps_line((zero_based_line_number, numa_map_line): (usize, Vec<u8>), expected_from: VirtualAddress, expected_kind: &MemoryMapEntryKind, have_movable_memory: &BitSet<NumaNode>) -> Result<(NumaMemoryPolicyDetails, Option<PageCounts>), MemoryMapParseError>
	{
		let mut fields = numa_map_line.split(|byte| *byte == b' ');

		if unlikely!(Self::parse_from(&mut fields, zero_based_line_number)? != expected_from)
		{
			return Self::mismatched("Line had different 'From' virtual address. Lines in maps, smaps and numa_maps are sorted in ascending order and each line should refer to the same mapping.")
		}

		let numa_memory_policy_details = Self::parse_numa_memory_policy_details(&mut fields, zero_based_line_number)?;

		match Self::validate_kind(&mut fields, expected_kind)?
		{
			true => return Ok((numa_memory_policy_details, None)),
			false => (),
		};

		let (mut next_field, pages_are_hugetlb_pages) = Self::parse_huge(&mut fields)?;

		let mut anonymous: Option<NumberOfPages> = None;
		let mut dirty: Option<NumberOfPages> = None;
		let mut mapped: Option<NumberOfPages> = None;
		let mut map_count_maximum: Option<NumberOfPages> = None;
		let mut swap_cache: Option<NumberOfPages> = None;
		let mut active: Option<NumberOfPages> = None;
		let mut write_back: Option<NumberOfPages> = None;
		let mut by_numa_node: HashMap<NumaNode, NumberOfPages> = HashMap::with_capacity(1);
		loop
		{
			match next_field
			{
				Some((b"anon", value @ _)) => Self::assign_field_once("anon", &mut anonymous, value)?,
				Some((b"dirty", value @ _)) => Self::assign_field_once("dirty", &mut dirty, value)?,
				Some((b"mapped", value @ _)) => Self::assign_field_once("mapped", &mut mapped, value)?,
				Some((b"mapmax", value @ _)) => Self::assign_field_once("mapmax", &mut map_count_maximum, value)?,
				Some((b"swapcache", value @ _)) => Self::assign_field_once("swapcache", &mut swap_cache, value)?,
				Some((b"active", value @ _)) => Self::assign_field_once("active", &mut active, value)?,
				Some((b"writeback", value @ _)) => Self::assign_field_once("writeback", &mut write_back, value)?,
				Some((b"kernelpagesize_kB", value @ _)) =>
				{
					let _unused_because_expected_value_is_in_smaps_memory_statistics = Self::parse_kernel_page_size(zero_based_line_number, value)?;

					return Ok
					((
						numa_memory_policy_details,
						Some
						(
							PageCounts
							{
								pages_are_hugetlb_pages,
								anonymous: anonymous.unwrap_or(0),
								dirty: dirty.unwrap_or(0),
								mapped: mapped.unwrap_or(0),
								map_count_maximum:
								{
									let map_count_maximum = match map_count_maximum
									{
										None => 1,

										Some(value) => if unlikely!(value <= 1)
										{
											return Err(NumaMapMaxValueWasZeroOrOne)
										}
										else
										{
											value
										}
									};
									unsafe { NonZeroU64::new_unchecked(map_count_maximum) }
								},
								swap_cache: swap_cache.unwrap_or(0),
								active: if pages_are_hugetlb_pages
								{
									if unlikely!(active.is_some())
									{
										return Err(ActiveWasPresentWhenHugePagesArePresent);
									}
									else
									{
										None
									}
								}
								else
								{
									active
								},
								write_back: write_back.unwrap_or(0),
								by_numa_node:
								{
									for numa_node in have_movable_memory.iterate()
									{
										by_numa_node.entry(numa_node).or_insert(0);
									}
									by_numa_node
								},
							}
						)
					))
				},

				Some((key @ _, value @ _)) => if likely!(key.starts_with(b"N"))
				{
					let numa_node = NumaNode::parse_decimal_number(&key[1 .. ]).map_err(|cause| CouldNotParseNumaMapsKeyValueField { cause })?;
					let number_of_pages = Self::parse_number_of_pages(value)?;

					if unlikely!(!have_movable_memory.contains(numa_node))
					{
						return Self::mismatched("List of online nodes does not contain node in numa_maps lines; list of online nodes might be out-of-date")
					}

					if unlikely!(by_numa_node.insert(numa_node, number_of_pages).is_some())
					{
						return Err(NumaMapNodeKeyRepeated { numa_node })
					}
				}
				else
				{
					// Unknown keys are deliberately ignored.
				}

				None => return Err(NumaMapMissingKernelPageSize),
			}

			next_field = Self::key_value_field(fields.next())?;
		}
	}

	#[inline(always)]
	fn mismatched<R>(explanation: &'static str) -> Result<R, MemoryMapParseError>
	{
		Err(Mismatched { explanation })
	}

	#[inline(always)]
	fn parse_kernel_page_size(zero_based_line_number: usize, value: &[u8]) -> Result<PageSizeOrHugePageSize, MemoryMapParseError>
	{
		PageSizeOrHugePageSize::from_kilobytes(Self::parse_number_of_pages(value)?).ok_or(StatisticWasNotAPageSize { zero_based_line_number })
	}

	#[inline(always)]
	fn parse_from<'a>(fields: &mut impl Iterator<Item=&'a [u8]>, zero_based_line_number: usize) -> Result<VirtualAddress, MemoryMapParseError>
	{
		let from_bytes = fields.next().unwrap();
		VirtualAddress::parse_hexadecimal_number_lower_case(from_bytes).map_err(|cause| CouldNotParseNumberField { zero_based_line_number, zero_based_field_index: 1, name: "memory_policy", cause })
	}

	/// Can be `None` in the vary rare circumstance Linux returns `unknown` (which is a Linux bug that can occur because of the lack of storng typing inside the Linux code base).
	#[inline(always)]
	fn parse_numa_memory_policy_details<'a>(fields: &mut impl Iterator<Item=&'a [u8]>, zero_based_line_number: usize) -> Result<NumaMemoryPolicyDetails, MemoryMapParseError>
	{
		let (set_memory_policy_bytes, memory_policy_dynamism_bytes, numa_node_bit_mask_bytes) =
		{
			let field_bytes = fields.next().ok_or(MissingRequiredField { zero_based_line_number, zero_based_field_index: 1, name: "memory_policy" })?;
			let equals_index = memchr(b'=', field_bytes);
			let colon_index = memrchr(b':', field_bytes);

			const Nothing: &'static [u8] = b"";
			match (equals_index, colon_index)
			{
				(Some(equals), Some(colon)) => (&field_bytes[..equals], &field_bytes[equals + 1..colon], &field_bytes[colon + 1..]),

				(Some(equals), None) => (&field_bytes[..equals], &field_bytes[equals + 1..], Nothing),

				(None, Some(colon)) => (&field_bytes[..colon], Nothing, &field_bytes[colon + 1..]),

				(None, None) => (field_bytes, Nothing, Nothing),
			}
		};

		let memory_policy_dynamism = Self::parse_memory_policy_dynamism(memory_policy_dynamism_bytes)?;

		use self::SetMemoryPolicy::*;
		match set_memory_policy_bytes
		{
			b"default" => Self::default_or_local(Default, memory_policy_dynamism, numa_node_bit_mask_bytes),

			b"local" => Self::default_or_local(Local, memory_policy_dynamism, numa_node_bit_mask_bytes),

			b"bind" => Self::bind_or_interleave(|numa_node_bit_set| Bind { numa_node_bit_set}, memory_policy_dynamism, numa_node_bit_mask_bytes),

			b"interleave" => Self::bind_or_interleave(|numa_node_bit_set| Interleave { numa_node_bit_set}, memory_policy_dynamism, numa_node_bit_mask_bytes),

			b"prefer" => Self::prefer(memory_policy_dynamism, numa_node_bit_mask_bytes),

			b"unknown" => Err(Self::unknown(memory_policy_dynamism, numa_node_bit_mask_bytes)),

			_ => Err(MemoryPolicyUnrecognised { zero_based_line_number, unrecognised_memory_policy: set_memory_policy_bytes.to_vec().into_boxed_slice() }),
		}
	}

	#[inline(always)]
	fn default_or_local(set_memory_policy: SetMemoryPolicy, memory_policy_dynamism: MemoryPolicyDynamism, numa_node_bit_mask_bytes: &[u8]) -> Result<NumaMemoryPolicyDetails, MemoryMapParseError>
	{
		if likely!(numa_node_bit_mask_bytes.is_empty())
		{
			Ok(NumaMemoryPolicyDetails::new(set_memory_policy, memory_policy_dynamism))
		}
		else
		{
			Err(DefaultOrLocalMemoryPolicyHasNumaNodes)
		}
	}

	#[inline(always)]
	fn bind_or_interleave(constructor: impl FnOnce(BitSet<NumaNode>) -> SetMemoryPolicy, memory_policy_dynamism: MemoryPolicyDynamism, numa_node_bit_mask_bytes: &[u8]) -> Result<NumaMemoryPolicyDetails, MemoryMapParseError>
	{
		let numa_node_bit_set = BitSet::parse_linux_list_string(numa_node_bit_mask_bytes)?;
		Ok(NumaMemoryPolicyDetails::new(constructor(numa_node_bit_set), memory_policy_dynamism))
	}

	#[inline(always)]
	fn prefer(memory_policy_dynamism: MemoryPolicyDynamism, numa_node_bit_mask_bytes: &[u8]) -> Result<NumaMemoryPolicyDetails, MemoryMapParseError>
	{
		let numa_node_bit_set = BitSet::<NumaNode>::parse_linux_list_string(numa_node_bit_mask_bytes)?;
		let mut iterator = numa_node_bit_set.iterate();
		let first = iterator.next();
		if unlikely!(first.is_none())
		{
			return Err(PreferredMemoryPolicyHasNoNumaNodes)
		}
		let second = iterator.next();
		if unlikely!(second.is_none())
		{
			return Err(PreferredMemoryPolicyHasMoreThanOneNumaNode)
		}
		Ok(NumaMemoryPolicyDetails::new(SetMemoryPolicy::Preferred(first.unwrap()), memory_policy_dynamism))
	}

	#[inline(always)]
	fn unknown(memory_policy_dynamism: MemoryPolicyDynamism, numa_node_bit_mask_bytes: &[u8]) -> MemoryMapParseError
	{
		if unlikely!(memory_policy_dynamism != MemoryPolicyDynamism::NoDynamism)
		{
			UnknownSetMemoryPolicyHasDetails
		}
		else if unlikely!(numa_node_bit_mask_bytes.is_empty())
		{
			UnknownSetMemoryPolicyHasDetails
		}
		else
		{
			UnknownSetMemoryPolicyReportedByLinuxIndicatesABugInLinux
		}
	}

	#[inline(always)]
	fn parse_memory_policy_dynamism(memory_policy_dynamism_bytes: &[u8]) -> Result<MemoryPolicyDynamism, MemoryMapParseError>
	{
		use self::MemoryPolicyDynamism::*;

		match memory_policy_dynamism_bytes
		{
			b"" => Ok(NoDynamism),

			b"relative" => Ok(Relative),

			b"static" => Ok(Static),

			_ => Err(UnrecognisedMemoryPolicyDynamism { value: memory_policy_dynamism_bytes.to_vec().into_boxed_slice() }),
		}
	}

	#[inline(always)]
	fn validate_kind<'a>(fields: &mut impl Iterator<Item=&'a [u8]>, expected_kind: &MemoryMapEntryKind) -> Result<bool, MemoryMapParseError>
	{
		use self::MemoryMapEntryKind::*;
		use self::MemoryMapEntryKindSpecial::*;
		match expected_kind
		{
			&Anonymous { .. } => Ok(false),

			&Special(Heap { .. }) => Self::validate_special_heap_or_stack(fields, b"heap"),

			&Special(Stack { .. }) => Self::validate_special_heap_or_stack(fields, b"stack"),

			&Special(vDSO) | &Special(VVAR) => match fields.next()
			{
				None => Ok(true),

				Some(b"huge") => Err(HugePagesForVdsoOrVvarMapping),

				Some(_) => Self::mismatched("Line should have had only 2 fields as it refers to either a vDSO or a VVAR special mapping")
			},

			&File { ref file_path, .. } => if let Some(field_bytes) = fields.next()
			{
				const StartsWith: &'static [u8] = b"file=";
				if unlikely!(!field_bytes.starts_with(StartsWith))
				{
					return Self::mismatched("Line did not start with 'file='. Lines in maps, smaps and numa_maps are sorted in ascending order and each line should refer to the same mapping.")
				}

				let file_path_with_escaped_line_feed_tab_and_space = &field_bytes[StartsWith.len() .. ];
				let our_file_path = PathBuf::from(OsString::from_vec(LinuxStringEscapeSequence::unescape_linux_string(file_path_with_escaped_line_feed_tab_and_space.to_vec(), &[LinuxStringEscapeSequence::LineFeed, LinuxStringEscapeSequence::HorizontalTab, LinuxStringEscapeSequence::Space])));

				if unlikely!(file_path != &our_file_path)
				{
					Self::mismatched("Line had different file path. Lines in maps, smaps and numa_maps are sorted in ascending order and each line should refer to the same mapping.")
				}
				else
				{
					Ok(false)
				}
			}
			else
			{
				Self::mismatched("Line should have had a 'file=' key-value as the third field but instead had only 2 fields")
			},
		}
	}

	#[inline(always)]
	fn validate_special_heap_or_stack<'a>(fields: &mut impl Iterator<Item=&'a [u8]>, expected_name: &'static [u8]) -> Result<bool, MemoryMapParseError>
	{
		match fields.next()
		{
			Some(name) => if likely!(name == expected_name)
			{
				Ok(false)
			}
			else
			{
				Self::mismatched("Line should have had 'heap' or 'stack' as the third field but instead had something else")
			},

			None => Self::mismatched("Line should have had 'heap' or 'stack' as the third field but instead had only 2 fields"),
		}
	}

	#[inline(always)]
	fn parse_huge<'a>(fields: &mut impl Iterator<Item=&'a [u8]>) -> Result<(Option<(&'a [u8], &'a [u8])>, bool), MemoryMapParseError>
	{
		let next_field = fields.next();
		if next_field == Some(b"huge" as &[u8])
		{
			Ok((Self::key_value_field(fields.next())?, true))
		}
		else
		{
			Ok((Self::key_value_field(next_field)?, false))
		}
	}

	#[inline(always)]
	fn key_value_field(next_field: Option<&[u8]>) -> Result<Option<(&[u8], &[u8])>, MemoryMapParseError>
	{
		if let Some(field_bytes) = next_field
		{
			match memchr(b'=', field_bytes)
			{
				None => Err(NumaMapKeyValueDidNotHaveEqualsSign),

				Some(index) =>
				{
					let key = &field_bytes[..index];
					let value = &field_bytes[index + 1..];
					Ok(Some((key, value)))
				},
			}
		}
		else
		{
			Ok(None)
		}
	}

	#[inline(always)]
	fn assign_field_once(name: &'static str, field: &mut Option<NumberOfPages>, value: &[u8]) -> Result<(), MemoryMapParseError>
	{
		if unlikely!(field.is_some())
		{
			return Err(NumaMapKeyRepeated { name })
		}

		let value = Self::parse_number_of_pages(value)?;
		*field = Some(value);
		Ok(())
	}

	#[inline(always)]
	fn parse_number_of_pages(value_bytes: &[u8]) -> Result<NumberOfPages, MemoryMapParseError>
	{
		NumberOfPages::parse_decimal_number(value_bytes).map_err(|cause| CouldNotParseNumaMapsKeyValueField { cause })
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
