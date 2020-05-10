// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Statistics for this memory map entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryMapEntryStatistics
{
	/// Known as `KernelPageSize`.
	pub kernel_page_size: PageSizeOrHugePageSize,

	/// Memory Management Unit (MMU) page size.
	///
	/// Must always be the same or less than `kernel_page_size`.
	/// Identical to `kernel_page_size` except on powerpc64.
	///
	/// Known as `MMUPageSize`.
	#[cfg(target_arch = "powerpc64")]
	pub memory_management_unit_page_size: PageSizeOrHugePageSize,

	/// Kilobyte statistics.
	pub kilobyte_statistics: MemoryMapEntryKilobyteStatistics,

	/// Known as `THPeligible`.
	///
	/// Can be backed by transparent huge pages.
	pub eligible_for_transparent_huge_pages: bool,

	/// A protection key.
	/// 
	/// Only present if the kernel was compiled with `CONFIG_X86_INTEL_MEMORY_PROTECTION_KEYS`.
	///
	/// Since Linux 4.6.
	pub protection_key: Option<u64>,

	/// Unknown count statistics.
	pub unknown_count: HashMap<Box<[u8]>, u64>,

	/// Known as `VMFlags`.
	pub vm_flags: HashSet<VmFlag>,
}

impl MemoryMapEntryStatistics
{
	/*
		Example for /proc/<pid>/smaps:-

		7f0951bb2000-7f0951bb3000 rw-p 00091000 08:03 2097277                    /lib/ld-musl-x86_64.so.1
		Size:                  4 kB
		KernelPageSize:        4 kB
		MMUPageSize:           4 kB
		Rss:                   4 kB
		Pss:                   4 kB
		Shared_Clean:          0 kB
		Shared_Dirty:          0 kB
		Private_Clean:         0 kB
		Private_Dirty:         4 kB
		Referenced:            4 kB
		Anonymous:             4 kB
		LazyFree:              0 kB
		AnonHugePages:         0 kB
		ShmemPmdMapped:        0 kB
		FilePmdMapped:        0 kB
		Shared_Hugetlb:        0 kB
		Private_Hugetlb:       0 kB
		Swap:                  0 kB
		SwapPss:               0 kB
		Locked:                0 kB
		THPeligible:		0
		VmFlags: rd wr mr mw me dw ac

		First line is identical to /proc/<pid>/maps, and is not handled below.
		It is assumed that the lines are pointing to a statistic name; statistics are assumed to always end with `VmFlags`.
	*/
	fn parse_statistics_lines(lines: &mut impl Iterator<Item=Result<(usize, Vec<u8>), MemoryMapParseError>>, memory_range: Range<VirtualAddress>, our_protection: Protection, our_sharing: Sharing) -> Result<Self, MemoryMapParseError>
	{
		use self::MemoryMapParseError::*;

		let mut size: Option<Kilobyte> = None;
		let mut kernel_page_size: Option<PageSizeOrHugePageSize> = None;
		let mut memory_management_unit_page_size: Option<PageSizeOrHugePageSize> = None;
		let mut resident_set_size: Option<Kilobyte> = None;
		let mut process_share_of_resident_set_size: Option<Kilobyte> = None;
		let mut shared_clean: Option<Kilobyte> = None;
		let mut shared_dirty: Option<Kilobyte> = None;
		let mut private_clean: Option<Kilobyte> = None;
		let mut private_dirty: Option<Kilobyte> = None;
		let mut referenced: Option<Kilobyte> = None;
		let mut anonymous: Option<Kilobyte> = None;
		let mut lazy_free: Option<Kilobyte> = None;
		let mut anonymous_huge_pages: Option<Kilobyte> = None;
		let mut shmem_memory_mapped_into_user_space_using_huge_pages: Option<Kilobyte> = None;
		let mut file_memory_mapped_into_user_space_using_huge_pages: Option<Kilobyte> = None;
		let mut shared_hugetlb: Option<Kilobyte> = None;
		let mut private_hugetlb: Option<Kilobyte> = None;
		let mut swap: Option<Kilobyte> = None;
		let mut process_share_of_swap: Option<Kilobyte> = None;
		let mut locked: Option<Kilobyte> = None;
		let mut eligible_for_transparent_huge_pages: Option<bool> = None;
		let mut protection_key = None;
		let mut unknown_kilobyte = HashMap::new();
		let mut unknown_count = HashMap::new();

		let vm_flags = loop
		{
			let line = lines.next().ok_or(ExpectedStatisticLine)?;
			let (zero_based_line_number, line_bytes) = line?;
			let line_bytes = &line_bytes[..];

			let (statistic_name, remaining_line_bytes) =
			{
				let colon_index = memchr(b':', line_bytes).ok_or(StatisticMissingColon { zero_based_line_number })?;
				(&line_bytes[0 .. colon_index], &line_bytes[colon_index + 1 ..])
			};

			match statistic_name
			{
				b"VmFlags" =>
				{
					break Self::parse_vmflags(remaining_line_bytes, our_protection, our_sharing)?
				}
				
				b"Size" => Self::parse_mandatory_kilobyte(&mut size, zero_based_line_number, statistic_name, remaining_line_bytes)?,

				b"KernelPageSize" => Self::parse_mandatory_page_size(&mut kernel_page_size, zero_based_line_number, statistic_name, remaining_line_bytes)?,
				b"MMUPageSize" => Self::parse_mandatory_page_size(&mut memory_management_unit_page_size, zero_based_line_number, statistic_name, remaining_line_bytes)?,

				b"Rss" => Self::parse_mandatory_kilobyte(&mut resident_set_size, zero_based_line_number, statistic_name, remaining_line_bytes)?,
				b"Pss" => Self::parse_mandatory_kilobyte(&mut process_share_of_resident_set_size, zero_based_line_number, statistic_name, remaining_line_bytes)?,
				b"Shared_Clean" => Self::parse_mandatory_kilobyte(&mut shared_clean, zero_based_line_number, statistic_name, remaining_line_bytes)?,
				b"Shared_Dirty" => Self::parse_mandatory_kilobyte(&mut shared_dirty, zero_based_line_number, statistic_name, remaining_line_bytes)?,
				b"Private_Clean" => Self::parse_mandatory_kilobyte(&mut private_clean, zero_based_line_number, statistic_name, remaining_line_bytes)?,
				b"Private_Dirty" => Self::parse_mandatory_kilobyte(&mut private_dirty, zero_based_line_number, statistic_name, remaining_line_bytes)?,
				b"Referenced" => Self::parse_mandatory_kilobyte(&mut referenced, zero_based_line_number, statistic_name, remaining_line_bytes)?,
				b"Anonymous" => Self::parse_mandatory_kilobyte(&mut anonymous, zero_based_line_number, statistic_name, remaining_line_bytes)?,
				b"LazyFree" => Self::parse_mandatory_kilobyte(&mut lazy_free, zero_based_line_number, statistic_name, remaining_line_bytes)?,
				b"AnonHugePages" => Self::parse_mandatory_kilobyte(&mut anonymous_huge_pages, zero_based_line_number, statistic_name, remaining_line_bytes)?,
				b"ShmemPmdMapped" => Self::parse_mandatory_kilobyte(&mut shmem_memory_mapped_into_user_space_using_huge_pages, zero_based_line_number, statistic_name, remaining_line_bytes)?,
				b"FilePmdMapped" => Self::parse_mandatory_kilobyte(&mut file_memory_mapped_into_user_space_using_huge_pages, zero_based_line_number, statistic_name, remaining_line_bytes)?,
				b"Shared_Hugetlb" => Self::parse_mandatory_kilobyte(&mut shared_hugetlb, zero_based_line_number, statistic_name, remaining_line_bytes)?,
				b"Private_Hugetlb" => Self::parse_mandatory_kilobyte(&mut private_hugetlb, zero_based_line_number, statistic_name, remaining_line_bytes)?,
				b"Swap" => Self::parse_mandatory_kilobyte(&mut swap, zero_based_line_number, statistic_name, remaining_line_bytes)?,
				b"SwapPss" => Self::parse_mandatory_kilobyte(&mut process_share_of_swap, zero_based_line_number, statistic_name, remaining_line_bytes)?,
				b"Locked" => Self::parse_mandatory_kilobyte(&mut locked, zero_based_line_number, statistic_name, remaining_line_bytes)?,

				b"THPeligible" => Self::parse_mandatory_bool(&mut eligible_for_transparent_huge_pages, zero_based_line_number, statistic_name, remaining_line_bytes)?,

				b"ProtectionKey" => Self::parse_optional_count(&mut protection_key, zero_based_line_number, statistic_name, remaining_line_bytes)?,

				_ => MemoryMapEntryStatistics::parse_unknown(&mut unknown_kilobyte, &mut unknown_count, zero_based_line_number, statistic_name, remaining_line_bytes)?,
			}
		};

		let size = size.ok_or(MissingStatistic("Size"))?;
		if unlikely!(size != ((memory_range.end - memory_range.start)) as u64)
		{
			return Err(SizeDoesMatchMemoryRange)
		}

		let kernel_page_size = kernel_page_size.ok_or(MissingStatistic("KernelPageSize"))?;
		let memory_management_unit_page_size = memory_management_unit_page_size.ok_or(MissingStatistic("MMUPageSize"))?;

		#[cfg(not(target_arch = "powerpc64"))]
		if unlikely!(kernel_page_size != memory_management_unit_page_size)
		{
			return Err(KernelPageSizeAndMemoryManagementUnitPageSizeDiffer)
		}

		#[cfg(target_arch = "powerpc64")]
		if unlikely!(kernel_page_size < memory_management_unit_page_size)
		{
			return Err(KernelPageSizeLessThanMemoryManagementUnitPageSize)
		}

		let resident_set_size = Self::mandatory_and_does_not_exceed_size(resident_set_size, "Rss", size)?;
		let swap = Self::mandatory_and_does_not_exceed_size(swap, "Swap", size)?;
		Ok
		(
			Self
			{
				kernel_page_size,
				#[cfg(target_arch = "powerpc64")] memory_management_unit_page_size,
				kilobyte_statistics: MemoryMapEntryKilobyteStatistics
				{
					resident_set_size: SizeAndProcessShareOfSize
					{
						size: resident_set_size,
						process_share_of_size: Self::mandatory_and_does_not_exceed_size(process_share_of_resident_set_size, "Pss", resident_set_size)?,
					},
					shared: CleanDirtyAndHuge
					{
						clean: Self::mandatory_and_does_not_exceed_size(shared_clean, "Shared_Clean", size)?,
						dirty: Self::mandatory_and_does_not_exceed_size(shared_dirty, "Shared_Dirty", size)?,
						hugetlb: Self::mandatory_and_does_not_exceed_size(shared_hugetlb, "Shared_Hugetlb", size)?,
					},
					private: CleanDirtyAndHuge
					{
						clean: Self::mandatory_and_does_not_exceed_size(private_clean, "Private_Clean", size)?,
						dirty: Self::mandatory_and_does_not_exceed_size(private_dirty, "Private_Dirty", size)?,
						hugetlb: Self::mandatory_and_does_not_exceed_size(private_hugetlb, "Private_Hugetlb", size)?,
					},
					referenced: Self::mandatory_and_does_not_exceed_size(referenced, "Referenced", size)?,
					anonymous: Self::mandatory_and_does_not_exceed_size(anonymous, "Anonymous", size)?,
					anonymous_huge_pages: Self::mandatory_and_does_not_exceed_size(anonymous_huge_pages, "AnonHugePages", size)?,
					lazy_free: Self::mandatory_and_does_not_exceed_size(lazy_free, "LazyFree", size)?,
					shmem_memory_mapped_into_user_space_using_huge_pages: Self::mandatory_and_does_not_exceed_size(shmem_memory_mapped_into_user_space_using_huge_pages, "ShmemPmdMapped", size)?,
					file_memory_mapped_into_user_space_using_huge_pages: Self::mandatory_and_does_not_exceed_size(file_memory_mapped_into_user_space_using_huge_pages, "FilePmdMapped", size)?,
					swap: SizeAndProcessShareOfSize
					{
						size: swap,
						process_share_of_size: Self::mandatory_and_does_not_exceed_size(process_share_of_swap, "SwapPss", swap)?,
					},
					locked: Self::mandatory_and_does_not_exceed_size(locked, "Locked", size)?,
					unknown: unknown_kilobyte,
				},
				eligible_for_transparent_huge_pages: Self::mandatory(eligible_for_transparent_huge_pages, "THPeligible")?,

				protection_key,

				unknown_count,

				vm_flags,
			}
		)
	}

