// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a logical hyper thread, which in Operating System terms is usually a logical CPU (core).
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct HyperThread(pub u16);

impl From<u8> for HyperThread
{
	#[inline(always)]
	fn from(value: u8) -> Self
	{
		HyperThread(value as u16)
	}
}

impl From<u16> for HyperThread
{
	#[inline(always)]
	fn from(value: u16) -> Self
	{
		HyperThread(value)
	}
}

impl Into<u16> for HyperThread
{
	#[inline(always)]
	fn into(self) -> u16
	{
		self.0
	}
}

impl Into<u32> for HyperThread
{
	#[inline(always)]
	fn into(self) -> u32
	{
		self.0 as u32
	}
}

impl Into<u64> for HyperThread
{
	#[inline(always)]
	fn into(self) -> u64
	{
		self.0 as u64
	}
}

impl Into<usize> for HyperThread
{
	#[inline(always)]
	fn into(self) -> usize
	{
		self.0 as usize
	}
}

impl HyperThread
{
	/// Reads the hyper thread and NUMA node of the currently executing CPU from the `IA32_TSC_AUX` model state register, which Linux populates.
	///
	/// Currently uses the `RDTSCP` instruction, but, once Ice Lake is widely available, could be changed to use the `RDPID` instruction.
	#[inline(always)]
	pub fn current_numa_node_and_hyper_thread() -> (NumaNode, HyperThread)
	{
		NumaNode::current_numa_node_and_hyper_thread()
	}

	/// Valid logical cores for the current process.
	///
	/// ***Only valid at start up before `sched_setaffinity()` has been called.***
	///
	/// Logic inspired by [libnuma](https://github.com/numactl/numactl)'s `numa_num_task_cpus()` function.
	///
	/// Slow as it will parse the file `/proc/self/status`.
	pub fn valid_hyper_threads_for_the_current_process(proc_path: &ProcPath) -> BTreeSet<Self>
	{
		#[inline(always)]
		fn all_available_to_process_even_if_they_do_not_exist(proc_path: &ProcPath) -> BTreeSet<HyperThread>
		{
			let process_status_statistics = ProcessStatusStatistics::self_status(proc_path).unwrap();
			process_status_statistics.cpus_allowed_list.unwrap()
		}

		let all_available_to_process_even_if_they_do_not_exist = all_available_to_process_even_if_they_do_not_exist(proc_path);

		// This logic is borrowed from libnuma; internally `sysconf(_SC_NPROCESSORS_CONF)`, in musl, uses the system call `SYS_sched_getaffinity()`.
		let number_of_logical_cores = unsafe { sysconf(_SC_NPROCESSORS_CONF) } - 1;
		let maximum_logical_core_identifier = if unlikely!(number_of_logical_cores == 0)
		{
			0
		}
		else
		{
			(number_of_logical_cores - 1) as u16
		};

		let mut hyper_threads = BTreeSet::new();
		for hyper_thread in all_available_to_process_even_if_they_do_not_exist.range(Self::from(0u16) ..= Self::from(maximum_logical_core_identifier))
		{
			hyper_threads.insert(*hyper_thread);
		}
		hyper_threads
	}

	/// Hyper Threads to mask.
	#[inline(always)]
	pub fn hyper_threads_to_mask(hyper_threads: &BTreeSet<Self>) -> Vec<u8>
	{
		let mut mask: u32 = 0;
		for hyper_thread in hyper_threads.iter()
		{
			let bit = (1 << hyper_thread.0) as u32;
			mask |= bit;
		}
		format!("{:08x}\n", mask).into_bytes()
	}

	/// Hyper Threads to list.
	#[inline(always)]
	fn hyper_threads_to_list(hyper_threads: &BTreeSet<Self>) -> Vec<u8>
	{
		let mut list = Vec::with_capacity(hyper_threads.len() * 4);
		for hyper_thread in hyper_threads.iter()
		{
			if !list.is_empty()
			{
				list.push(b',');
			}
			let x = hyper_thread.0.into_line_feed_terminated_byte_string();
			list.extend_from_slice(&x[.. x.len() - 1])
		}
		list
	}

	/// Sets workqueue hyper thread affinity.
	#[inline(always)]
	pub fn set_work_queue_hyper_thread_affinity(hyper_threads: &BTreeSet<Self>, sys_path: &SysPath) -> io::Result<()>
	{
		let mask = Self::hyper_threads_to_mask(hyper_threads);

		sys_path.hyper_thread_workqueue_file_path("cpumask").write_value(&mask)?;
		sys_path.hyper_thread_workqueue_file_path("writeback/cpumask").write_value(&mask)
	}

	/// We ignore failures as the `/proc` for this is brittle.
	///
	/// Should not be needed if `nohz_full` was specified on the Linux command line.
	#[inline(always)]
	pub fn force_watchdog_to_just_these_hyper_threads(hyper_threads: &BTreeSet<Self>, proc_path: &ProcPath) -> io::Result<()>
	{
		if cfg!(any(target_os = "android", target_os = "linux"))
		{
			let yes_a_list_even_though_file_is_named_a_cpumask = Self::hyper_threads_to_list(hyper_threads);
			proc_path.sys_kernel_file_path("watchdog_cpumask").write_value(yes_a_list_even_though_file_is_named_a_cpumask)
		}
		else
		{
			Ok(())
		}
	}

