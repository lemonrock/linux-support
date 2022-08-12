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
			let mut cpu_set = unsafe_uninitialized();
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
	
	/// For one hyper thread.
	#[inline(always)]
	pub fn for_one(hyper_thread: HyperThread) -> Self
	{
		Self(BitSet::for_one(hyper_thread))
	}
	
	/// A new bit set suitable for converting to a cpu_set_t.
	#[inline(always)]
	pub fn new_for_cpu_set_t() -> Self
	{
		Self(BitSet::<HyperThread>::with_capacity_in_words(Self::CpuSetTSizeInBytes / Self::UsizeSizeInBytes))
	}

	/// Valid `HyperThread`s.
	///
	/// Removes those not available to the current process if `proc_path` is `Some`.
	#[inline(always)]
	pub fn valid(sys_path: &SysPath, proc_path: Option<&ProcPath>) -> Self
	{
		Self::has_a_folder_path(sys_path).validate(sys_path, proc_path)
	}
	
	/// Valid `HyperThread`s.
	///
	/// Removes those not available to the current process if `proc_path` is `Some`.
	#[inline(always)]
	pub fn validate(mut self, sys_path: &SysPath, proc_path: Option<&ProcPath>) -> Self
	{
		if let Some(proc_path) = proc_path
		{
			self.intersection(&Self::is_in_proc_self_status(proc_path));
		}
		
		// In a system that does not support CPU hotplug (ie `CONFIG_HOTPLUG_CPU` is not define), `present` is the same as `possible`.
		// Otherwise, `present` is a subset of `possible` and can vary over time.
		// See `include/linux/cpumask.h` in the Linux kernel sources.
		self.intersection(&Self::possible(sys_path));
		self.intersection(&Self::present(sys_path));
		
		// In a system that does not support CPU hotplug (ie `CONFIG_HOTPLUG_CPU` is not define), `online` is the same as `present`.
		// Otherwise, `online` is a subset of `present` and can vary over time.
		// See `include/linux/cpumask.h` in the Linux kernel sources.
		self.intersection(&Self::online(sys_path));
		
		self.remove_all(&Self::offline(sys_path));
		self.remove_any_that_are_not_actually_online(sys_path);
		self.remove_any_without_associated_numa_nodes(sys_path);
		
		self.shrink_to_fit();
		
		self
	}
	
	/// Current process affinity.
	#[inline(always)]
	pub fn current_process_affinity() -> Result<Self, String>
	{
		Self::process_affinity(ProcessIdentifierChoice::Current)
	}
	
	/// Get process affinity.
	#[inline(always)]
	pub fn process_affinity(process_identifier: ProcessIdentifierChoice) -> Result<Self, String>
	{
		let mut this = Self::new_for_cpu_set_t();
		let result = unsafe { sched_getaffinity(process_identifier.into(), this.cpu_set_t_size_in_bytes(), this.cpu_set_t_pointer() as *mut cpu_set_t) };
		
		if likely!(result == 0)
		{
			this.shrink_to_fit();
			Ok(this)
		}
		else if likely!(result == -1)
		{
			match SystemCallErrorNumber::from_errno()
			{
				EINVAL => Err("The affinity bit mask mask contains no processors that are currently physically on the system and permitted to the process according to any restrictions that may be imposed by the cpuset mechanism described in man cpuset(7)".to_string()),
				
				EFAULT => panic!("A supplied memory address was invalid"),
				
				unknown @ _ => panic!("Unknown error number {}", unknown),
			}
		}
		else
		{
			panic!("Unexpected result {}", result)
		}
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
			match SystemCallErrorNumber::from_errno()
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
	
	/// Current thread affinity.
	#[inline(always)]
	pub fn current_thread_affinity() -> Result<Option<Self>, String>
	{
		Self::get_thread_affinity(unsafe { pthread_self() })
	}
	
	/// Thread affinity.
	#[inline(always)]
	pub fn get_thread_affinity(thread_identifier: pthread_t) -> Result<Option<Self>, String>
	{
		let mut this = Self::new_for_cpu_set_t();
		let result = unsafe { pthread_setaffinity_np(thread_identifier, this.cpu_set_t_size_in_bytes(), this.cpu_set_t_pointer()) };
		if likely!(result == 0)
		{
			this.shrink_to_fit();
			Ok(Some(this))
		}
		else if likely!(result == -1)
		{
			match SystemCallErrorNumber::from_errno()
			{
				EINVAL => Err("setsize is smaller than the size of the affinity mask used by the kernel".to_string()),
				
				ESRCH => Ok(None),

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
			match SystemCallErrorNumber::from_errno()
			{
				EINVAL => Err("The affinity bit mask mask contains no processors that are currently physically on the system and permitted to the process according to any restrictions that may be imposed by the cpuset mechanism described in cpuset(7)".to_string()),

				ESRCH => if unlikely!(thread_identifier == unsafe { pthread_self() })
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
	///
	/// Does a cursory check that the `path` exists (but is subject to a TOCTOU flaw).
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

	/// Sets affinity.
	///
	/// Does a cursory check that the `path` exists (but is subject to a TOCTOU flaw).
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
	
	/// Which HyperThreads use flow limit tables.
	#[inline(always)]
	pub fn work_queue_hyper_thread_affinity(sys_path: &SysPath) -> (io::Result<Self>, io::Result<Self>)
	{
		(
			sys_path.hyper_thread_work_queue_file_path("cpumask").read_hyper_thread_or_numa_node_list().map(Self),
			sys_path.hyper_thread_work_queue_file_path("writeback/cpumask").read_hyper_thread_or_numa_node_list().map(Self),
		)
	}
	
	/// Which HyperThreads use flow limit tables.
	#[inline(always)]
	pub fn watchdog(proc_path: &ProcPath) -> io::Result<Self>
	{
		proc_path.sys_kernel_file_path("watchdog_cpumask").read_hyper_thread_or_numa_node_list().map(Self)
	}
	
	/// Should not be needed if `nohz_full` was specified on the Linux command line.
	#[inline(always)]
	pub fn force_watchdog_to_just_these_hyper_threads(&self, proc_path: &ProcPath) -> io::Result<()>
	{
		self.set_affinity_list(proc_path.sys_kernel_file_path("watchdog_cpumask"))
	}
	
	/// Which HyperThreads use flow limit tables.
	#[inline(always)]
	pub fn receive_packet_steering_flow_limit_tables_affinity(proc_path: &ProcPath) -> io::Result<Self>
	{
		proc_path.sys_net_core_file_path("flow_limit_cpu_bitmap").read_hyper_thread_or_numa_node_list().map(Self)
	}
	
	/// Which HyperThreads use flow limit tables.
	#[inline(always)]
	pub fn set_receive_packet_steering_flow_limit_tables_affinity(&self, proc_path: &ProcPath) -> io::Result<()>
	{
		self.set_affinity(proc_path.sys_net_core_file_path("flow_limit_cpu_bitmap"))
	}
	
	/// Which HyperThreads use Receive Packet Steering (RPS) for a receive queue on a network device.
	///
	/// Default is `HyperThreads::empty()`.
	#[inline(always)]
	pub fn receive_packet_steering_affinity_for_receive_queue(&self, sys_path: &SysPath, receive_queue: ReceiveSysfsQueue) -> io::Result<Self>
	{
		receive_queue.receive_packet_steering_affinity(sys_path)
	}
	
	/// Which HyperThreads use Receive Packet Steering (RPS) for a receive queue on a network device.
	///
	/// Default is `HyperThreads::empty()`.
	#[inline(always)]
	pub fn set_receive_packet_steering_affinity_for_receive_queue(&self, sys_path: &SysPath, receive_queue: ReceiveSysfsQueue) -> io::Result<()>
	{
		receive_queue.set_receive_packet_steering_affinity(sys_path, self)
	}
	
	/// Which HyperThreads use Transmit Packet Steering (XPS) for a transmit queue on a network device.
	///
	/// Default is `HyperThreads::empty()`.
	///
	/// Returns `Ok(Some(_))` if successfully retrieved.
	///
	/// ***Only supported if the network device is multiqueue (ie has more than one transmit queue); if not supported, `Ok(None)` is returned`.
	/// Most virtual network devices are not multiqueue.
	#[inline(always)]
	pub fn transmit_packet_steering_affinity_for_receive_queue(&self, sys_path: &SysPath, transmit_queue: TransmitSysfsQueue) -> io::Result<Option<Self>>
	{
		transmit_queue.transmit_packet_steering_hyper_thread_affinity(sys_path)
	}
	
	/// Which HyperThreads use Transmit Packet Steering (XPS) for a transmit queue on a network device.
	///
	/// Default is `HyperThreads::empty()`.
	///
	/// Returns `Ok(true)` if successfully set.
	///
	/// ***Only supported if the network device is multiqueue (ie has more than one transmit queue); if not supported, `Ok(false)`.
	/// Most virtual network devices are not multiqueue.
	/// `Ok(false)` is also returned if the file does not exist.
	#[inline(always)]
	pub fn set_transmit_packet_steering_affinity_for_receive_queue(&self, sys_path: &SysPath, transmit_queue: TransmitSysfsQueue) -> io::Result<bool>
	{
		transmit_queue.set_transmit_packet_steering_hyper_thread_affinity(sys_path, self)
	}
	
	/// Mirrors `num_possible_cpus()` in the Linux kernel but with a twist.
	///
	/// There is a design flaw in BPF 's `PER_CPU` maps such that access a particular CPU's value is incorrect if `/sys/devices/system/cpu/possible` has a CPU mask which does not include all possible CPUs!
	///
	/// It is too hard to engage with the libbpf mailing list at <https://lore.kernel.org/bpf/>.
	#[inline(always)]
	pub(crate) fn number_of_possible_hyper_threads_unless_there_are_missing_possible_hyper_threads(sys_path: &SysPath) -> Option<usize>
	{
		Self::possible(sys_path).len_if_full()
	}
	
	/// CPU nodes that exist in the file system.
	#[inline(always)]
	pub(crate) fn has_a_folder_path(sys_path: &SysPath) -> Self
	{
		Self(sys_path.cpu_system_devices_folder_path().entries_in_folder_path().unwrap().unwrap())
	}

	/// CPU nodes that could possibly be online at some point.
	#[inline(always)]
	pub(crate) fn is_in_proc_self_status(proc_path: &ProcPath) -> Self
	{
		let process_status_statistics = Status::self_status(proc_path).unwrap();
		process_status_statistics.cpus_allowed
	}
	
	/// Hyper threaded logical cores that could possibly be online at some point.
	///
	/// Close to very useless.
	///
	/// See <https://www.kernel.org/doc/Documentation/cputopology.txt>.
	#[inline(always)]
	pub(crate) fn possible(sys_path: &SysPath) -> Self
	{
		Self::read_hyper_thread_list(sys_path, "possible")
	}

	/// CPUs (hyper threaded logical cores) that are present and that could become online.
	///
	/// A dynamic subset of `possible()` in a system configured with `CONFIG_CPU_HOTPLUG`, otherwise the same as `possible()`.
	/// See comments in the Linux source code file `include/linux/cpumask.h`.
	///
	/// See <https://www.kernel.org/doc/Documentation/cputopology.txt>.
	#[inline(always)]
	pub(crate) fn present(sys_path: &SysPath) -> Self
	{
		Self::read_hyper_thread_list(sys_path, "present")
	}

	/// Hyper threaded logical cores that are online at some point.
	///
	/// A dynamic subset of `present()` in a system configured with `CONFIG_CPU_HOTPLUG`, otherwise the same as `present()`.
	/// See comments in the Linux source code file `include/linux/cpumask.h`.
	///
	/// See <https://www.kernel.org/doc/Documentation/cputopology.txt>.
	#[inline(always)]
	pub(crate) fn online(sys_path: &SysPath) -> Self
	{
		Self::read_hyper_thread_list(sys_path, "online")
	}

	/// Hyper threaded logical cores that are offline.
	///
	/// Close to useless.
	///
	/// See <https://www.kernel.org/doc/Documentation/cputopology.txt>.
	#[inline(always)]
	pub(crate) fn offline(sys_path: &SysPath) -> Self
	{
		Self::read_hyper_thread_list(sys_path, "offline")
	}

	#[inline(always)]
	fn remove_any_that_are_not_actually_online(&mut self, sys_path: &SysPath)
	{
		let mut invalid_hyper_threads = Self(BitSet::<HyperThread>::empty());
		for hyper_thread in self.iterate()
		{
			if !hyper_thread.is_online(sys_path).unwrap()
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
