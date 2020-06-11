// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a Linux interrupt request line (IRQ).
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct InterruptRequest(u8);

#[allow(missing_docs)]
impl InterruptRequest
{
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
		let file_path = self.file_path(proc_path, "node");
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
		affinity.set_affinity(self.file_path(proc_path, "smp_affinity"))
	}
	
	#[inline(always)]
	pub fn smp_affinity(self, proc_path: &ProcPath) -> HyperThreads
	{
		self.get_hyper_threads(proc_path, "smp_affinity")
	}
	
	#[inline(always)]
	pub fn set_smp_affinity_list(self, proc_path: &ProcPath, affinity: &HyperThreads) -> io::Result<()>
	{
		affinity.set_affinity_list(self.file_path(proc_path, "smp_affinity_list"))
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
		let file_path = self.file_path(proc_path, "spurious");
		
		let data = file_path.read_raw()?;
		let mut lines = (&data[..]).splitn(3, |byte| *byte == b'\n');
		
		fn parse_line<'a>(lines: &mut impl Iterator<Item=&'a [u8]>, expected_field_name: &'static [u8], ends_with: &'static [u8]) -> io::Result<usize>
		{
			let line = lines.next().unwrap();
			let mut fields = line.splitn(2, |byte| *byte == b' ');
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
	pub(crate) fn file_name(self) -> String
	{
		format!("{}", self.0)
	}
	
	#[inline(always)]
	fn get_hyper_threads(self, proc_path: &ProcPath, file_name: &str) -> HyperThreads
	{
		HyperThreads(self.file_path(proc_path, file_name).parse_hyper_thread_or_numa_node_bit_set().unwrap())
	}
	
	#[inline(always)]
	fn get_hyper_threads_list(self, proc_path: &ProcPath, file_name: &str) -> HyperThreads
	{
		HyperThreads(self.file_path(proc_path, file_name).read_hyper_thread_or_numa_node_list().unwrap())
	}
	
	#[inline(always)]
	fn file_path(self, proc_path: &ProcPath, file_name: &str) -> PathBuf
	{
		proc_path.irq_number_file_path(self, file_name)
	}
}
