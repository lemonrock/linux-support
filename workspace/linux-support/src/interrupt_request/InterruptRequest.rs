// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a Linux interrupt request line (IRQ).
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct InterruptRequest(u8);

#[allow(missing_docs)]
impl From<u8> for InterruptRequest
{
	#[inline(always)]
	fn from(value: u8) -> Self
	{
		Self(value)
	}
}

#[allow(missing_docs)]
impl Into<u8> for InterruptRequest
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self.0
	}
}

impl ParseNumber for InterruptRequest
{
	#[inline(always)]
	fn parse_number(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Self, ParseNumberError>
	{
		u8::parse_number(bytes, radix, parse_byte).map(Self)
	}
}

#[allow(missing_docs)]
impl InterruptRequest
{
	/// All known interrupt request numbers.
	pub fn all(sys_path: &SysPath) -> io::Result<impl Iterator<Item=Self>>
	{
		fn map(dir_entry: io::Result<DirEntry>) -> Option<InterruptRequest>
		{
			let dir_entry = match dir_entry
			{
				Err(_) => return None,
				
				Ok(dir_entry) => dir_entry
			};
			
			match dir_entry.file_type()
			{
				Err(_) => return None,
				
				Ok(file_type) => if !file_type.is_dir()
				{
					return None
				}
			}
			
			let file_name = dir_entry.file_name().into_vec();
			u8::from_bytes(&file_name[..]).ok().map(|irq| InterruptRequest(irq))
		}
		
		Ok(sys_path.kernel_irq_folder_path().read_dir()?.filter_map(map))
	}
	
	/// Actions (Interrupt Names).
	///
	/// May be empty.
	///
	/// See also `self.procfs_actions()`.
	///
	/// Values observed on a Parallels VM:-
	///
	/// * (empty string).
	/// * `acpi`.
	/// * `ahci[0000:00:1f.2]`.
	/// * `ata_piix`.
	/// * `i8042`.
	/// * `rtc0`.
	/// * `timer`.
	/// * `virtio1`.
	/// * `virtio0-config`.
	/// * `virtio0-input.0`.
	/// * `virtio0-output.0`.
	#[inline(always)]
	pub fn sysfs_actions(self, sys_path: &SysPath) -> io::Result<Vec<InterruptRequestActionName>>
	{
		let file_path = self.sys_file_path(sys_path, "actions");
		
		match Self::raw_data_if_empty(file_path)?
		{
			None => Ok(Vec::new()),
			
			Some(raw_data_without_line_feed) => Ok
			(
				{
					let mut actions = Vec::new();
					for action in raw_data_without_line_feed.split_bytes(b',')
					{
						if !action.is_empty()
						{
							actions.push(unsafe { CString::from_vec_unchecked(action.to_vec()) });
						}
					}
					actions
				}
			)
		}
	}
	
	/// Actions (Interrupt Names).
	///
	/// May be empty.
	///
	/// See also `self.sysfs_actions()`.
	pub fn procfs_actions(self, proc_path: &ProcPath) -> io::Result<Vec<InterruptRequestActionName>>
	{
		let folder_path = self.proc_folder_path(proc_path);
		let read_dir = folder_path.read_dir()?;
		let mut actions = Vec::new();
		for dir_entry in read_dir
		{
			if let Ok(dir_entry) = dir_entry
			{
				if let Ok(file_type) = dir_entry.file_type()
				{
					if !file_type.is_dir()
					{
						continue
					}
					
					let action = dir_entry.file_name().os_str_to_c_string();
					actions.push(action);
				}
			}
		}
		Ok(actions)
	}
	
	
	/// Chip name.
	///
	/// Values observed on a Parallels VM:-
	///
	/// * `IO-APIC`.
	/// * `PCI-MSI`.
	/// * `XT-PIC`.
	#[inline(always)]
	pub fn chip_name(self, sys_path: &SysPath) -> io::Result<Option<CString>>
	{
		let file_path = self.sys_file_path(sys_path, "chip_name");
		Ok
		(
			Self::raw_data_if_empty(file_path)?.map(|raw_data_without_line_feed|
			{
				unsafe { CString::from_vec_unchecked(raw_data_without_line_feed) }
			})
		)
	}
	