	#[inline(always)]
	fn parse_unknown<'a, 'b>(unknown_kilobyte: &'a mut HashMap<Box<[u8]>, u64>, unknown_count: &'a mut HashMap<Box<[u8]>, u64>, zero_based_line_number: usize, statistic_name: &'b [u8], remaining_line_bytes: &'b [u8]) -> Result<(), MemoryMapParseError>
	{
		let (statistic_value_bytes, memory_information_unit) = Self::statistic_value_bytes_except_vmflags(remaining_line_bytes, zero_based_line_number)?;

		use self::MemoryInformationUnit::*;
		let unknown = match memory_information_unit
		{
			Kilobyte => unknown_kilobyte,
			Count => unknown_count,
		};

		let previous = unknown.insert(Self::box_statistic_name(statistic_name), Self::parse_value_number(statistic_value_bytes, zero_based_line_number)?);
		if unlikely!(previous.is_some())
		{
			Err(DuplicateStatistic { zero_based_line_number, statistic_name: Self::box_statistic_name(statistic_name) })
		}
		else
		{
			Ok(())
		}
	}

	#[inline(always)]
	fn parse_mandatory_kilobyte(field: &mut Option<Kilobyte>, zero_based_line_number: usize, statistic_name: &[u8], remaining_line_bytes: &[u8]) -> Result<(), MemoryMapParseError>
	{
		if unlikely!(field.is_some())
		{
			return Err(DuplicateStatistic { zero_based_line_number, statistic_name: Self::box_statistic_name(statistic_name) })
		}
		let (statistic_value_bytes, memory_information_unit) = Self::statistic_value_bytes_except_vmflags(remaining_line_bytes, zero_based_line_number)?;

		if unlikely!(memory_information_unit != MemoryInformationUnit::Kilobyte)
		{
			return Err(StatisticWasNotKilobyte { zero_based_line_number, statistic_name: Self::box_statistic_name(statistic_name) })
		}
		*field = Some(Self::parse_value_number(statistic_value_bytes, zero_based_line_number)?);
		Ok(())
	}

