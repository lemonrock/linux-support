// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Status statistics for `/proc/self/status` or `/proc/<identifier>/status`.
///
/// `VmPMD` is not tested for (it was removed in Linux 4.15).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Status
{
	/// Process name.
	///
	/// Known as `Name`.
	pub process_name: Box<[u8]>,

	/// File creation mode mask (`umask`).
	///
	/// Known as `Umask`.
	///
	/// Since Linux 4.7.
	pub file_mode_creation_mask: mode_t,

	/// State.
	///
	/// Known as `State`.
	pub state: ProcessState,

	/// Thread group identifier.
	///
	/// Known as `Tgid`.
	pub thread_group_identifier: ProcessIdentifier,

	/// NUMA group identifier.
	///
	/// Known as `Ngid`.
	///
	/// None if the Linux kernel wasn't configured with `CONFIG_NUMA_BALANCING`.
	///
	/// Sincee Linux 3.13.
	pub numa_group_identifier: Option<ProcessIdentifier>,

	/// Process identifier.
	///
	/// Known as `Pid`.
	pub process_identifier: ProcessIdentifier,

	/// Parent process identifier.
	///
	/// Can be zero if this is the `init` process (process `1`).
	///
	/// Known as `PPid`.
	pub parent_process_identifier: Option<ProcessIdentifier>,

	/// Usually zero, implying no tracer process.
	///
	/// Known as `TracerPid`.
	pub tracer_process_identifier: Option<ProcessIdentifier>,

	/// User identifiers.
	///
	/// Known as `Uid`.
	pub user_identifiers: UserIdentifiers,

	/// Group identifiers.
	///
	/// Known as `Gid`.
	pub group_identifiers: GroupIdentifiers,

	/// Number of file descriptor slots currently allocated.
	///
	/// Known as `FDSize`.
	///
	/// eg `64`.
	pub number_of_file_descriptor_slots_currently_allocated: u64,

	/// Other group memberships.
	///
	/// Known as `Groups`.
	///
	/// Seems to always contain at least one member, which is the same as the primary group of the user.
	pub groups: Groups,

	/// Descendant namespace thread group identifiers.
	///
	/// Known as `NStgid`.
	///
	/// The leftmost entry shows the value with respect to the PID namespace of the process that mounted this procfs (or the root namespace if mounted by the kernel), followed by the value in successively nested inner namespaces.
	///
	/// Since Linux 4.1.
	pub descendant_namespace_thread_group_identifier: NestedProcessIdentifiers,

	/// Descendant namespace process identifiers.
	///
	/// Known as `NSpid`.
	///
	/// The leftmost entry shows the value with respect to the PID namespace of the process that mounted this procfs (or the root namespace if mounted by the kernel), followed by the value in successively nested inner namespaces.
	///
	/// Since Linux 4.1.
	pub descendant_namespace_process_identifier: NestedProcessIdentifiers,

	/// Descendant namespace process group identifiers.
	///
	/// Known as `NSpgid`.
	///
	/// The leftmost entry shows the value with respect to the PID namespace of the process that mounted this procfs (or the root namespace if mounted by the kernel), followed by the value in successively nested inner namespaces.
	///
	/// Since Linux 4.1.
	pub descendant_namespace_process_group_identifier: NestedProcessGroupIdentifiers,

	/// Descendant namespace session identifiers.
	///
	/// Known as `NSsid`.
	///
	/// The leftmost entry shows the value with respect to the PID namespace of the process that mounted this procfs (or the root namespace if mounted by the kernel), followed by the value in successively nested inner namespaces.
	///
	/// Since Linux 4.1.
	pub descendant_namespace_session_identifier: NestedProcessGroupIdentifiers,

	/// Peak virtual memory size.
	///
	/// Known as `VmPeak`.
	pub peak_virtual_memory_size: Kilobyte,

	/// Total program size.
	///
	/// Known as `VmSize`.
	///
	/// Also present as first column in `/proc/<N>/statm` where it is measured in number of pages.
	pub total_program_size: Kilobyte,

	/// Locked memory size.
	///
	/// Known as `VmLck`.
	///
	/// See `man 3 lock`.
	pub locked_memory_size: Kilobyte,

	/// Pinned memory size.
	///
	/// Known as `VmPin`.
	///
	/// These are pages that can't be moved because something needs to directly access physical memory.
	///
	/// Since Linux 3.2.
	pub pinned_memory_size: Kilobyte,

	/// Peak resident set size ("High Water Mark").
	///
	/// Known as `VmHWM`.
	pub peak_resident_set_size: Kilobyte,

	/// The sum of `anonymous_resident_set_memory_size`, `resident_set_file_mappings_memory_size` and `resident_set_shared_memory_size`.
	///
	/// Known as `VmRSS`.
	///
	/// Also present as second column in `/proc/<N>/statm` where it is measured in number of pages.
	pub resident_set_memory_size: Kilobyte,

	/// Size of resident set anonymous memory.
	///
	/// Known as `RssAnon`.
	///
	/// Since Linux 4.5.
	pub anonymous_resident_set_memory_size: Kilobyte,

	/// Size of resident set file mappings.
	///
	/// Known as `RssFile`.
	///
	/// Since Linux 4.5.
	///
	/// This plus `RssShmem` is also present as the third column in `/proc/<N>/statm` where it is measured in number of pages.
	pub resident_set_file_mappings_memory_size: Kilobyte,

	/// Size of resident set shared memory (`shmem`).
	///
	/// Known as `RssShmem`.
	///
	/// Includes Sys_v `shm`, any mappings from `tmpfs` and shared anonymous mappings.
	///
	/// Since Linux 4.5.
	///
	/// This plus `RssFile` is also present as the third column in `/proc/<N>/statm` where it is measured in number of pages.
	pub resident_set_shared_memory_size: Kilobyte,

	/// Size of private data segments.
	///
	/// Known as `VmData`.
	///
	/// This plus `VmStk` is also present as the fifth column in `/proc/<N>/statm` where it is measured in number of pages.
	pub private_data_segments_size: Kilobyte,

	/// Size of stack segments.
	///
	/// Known as `VmStk`.
	///
	/// This plus `VmData` is also present as the fifth column in `/proc/<N>/statm` where it is measured in number of pages.
	pub stack_segments_size: Kilobyte,

	/// Size of text (program code) segment.
	///
	/// Known as `VmExe`.
	///
	/// Also present as fourth column in `/proc/<N>/statm` where it is measured in number of pages.
	pub text_segment_size: Kilobyte,

	/// Size of shared library code.
	///
	/// Known as `VmLib`.
	pub dynamically_loaded_shared_library_size: Kilobyte,

	/// Size of page table entries.
	///
	/// Known as `VmPTE`.
	///
	/// Since Linux 2.6.10.
	pub page_table_entries_size: Kilobyte,

	/// The amount of swap used by anonymous private data.
	///
	/// Known as `VmSwap`.
	///
	/// Shared memory `shmem` swap usage is not included.
	///
	/// Since Linux 2.6.34.
	pub swap_memory_size: Kilobyte,

	/// Size of `hugetlb` memory portions.
	///
	/// Known as `HugetlbPages`.
	///
	/// Since Linux 4.4.
	pub huge_tlb_pages_memory_size: Kilobyte,

	/// Is the process currently dumping core?
	///
	/// Known as `CoreDumping`.
	///
	/// This information can be used by a monitoring process to avoid killing a process that is currently dumping core.
	/// Killing a process whilst it is dumping core could result in a corrupted core dump file.
	///
	/// Since Linux 4.15.
	pub currently_dumping_core: Option<bool>,

	/// Number of threads.
	///
	/// Known as `Threads`.
	pub threads: u64,

	/// Signal queue status.
	///
	/// Known as `SigQ`.
	pub signal_queue: SignalQueueStatus,

	/// Pending signals for the thread.
	///
	/// Known as `SigPnd`.
	pub thread_pending_signals: BitSet<Signal>,

	/// Shared pending signals for the process.
	///
	/// Known as `ShdPnd`.
	pub process_shared_pending_signals: BitSet<Signal>,

	/// Blocked signals.
	///
	/// Known as `SigBlk`.
	pub blocked_signals: BitSet<Signal>,

	/// Ignored signals.
	///
	/// Known as `SigIgn`.
	pub ignored_signals: BitSet<Signal>,

	/// Caught signals.
	///
	/// Known as `SigCgt`.
	pub caught_signals: BitSet<Signal>,

	/// Inheritable capabilities.
	///
	/// Known as `CapInh`.
	pub inheritable_capabilities_mask: BitSet<Capability>,

	/// Permitted capabilities.
	///
	/// Known as `CapPrm`.
	pub permitted_capabilities_mask: BitSet<Capability>,

	/// Effective capabilities.
	///
	/// Known as `CapEff`.
	pub effective_capabilities_mask: BitSet<Capability>,

	/// Capabilities bounding set.
	///
	/// Known as `CapBnd`.
	///
	/// Since Linux 2.6.26.
	pub capabilities_bounding_set: BitSet<Capability>,

	/// Ambient capabilities.
	///
	/// Known as `CapAmb`.
	///
	/// Since Linux 4.3.
	pub ambient_capabilities_set: BitSet<Capability>,

	/// Thread's `no_new_privs` bit (see `man 2 prctl` description for `PR_GET_NO_NEW_PRIVS`).
	///
	/// Known as `NoNewPrivs`.
	///
	/// Since Linux 4.10.
	pub thread_no_new_privileges_bit: Option<bool>,

	/// Seccomp mode.
	///
	/// Known as `Seccomp`.
	///
	/// This field is provided only if the kernel was built with the `CONFIG_SECCOMP` kernel configuration option enabled.
	///
	/// Since Linux 3.8.
	pub seccomp_mode: SeccompMode,

	/// Speculation store ('Spectre' vulnerability) bypass status.
	///
	/// Known as `Speculation_Store_Bypass`.
	///
	/// Since Linux 4.17.
	///
	/// Problematic for Ubuntu Server LTS 18.04, which uses Linux 4.15.
	pub speculation_store_bypass: Option<SpeculationStoreBypassStatus>,

	/// CPUs (actually, hyper threaded cores) allowed for the current process.
	///
	/// Known as `Cpus_allowed`.
	///
	/// May have bits set well beyond those than the number of cores on the system.
	///
	/// Tuples of 32-bit, LSB to the far right, eg `ffffffff,ffffffff,ffffffff,ffffffff`.
	///
	/// Since Linux 2.6.24.
	pub cpus_allowed: BitSet<HyperThread>,

	/// CPUs (actually, hyper threaded cores) allowed for the current process.
	///
	/// Known as `Cpus_allowed_list`.
	///
	/// Since Linux 2.6.26.
	pub cpus_allowed_list: BitSet<HyperThread>,

	/// NUMA nodes allowed for the current process.
	///
	/// Known as `Mems_allowed`.
	///
	/// Linux defines the config option `NODES_SHIFT` (aka `CONFIG_NODES_SHIFT`) to be 1 to 10 if defined and 0 if not defined, giving a maximum of 2^10 (1024) NUMA nodes, if defaults to 6 (ie 64 NUMA nodes) on x86_64.
	///
	/// Tuples of 32-bit, LSB to the far right, eg `00000000,00000001`.
	///
	/// Since Linux 2.6.24.
	pub numa_nodes_allowed: BitSet<NumaNode>,

	/// NUMA nodes allowed for the current process.
	///
	/// Known as `Mems_allowed_list`.
	///
	/// If the Linux kernel wasn't configured with `CONFIG_NUMA`, defaults to 0.
	///
	/// Since Linux 2.6.26.
	pub numa_nodes_allowed_list: BitSet<NumaNode>,

	/// Voluntary context switches.
	///
	/// Known as `voluntary_ctxt_switches`.
	///
	/// Since Linux 2.6.23.
	pub voluntary_context_switches: u64,

	/// Involuntary context switches.
	///
	/// Known as `nonvoluntary_ctxt_switches`.
	///
	/// Since Linux 2.6.23.
	pub involuntary_context_switches: u64,

	/// May include:-
	///
	/// * `VmPMD`: Size of second-level page tables (added in Linux 4.0; removed in Linux 4.15).
	/// * `THP_enabled`: Undocumented.
	unrecognised: HashMap<Box<[u8]>, Box<[u8]>>,
}

