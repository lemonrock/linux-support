// TODO: userfaultfd: http://man7.org/linux/man-pages/man2/userfaultfd.2.html

// TODO: Rework kernel validator into sections for numa, memory, etc.

// TODO: kernel validator proper errors.

// TODO: Numa / hyper thread valid threads, master loops, allocating kernel and other processes (including forcibly moving them), etc

// TODO: ProcStat parsing (has extra information). (http://man7.org/linux/man-pages/man5/proc.5.html)

// TODO: kernel validator huge pages - make generic for powerpc, aarch64 and riscv64.

// TODO: /proc/<N>/oom* files and stuff in /proc/sys

// TODO: Adjust CommitLimit to prevent future out-of-memory.

// TODO: libcpuset and /dev/cpuset and cpusetfs (which has several useful details)

// TODO: Finish NUMA distances.

// TODO: mmap to physicalling contig.
    // TODO: mlock, sync and ?mremap additions first.

// TODO: memfd seals (https://lwn.net/Articles/591108/ http://man7.org/linux/man-pages/man2/userfaultfd.2.html https://github.com/lucab/memfd-rs)

// TODO: Allocate physically contiguous memory (using huge / gigantic pages; using page map to finding virt to phys mappings)

// TODO:  Automatic NUMA balancing can be enabled or disabled for the current session by writing 1 or 0 to /proc/sys/kernel/numa_balancing which will enable or disable the feature respectively. To permanently enable or disable it, use the kernel command line option numa_balancing=[enable|disable].



Automatic NUMA balancing notes:-
If Automatic NUMA Balancing is enabled, the task scanner behavior can be configured. The task scanner balances the overhead of Automatic NUMA Balancing with the amount of time it takes to identify the best placement of data.

numa_balancing_scan_delay_ms:    The amount of CPU time a thread must consume before its data is scanned. This prevents creating overhead because of short-lived processes.

numa_balancing_scan_period_min_ms and numa_balancing_scan_period_max_ms: Controls how frequently a task's data is scanned. Depending on the locality of the faults the scan rate will increase or decrease. These settings control the min and max scan rates.

numa_balancing_scan_size_mb: Controls how much address space is scanned when the task scanner is active.

pub struct ValidatedNumaNodeToHyperThreadMap
{
	all_valid_hyper_threads: BitSet<HyperThread>,
	map: HashMap<NumaNode, BitSet<HyperThread>>,
}

impl ValidatedNumaNodeToHyperThreadMap
{
	pub fn x(&self)
	{
		/*
			TODO: Which hyper threads for linux kernel watchdog?
				- must not use anything in the isolcpus list
			TODO: Which hyper threads are being used for the linux kernel and general userspace programs?
				- anything not in the isolcpus list
					- "Kernel parameters `isolcpus`, `rcu_nocbs` and `nohz_full` should all match"
			TODO: Which hyper threads to use to best access a PCI device?
			TODO: Which hyper thread to use to best run a control (master) loop?
			TODO: Which hyper threads to use to best run background tasks?

			Can we enable isolcpus after boot?
				TODO: Explore use of cpuset file system to isolate cpus: http://man7.org/linux/man-pages/man7/cpuset.7.html
				TODO: If running as root, move all other processes onto the 'shared' cpuset which overlaps with the Kernel.
				/dev/cpuset allows for exlusive cpus and exclusive NUMA nodes (?how?)
				cpuset.memory_spread_page and cpuset.memory_spread_slab override mbind and set_memory_policy (oh great)
				Look at libcpuset

			TODO: Parse /proc/self/stat, which contains last CPU a process ran on.

			TODO: Modify li
		*/

		/*
		 TODO: PCI device, check that (a) associated_hyper_threads_bit_set == associated_hyper_threads_bitmask and (b), for associated_numa_node == associated_hyper_threads_bitmask
	pub associated_numa_node: Option<NumaNode>,

	pub associated_hyper_threads_bit_set: BitSet<HyperThread>,

	pub associated_hyper_threads_bitmask: BitSet<HyperThread>,
		*/
	}

	pub fn create(sys_path: &SysPath, proc_path: &ProcPath) -> Self
	{
		let mut this = Self
			{
				all_valid_hyper_threads: BitSet::<HyperThread>::valid(sys_path, proc_path);
				map: HashMap::default(),
			};

		if let Some(ref all_valid_numa_nodes) = BitSet::<NumaNode>::valid(sys_path, proc_path)
		{
			for numa_node in all_valid_numa_nodes.iterate()
				{
					let mut associated_hyper_threads = numa_node.associated_hyper_threads(sys_path).unwrap();
					associated_hyper_threads.intersection(&this.all_valid_hyper_threads);
					this.map.insert(numa_node, associated_hyper_threads)
				}
		}

		// TODO: Check each NUMA node has hyper threads unique to it.

		this
	}
}