	#[inline(always)]
	fn parse_mandatory_page_size(field: &mut Option<PageSizeOrHugePageSize>, zero_based_line_number: usize, statistic_name: &[u8], remaining_line_bytes: &[u8]) -> Result<(), MemoryMapParseError>
	{
		if unlikely!(field.is_some())
		{
			return Err(DuplicateStatistic { zero_based_line_number, statistic_name: Self::box_statistic_name(statistic_name) })
		}
		let (statistic_value_bytes, memory_information_unit) = Self::statistic_value_bytes_except_vmflags(remaining_line_bytes, zero_based_line_number)?;

		if unlikely!(memory_information_unit != MemoryInformationUnit::Kilobyte)
		{
			return Err(StatisticWasNotKilobyte { zero_based_line_number, statistic_name: Self::box_statistic_name(statistic_name) })
		}
		*field = Some(Self::parse_value_page_size(statistic_value_bytes, zero_based_line_number)?);
		Ok(())
	}

	#[inline(always)]
	fn parse_mandatory_bool(field: &mut Option<bool>, zero_based_line_number: usize, statistic_name: &[u8], remaining_line_bytes: &[u8]) -> Result<(), MemoryMapParseError>
	{
		if unlikely!(field.is_some())
		{
			return Err(DuplicateStatistic { zero_based_line_number, statistic_name: Self::box_statistic_name(statistic_name) })
		}
		let (statistic_value_bytes, memory_information_unit) = Self::statistic_value_bytes_except_vmflags(remaining_line_bytes, zero_based_line_number)?;

		if unlikely!(memory_information_unit != MemoryInformationUnit::Count)
		{
			return Err(StatisticWasNotCount { zero_based_line_number, statistic_name: Self::box_statistic_name(statistic_name) })
		}
		*field = Some(Self::parse_value_bool(statistic_value_bytes, zero_based_line_number)?);
		Ok(())
	}

