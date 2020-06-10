// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// `BitSet` of `HyperThread`.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct HyperThreads(pub BitSet<HyperThread>);

impl Deref for HyperThreads
{
	type Target = BitSet<HyperThread>;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl DerefMut for HyperThreads
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.0
	}
}

impl<'a> From<&'a cpu_set_t> for HyperThreads
{
	#[inline(always)]
	fn from(value: &'a cpu_set_t) -> Self
	{
		HyperThreads(BitSet::new_from_words(value as *const cpu_set_t as *const usize, Self::CpuSetTSizeInWords))
	}
}

/// Returns a cpu_set_t and the size, in bytes, of the valid range of `HyperThread`s in the `cpu_set_t`.
///
/// HyperThreads outside of this range will be `0`, ie not in the `cpu_set_t`.
///
/// This seconds value is `<= size_of::<cpu_set_t>()`.
impl TryInto<(cpu_set_t, usize)> for HyperThreads
{
	type Error = ();

	/// Fails if the set would contain more `HyperThread`s than can fit into `cpu_set_t` (currently this is more than 1024 `HyperThread`s).
	///
	/// Expands with unset `HyperThread`s if size if less than `cpu_set_t`.
	#[inline(always)]
	fn try_into(self) -> Result<(cpu_set_t, usize), Self::Error>
	{
		let size_in_words = Self::CpuSetTSizeInBytes / Self::UsizeSizeInBytes;

		if unlikely!(self.capacity_in_words() > size_in_words)
		{
			return Err(())
		}

		let cpu_set =
		{
			#[allow(deprecated)] let mut cpu_set = unsafe { uninitialized() };
			let (pointer, length) = self.to_raw_parts();
			let cpu_set_usize_pointer = &mut cpu_set as *mut cpu_set_t as *mut usize;
			unsafe { pointer.copy_to_nonoverlapping(cpu_set_usize_pointer, length) };
			unsafe { cpu_set_usize_pointer.add(Self::CpuSetTSizeInWords).write_bytes(0x00, length - Self::CpuSetTSizeInWords) };
			cpu_set
		};

		Ok((cpu_set, self.cpu_set_t_size_in_bytes()))
	}
}

impl HyperThreads
{
	const CpuSetTSizeInBytes: usize = size_of::<cpu_set_t>();

	const UsizeSizeInBytes: usize = size_of::<usize>();

	/// Maximum C library Hyper Thread.
	pub const MaximumCLibrary: u16 = (Self::CpuSetTSizeInBytes * BitsInAByte) as u16;

	const CpuSetTSizeInWords: usize = (Self::MaximumCLibrary as usize) / Self::UsizeSizeInBytes;
	
	/// A new bit set suitable for converting to a cpu_set_t.
	#[inline(always)]
	pub fn new_for_cpu_set_t() -> Self
	{
		Self(BitSet::<HyperThread>::with_capacity_in_words(Self::CpuSetTSizeInBytes / Self::UsizeSizeInBytes))
	}

	/// Valid `HyperThread`s.
	///
	/// Validated to make sure the result is not empty.
	#[inline(always)]
	pub fn valid(sys_path: &SysPath, proc_path: &ProcPath) -> Self
	{
		let mut valid = Self::has_a_folder_path(sys_path);
		valid.intersection(&Self::is_in_proc_self_status(proc_path));
		valid.intersection(&Self::present(sys_path));
		valid.intersection(&Self::possible(sys_path));
		valid.intersection(&Self::online(sys_path));
		valid.remove_all(&Self::offline(sys_path));
		valid.remove_any_that_are_actually_online(sys_path);
		valid.remove_any_without_associated_numa_nodes(sys_path);

		assert!(!valid.is_empty());

		valid.shrink_to_fit();

		valid
	}

	/// Set process affinity for current process.
	#[inline(always)]
	pub fn set_current_process_affinity(&self) -> Result<(), String>
	{
		self.set_process_affinity(ProcessIdentifierChoice::Current)
	}

