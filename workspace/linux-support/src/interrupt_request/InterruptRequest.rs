// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a Linux interrupt request line (IRQ).
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct InterruptRequest(u8);

#[allow(missing_docs)]
impl Into<u8> for InterruptRequest
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self.0
	}
}

#[allow(missing_docs)]
impl InterruptRequest
{
	/// Actions.
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
	pub fn actions(self, sys_path: &SysPath) -> Box<[u8]>
	{
		let file_path = self.sys_file_path(sys_path, "actions");
		file_path.read_raw_without_line_feed().unwrap()
	}
	
	/// Chip name.
	///
	/// Values observed on a Parallels VM:-
	///
	/// * `IO-APIC`.
	/// * `PCI-MSI`.
	/// * `XT-PIC`.
	#[inline(always)]
	pub fn chip_name(self, sys_path: &SysPath) -> Box<[u8]>
	{
		let file_path = self.sys_file_path(sys_path, "chip_name");
		file_path.read_raw_without_line_feed().unwrap()
	}
	
	/// Hardware interrupt request line.
	///
	/// eg `10`; usually the same value as `self` but can be a large value, eg `512000`.
	#[inline(always)]
	pub fn hardware_interrupt_request_line(self, sys_path: &SysPath) -> u32
	{
		let file_path = self.sys_file_path(sys_path, "hwirq");
		file_path.read_value().unwrap()
	}
	
	/// Name.
	///
	/// Values observed on a Parallels VM:-
	///
	/// * `edge`.
	/// * `fasteoi`.
	#[inline(always)]
	pub fn name(self, sys_path: &SysPath) -> Box<[u8]>
	{
		let file_path = self.sys_file_path(sys_path, "name");
		file_path.read_raw_without_line_feed().unwrap()
	}
	
	/// Type.
	///
	/// Values observed on a Parallels VM:-
	///
	/// * `edge`.
	/// * `level`.
	#[inline(always)]
	pub fn type_(self, sys_path: &SysPath) -> Box<[u8]>
	{
		let file_path = self.sys_file_path(sys_path, "type");
		file_path.read_raw_without_line_feed().unwrap()
	}
	
	/// Wake up.
	///
	/// Values observed on a Parallels VM:-
	///
	/// * `disabled`.
	#[inline(always)]
	pub fn wake_up(self, sys_path: &SysPath) -> Box<[u8]>
	{
		let file_path = self.sys_file_path(sys_path, "wakeup");
		file_path.read_raw_without_line_feed().unwrap()
	}
	
	/// Number of occurrences per-HyperThread.
	///
	/// Number of indices is the number of possible HyperThreads (`/sys/devices/system/cpu/possible`).
	#[inline(always)]
	pub fn occurrences_per_hyper_thread(self, sys_path: &SysPath) -> PerBitSetAwareData<HyperThread, u64>
	{
		let file_path = self.sys_file_path(sys_path, "per_cpu_count");
		let comma_separated_string = file_path.read_raw_without_line_feed().unwrap();
		
		#[inline(always)]
		fn mapper((index, count_in_bytes): (usize, &[u8])) -> Result<(HyperThread, u64), BitSetAwareTryFromU16Error>
		{
			let count = u64::parse_decimal_number(count_in_bytes)?;
			let hyper_thread = HyperThread::try_from(index)?;
			Ok((hyper_thread, count))
		}
		
		let constructor = comma_separated_string.split_bytes(b',').enumerate().map(mapper);
		PerBitSetAwareData::from_iterator(constructor).expect("Invalid count or HyperThread")
	}
	
	/// Usually `ffffffff` (ie `/sys/devices/system/cpu/possible` but as a bitmask not a list).
	#[inline(always)]
	pub fn default_smp_affinity(proc_path: &ProcPath) -> HyperThreads
	{
		HyperThreads(proc_path.irq_file_path("default_smp_affinity").parse_hyper_thread_or_numa_node_bit_set().unwrap())
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
	pub fn spurious(self, proc_path: &ProcPath) -> io::Result<(usize, usize, usize)>
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
				return Err(io::Error::new(ErrorKind::Other, "Invalid count field name"))
			}
			let field_value_and_unit = fields.next().unwrap();
			if unlikely!(!field_value_and_unit.ends_with(ends_with))
			{
				return Err(io::Error::new(ErrorKind::Other, "Does not end with unit expected"))
			}
			let field_value = &field_value_and_unit[ .. field_value_and_unit.len() - ends_with.len()];
			usize::from_bytes(field_value).map_err(|_| io::Error::new(ErrorKind::Other, "Invalid field value"))
		}
		
		let count = parse_line(&mut lines, b"count", b"")?;
		let unhandled = parse_line(&mut lines, b"unhandled", b"")?;
		let last_unhandled_milliseconds = parse_line(&mut lines, b"last_unhandled", b" ms")?;
		
		Ok((count, unhandled, last_unhandled_milliseconds))
	}
	
	#[inline(always)]
	fn get_hyper_threads(self, proc_path: &ProcPath, file_name: &str) -> HyperThreads
	{
		HyperThreads(self.proc_file_path(proc_path, file_name).parse_hyper_thread_or_numa_node_bit_set().unwrap())
	}
	
	#[inline(always)]
	fn get_hyper_threads_list(self, proc_path: &ProcPath, file_name: &str) -> HyperThreads
	{
		HyperThreads(self.proc_file_path(proc_path, file_name).read_hyper_thread_or_numa_node_list().unwrap())
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
	pub(crate) fn file_name(self) -> String
	{
		format!("{}", self.0)
	}
}