impl Status
{
	/// Get an unrecognised static's value using a `statistic_name` byte string.
	#[inline(always)]
	pub fn unrecognised_statistic(&self, statistic_name: &[u8]) -> Option<&Box<[u8]>>
	{
		self.unrecognised.get(statistic_name)
	}

	/// Status information from `/proc/self/status`.
	#[inline(always)]
	pub fn self_status(proc_path: &ProcPath) -> Result<Self, StatusFileParseError>
	{
		Self::process_status(proc_path, ProcessIdentifierChoice::Current)
	}

	/// Status information from `/proc/<IDENTIFIER>/status` where `<IDENTIFIER>` is `process_identifier`.
	///
	/// When in doubt, check the source code for status files at <https://github.com/torvalds/linux/blob/f346b0becb1bc62e45495f9cdbae3eef35d0b635/fs/proc/array.c> and the documentation at <http://man7.org/linux/man-pages/man5/proc.5.html>.
	#[inline(always)]
	pub fn process_status(proc_path: &ProcPath, process_identifier: ProcessIdentifierChoice) -> Result<Self, StatusFileParseError>
	{
		macro_rules! parse
		{
			($reader: ident, $this: ident, $($proc_status_name: literal => $struct_field: ident @ $parse: ident,)*) =>
			{
				use self::StatusFileParseError::*;
				use self::StatusStatisticParseError::*;

				$(
					let mut $struct_field: bool = false;
				)*

				let mut zero_based_line_number = 0;
				for line in $reader.split(b'\n')
				{
					let line = line.map_err(|cause| CouldNotReadLine { zero_based_line_number, cause })?;
					let mut split = line.splitn(2, |byte| *byte == b':');

					let statistic_name = split.next().unwrap();
					let tab_then_statistic_value = split.next().ok_or(CouldNotParseLine { zero_based_line_number, cause: NoValue })?;
					let statistic_value = if likely!(tab_then_statistic_value.starts_with(b"\t"))
					{
						&tab_then_statistic_value[1 .. ]
					}
					else
					{
						return Err(CouldNotParseLine { zero_based_line_number, cause: ValueNotPreceededByHorizontalTab })
					};

					match statistic_name
					{
						$(
							$proc_status_name => if unlikely!($struct_field)
							{
								return Err(CouldNotParseLine { zero_based_line_number, cause: DuplicatedStatistic })
							}
							else
							{
								$struct_field = true;

								let result = Self::$parse(statistic_value).map_err(|cause| CouldNotParseLine { zero_based_line_number, cause})?;
								unsafe { write(&mut $this.$struct_field, result) };
							},
						)*

						_ => $this.parse_optional_statistics_and_future_statistics(statistic_name, statistic_value, zero_based_line_number)?,
					}

					$(
						if unlikely!(!$struct_field)
						{
							return Err(MissingRequiredField(stringify!($struct_field)))
						}
					)*

					zero_based_line_number += 1;
				}
			}
		}

		let file_path = proc_path.process_file_path(process_identifier, "status");
		let reader = BufReader::with_capacity(4096, File::open(file_path)?);

		#[allow(deprecated, invalid_value)] let mut this: Self = unsafe { uninitialized() };
		unsafe { write(&mut this.currently_dumping_core, None) };
		unsafe { write(&mut this.thread_no_new_privileges_bit, None) };
		unsafe { write(&mut this.speculation_store_bypass, None) };
		unsafe { write(&mut this.unrecognised, HashMap::default())};

		parse!
		(
			reader, this,

			b"Name" => process_name @ parse_token,
			b"Umask" => file_mode_creation_mask @ parse_mode,
			b"State" => state @ parse_process_state,
			b"Tgid" => thread_group_identifier @ parse_process_identifier,
			b"Ngid" => numa_group_identifier @ parse_maybe_zero_process_identifier,
			b"Pid" => process_identifier @ parse_process_identifier,
			b"PPid" => parent_process_identifier @ parse_maybe_zero_process_identifier,
			b"TracerPid" => tracer_process_identifier @ parse_maybe_zero_process_identifier,
			b"Uid" => user_identifiers @ parse_user_identifiers,
			b"Gid" => group_identifiers @ parse_group_identifiers,
			b"FDSize" => number_of_file_descriptor_slots_currently_allocated @ parse_u64,
			b"Groups" => groups @ parse_groups,
			b"NStgid" => descendant_namespace_thread_group_identifier @ parse_process_identifiers,
			b"NSpid" => descendant_namespace_process_identifier @ parse_process_identifiers,
			b"NSpgid" => descendant_namespace_process_group_identifier @ parse_process_group_identifiers,
			b"NSsid" => descendant_namespace_session_identifier @ parse_process_group_identifiers,
			b"VmPeak" => peak_virtual_memory_size @ parse_kilobyte,
			b"VmSize" => total_program_size @ parse_kilobyte,
			b"VmLck" => locked_memory_size @ parse_kilobyte,
			b"VmPin" => pinned_memory_size @ parse_kilobyte,
			b"VmHWM" => peak_resident_set_size @ parse_kilobyte,
			b"VmRSS" => resident_set_memory_size @ parse_kilobyte,
			b"RssAnon" => anonymous_resident_set_memory_size @ parse_kilobyte,
			b"RssFile" => resident_set_file_mappings_memory_size @ parse_kilobyte,
			b"RssShmem" => resident_set_shared_memory_size @ parse_kilobyte,
			b"VmData" => private_data_segments_size @ parse_kilobyte,
			b"VmStk" => stack_segments_size @ parse_kilobyte,
			b"VmExe" => text_segment_size @ parse_kilobyte,
			b"VmLi" => dynamically_loaded_shared_library_size @ parse_kilobyte,
			b"VmPTE" => page_table_entries_size @ parse_kilobyte,
			b"VmSwap" => swap_memory_size @ parse_kilobyte,
			b"HugetlbPages" => huge_tlb_pages_memory_size @ parse_kilobyte,
			b"Threads" => threads @ parse_u64,
			b"SigQ" => signal_queue @ parse_signal_queue,
			b"SigPnd" => thread_pending_signals @ parse_signal_bit_set,
			b"ShdPnd" => process_shared_pending_signals @ parse_signal_bit_set,
			b"SigBlk" => blocked_signals @ parse_signal_bit_set,
			b"SigIgn" => ignored_signals @ parse_signal_bit_set,
			b"SigCgt" => caught_signals @ parse_signal_bit_set,
			b"CapInh" => inheritable_capabilities_mask @ parse_capability_mask_or_set,
			b"CapPrm" => permitted_capabilities_mask @ parse_capability_mask_or_set,
			b"CapEff" => effective_capabilities_mask @ parse_capability_mask_or_set,
			b"CapBnd" => capabilities_bounding_set @ parse_capability_mask_or_set,
			b"CapAm" => ambient_capabilities_set @ parse_capability_mask_or_set,
			b"Seccomp" => seccomp_mode @ parse_seccomp_mode,
			b"Cpus_allowed" => cpus_allowed @ parse_cpus_or_numa_nodes_allowed,
			b"Cpus_allowed_list" => cpus_allowed_list @ parse_cpus_allowed_list,
			b"Mems_allowed" => numa_nodes_allowed @ parse_cpus_or_numa_nodes_allowed,
			b"Mems_allowed_list" => numa_nodes_allowed_list @ parse_numa_nodes_allowed_list,
			b"voluntary_ctxt_switches" => voluntary_context_switches @ parse_u64,
			b"nonvoluntary_ctxt_switches" => involuntary_context_switches @ parse_u64,
		);

		if cfg!(debug_assertions)
		{
			if this.parent_process_identifier.is_none()
			{
				debug_assert!(!this.process_identifier.should_have_parent())
			}
			else
			{
				debug_assert!(this.process_identifier.should_have_parent())
			}
		}
		debug_assert!(!this.cpus_allowed.is_empty());
		debug_assert_eq!(this.cpus_allowed, this.cpus_allowed_list);
		debug_assert_eq!(this.numa_nodes_allowed, this.numa_nodes_allowed_list);

		this.unrecognised.shrink_to_fit();
		Ok(this)
	}

