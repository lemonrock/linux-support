// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a logical hyper thread, which in Operating System terms is usually a logical CPU (core).
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct HyperThread(u16);

bit_set_aware!(HyperThread);

impl Into<u16> for HyperThread
{
	#[inline(always)]
	fn into(self) -> u16
	{
		self.0
	}
}

impl BitSetAware for HyperThread
{
	/// Maximum value of `CONFIG_NR_CPUS`.
	const LinuxMaximum: u16 = 8192;

	const InclusiveMinimum: Self = Self(0);

	const InclusiveMaximum: Self = Self(Self::LinuxMaximum - 1);

	const Prefix: &'static [u8] = b"cpu";

	#[inline(always)]
	fn from_validated_u16(value: u16) -> Self
	{
		debug_assert!(value < Self::LinuxMaximum);

		Self(value)
	}
}

impl HyperThread
{
	/// This is *unreliable*:
	///
	/// * it reports `false` on a test virtual machine;
	/// * it is absent on some machines.
	#[inline(always)]
	pub fn is_hyper_threading_active(sys_path: &SysPath) -> Option<bool>
	{
		let file_path = sys_path.hyper_thread_smt_file_path("active");
		if file_path.exists()
		{
			Some(file_path.read_zero_or_one_bool().unwrap())
		}
		else
		{
			None
		}
	}

	/// This is much more reliable that `is_hyper_threading_active()` but may still be absent.
	#[inline(always)]
	pub fn hyper_threading_control(sys_path: &SysPath) -> Option<HyperThreadingStatus>
	{
		let file_path = HyperThread::smt_control_file_path(sys_path);
		if file_path.exists()
		{
			Some(file_path.read_value().unwrap())
		}
		else
		{
			None
		}
	}

	/// Try to enable hyper threading.
	#[inline(always)]
	pub fn try_to_enable_hyper_threading(sys_path: &SysPath, current_status: HyperThreadingStatus) -> HyperThreadingStatus
	{
		use self::HyperThreadingStatus::*;

		match current_status
		{
			On | ForceOff | NotSupported | NotImplemented => current_status,
			Off =>
			{
				Self::smt_control_file_path(sys_path).write_value(b"on\n" as &[u8]).unwrap();
				Self::hyper_threading_control(sys_path).unwrap()
			}
		}
	}

	/// Try to enable disable threading.
	#[inline(always)]
	pub fn try_to_disable_hyper_threading(sys_path: &SysPath, current_status: HyperThreadingStatus) -> HyperThreadingStatus
	{
		use self::HyperThreadingStatus::*;

		match current_status
		{
			Off | ForceOff | NotSupported | NotImplemented => current_status,
			On =>
			{
				Self::smt_control_file_path(sys_path).write_value(b"off\n" as &[u8]).unwrap();
				Self::hyper_threading_control(sys_path).unwrap()
			}
		}
	}

	/// Reads the hyper thread and NUMA node of the currently executing CPU from the `IA32_TSC_AUX` model state register, which Linux populates.
	#[inline(always)]
	pub fn current() -> (NumaNode, Self)
	{
		current_numa_node_and_hyper_thread()
	}

	/// Current hyper thread index that this thread is running on.
	///
	/// Unless this thread has been scheduled to only run on this hyper thread, then the result is close to useless.
	///
	/// Prefer `current()`.
	#[deprecated]
	pub fn current_hyper_thread() -> Self
	{
		let result = unsafe { sched_getcpu() };
		debug_assert!(result >= 0, "sched_getcpu() was negative");

		debug_assert!(result <= u16::MAX as i32, "sched_getcpu() was too large");
		let result = result as u16;

		debug_assert!(result < Self::LinuxMaximum);

		Self(result as u16)
	}

	/// Value of `CONFIG_NR_CPUS`.
	#[inline(always)]
	pub fn kernel_exclusive_maximum(sys_path: &SysPath) -> u16
	{
		let value: u16 = sys_path.hyper_threads_folder_path("kernel_max").read_value().unwrap();
		let this = Self::try_from(value).unwrap();
		this.0 + 1
	}
	
	/// Uses `sysconf(_SC_NPROCESSORS_CONF)` which is ***only valid*** if `sched_setaffinity()` has not been called.
	///
	/// Internally `sysconf(_SC_NPROCESSORS_CONF)`, in musl, uses the system call `SYS_sched_getaffinity()`.
	#[inline(always)]
	pub fn sysconf_exclusive_maximum() -> u16
	{
		let result = unsafe { sysconf(_SC_NPROCESSORS_CONF) };
		if result <= 0
		{
			1
		}
		else if result > Self::LinuxMaximum as c_long
		{
			Self::LinuxMaximum
		}
		else
		{
			result as u16
		}
	}

	/// Isolated HyperThreads.
	///
	/// These are hyper threads isolated from use by Linux itself and normal process (and thread) schedulers.
	#[inline(always)]
	pub fn isolated(&self, linux_kernel_command_line: &LinuxKernelCommandLineParameters, isolated_cpus_required: bool) -> Result<HyperThreads, &'static str>
	{
		if let Some((isolated_cpu_flags, isolated_cpus)) = linux_kernel_command_line.isolcpus()
		{
			if !isolated_cpu_flags.contains(&IsolatedCpuFlags::Domain)
			{
				return Err("Kernel parameter `isolcpus` does not contain (or imply) the domain flag")
			}

			let rcu_nocbs = linux_kernel_command_line.rcu_nocbs().ok_or("Kernel parameter `rcu_nocbs` should be specified because isolcpus was specified")?;

			let nohz_full = linux_kernel_command_line.nohz_full().ok_or("Kernel parameter `nohz_full` should be specified because isolcpus was specified")?;

			// let irqaffinity = linux_kernel_command_line.irqaffinity().ok_or("Kernel parameter `irqaffinity` should be specified because isolcpus was specified")?;

			if isolated_cpus != rcu_nocbs
			{
				return Err("Kernel parameters `isolcpus` and `rcu_nocbs` should match")
			}

			if isolated_cpus != nohz_full
			{
				return Err("Kernel parameters `isolcpus` and `nohz_full` should match")
			}

			Ok(isolated_cpus)
		}
		else
		{
			if isolated_cpus_required
			{
				return Err("Kernel parameter `isolcpus` should be specified")
			}
			else
			{
				Ok(HyperThreads(BitSet::empty()))
			}
		}
	}