	#[inline(always)]
	fn parse_optional_count(field: &mut Option<u64>, zero_based_line_number: usize, statistic_name: &[u8], remaining_line_bytes: &[u8]) -> Result<(), MemoryMapParseError>
	{
		if unlikely!(field.is_some())
		{
			return Err(DuplicateStatistic { zero_based_line_number, statistic_name: Self::box_statistic_name(statistic_name) })
		}
		let (statistic_value_bytes, memory_information_unit) = Self::statistic_value_bytes_except_vmflags(remaining_line_bytes, zero_based_line_number)?;

		if unlikely!(memory_information_unit != MemoryInformationUnit::Count)
		{
			return Err(StatisticWasNotCount { zero_based_line_number, statistic_name: Self::box_statistic_name(statistic_name) })
		}
		*field = Some(Self::parse_value_number(statistic_value_bytes, zero_based_line_number)?);
		Ok(())
	}

	#[inline(always)]
	fn parse_vmflags(remaining_line_bytes: &[u8], expected_protection: Protection, expected_sharing: Sharing) -> Result<HashSet<VmFlag>, MemoryMapParseError>
	{
		// `remaining_line_bytes` is eg ` rd wr mr mw me dw ac `.
		let without_leading_space = &remaining_line_bytes[1 .. ];
		let length = without_leading_space.len();

		const ElementSize: usize = 3;

		if unlikely!(length % ElementSize != 0)
		{
			return Err(VmFlagsByteLengthNotAMultipleOfThree)
		}

		let mut index = 0;
		let mut our_read = None;
		let mut our_write = None;
		let mut our_executable = None;
		let mut our_sharing = None;
		let mut vm_flags = HashSet::new();
		while index != length
		{
			use self::VmFlag::*;

			#[inline(always)]
			fn guard_permission(field: &mut Option<bool>) -> Result<(), MemoryMapParseError>
			{
				if unlikely!(field.is_some())
				{
					Err(DuplicateVmFlagPermission)
				}
				else
				{
					*field = Some(true);
					Ok(())
				}
			}

			#[inline(always)]
			fn insert_vm_flag(vm_flags: &mut HashSet<VmFlag>, vm_flag: VmFlag) -> Result<(), MemoryMapParseError>
			{
				if unlikely!(vm_flags.insert(vm_flag))
				{
					Err(DuplicateVmFlag(vm_flag))
				}
				else
				{
					Ok(())
				}
			}

			match &without_leading_space[index .. index + ElementSize]
			{
				b"rd " => guard_permission(&mut our_read)?,
				b"wr " => guard_permission(&mut our_write)?,
				b"ex " => guard_permission(&mut our_executable)?,
				b"sh " => guard_permission(&mut our_sharing)?,

				b"mr " => insert_vm_flag(&mut vm_flags, MayRead)?,
				b"mw " => insert_vm_flag(&mut vm_flags, MayWrite)?,
				b"me " => insert_vm_flag(&mut vm_flags, MayExecute)?,
				b"ms " => insert_vm_flag(&mut vm_flags, MayShare)?,
				b"gd " => insert_vm_flag(&mut vm_flags, StackSegmentGrowsDown)?,
				b"pf " => insert_vm_flag(&mut vm_flags, PurePageFrameNumberRange)?,
				b"dw " => insert_vm_flag(&mut vm_flags, DisabledWriteToTheMappedFile)?,
				b"lo " => insert_vm_flag(&mut vm_flags, PagesAreLockedInMemory)?,
				b"io " => insert_vm_flag(&mut vm_flags, MemoryMappedInputOutput)?,
				b"sr " => insert_vm_flag(&mut vm_flags, SequentialReadAdvised)?,
				b"rr " => insert_vm_flag(&mut vm_flags, RandomReadAdvised)?,
				b"dc " => insert_vm_flag(&mut vm_flags, DoNotCopyOnFork)?,
				b"de " => insert_vm_flag(&mut vm_flags, DoNotExpandOnRemapping)?,
				b"ac " => insert_vm_flag(&mut vm_flags, Accountable)?,
				b"nr " => insert_vm_flag(&mut vm_flags, SwapSpaceNotReserved)?,
				b"ht " => insert_vm_flag(&mut vm_flags, UsesHugeTlbPages)?,
				b"hg " => insert_vm_flag(&mut vm_flags, HugePageAdvised)?,
				b"nh " => insert_vm_flag(&mut vm_flags, NoHugePageAdvised)?,
				b"nl " => insert_vm_flag(&mut vm_flags, NonLinearMapping)?,
				b"ar " => insert_vm_flag(&mut vm_flags, ArchitectureSpecificFlag)?,
				b"dd " => insert_vm_flag(&mut vm_flags, DoNotIncludeInCoreDump)?,
				b"sd " => insert_vm_flag(&mut vm_flags, SoftDirty)?,
				b"mm " => insert_vm_flag(&mut vm_flags, MixedMap)?,
				b"mg " => insert_vm_flag(&mut vm_flags, MergeableAdvised)?,

				value @ _ => return Err(UnrecognisedVmFlag((&value[0 .. 2]).try_into().unwrap()))
			};

			index += ElementSize
		}

		use self::Protection::*;
		let our_protection = match (our_read, our_write, our_executable)
		{
			(None, None, None) => Inaccessible,

			(Some(true), None, None) => Read,

			(Some(true), Some(true), None) => ReadWrite,

			(Some(true), None, Some(true)) => ReadExecutable,

			(Some(true), Some(true), Some(true)) => ReadWriteExecutable,

			_ => return Err(IllegalCombinationOfVmFlagPermissions)
		};
		if unlikely!(our_protection != expected_protection)
		{
			return Err(InvalidVmFlagProtection)
		}

		use self::Sharing::*;
		let our_sharing = match our_sharing
		{
			None => Private,

			Some(true) => Shared,

			_ => unreachable!(),
		};
		if unlikely!(our_sharing != expected_sharing)
		{
			return Err(InvalidVmFlagSharing)
		}

		Ok(vm_flags)
	}