	/// Set process affinity.
	#[inline(always)]
	pub fn set_process_affinity(&self, process_identifier: ProcessIdentifierChoice) -> Result<(), String>
	{
		let result = unsafe { sched_setaffinity(process_identifier.into(), self.cpu_set_t_size_in_bytes(), self.cpu_set_t_pointer()) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EINVAL => Err("The affinity bit mask mask contains no processors that are currently physically on the system and permitted to the process according to any restrictions that may be imposed by the cpuset mechanism described in man cpuset(7)".to_string()),

				EPERM => if let ProcessIdentifierChoice::Other(process_identifier) = process_identifier
				{
					Err(format!("The calling process does not have appropriate privileges. The caller needs an effective user ID equal to the real user ID or effective user ID of the process identified by process_identifier {:?}, or it must possess the CAP_SYS_NICE capability", process_identifier).to_string())
				}
				else
				{
					panic!("Can not set our own affinity")
				},

				ESRCH =>  if let ProcessIdentifierChoice::Other(process_identifier) = process_identifier
				{
					Err(format!("The thread whose ID is process_identifier '{:?}' could not be found", process_identifier).to_string())
				}
				else
				{
					panic!("Can not set our own process affinity")
				},

				EFAULT => panic!("A supplied memory address was invalid"),

				unknown @ _ => panic!("Unknown error number {}", unknown),
			}
		}
		else
		{
			panic!("Unexpected result {}", result)
		}
	}

	/// Set thread affinity for current thread.
	#[inline(always)]
	pub fn set_current_thread_affinity(&self) -> Result<(), String>
	{
		self.set_thread_affinity(unsafe { pthread_self() })
	}