	#[inline(always)]
	fn parse_optional_statistics_and_future_statistics(&mut self, statistic_name: &[u8], statistic_value: &[u8], zero_based_line_number: usize) -> Result<(), StatusFileParseError>
	{
		use self::StatusFileParseError::*;
		use self::StatusStatisticParseError::DuplicatedStatistic;;

		#[inline(always)]
		fn parse_optional_statistic<S>(statistic: &mut Option<S>, statistic_value: &[u8], zero_based_line_number: usize, parse: impl FnOnce(&[u8]) -> Result<S, StatusStatisticParseError>) -> Result<(), StatusFileParseError>
		{
			if unlikely!(statistic.is_some())
			{
				Err(CouldNotParseLine { zero_based_line_number, cause: DuplicatedStatistic })
			}
			else
			{
				let result = parse(statistic_value).map_err(|cause| CouldNotParseLine { zero_based_line_number, cause})?;
				*statistic = Some(result);
				Ok(())
			}
		}

		match statistic_name
		{
			b"CoreDumping" => parse_optional_statistic(&mut self.currently_dumping_core, statistic_value, zero_based_line_number, Self::parse_bool),

			b"NoNewPrivs" => parse_optional_statistic(&mut self.thread_no_new_privileges_bit, statistic_value, zero_based_line_number, Self::parse_bool),

			b"Speculation_Store_Bypass" => parse_optional_statistic(&mut self.speculation_store_bypass, statistic_value, zero_based_line_number, Self::parse_speculation_store_bypass),

			_ => self.insert_unrecognised(statistic_name, statistic_value, zero_based_line_number),
		}
	}

