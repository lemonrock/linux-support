/proc/x/stat parsing bug in `comm`:-
- Can have embedded ')' in it legitimately
- Maximum of 255 characters,
- Might actually only be 16 characters, due to kernel task struct limit
- Also get this from /proc/x/comm (which doubles as thread name)

FIND ALL `*at()` functions that can take an empty path.
Directory iteration eg getdents
statx

fstatvfs (files, directories, fifos, character/block)
futimens (files, directories, fifos, character/block)
fgetxattr
fsetxattr
flistxattr
fremovexattr
open_by_handle_at
http://man7.org/linux/man-pages/man2/ioctl_ficlonerange.2.html
http://man7.org/linux/man-pages/man2/ioctl_fideduperange.2.html
timerfd.h: TFD_IOC_SET_TICKS
userfaultfd: http://man7.org/linux/man-pages/man2/userfaultfd.2.html
F_GET_RW_HINT: read-write hints http://man7.org/linux/man-pages/man2/fcntl.2.html
PathFileDescriptor (O_PATH)
Review http://man7.org/linux/man-pages/man2/ioctl_list.2.html
- sockios.h
- netrom.h?
- fs.h?
pidfd_getfd: http://man7.org/linux/man-pages/man2/pidfd_getfd.2.html
/// Permission to duplicate another process's file descriptor is governed by a ptrace access mode `PTRACE_MODE_ATTACH_REALCREDS` check.
///
/// Since Linux 5.6.
#[inline(always)]
pub fn duplicate_file_descriptor_from_other_process(file_descriptor: RawFd) -> Result<RawFD, CreationError>
{
}

// TODO: Rework kernel validator into sections for numa, memory, etc.

// TODO: kernel validator proper errors.

// TODO: Numa / hyper thread valid threads, master loops, allocating kernel and other processes (including forcibly moving them), etc

// TODO: ProcStat parsing (has extra information). (http://man7.org/linux/man-pages/man5/proc.5.html)

// TODO: kernel validator huge pages - make generic for powerpc, aarch64 and riscv64.

// TODO: /proc/<N>/oom* files and stuff in /proc/sys
    * /proc/sys/vm/oom_dump_tasks
    * /proc/sys/vm/oom_kill_allocating_task
    * /proc/sys/vm/overcommit_kbytes
    * /proc/sys/vm/overcommit_memory
    * /proc/sys/vm/overcommit_ratio
    * /proc/sys/vm/panic_on_oom

// TODO: Adjust CommitLimit to prevent future out-of-memory.

// TODO: libcpuset and /dev/cpuset and cpusetfs (which has several useful details)

// TODO: Finish NUMA distances.

// TODO: mmap to physicalling contig.
    // TODO: mlock, sync and ?mremap additions first.

// TODO: Allocate physically contiguous memory (using huge / gigantic pages; using page map to finding virt to phys mappings)

// TODO: replace use of bytes.split<n>() with use of memchr()

// TODO: Re-introduce process scheduling which we seem to have lost (?in http server)
    and then add:-
        * /proc/sys/kernel/sched_rr_timeslice_ms
        * /proc/sys/kernel/sched_rt_period_us
        * /proc/sys/kernel/sched_rt_runtime_us

// TODO:  Automatic NUMA balancing can be enabled or disabled for the current session by writing 1 or 0 to /proc/sys/kernel/numa_balancing which will enable or disable the feature respectively. To permanently enable or disable it, use the kernel command line option numa_balancing=[enable|disable].

// TODO: Security changes in process:-
    * /proc/sys/fs/protected_hardlinks
    * /proc/sys/fs/protected_symlinks
    * /proc/sys/fs/suid_dumpable
    * /proc/sys/kernel/dmesg_restrict
    * /proc/sys/kernel/kptr_restrict
    * /proc/sys/kernel/modules_disabled
    * /proc/sys/kernel/randomize_va_space
    * /proc/sys/kernel/sysrq
    * /proc/sys/vm/unprivileged_userfaultfd
    * Disable dnotify using  /proc/sys/fs/dir-notify-enable
// TODO: Near security:-
    * /proc/sys/kernel/panic
    * /proc/sys/kernel/panic_on_oops
    * /proc/sys/kernel/pid_max
// TODO: More memory compaction
    * /proc/sys/vm/compact_memory
    * /proc/sys/vm/drop_caches
    * /proc/sys/vm/swappiness
    * ?/proc/sys/vm/user_reserve_kbytes
    * ?/proc/sys/vm/admin_reserve_kbytes