	/// Last hyper thread.
	#[inline(always)]
	pub fn last(hyper_threads: &BTreeSet<Self>) -> Option<&Self>
	{
		hyper_threads.iter().last()
	}

	/// The complement of `hyper_threads`.
	#[inline(always)]
	pub fn complement(hyper_threads: &BTreeSet<Self>, sys_path: &SysPath) -> BTreeSet<Self>
	{
		let present = Self::present(sys_path);
		present.difference(hyper_threads).cloned().collect()
	}

	/// Remove as offline `hyper_threads`.
	#[inline(always)]
	pub fn remove_those_offline(hyper_threads: &BTreeSet<Self>, sys_path: &SysPath) -> BTreeSet<Self>
	{
		let online = Self::online(sys_path);
		online.intersection(hyper_threads).cloned().collect()
	}

	/// CPUs (hyper threaded logical cores) that are present and that could become online.
	///
	/// Consider using libnuma instead of this call.
	///
	/// See <https://www.kernel.org/doc/Documentation/cputopology.txt>.
	#[inline(always)]
	pub fn present(sys_path: &SysPath) -> BTreeSet<Self>
	{
		Self::parse_list_mask(sys_path, "present")
	}

	/// Hyper threaded logical cores that are online at some point.
	///
	/// Consider using libnuma instead of this call.
	///
	/// See <https://www.kernel.org/doc/Documentation/cputopology.txt>.
	#[inline(always)]
	pub fn online(sys_path: &SysPath) -> BTreeSet<Self>
	{
		Self::parse_list_mask(sys_path, "online")
	}

	/// Hyper threaded logical cores that are offline.
	///
	/// The maximum CPU index in this list ***can exceed the kernel's maximum in `self.kernel_maximum_index`***.
	///
	/// Close to useless.
	///
	/// See <https://www.kernel.org/doc/Documentation/cputopology.txt>.
	#[inline(always)]
	pub fn offline(sys_path: &SysPath) -> BTreeSet<Self>
	{
		Self::parse_list_mask(sys_path, "offline")
	}

	/// Hyper threaded logical cores that could possibly be online at some point.
	///
	/// Close to very useless.
	///
	/// See <https://www.kernel.org/doc/Documentation/cputopology.txt>.
	#[inline(always)]
	pub fn possible(sys_path: &SysPath) -> BTreeSet<Self>
	{
		Self::parse_list_mask(sys_path, "possible")
	}

	/// Is this hyper thread online?
	///
	/// See <https://www.kernel.org/doc/Documentation/core-api/cpu_hotplug.rst>.
	#[inline(always)]
	pub fn is_online(self, sys_path: &SysPath) -> bool
	{
		match &self.online_file_path(sys_path).read_raw_without_line_feed().unwrap()[..]
		{
			b"0" => false,
			b"1" => true,
			invalid @ _ => panic!("Invalid value for CPU online '{:?}'", invalid),
		}
	}

	/// Is this hyper thread offline?
	///
	/// See <https://www.kernel.org/doc/Documentation/core-api/cpu_hotplug.rst>.
	#[inline(always)]
	pub fn is_offline(self, sys_path: &SysPath) -> bool
	{
		!self.is_online(sys_path)
	}

	/// Disable (offline) this hyper thread.
	///
	/// Requires root.
	///
	/// Hyper thread (CPU) zero (0) is special on x86 / x86-64 and can not ordinarily be offlined.
	///
	/// See <https://www.kernel.org/doc/Documentation/core-api/cpu_hotplug.rst>.
	#[inline(always)]
	pub fn set_offline(self, sys_path: &SysPath) -> io::Result<()>
	{
		assert_effective_user_id_is_root(&format!("Offline CPU '{}'", self.0));

		self.online_file_path(sys_path).write_value(0)
	}

	/// Enable (online) this hyper thread.
	///
	/// Requires root.
	///
	/// See <https://www.kernel.org/doc/Documentation/core-api/cpu_hotplug.rst>.
	#[inline(always)]
	pub fn set_online(self, sys_path: &SysPath) -> io::Result<()>
	{
		assert_effective_user_id_is_root(&format!("Online CPU '{}'", self.0));

		self.online_file_path(sys_path).write_value(1)
	}

	#[inline(always)]
	fn online_file_path(self, sys_path: &SysPath) -> PathBuf
	{
		sys_path.hyper_thread_path(self, "online")
	}

	/// Hyper threaded logical cores that are siblings of this one.
	///
	/// Will include `self`.
	///
	/// See <https://www.kernel.org/doc/Documentation/cputopology.txt>.
	#[inline(always)]
	pub fn siblings(self, sys_path: &SysPath) -> BTreeSet<Self>
	{
		sys_path.hyper_thread_path(self, "topology/core_siblings_list").read_linux_core_or_numa_list(|value_u16| Ok(Self(value_u16))).unwrap()
	}