	#[inline(always)]
	fn insert_unrecognised(&mut self, statistic_name: &[u8], statistic_value: &[u8], zero_based_line_number: usize) -> Result<(), StatusFileParseError>
	{
		let previous = self.unrecognised.insert(Self::to_box(statistic_name), Self::to_box(statistic_value));
		if unlikely!(previous.is_some())
		{
			return Err(StatusFileParseError::CouldNotParseLine { zero_based_line_number, cause: StatusStatisticParseError::DuplicatedStatistic })
		}
		Ok(())
	}

	#[inline(always)]
	fn to_box(value: &[u8]) -> Box<[u8]>
	{
		value.to_vec().into_boxed_slice()
	}

	#[inline(always)]
	fn parse_token(value: &[u8]) -> Result<Box<[u8]>, StatusStatisticParseError>
	{
		Ok(Self::to_box(value))
	}

	#[inline(always)]
	fn parse_mode(value: &[u8]) -> Result<mode_t, StatusStatisticParseError>
	{
		Ok(mode_t::parse_octal_number_fixed_width(value, 4)?)
	}

	#[inline(always)]
	fn parse_process_state(value: &[u8]) -> Result<ProcessState, StatusStatisticParseError>
	{
		Ok(ProcessState::from_bytes(value)?)
	}