// TODO: Memory has gone bad
    * /proc/sys/vm/memory_failure_early_kill
    * /proc/sys/vm/memory_failure_recovery
    

/*
/proc/[pid]/smaps_rollup with 2 extra Pss statistics

/proc/zoneinfo
    - more detailed view of /proc/buddyinfo

/proc/buddyinfo
              This file contains information which is used for diagnosing
              memory fragmentation issues.  Each line starts with the iden‐
              tification of the node and the name of the zone which together
              identify a memory region This is then followed by the count of
              available chunks of a certain order in which these zones are
              split.  The size in bytes of a certain order is given by the
              formula:

                  (2^order) * PAGE_SIZE

              The binary buddy allocator algorithm inside the kernel will
              split one chunk into two chunks of a smaller order (thus with
              half the size) or combine two contiguous chunks into one
              larger chunk of a higher order (thus with double the size) to
              satisfy allocation requests and to counter memory fragmenta‐
              tion.  The order matches the column number, when starting to
              count at zero.

              For example on an x86-64 system:

  Node 0, zone     DMA     1    1    1    0    2    1    1    0    1    1    3
  Node 0, zone   DMA32    65   47    4   81   52   28   13   10    5    1  404
  Node 0, zone  Normal   216   55  189  101   84   38   37   27    5    3  587

              In this example, there is one node containing three zones and
              there are 11 different chunk sizes.  If the page size is 4
              kilobytes, then the first zone called DMA (on x86 the first 16
              megabyte of memory) has 1 chunk of 4 kilobytes (order 0)
              available and has 3 chunks of 4 megabytes (order 10) avail‐
              able.

              If the memory is heavily fragmented, the counters for higher
              order chunks will be zero and allocation of large contiguous
              areas will fail.

              Further information about the zones can be found in
              /proc/zoneinfo.

// The /proc/[pid]/pagemap file is present only if the CON‐
//              FIG_PROC_PAGE_MONITOR kernel configuration option is enabled.
//
//              Permission to access this file is governed by a ptrace access
//              mode PTRACE_MODE_READ_FSCREDS check; see ptrace(2).
/proc/pid/pagemap

/proc/kpagecount (since Linux 2.6.25)
              This file contains a 64-bit count of the number of times each
              physical page frame is mapped, indexed by page frame number
              (see the discussion of /proc/[pid]/pagemap).

              The /proc/kpagecount file is present only if the CON‐
              FIG_PROC_PAGE_MONITOR kernel configuration option is enabled.

/proc/kpageflags (since Linux 2.6.25)
              This file contains 64-bit masks corresponding to each physical
              page frame; it is indexed by page frame number (see the dis‐
              cussion of /proc/[pid]/pagemap).  The bits are as follows:
                   0 - KPF_LOCKED
                   1 - KPF_ERROR
                   2 - KPF_REFERENCED
                   3 - KPF_UPTODATE
                   4 - KPF_DIRTY
                   5 - KPF_LRU
                   6 - KPF_ACTIVE
                   7 - KPF_SLAB
                   8 - KPF_WRITEBACK
                   9 - KPF_RECLAIM
                  10 - KPF_BUDDY
                  11 - KPF_MMAP           (since Linux 2.6.31)
                  12 - KPF_ANON           (since Linux 2.6.31)
                  13 - KPF_SWAPCACHE      (since Linux 2.6.31)
                  14 - KPF_SWAPBACKED     (since Linux 2.6.31)
                  15 - KPF_COMPOUND_HEAD  (since Linux 2.6.31)
                  16 - KPF_COMPOUND_TAIL  (since Linux 2.6.31)
                  17 - KPF_HUGE           (since Linux 2.6.31)
                  18 - KPF_UNEVICTABLE    (since Linux 2.6.31)
                  19 - KPF_HWPOISON       (since Linux 2.6.31)
                  20 - KPF_NOPAGE         (since Linux 2.6.31)
                  21 - KPF_KSM            (since Linux 2.6.32)
                  22 - KPF_THP            (since Linux 3.4)
                  23 - KPF_BALLOON        (since Linux 3.18)
                  24 - KPF_ZERO_PAGE      (since Linux 4.0)
                  25 - KPF_IDLE           (since Linux 4.3)

              For further details on the meanings of these bits, see the
              kernel source file Documentation/admin-guide/mm/pagemap.rst.
              Before kernel 2.6.29, KPF_WRITEBACK, KPF_RECLAIM, KPF_BUDDY,
              and KPF_LOCKED did not report correctly.

              The /proc/kpageflags file is present only if the CON‐
              FIG_PROC_PAGE_MONITOR kernel configuration option is enabled.

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
