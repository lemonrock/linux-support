// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Parsing of a `/proc/<pid>/smaps` or `/proc/<pid>/numa_maps` file failed.
#[derive(Debug)]
pub enum MemoryMapParseError
{
	/// Mismatched lines in `/proc/<pid>/smaps` (or `/proc/<pid>/maps`), `/proc/<pid>/numa_maps` and`/sys/devices/system/online` .
	///
	/// This can occur legitimately in rare occassions where:-
	///
	/// * the list of maps has changed in reading one file and then another;
	/// * the list of online NUMA nodes has changed between reading it from `/sys/devices/system/online` and reading `/proc/<pid>/numa_maps`.
	///
	/// This is the ***ONLY*** recoverable error defined in `MemoryMapParseError`.
	///
	/// Only occurs if the kernel is build with `CONFIG_NUMA`.
	Mismatched
	{
		/// Explanation.
		explanation: &'static str,
	},

	/// Could not open a file.
	CouldNotOpenFile(io::Error),

	/// `/proc/<pid>/smaps` file does not exist.
	SmapsFileDoesNotExist,

	/// Could not read a line of data.
	CouldNotReadLine
	{
		/// Zero-based line number.
		zero_based_line_number: usize,

		/// Cause.
		cause: io::Error,
	},

	/// Missing a required field.
	MissingRequiredField
	{
		/// Zero-based line number.
		zero_based_line_number: usize,

		/// Zero-based field index.
		zero_based_field_index: usize,

		/// Name.
		name: &'static str,
	},

	/// Could not parse a field.
	CouldNotParseNumberField
	{
		/// Zero-based line number.
		zero_based_line_number: usize,

		/// Zero-based field index.
		zero_based_field_index: usize,

		/// Name.
		name: &'static str,

		/// Cause
		cause: ParseNumberError
	},

	/// Could not parse a memory policy field.
	MemoryPolicyUnrecognised
	{
		/// Zero-based line number.
		zero_based_line_number: usize,

		/// Cause
		unrecognised_memory_policy: Box<[u8]>,
	},

	/// Could not parse permissions field.
	PermissionsFieldIsWrongLength
	{
		/// Zero-based line number.
		zero_based_line_number: usize,

		/// Zero-based field index.
		zero_based_field_index: usize,
	},

	/// Could not parse permissions field.
	PermissionsFieldUnrecognised
	{
		/// Zero-based line number.
		zero_based_line_number: usize,

		/// Zero-based field index.
		zero_based_field_index: usize,

		/// Unrecognised.
		unrecognised: [u8; 3],
	},

	/// Could not parse permissions field.
	SharingFieldUnrecognised
	{
		/// Zero-based line number.
		zero_based_line_number: usize,

		/// Zero-based field index.
		zero_based_field_index: usize,

		/// Unrecognised.
		unrecognised: u8,
	},

	/// From is greater than or equal to To.
	FromAndToAreInvalid
	{
		/// Zero-based line number.
		zero_based_line_number: usize,

		/// From.
		from: VirtualAddress,

		/// To.
		to: VirtualAddress,
	},

	/// From is too small; usually indicates that a file is not sorted.
	FromTooSmall
	{
		/// Zero-based line number.
		zero_based_line_number: usize,

		/// From.
		from: VirtualAddress,

		/// To.
		to: VirtualAddress,
	},

	/// Unknown special file name starting `[`.
	UnknownSpecialFileName
	{
		/// Zero-based field index.
		zero_based_line_number: usize,

		/// Special file name starting `[`.
		special_file_name: Vec<u8>,
	},

	/// More than one mapping.
	RepeatedSpecialFileName
	{
		/// Zero-based field index.
		zero_based_line_number: usize,

		/// Known special file name.
		special_file_name: MemoryMapEntryKindSpecial,
	},

	/// Offset was not zero.
	OffsetWasNotZeroForAnonymous
	{
		/// Zero-based field index.
		zero_based_line_number: usize,

		/// Offset.
		offset: u32,
	},

	/// Block device major was not zero.
	BlockDeviceWasNotZeroZeroForAnonymous
	{
		/// Zero-based field index.
		zero_based_line_number: usize,

		/// Block device.
		block_device: BlockDevice,
	},

	/// Inode was not zero.
	InodeWasNotZeroForAnonymous
	{
		/// Zero-based field index.
		zero_based_line_number: usize,

		/// Inode.
		inode: Inode,
	},

	/// Offset was not zero.
	OffsetWasNotZeroForSpecialFileName
	{
		/// Zero-based field index.
		zero_based_line_number: usize,

		/// Known special file name.
		special_file_name: MemoryMapEntryKindSpecial,

		/// Offset.
		offset: u32,
	},

	/// Block device minor was not zero.
	BlockDeviceWasNotZeroZeroForSpecialFileName
	{
		/// Zero-based field index.
		zero_based_line_number: usize,

		/// Known special file name.
		special_file_name: MemoryMapEntryKindSpecial,

		/// Block device.
		block_device: BlockDevice,
	},