	/// Finds this hyper thread's NUMA node.
	///
	/// Returns `None` if the Linux kernel wasn't configured with `CONFIG_NUMA`.
	#[inline(always)]
	pub fn numa_node(self, sys_path: &SysPath) -> Option<NumaNode>
	{
		match sys_path.hyper_thread_file_path(self, "node").canonicalize()
		{
			Err(_) => None,

			Ok(canonical) => match canonical.file_name()
			{
				None => None,

				Some(file_name) => NumaNode::parse_file_name(file_name).unwrap(),
			},
		}
	}

	/// Is this hyper thread online?
	///
	/// See <https://www.kernel.org/doc/Documentation/core-api/cpu_hotplug.rst>.
	///
	/// The Linux kernel needs to have been compiled with `CONFIG_HOTPLUG_CPU`.
	///
	/// To check a HyperThread is offline, NOT the result of this function.
	#[inline(always)]
	fn is_online(self, sys_path: &SysPath) -> bool
	{
		let file_path = self.online_file_path(sys_path);

		// There is a weird bug in Linux 5.4 (and maybe other versions) where the `online` file does not appear in file listings and can not be used until it has been written to at least once.
		if !file_path.exists()
		{
			return true
		}

		file_path.read_zero_or_one_bool().unwrap()
	}

	/// Online or offline this hyper thread.
	///
	/// Requires root.
	///
	/// See <https://www.kernel.org/doc/Documentation/core-api/cpu_hotplug.rst>.
	///
	/// The Linux kernel needs to have been compiled with `CONFIG_HOTPLUG_CPU`.
	#[inline(always)]
	pub fn set_online(self, sys_path: &SysPath, online: bool) -> io::Result<()>
	{
		assert_effective_user_id_is_root(&format!("Online/Offline HyperThread '{}'", self.0));

		self.online_file_path(sys_path).write_value(online)
	}

	/// Hyper threaded logical cores that are siblings of this one.
	///
	/// Will include `self`.
	///
	/// See <https://www.kernel.org/doc/Documentation/cputopology.txt>.
	///
	/// Does not validate the siblings.
	#[inline(always)]
	pub fn siblings(self, sys_path: &SysPath) -> BitSet<Self>
	{
		sys_path.hyper_thread_topology_file_path(self, "core_siblings_list").read_hyper_thread_or_numa_node_list().unwrap()
	}

	/// Hyper threaded logical cores that are hyper-thread-siblings of this one.
	///
	/// Will include `self`.
	///
	/// See <https://www.kernel.org/doc/Documentation/cputopology.txt>.
	///
	/// Does not validate the siblings.
	///
	/// Usually wrong on virtual machines (eg Parallels Desktop).
	#[inline(always)]
	pub fn thread_siblings(self, sys_path: &SysPath) -> BitSet<Self>
	{
		sys_path.hyper_thread_topology_file_path(self, "thread_siblings_list").read_hyper_thread_or_numa_node_list().unwrap()
	}

	/// Hyper threaded logical cores that are thread-siblings of this one according to the level 1 cache.
	///
	/// Will include `self`.
	///
	/// Does not validate the siblings.
	///
	/// Usually reliable.
	#[inline(always)]
	pub fn level1_cache_hyper_thread_siblings_including_self(self, sys_path: &SysPath) -> BitSet<Self>
	{
		sys_path.hyper_thread_cache_file_path(self, "index0/shared_cpu_list").read_hyper_thread_or_numa_node_list().unwrap()
	}

	/// Hyper threaded logical cores that are thread-siblings of this one according to the level 1 cache.
	///
	/// Will exclude `self`.
	///
	/// Does not validate the siblings.
	///
	/// Usually reliable.
	#[inline(always)]
	pub fn level1_cache_hyper_thread_siblings_excluding_self(self, sys_path: &SysPath) -> BitSet<Self>
	{
		let mut hyper_threads = self.level1_cache_hyper_thread_siblings_including_self(sys_path);
		hyper_threads.remove(self);
		hyper_threads
	}

	/// Underlying hardware, not Linux, core identifier.
	///
	/// See <https://www.kernel.org/doc/Documentation/cputopology.txt>.
	#[inline(always)]
	pub fn underlying_hardware_physical_core_identifier(self, sys_path: &SysPath) -> io::Result<u16>
	{
		sys_path.hyper_thread_topology_file_path(self, "core_id").read_value()
	}

	/// Underlying hardware, not Linux, socket identifier.
	///
	/// See <https://www.kernel.org/doc/Documentation/cputopology.txt>.
	#[inline(always)]
	pub fn underlying_hardware_physical_socket_identifier(self, sys_path: &SysPath) -> io::Result<u16>
	{
		sys_path.hyper_thread_topology_file_path(self, "physical_package_id").read_value()
	}

	#[inline(always)]
	fn online_file_path(self, sys_path: &SysPath) -> PathBuf
	{
		sys_path.hyper_thread_file_path(self, "online")
	}

	#[inline(always)]
	fn smt_control_file_path(sys_path: &SysPath) -> PathBuf
	{
		sys_path.hyper_thread_smt_file_path("control")
	}
}