	#[inline(always)]
	fn parse_process_identifier(value: &[u8]) -> Result<ProcessIdentifier, StatusStatisticParseError>
	{
		Ok(ProcessIdentifier::parse_decimal_number(value)?)
	}

	#[inline(always)]
	fn parse_maybe_zero_process_identifier(value: &[u8]) -> Result<Option<ProcessIdentifier>, StatusStatisticParseError>
	{
		Ok(Option::<ProcessIdentifier>::parse_decimal_number(value)?)
	}

	#[inline(always)]
	fn parse_user_identifiers(value: &[u8]) -> Result<UserIdentifiers, StatusStatisticParseError>
	{
		UserIdentifiers::from_bytes(value)
	}

	#[inline(always)]
	fn parse_group_identifiers(value: &[u8]) -> Result<GroupIdentifiers, StatusStatisticParseError>
	{
		GroupIdentifiers::from_bytes(value)
	}

	#[inline(always)]
	fn parse_groups(value: &[u8]) -> Result<Groups, StatusStatisticParseError>
	{
		Groups::from_bytes(value)
	}

	#[inline(always)]
	fn parse_process_identifiers(value: &[u8]) -> Result<NestedProcessIdentifiers, StatusStatisticParseError>
	{
		NestedProcessIdentifiers::from_bytes(value)
	}