	/// Hardware interrupt request line.
	///
	/// eg `10`; usually the same value as `self` but can be a large value, eg `512000`.
	///
	/// Note that `0xFFFF_FFFF` is invalid (this is not an error).
	#[inline(always)]
	pub fn hardware_interrupt_request_line(self, sys_path: &SysPath) -> io::Result<Option<u32>>
	{
		let file_path = self.sys_file_path(sys_path, "hwirq");
		
		match Self::raw_data_if_empty(file_path)?
		{
			None => Ok(None),
			
			Some(bytes) =>
			{
				let value = i32::from_bytes(&bytes[..]).map_err(io_error_invalid_data)?;
				Ok(Some(value as u32))
			}
		}
	}
	
	/// Name.
	///
	/// Values observed on a Parallels VM:-
	///
	/// * `Some(edge)`.
	/// * `Some(fasteoi)`.
	#[inline(always)]
	pub fn name(self, sys_path: &SysPath) -> io::Result<Option<CString>>
	{
		let file_path = self.sys_file_path(sys_path, "name");
		Ok
		(
			Self::raw_data_if_empty(file_path)?.map(|raw_data_without_line_feed|
			{
				unsafe { CString::from_vec_unchecked(raw_data_without_line_feed) }
			})
		)
	}
	
	/// Type.
	#[inline(always)]
	pub fn type_(self, sys_path: &SysPath) -> io::Result<InterruptRequestType>
	{
		let file_path = self.sys_file_path(sys_path, "type");
		file_path.read_value()
	}
	
	/// Wake up.
	#[inline(always)]
	pub fn wake_up(self, sys_path: &SysPath) -> io::Result<InterruptRequestWakeUp>
	{
		let file_path = self.sys_file_path(sys_path, "wakeup");
		file_path.read_value()
	}
	
	/// Number of occurrences per-HyperThread.
	///
	/// Number of indices is the number of possible HyperThreads (`/sys/devices/system/cpu/possible`).
	#[inline(always)]
	pub fn occurrences_per_hyper_thread(self, sys_path: &SysPath) -> io::Result<PerBitSetAwareData<HyperThread, u64>>
	{
		let file_path = self.sys_file_path(sys_path, "per_cpu_count");
		let comma_separated_string = file_path.read_raw_without_line_feed()?;
		
		#[inline(always)]
		fn mapper((index, count_in_bytes): (usize, &[u8])) -> Result<(HyperThread, u64), BitSetAwareTryFromU16Error>
		{
			let count = u64::parse_decimal_number(count_in_bytes)?;
			let hyper_thread = HyperThread::try_from(index)?;
			Ok((hyper_thread, count))
		}
		
		let constructor = comma_separated_string.split_bytes(b',').enumerate().map(mapper);
		let result = PerBitSetAwareData::from_iterator(constructor);
		result.map_err(io_error_invalid_data)
	}
	
	/// Usually `ffffffff` (ie `/sys/devices/system/cpu/possible` but as a bitmask not a list).
	#[inline(always)]
	pub fn default_smp_affinity(proc_path: &ProcPath) -> io::Result<HyperThreads>
	{
		proc_path.irq_file_path("default_smp_affinity").parse_comma_separated_bit_set().map(HyperThreads)
	}
	
	#[inline(always)]
	pub fn set_default_smp_affinity(proc_path: &ProcPath, affinity: &HyperThreads) -> io::Result<()>
	{
		affinity.set_affinity(proc_path.irq_file_path("default_smp_affinity"))
	}
	
	#[inline(always)]
	pub fn affinity_hint(self, proc_path: &ProcPath) -> HyperThreads
	{
		self.get_hyper_threads(proc_path, "affinity_hint")
	}
	
	#[inline(always)]
	pub fn effective_affinity(self, proc_path: &ProcPath) -> HyperThreads
	{
		self.get_hyper_threads(proc_path, "effective_affinity")
	}
	
	#[inline(always)]
	pub fn effective_affinity_list(self, proc_path: &ProcPath) -> HyperThreads
	{
		self.get_hyper_threads_list(proc_path, "effective_affinity_list")
	}
	
	/// Returns `None` if not configured for NUMA.
	#[inline(always)]
	pub fn numa_node(self, proc_path: &ProcPath) -> Option<NumaNode>
	{
		let file_path = self.proc_file_path(proc_path, "node");
		if file_path.exists()
		{
			Some(file_path.read_value().unwrap())
		}
		else
		{
			None
		}
	}
	
	#[inline(always)]
	pub fn set_smp_affinity(self, proc_path: &ProcPath, affinity: &HyperThreads) -> io::Result<()>
	{
		affinity.set_affinity(self.proc_file_path(proc_path, "smp_affinity"))
	}
	