	/// Set thread affinity.
	#[inline(always)]
	pub fn set_thread_affinity(&self, thread_identifier: pthread_t) -> Result<(), String>
	{
		let result = unsafe { pthread_setaffinity_np(thread_identifier, self.cpu_set_t_size_in_bytes(), self.cpu_set_t_pointer()) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EINVAL => Err("The affinity bit mask mask contains no processors that are currently physically on the system and permitted to the process according to any restrictions that may be imposed by the cpuset mechanism described in cpuset(7)".to_string()),

				ESRCH => if unlikely!(thread_identifier == pthread_self())
				{
					panic!("Can not set our own thread affinity")
				}
				else
				{
					Err(format!("The thread whose ID is thread_identifier '{:?}' could not be found", thread_identifier).to_string())
				},

				EFAULT => panic!("A supplied memory address was invalid"),

				unknown @ _ => panic!("Unknown error number {}", unknown),
			}
		}
		else
		{
			panic!("Unexpected result {}", result)
		}
	}
	
	/// Sets affinity.
	#[inline(always)]
	pub fn set_affinity(&self, path: impl AsRef<Path>) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write affinity to path");
		
		let path = path.as_ref();
		if path.exists()
		{
			let mask = IntoBitMask(self);
			path.write_value(mask)
		}
		else
		{
			Ok(())
		}
	}

	/// Sets affinity
	#[inline(always)]
	pub fn set_affinity_list(&self, path: impl AsRef<Path>) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write affinity to path");
		
		let path = path.as_ref();
		if path.exists()
		{
			let list = IntoList(&self.0);
			path.write_value(list)
		}
		else
		{
			Ok(())
		}
	}
	
	/// Sets work queue hyper thread affinity.
	#[inline(always)]
	pub fn set_work_queue_hyper_thread_affinity(&self, sys_path: &SysPath) -> io::Result<()>
	{
		let mask = IntoBitMask(self).into_line_feed_terminated_byte_string();
		sys_path.hyper_thread_work_queue_file_path("cpumask").write_value(mask.as_ref())?;
		sys_path.hyper_thread_work_queue_file_path("writeback/cpumask").write_value(mask)
	}
	
	/// Should not be needed if `nohz_full` was specified on the Linux command line.
	#[inline(always)]
	pub fn force_watchdog_to_just_these_hyper_threads(&self, proc_path: &ProcPath) -> io::Result<()>
	{
		self.set_affinity_list(proc_path.sys_kernel_file_path("watchdog_cpumask"))
	}

	/// CPU nodes that exist in the file system.
	#[inline(always)]
	fn has_a_folder_path(sys_path: &SysPath) -> Self
	{
		Self(sys_path.cpu_system_devices_folder_path().entries_in_folder_path().unwrap().unwrap())
	}

	/// CPU nodes that could possibly be online at some point.
	#[inline(always)]
	fn is_in_proc_self_status(proc_path: &ProcPath) -> Self
	{
		let process_status_statistics = Status::self_status(proc_path).unwrap();
		process_status_statistics.cpus_allowed
	}

	/// CPUs (hyper threaded logical cores) that are present and that could become online.
	///
	/// See <https://www.kernel.org/doc/Documentation/cputopology.txt>.
	#[inline(always)]
	fn present(sys_path: &SysPath) -> Self
	{
		Self::read_hyper_thread_list(sys_path, "present")
	}

	/// Hyper threaded logical cores that could possibly be online at some point.
	///
	/// Close to very useless.
	///
	/// See <https://www.kernel.org/doc/Documentation/cputopology.txt>.
	#[inline(always)]
	fn possible(sys_path: &SysPath) -> Self
	{
		Self::read_hyper_thread_list(sys_path, "possible")
	}

	/// Hyper threaded logical cores that are online at some point.
	///
	/// See <https://www.kernel.org/doc/Documentation/cputopology.txt>.
	#[inline(always)]
	fn online(sys_path: &SysPath) -> Self
	{
		Self::read_hyper_thread_list(sys_path, "online")
	}

	/// Hyper threaded logical cores that are offline.
	///
	/// Close to useless.
	///
	/// See <https://www.kernel.org/doc/Documentation/cputopology.txt>.
	#[inline(always)]
	fn offline(sys_path: &SysPath) -> Self
	{
		Self::read_hyper_thread_list(sys_path, "offline")
	}

	#[inline(always)]
	fn remove_any_that_are_actually_online(&mut self, sys_path: &SysPath)
	{
		let mut invalid_hyper_threads = Self(BitSet::<HyperThread>::empty());
		for hyper_thread in self.iterate()
		{
			if !hyper_thread.is_online(sys_path)
			{
				invalid_hyper_threads.add(hyper_thread)
			}
		}
		self.remove_all(&invalid_hyper_threads)
	}

	#[inline(always)]
	fn remove_any_without_associated_numa_nodes(&mut self, sys_path: &SysPath)
	{
		let is_a_numa_machine = NumaNode::is_a_numa_machine(sys_path);

		let mut invalid_hyper_threads = BitSet::empty();
		for hyper_thread in self.iterate()
		{
			if let Some(numa_node) = hyper_thread.numa_node(sys_path)
			{
				match numa_node.associated_hyper_threads(sys_path)
				{
					None => invalid_hyper_threads.add(hyper_thread),
					Some(associated_hyper_threads) => if !associated_hyper_threads.contains(hyper_thread)
					{
						invalid_hyper_threads.add(hyper_thread)
					}
				}
			}
			else if is_a_numa_machine
			{
				invalid_hyper_threads.add(hyper_thread)
			}
		}
		self.remove_all(&invalid_hyper_threads);
	}

	#[inline(always)]
	fn cpu_set_t_pointer(&self) -> *const cpu_set_t
	{
		self.to_raw_parts().0 as _
	}

	#[inline(always)]
	fn cpu_set_t_size_in_bytes(&self) -> usize
	{
		self.capacity_in_words() * Self::UsizeSizeInBytes
	}
	
	#[inline(always)]
	fn read_hyper_thread_list(sys_path: &SysPath, file_name: &str) -> Self
	{
		Self(sys_path.hyper_threads_folder_path(file_name).read_hyper_thread_or_numa_node_list().unwrap())
	}
}