	#[inline(always)]
	fn statistic_value_bytes_except_vmflags(remaining_line_bytes: &[u8], zero_based_line_number: usize) -> Result<(&[u8], MemoryInformationUnit), MemoryMapParseError>
	{
		use self::MemoryInformationUnit::*;

		const KilobytesEnding: &'static [u8] = b" kB";
		let (space_value_bytes, memory_information_unit) = if likely!(remaining_line_bytes.ends_with(KilobytesEnding))
		{
			(&remaining_line_bytes[ .. remaining_line_bytes.len() - KilobytesEnding.len()], Kilobyte)
		}
		else
		{
			(remaining_line_bytes, Count)
		};

		let statistic_value_start_index = memrchr(b' ', space_value_bytes).ok_or(StatisticMissingSpaceAfterColon { zero_based_line_number })?;
		let statistic_value_bytes = &space_value_bytes[statistic_value_start_index .. ];

		Ok((statistic_value_bytes, memory_information_unit))
	}

	#[inline(always)]
	fn parse_value_number(statistic_value_bytes: &[u8], zero_based_line_number: usize) -> Result<u64, MemoryMapParseError>
	{
		u64::parse_decimal_number(statistic_value_bytes).map_err(|cause| CouldNotParseNumberField { zero_based_line_number, zero_based_field_index: 1, name: "statistic_value", cause })
	}