	/// Inode was not zero.
	InodeWasNotZeroForSpecialFileName
	{
		/// Zero-based field index.
		zero_based_line_number: usize,

		/// Known special file name.
		special_file_name: MemoryMapEntryKindSpecial,

		/// Inode.
		inode: Inode,
	},

	/// Protection was not expected.
	ProtectionWasNotExpectedForSpecialFileName
	{
		/// Zero-based field index.
		zero_based_line_number: usize,

		/// Known special file name.
		special_file_name: MemoryMapEntryKindSpecial,

		/// Protection.
		protection: Protection,

		/// Expected Protection.
		expected_protection: Protection,
	},

	/// Sharing was not private.
	SharingWasNotPrivateForSpecialFileName
	{
		/// Zero-based field index.
		zero_based_line_number: usize,

		/// Known special file name.
		special_file_name: MemoryMapEntryKindSpecial,

		/// Sharing.
		sharing: Sharing,
	},

	/// Missing `[stack]` mapping.
	MissingStackMapping,

	/// Missing `[vdso]` mapping.
	MissingVdsoMapping,

	/// Missing `[vvar]` mapping.
	MissingVvarMapping,

	/// Expected a `/proc/<pid>/smaps` or `/proc/<pid>/smaps_rollup` statistic line.
	ExpectedStatisticLine,

	/// Statistic missing colon.
	StatisticMissingColon
	{
		/// Zero-based field index.
		zero_based_line_number: usize,
	},

	/// Statistic missing space after colon.
	StatisticMissingSpaceAfterColon
	{
		/// Zero-based field index.
		zero_based_line_number: usize,
	},

	/// Statistic was not a page size.
	StatisticWasNotAPageSize
	{
		/// Zero-based field index.
		zero_based_line_number: usize,
	},

	/// Statistic was not a page size.
	StatisticWasNotABoolean
	{
		/// Zero-based field index.
		zero_based_line_number: usize,

		/// Value that was not 0 or 1.
		number: u64,
	},

	/// Duplicate statistic.
	DuplicateStatistic
	{
		/// Zero-based field index.
		zero_based_line_number: usize,

		/// Name.
		statistic_name: Box<[u8]>,
	},

	/// Statistic should have been in kB.
	StatisticWasNotKilobyte
	{
		/// Zero-based field index.
		zero_based_line_number: usize,

		/// Name.
		statistic_name: Box<[u8]>,
	},

	/// Statistic should have been a count.
	StatisticWasNotCount
	{
		/// Zero-based field index.
		zero_based_line_number: usize,

		/// Name.
		statistic_name: Box<[u8]>,
	},