	/// Hyper threaded logical cores that are hyper-thread-siblings of this one.
	///
	/// Will include `self`.
	///
	/// Usually wrong on virtual machines (eg Parallels Desktop).
	///
	/// See <https://www.kernel.org/doc/Documentation/cputopology.txt>.
	#[inline(always)]
	pub fn thread_siblings(self, sys_path: &SysPath) -> BTreeSet<Self>
	{
		sys_path.hyper_thread_path(self, "topology/thread_siblings_list").read_linux_core_or_numa_list(|value_u16| Ok(Self(value_u16))).unwrap()
	}

	/// Hyper threaded logical cores grouped as hyper thread groups (eg HT 0 and 1, 2 and 3, etc).
	#[inline(always)]
	pub fn hyper_thread_groups(hyper_threads: &BTreeSet<Self>, sys_path: &SysPath) -> BTreeSet<BTreeSet<Self>>
	{
		let mut hyper_thread_groups = BTreeSet::new();
		for hyper_thread in hyper_threads.iter()
		{
			let hyper_thread_group = (*hyper_thread).level1_cache_hyper_thread_siblings_including_self(sys_path);
			hyper_thread_groups.insert(hyper_thread_group);
		}
		hyper_thread_groups
	}

	/// Tries to find this hyper thread's NUMA node, if this is a NUMA machine.
	#[inline(always)]
	pub fn numa_node(self, sys_path: &SysPath) -> Option<u8>
	{
		match sys_path.hyper_thread_path(self, "node").canonicalize()
		{
			Err(_) => None,
			Ok(canonical) => match canonical.file_name()
			{
				None => None,
				Some(file_name) => match file_name.to_str()
				{
					None => None,
					Some(file_name) => if file_name.starts_with("node")
					{
						u8::from_str(&file_name[ ("node".len()) .. ]).ok()
					}
					else
					{
						None
					}
				},
			},
		}
	}

	// there is a /node file that symlinks to a NUMA node location.

	/// Hyper threaded logical cores that are thread-siblings of this one according to the level 1 cache.
	///
	/// Will include `self`.
	///
	/// Usually reliable.
	#[inline(always)]
	pub fn level1_cache_hyper_thread_siblings_including_self(self, sys_path: &SysPath) -> BTreeSet<Self>
	{
		sys_path.hyper_thread_path(self, "cache/index0/shared_cpu_list").read_linux_core_or_numa_list(|value_u16| Ok(Self(value_u16))).unwrap()
	}

	/// Hyper threaded logical cores that are thread-siblings of this one according to the level 1 cache.
	///
	/// Will exclude `self`.
	///
	/// Usually reliable.
	#[inline(always)]
	pub fn level1_cache_hyper_thread_siblings_excluding_self(self, sys_path: &SysPath) -> BTreeSet<Self>
	{
		let mut hyper_threads = self.level1_cache_hyper_thread_siblings_including_self(sys_path);
		hyper_threads.remove(&self);
		hyper_threads
	}

	/// Underlying hardware, not Linux, core identifier.
	///
	/// See <https://www.kernel.org/doc/Documentation/cputopology.txt>.
	#[inline(always)]
	pub fn underlying_hardware_physical_core_identifier(self, sys_path: &SysPath) -> io::Result<u16>
	{
		sys_path.hyper_thread_path(self, "topology/core_id").read_value()
	}

	/// Underlying hardware, not Linux, socket identifier.
	///
	/// See <https://www.kernel.org/doc/Documentation/cputopology.txt>.
	#[inline(always)]
	pub fn underlying_hardware_physical_socket_identifier(self, sys_path: &SysPath) -> io::Result<u16>
	{
		sys_path.hyper_thread_path(self, "topology/physical_package_id").read_value()
	}

	/// Simply reports the maximum *identifier* that could be used by the Linux kernel upto the `CONFIG_` number of CPUs.
	///
	/// Add one to this to get the exclusive maximum.
	///
	/// Consider using libnuma instead of this call.
	#[inline(always)]
	pub fn kernel_maximum_index(sys_path: &SysPath) -> io::Result<Self>
	{
		sys_path.hyper_threads_path("kernel_max").read_value().map(|value| Self(value))
	}

	#[inline(always)]
	fn parse_list_mask(sys_path: &SysPath, file_name: &str) -> BTreeSet<Self>
	{
		sys_path.hyper_threads_path(file_name).read_linux_core_or_numa_list(|value_u16| Ok(Self(value_u16))).unwrap()
	}

	/// Current hyper thread index that this thread is running on.
	///
	/// Unless this thread has been scheduled to only run on this hyper thread, then the result is close to useless.
	pub fn current_hyper_thread() -> Self
	{
		let result = unsafe { sched_getcpu() };
		debug_assert!(result >= 0, "sched_getcpu() was negative");
		debug_assert!(result <= ::std::u16::MAX as i32, "sched_getcpu() was too large");
		HyperThread(result as u16)
	}
}
