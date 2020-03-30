
// TODO: Rework kernel validator into sections for numa, memory, etc.
// TODO: kernel validator proper errors.
// TODO: SignalNumber as BitSet; two kinds of bit sets.
// TODO: ProcStat parsing (has extra information).
// TODO: Check /proc/x/status parsing for [PT] restricted values.
// TODO: Numa Nodes and hyper threads validation (below)
// TODO: kernel validator huge pages - make generic for powerpc, aarch64 and riscv64.
// TODO: u8 - u64, i8 - i64, usize, isize byte string parsing and writing.


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