	#[inline(always)]
	fn parse_process_group_identifiers(value: &[u8]) -> Result<NestedProcessGroupIdentifiers, StatusStatisticParseError>
	{
		NestedProcessGroupIdentifiers::from_bytes(value)
	}

	#[inline(always)]
	fn parse_u64(value: &[u8]) -> Result<u64, StatusStatisticParseError>
	{
		Ok(u64::parse_decimal_number(value)?)
	}

	#[inline(always)]
	fn parse_kilobyte(value: &[u8]) -> Result<Kilobyte, StatusStatisticParseError>
	{
		const Ending: &'static [u8] = b" kB";

		if likely!(value.ends_with(b" kB"))
		{
			Self::parse_u64(&value[0 .. value.len() - Ending.len()])
		}
		else
		{
			Err(StatusStatisticParseError::InvalidEnding)
		}
	}

	#[inline(always)]
	fn parse_signal_queue(value: &[u8]) -> Result<SignalQueueStatus, StatusStatisticParseError>
	{
		SignalQueueStatus::from_bytes(value)
	}

	#[inline(always)]
	fn parse_hexadecimal_u64(value: &[u8]) -> Result<u64, StatusStatisticParseError>
	{
		Ok(u64::parse_hexadecimal_number_lower_case(value)?)
	}