	/// Missing statistic.
	MissingStatistic(&'static str),

	/// Size of statistic exceeds size of memory region.
	ExceedsSize(&'static str),

	/// Kernel page size and memory management unit (MMU) page size differ.
	#[cfg(not(target_arch = "powerpc64"))] KernelPageSizeAndMemoryManagementUnitPageSizeDiffer,

	/// Kernel page size should never be less than memory management unit (MMU) page size.
	#[cfg(target_arch = "powerpc64")] KernelPageSizeLessThanMemoryManagementUnitPageSize,

	/// Size does not match memory range.
	SizeDoesMatchMemoryRange,

	#[allow(missing_docs)]
	VmFlagsByteLengthNotAMultipleOfThree,

	#[allow(missing_docs)]
	UnrecognisedVmFlag([u8; 2]),

	#[allow(missing_docs)]
	DuplicateVmFlagPermission,

	#[allow(missing_docs)]
	DuplicateVmFlag(VmFlag),

	#[allow(missing_docs)]
	IllegalCombinationOfVmFlagPermissions,

	#[allow(missing_docs)]
	InvalidVmFlagProtection,

	#[allow(missing_docs)]
	InvalidVmFlagSharing,

	#[allow(missing_docs)]
	NumaMapKeyValueDidNotHaveEqualsSign,

	/// The mapmax value is only written by Linux if it exceeds 1.
	NumaMapMaxValueWasZeroOrOne,

	/// The active value is only written by Linux if it exceeds 0 and huge pages are not present.
	ActiveWasPresentWhenHugePagesArePresent,

	#[allow(missing_docs)]
	NumaMapMissingKernelPageSize,

	#[allow(missing_docs)]
	CouldNotParseNumaMapsKeyValueField
	{
		/// Cause
		cause: ParseNumberError
	},

	#[allow(missing_docs)]
	NumaMapKeyRepeated { name: &'static str },

	#[allow(missing_docs)]
	NumaMapNodeKeyRepeated { numa_node: NumaNode },

	#[allow(missing_docs)]
	UnknownSetMemoryPolicyHasDetails,

	#[allow(missing_docs)]
	UnknownSetMemoryPolicyReportedByLinuxIndicatesABugInLinux,

	#[allow(missing_docs)]
	DefaultOrLocalMemoryPolicyHasNumaNodes,

	#[allow(missing_docs)]
	PreferredMemoryPolicyHasNoNumaNodes,

	#[allow(missing_docs)]
	PreferredMemoryPolicyHasMoreThanOneNumaNode,

	#[allow(missing_docs)]
	UnrecognisedMemoryPolicyDynamism { value: Box<[u8]> },

	#[allow(missing_docs)]
	CouldNotParseMemoryPolicyNumaNodesList
	{
		/// Cause.
		cause: ListParseError
	},

	/// This code path might be possible in `show_numa_map()` in `task_mmu.c` as of Linux 5.4 but it seems that huge pages are never used for vDSO and VVAR maps.
	HugePagesForVdsoOrVvarMapping,
}

impl Display for MemoryMapParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl From<ListParseError> for MemoryMapParseError
{
	#[inline(always)]
	fn from(cause: ListParseError) -> Self
	{
		MemoryMapParseError::CouldNotParseMemoryPolicyNumaNodesList { cause }
	}
}

impl error::Error for MemoryMapParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::MemoryMapParseError::*;

		match self
		{
			&Mismatched { .. } => None,

			&CouldNotOpenFile(ref error) => Some(error),

			&SmapsFileDoesNotExist => None,

			&CouldNotReadLine { ref cause, .. } => Some(cause),

			&MissingRequiredField { .. } => None,

			&CouldNotParseNumberField { ref cause, .. } => Some(cause),

			&MemoryPolicyUnrecognised { .. } => None,

			&PermissionsFieldIsWrongLength { .. } => None,

			&PermissionsFieldUnrecognised { .. } => None,

			&SharingFieldUnrecognised { .. } => None,

			&FromAndToAreInvalid { .. } => None,

			&FromTooSmall { .. } => None,

			&UnknownSpecialFileName { .. } => None,

			&RepeatedSpecialFileName { .. } => None,

			&OffsetWasNotZeroForAnonymous { .. } => None,

			&BlockDeviceWasNotZeroZeroForAnonymous { .. } => None,

			&InodeWasNotZeroForAnonymous { .. } => None,

			&OffsetWasNotZeroForSpecialFileName { .. } => None,

			&BlockDeviceWasNotZeroZeroForSpecialFileName { .. } => None,

			&InodeWasNotZeroForSpecialFileName { .. } => None,

			&ProtectionWasNotExpectedForSpecialFileName { .. } => None,

			&SharingWasNotPrivateForSpecialFileName { .. } => None,

			&MissingStackMapping => None,

			&MissingVdsoMapping => None,

			&MissingVvarMapping => None,

			&ExpectedStatisticLine => None,

			&StatisticMissingColon { .. } => None,

			&StatisticMissingSpaceAfterColon { .. } => None,

			&StatisticWasNotAPageSize { .. } => None,

			&StatisticWasNotABoolean { .. } => None,

			&DuplicateStatistic { .. } => None,

			&StatisticWasNotKilobyte { .. } => None,

			&StatisticWasNotCount { .. } => None,

			&MissingStatistic(..) => None,

			&ExceedsSize(..) => None,

			#[cfg(not(target_arch = "powerpc64"))] &KernelPageSizeAndMemoryManagementUnitPageSizeDiffer => None,

			&SizeDoesMatchMemoryRange => None,

			&VmFlagsByteLengthNotAMultipleOfThree => None,

			&UnrecognisedVmFlag( .. ) => None,

			&DuplicateVmFlagPermission => None,

			&DuplicateVmFlag( .. ) => None,

			&IllegalCombinationOfVmFlagPermissions => None,

			&InvalidVmFlagProtection => None,

			&InvalidVmFlagSharing => None,

			&NumaMapKeyValueDidNotHaveEqualsSign => None,

			&NumaMapMaxValueWasZeroOrOne => None,

			&ActiveWasPresentWhenHugePagesArePresent => None,

			&NumaMapMissingKernelPageSize => None,

			&CouldNotParseNumaMapsKeyValueField { ref cause, .. } => Some(cause),

			&NumaMapKeyRepeated { .. } => None,

			&NumaMapNodeKeyRepeated { .. } => None,

			&UnknownSetMemoryPolicyHasDetails => None,

			&UnknownSetMemoryPolicyReportedByLinuxIndicatesABugInLinux => None,

			&DefaultOrLocalMemoryPolicyHasNumaNodes => None,

			&PreferredMemoryPolicyHasNoNumaNodes => None,

			&PreferredMemoryPolicyHasMoreThanOneNumaNode => None,

			&UnrecognisedMemoryPolicyDynamism { .. } => None,

			&CouldNotParseMemoryPolicyNumaNodesList { ref cause } => Some(cause),

			&HugePagesForVdsoOrVvarMapping => None,
		}
	}
}