	#[inline(always)]
	pub fn smp_affinity(self, proc_path: &ProcPath) -> HyperThreads
	{
		self.get_hyper_threads(proc_path, "smp_affinity")
	}
	
	#[inline(always)]
	pub fn set_smp_affinity_list(self, proc_path: &ProcPath, affinity: &HyperThreads) -> io::Result<()>
	{
		affinity.set_affinity_list(self.proc_file_path(proc_path, "smp_affinity_list"))
	}
	
	#[inline(always)]
	pub fn smp_affinity_list(self, proc_path: &ProcPath) -> HyperThreads
	{
		self.get_hyper_threads_list(proc_path, "smp_affinity_list")
	}
	
	/// Returns `count`, `unhandled` and `last_unhandled_milliseconds`.
	#[inline(always)]
	pub fn spurious(self, proc_path: &ProcPath) -> io::Result<SpuriousInterruptRequestInformation>
	{
		let file_path = self.proc_file_path(proc_path, "spurious");
		
		let data = file_path.read_raw()?;
		let mut lines = data.split_bytes_n(3, b'\n');
		
		fn parse_line<'a>(lines: &mut impl Iterator<Item=&'a [u8]>, expected_field_name: &'static [u8], ends_with: &'static [u8]) -> io::Result<usize>
		{
			let line = lines.next().unwrap();
			let mut fields = line.split_bytes_n(2, b' ');
			let field_name = fields.next().unwrap();
			if unlikely!(field_name != expected_field_name)
			{
				return Err(io_error_other("Invalid count field name"))
			}
			let field_value_and_unit = fields.next().unwrap();
			if unlikely!(!field_value_and_unit.ends_with(ends_with))
			{
				return Err(io_error_other( "Does not end with unit expected"))
			}
			let field_value = &field_value_and_unit[ .. field_value_and_unit.len() - ends_with.len()];
			usize::from_bytes(field_value).map_err(|_| io_error_other("Invalid field value"))
		}
		
		let count = parse_line(&mut lines, b"count", b"")?;
		let unhandled = parse_line(&mut lines, b"unhandled", b"")?;
		let last_unhandled_milliseconds = parse_line(&mut lines, b"last_unhandled", b" ms")?;
		
		Ok
		(
			SpuriousInterruptRequestInformation
			{
				count,
				unhandled,
				last_unhandled_milliseconds,
			}
		)
	}
	
	#[inline(always)]
	fn get_hyper_threads(self, proc_path: &ProcPath, file_name: &str) -> HyperThreads
	{
		HyperThreads(self.proc_file_path(proc_path, file_name).parse_comma_separated_bit_set().unwrap())
	}
	
	#[inline(always)]
	fn get_hyper_threads_list(self, proc_path: &ProcPath, file_name: &str) -> HyperThreads
	{
		HyperThreads(self.proc_file_path(proc_path, file_name).read_hyper_thread_or_numa_node_list().unwrap())
	}
	
	// Annoningly, Linux does not output "\n" for an empty value for actions and name but "".
	#[inline(always)]
	fn raw_data_if_empty(file_path: PathBuf) -> io::Result<Option<Vec<u8>>>
	{
		let raw = file_path.read_raw()?;
		let length = raw.len();
		if length == 0
		{
			Ok(None)
		}
		else
		{
			let mut vec = raw.into_vec();
			let should_be_line_feed = vec.remove(length - 1);
			if unlikely!(should_be_line_feed != b'\n')
			{
				Err(io_error_invalid_data("File lacks terminating line feed"))
			}
			else
			{
				Ok(Some(vec))
			}
		}
	}
	
	#[inline(always)]
	fn sys_file_path(self, sys_path: &SysPath, file_name: &str) -> PathBuf
	{
		sys_path.global_irq_file_path(self, file_name)
	}
	
	#[inline(always)]
	fn proc_file_path(self, proc_path: &ProcPath, file_name: &str) -> PathBuf
	{
		proc_path.irq_number_file_path(self, file_name)
	}
	
	#[inline(always)]
	fn proc_folder_path(self, proc_path: &ProcPath) -> PathBuf
	{
		proc_path.irq_number_folder_path(self)
	}
	
	#[inline(always)]
	pub(crate) fn file_name(self) -> String
	{
		format!("{}", self.0)
	}
}