	#[inline(always)]
	fn parse_signal_bit_set(value: &[u8]) -> Result<BitSet<Signal>, StatusStatisticParseError>
	{
		Ok(BitSet::new_from_u64(Self::parse_hexadecimal_u64(value)?))
	}

	#[inline(always)]
	fn parse_capability_mask_or_set(value: &[u8]) -> Result<BitSet<Capability>, StatusStatisticParseError>
	{
		Ok(BitSet::new_from_u64(Self::parse_hexadecimal_u64(value)?))
	}

	#[inline(always)]
	fn parse_bool(value: &[u8]) -> Result<bool, StatusStatisticParseError>
	{
		if likely!(value.len() == 1)
		{
			match value[0]
			{
				b'0' => Ok(false),
				b'1' => Ok(true),
				_ => Err(StatusStatisticParseError::OutOfRange)
			}
		}
		else
		{
			Err(StatusStatisticParseError::InvalidLength)
		}
	}

	#[inline(always)]
	fn parse_seccomp_mode(value: &[u8]) -> Result<SeccompMode, StatusStatisticParseError>
	{
		SeccompMode::from_bytes(value)
	}

	#[inline(always)]
	fn parse_speculation_store_bypass(value: &[u8]) -> Result<SpeculationStoreBypassStatus, StatusStatisticParseError>
	{
		SpeculationStoreBypassStatus::from_bytes(value)
	}

	#[inline(always)]
	fn parse_cpus_or_numa_nodes_allowed<BSA: BitSetAware>(value: &[u8]) -> Result<BitSet<BSA>, StatusStatisticParseError>
	{
		Ok(BitSet::parse_hyper_thread_or_numa_node_bit_set(&value))
	}

	#[inline(always)]
	fn parse_cpus_allowed_list(value: &[u8]) -> Result<BitSet<HyperThread>, StatusStatisticParseError>
	{
		Ok(BitSet::<HyperThread>::parse_linux_list_string(value)?)
	}

	#[inline(always)]
	fn parse_numa_nodes_allowed_list(value: &[u8]) -> Result<BitSet<NumaNode>, StatusStatisticParseError>
	{
		Ok(BitSet::<NumaNode>::parse_linux_list_string(value)?)
	}
}