	#[inline(always)]
	fn parse_value_page_size(statistic_value_bytes: &[u8], zero_based_line_number: usize) -> Result<PageSizeOrHugePageSize, MemoryMapParseError>
	{
		let number = Self::parse_value_number(statistic_value_bytes, zero_based_line_number)?;
		PageSizeOrHugePageSize::from_kilobytes(number).ok_or(StatisticWasNotAPageSize { zero_based_line_number })
	}

	#[inline(always)]
	fn parse_value_bool(statistic_value_bytes: &[u8], zero_based_line_number: usize) -> Result<bool, MemoryMapParseError>
	{
		let number = Self::parse_value_number(statistic_value_bytes, zero_based_line_number)?;
		match number
		{
			0 => Ok(false),
			1 => Ok(true),
			_ => Err(StatisticWasNotABoolean { zero_based_line_number, number })
		}
	}

	#[inline(always)]
	fn box_statistic_name(statistic_name: &[u8]) -> Box<[u8]>
	{
		statistic_name.to_vec().into_boxed_slice()
	}

	#[inline(always)]
	fn mandatory<A>(field: Option<A>, name: &'static str) -> Result<A, MemoryMapParseError>
	{
		field.ok_or(MissingStatistic(name))
	}

	#[inline(always)]
	fn mandatory_and_does_not_exceed_size(field: Option<Kilobyte>, name: &'static str, size: Kilobyte) -> Result<Kilobyte, MemoryMapParseError>
	{
		let value = Self::mandatory(field, name)?;
		if unlikely!(value > size)
		{
			Err(ExceedsSize(name))
		}
		else
		{
			Ok(value)
		}
	}
}
