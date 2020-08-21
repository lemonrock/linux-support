## Unfinished code

* GlobalNetworkDeviceConfiguration
* OwnedReceiveTransmitMemoryRingQueues::construct
    * Need to attach XDP program
    * Need to work how we use receive and transmit in-memory queues
* NUMA distances
* Report `/proc/sys/kernel/random/boot_id` UUID to DogStatsD as it identifies the current boot.
* Flow director code


## 
/*
	TODO:
	the CPU which handles the interrupt will also process the packet unless:-
		/sys/class/net/<device>/queues/<rx-queue>/rps_cpus - a cpu bitmap - is changed - to enable RPS - receive packet steering - which is a software tech, I think.
		See https://www.suse.com/support/kb/doc/?id=000018430
	
	On NUMA machines, best performance can be achieved by configuring RPS to use the CPUs on the same NUMA node as the interrupt IRQ for the interface's receive queue.
 */


// https://www.kernel.org/doc/html/v5.8/networking/scaling.html
/*

TODO: Receive Queue => CPU

--config-nfc / --config-ntuple
	do_srxclass
		"rx-flow-hash"
		"flow-type"
			do_srxntuple
		"delete"
		
--per-queue ... eg coalesce

Rework set ring params, eg Intel i40e returns EINVAL for values outside of the range I40E_MIN_NUM_DESCRIPTORS ..= I40E_MAX_NUM_DESCRIPTORS
	eg Intel i40e does not suppport rx_mini_pending or rx_jumbo_pending

Indirection RSS hash table
This table can be configured. For example, to make sure all traffic goes only to CPUs #0-#5 (the first NUMA node in our setup), we run:
client$ sudo ethtool -X eth2 weight 1 1 1 1 1 1 0 0 0 0 0
Finally, ensure the interrupts of multiqueue network cards are evenly distributed between CPUs. The irqbalance service is stopped and the interrupts are manually assigned. For simplicity let's pin the RX queue #0 to CPU #0, RX queue #1 to CPU #1 and so on.


We currently use the following in GlobalSchedulingConfiguration


MSI-X - Message Signal Interrupts (Extended).

When using MSI-X, an IRQ is raised for the RX queue the packet was written on.
This IRQ is then mapped to a CPU (or set of CPUs)


client$ (let CPU=0; cd /sys/class/net/eth0/device/msi_irqs/;
         for IRQ in *; do
            echo $CPU > /proc/irq/$IRQ/smp_affinity_list
            let CPU+=1
         done)
NOTE: /sys/class/net/eth2/device/msi_irqs may not exist, in which case:-

	grep eth0 /proc/interrupts
	32:	0	140	45	850264	PCI-MSI-edge	eth0
	
	This means eth0 has assigned IRQ number 32.
	There may be multiple lines.
	The device may not exist at all (eg a Parallels VM)
	
	Change /proc/irq/32/smp_affinity to change the CPUs dealing with that IRQ
		- The list of CPUs should be on the same NUMA node as the eth0 device (ie check eth0's PCI device).

	Other lines might look like this if MSI-X is available:-
	            CPU0       CPU1       CPU2       CPU3
	  65:          1          0          0          0 IR-PCI-MSI-edge      eth0
	  66:  863649703          0          0          0 IR-PCI-MSI-edge      eth0-TxRx-0
	  67:  986285573          0          0          0 IR-PCI-MSI-edge      eth0-TxRx-1
	  68:         45          0          0          0 IR-PCI-MSI-edge      eth0-TxRx-2
	  69:        394          0          0          0 IR-PCI-MSI-edge      eth0-TxRx-3

This is because each RX queue can have its own hardware interrupt assigned if using MSI-X.


Inputs into a weighting algorithm

	- number of receive queues, number_of_receive_queues (eg 2, 11)
	- indirection_table_size (eg 128) - this is the denominator.
	
	
	


https://docs.gz.ro/tuning-network-cards-on-linux.html
https://blog.cloudflare.com/how-to-achieve-low-latency/
https://serverfault.com/questions/772380/how-to-tell-if-nic-has-multiqueue-enabled (multiqueue - more than one rx or tx queue).
https://blog.packagecloud.io/eng/2016/06/22/monitoring-tuning-linux-networking-stack-receiving-data/


You can adjust the net_rx_action budget, which determines how much packet processing can be spent among all NAPI structures registered to a CPU
/proc/sys/net/core/netdev_budget
	- default is 300.

Tuning: Enabling accelerated RFS (aRFS)

Assuming that your NIC and driver support it, you can enable accelerated RFS by enabling and configuring a set of things:

    Have RPS enabled and configured.
    	So, for eth0 and receive queue 0, you would modify the file: /sys/class/net/eth0/queues/rx-0/rps_cpus with a hexadecimal number indicating which CPUs should process packets from eth0’s receive queue 0.
    	https://github.com/torvalds/linux/blob/v3.13/Documentation/networking/scaling.txt#L160-L164
    Have RFS enabled and configured.
    	Have RPS enabled and configured.
    	RFS keeps track of a global hash table of all flows and the size of this hash table can be adjusted by setting the net.core.rps_sock_flow_entries sysctl.
    	Next, you can also set the number of flows per RX queue by writing this value to the sysfs file named rps_flow_cnt for each RX queue.
		Example: increase the number of flows for RX queue 0 on eth0 to 2048.
		$ sudo bash -c 'echo 2048 > /sys/class/net/eth0/queues/rx-0/rps_flow_cnt'
    Your kernel has CONFIG_RFS_ACCEL enabled at compile time. The Ubuntu kernel 3.13.0 does.
    Have ntuple support enabled for the device, as described previously. You can use ethtool to verify that ntuple support is enabled for the device.
    Configure your IRQ settings to ensure each RX queue is handled by one of your desired network processing CPUs.

Once the above is configured, accelerated RFS will be used to automatically move data to the RX queue tied to a CPU core that is processing data for that flow and you won’t need to specify an ntuple filter rule manually for each flow.


/sys/class/net/eth0

    queues/
        tx-0/
            rps_cpus
                00000000
                read-write
            rps_flow_cnt
                0
                read-write
        rx-0/
            traffic_class
                couldn't be read
            tx_maxrate
                0
                read-write
            tx_timeout
                0 (without line feed)
            xps_cpus
                couldn't be read
                read-write
            xps_rxqs
                0
                read-write
            byte_queue_limits/
                hold_time
                    1000
                    read-write
                inflight
                    0
                    read-only
                limit
                    0
                    read-write
                limit_max
                    1879048192
                    read-write
                limit_min
                    0
                    read-write
    power/
        autosuspend_delay_ms
            couldn't be read
            read-write
        control
            "auto"
            read-write
        runtime_active_time
        runtime_status
            "unsupported"
        runtime_suspended_time
 */


## `/proc/sys` sysctls remaining to consider


### Memory (lowmem)

* `vm/lowmem_reserve_ratio`.


### Memory watermark

* `vm/min_free_kbytes`.
* `vm/watermark_boost_factor`.
* `vm/watermark_scale_factor`.


### Memory dirtiness

* `vm/dirty_background_bytes`.
* `vm/dirty_background_ratio`.
* `vm/dirty_bytes`.
* `vm/dirty_expire_centisecs` eg 500
* `vm/dirty_ratio`.
* `vm/dirty_writeback_centisecs` eg 3000
* `vm/dirtytime_expire_seconds`.
* `vm/extfrag_threshold`.


### Kernel miscellany in /proc/sys/kernel

* `acct`:-
    acct:
    
    highwater lowwater frequency
    
    If BSD-style process accounting is enabled these values control
    its behaviour. If free space on filesystem where the log lives
    goes below <lowwater>% accounting suspends. If free space gets
    above <highwater>% accounting resumes. <Frequency> determines
    how often do we check the amount of free space (value is in
    seconds). Default:
    4 2 30
    That is, suspend accounting if there left <= 2% free; resume it
    if we got >=4%; consider information about amount of free space
    valid for 30 seconds.
* `perf_cpu_time_max_percent`
* `perf_event_max_contexts_per_stack`
* `perf_event_max_sample_rate`
* `perf_event_mlock_kb`
* `pty/` (maximum number of pseudoterminals)
* `sched_domain/` (far too complicated)
* `keys/` (constrains memory usage of encryption keys in the kernel)
* `usermodehelper/` ?


## NUMA and hyper thread choices

* Finish use of NUMA distance calculations
* Finish validating NUMA nodes


```
pub struct ValidatedNumaNodeToHyperThreadMap
{
	all_valid_hyper_threads: HyperThreads,
	map: HashMap<NumaNode, HyperThreads>,
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

			TODO: Modify li
		*/

		/*
		 TODO: PCI device, check that (a) associated_hyper_threads_bit_set == associated_hyper_threads_bitmask and (b), for associated_numa_node == associated_hyper_threads_bitmask
	pub associated_numa_node: Option<NumaNode>,

	pub associated_hyper_threads_bit_set: HyperThreads,

	pub associated_hyper_threads_bitmask: HyperThreads,
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
```

### eBPF

* Finish eBPF XDP program attach and how we actually use the in-memory queues! vs just as a firewall


### ethtool

* Look at IRQ affinity mapping for Internet Flow Directory (look at irq script in `~/Downloads/25_2/*/set_irq_affinity`).
* Finish ethtool flow direction


### Diagnostics

* Dump out all known information


### Cgroups v2


#### Stuff

* How to migrate processes and threads inside cgroups?
    * eg Ours vs kthreads vs all others
* How to configure cpuset cgroup?
* How to delete an old cgroup hierarchy?


#### Low Priority

* Add reading and writing `io` controller files (files seem empty vs docs)
* Add configuring `io` controller


### Miscellaneous File system

#### `userfaultfd`
<http://man7.org/linux/man-pages/man2/userfaultfd.2.html>


#### `fcntl` `F_GET_RW_HINT` read-write hints
<http://man7.org/linux/man-pages/man2/fcntl.2.html>


#### POSIX ACLs
<http://man7.org/linux/man-pages/man5/acl.5.html>


#### Process control for `PR_TASK_PERF_EVENTS_DISABLE` and `PR_TASK_PERF_EVENTS_ENABLE`


#### Process control - adjust child death signal (deathsig)


#### Extended attributes:-

Flags `FS_IOC_FSGETXATTR` and `FS_IOC_FSSETXATTR`; uses structure `fsxattr`:-

```
#define FS_IOC_FSGETXATTR		_IOR('X', 31, struct fsxattr)
#define FS_IOC_FSSETXATTR		_IOW('X', 32, struct fsxattr)
/*
 * Structure for FS_IOC_FSGETXATTR[A] and FS_IOC_FSSETXATTR.
 */
struct fsxattr {
	__u32		fsx_xflags;	/* xflags field value (get/set) */
	__u32		fsx_extsize;	/* extsize field value (get/set)*/
	__u32		fsx_nextents;	/* nextents field value (get)	*/
	__u32		fsx_projid;	/* project identifier (get/set) */
	__u32		fsx_cowextsize;	/* CoW extsize field value (get/set)*/
	unsigned char	fsx_pad[8];
};

/*
 * Flags for the fsx_xflags field
 */
#define FS_XFLAG_REALTIME	0x00000001	/* data in realtime volume */
#define FS_XFLAG_PREALLOC	0x00000002	/* preallocated file extents */
#define FS_XFLAG_IMMUTABLE	0x00000008	/* file cannot be modified */
#define FS_XFLAG_APPEND		0x00000010	/* all writes append */
#define FS_XFLAG_SYNC		0x00000020	/* all writes synchronous */
#define FS_XFLAG_NOATIME	0x00000040	/* do not update access time */
#define FS_XFLAG_NODUMP		0x00000080	/* do not include in backups */
#define FS_XFLAG_RTINHERIT	0x00000100	/* create with rt bit set */
#define FS_XFLAG_PROJINHERIT	0x00000200	/* create with parents projid */
#define FS_XFLAG_NOSYMLINKS	0x00000400	/* disallow symlink creation */
#define FS_XFLAG_EXTSIZE	0x00000800	/* extent size allocator hint */
#define FS_XFLAG_EXTSZINHERIT	0x00001000	/* inherit inode extent size */
#define FS_XFLAG_NODEFRAG	0x00002000	/* do not defragment */
#define FS_XFLAG_FILESTREAM	0x00004000	/* use filestream allocator */
#define FS_XFLAG_DAX		0x00008000	/* use DAX for IO */
#define FS_XFLAG_COWEXTSIZE	0x00010000	/* CoW extent size allocator hint */
#define FS_XFLAG_HASATTR	0x80000000	/* no DIFLAG for this	*/
```

	// TODO: 8192 UDS packet size for dogstatsd
		https://docs.datadoghq.com/developers/dogstatsd/high_throughput/?tab=go#use-dogstatsd-over-uds-unix-domain-socket

	// TODO: Do we want to turn the socket's CPU into actual a CPU chosen from the same NUMA node the socket's CPU is on? A round-robin load balancer?

	// TODO: Actually send messages to DataDog in batches (outbound batches Unix domain socket packets w/o a coroutine potentially)

	// TODO: Review how to do RX queues and tie them to CPUs using https://blog.packagecloud.io/eng/2016/06/22/monitoring-tuning-linux-networking-stack-receiving-data/#preparing-to-receive-data-from-the-network
		// TODO: https://blog.cloudflare.com/how-to-achieve-low-latency/
	// TODO: SO_INCOMING_CPU might actually be broken: https://blog.cloudflare.com/perfect-locality-and-three-epic-systemtap-scripts/
	// TODO: Review the locality examples in https://lwn.net/Articles/675043/
	"We performed a bit of tuning, which included inspecting:

         number of RSS queues and their interrupts being pinned to right CPUs
         the indirection table
         correct XPS settings on the TX path"

	// TODO: "SO_INCOMING_NAPI_ID"

	// TODO: /proc/net/softnet_stat file with the softnet.sh script: https://github.com/majek/dump/blob/master/how-to-receive-a-packet/softnet.sh

/*
	Operation combinations

		Obtain Buffer then Operation Read Fixed link-with Operation Timeout then Release Buffer
			or, Cant Obtain so Operation Timeout and Try Again

		Obtain Buffer then Operation Write Fixed link-with Timeout then Release Buffer
			or, Cant Obtain so Operation Timeout and Try Again

		Obtain File Descriptor Index, Obtain Buffer then Operation Register File Descriptor link-with Operation Read/Write Fixed link-with Operation (De)Register File Descriptor link-with Operation Timeout then Release Buffer then Release File Descriptor Index
			- that's a lot of steps for a very, very minor performance gain of using File Descriptor Index
			- File Descriptor Index are in very short supply compared to buffers
			- Links are only performant on Linux 5.7, and, even then, probably add overhead
			- Registered file descriptor indices only make sense for long-lived network connections

		Multiple Queued operations (Reads, Writes, etc) link-with Timeout

		Obtain File Descriptor Index then Operation Accept then Operation Update File Descriptor
			or, Can't Obtain so Operation Timeout and Try Again
			alternative, Operation Accept, Obtain File Descriptor, can't so Operation Timeout and Try Again
				or, can't, so Operation Close

		Operation Connect link-with Operation Timeout

		Operation Close link-with Operation Update File Descriptor then Release File Descriptor Index
			- Need to decide order; File Descriptor Index is a precious resource
				- Close does not use a File Descriptor Index; not sure of impact of leaving a registered file descriptor index if using close

	Cleaning up a coroutine 'on drop'

		Need to cancel all outstanding SQEs
		Need to ignore all results
		Need to tell the coroutine "you are dead"
		Need to identify incoming CQE user data for a dead coroutine w/o having to keep coroutine heap and stack alive
		Need to Deregister Buffer(s)

	Question to answer
		Should sockets now be opened blocking? (some hints that is the better case)

	Tracking outstanding completions
		If we know the total we can wait for all of them
			- but no advantage, as we're block for all of them
		We can know the total for a particular coroutine, and so put in place a timeout (which we can then cancel)
			We can have a timeout to kill a coroutine after X seconds regardless of what's pending, a sort-of coroutine inactivity timeout, but I think a coroutine is best placed to know this

	Coroutine wake up combinations
		All pending completions have completed
		Some pending completions have completed
		No pending completions have completed but some sort of timeout has occurred
		Non-IO completion to wake up to try something again (eg obtaining a buffer, posting a message)
		Wake ups are very cheap, the cost of an indirect function call, so it's probably best to just get a coroutine to decide whether to continue by waking it up.
		One worry is the use of multiple buffers by a coroutine

	Forgetting file descriptors
		We need to change file descriptor logic to 'know' they shouldn't close on drop, or come up with a better way
			Close on drop works well for a panic or unexpected exit, however

	Coroutine completion coalescing
		Instead of waking-up a coroutine immediately for each completion, we pull off all of them and push them into an internal queue or array (indexed by a 4-bit operation index code).
		We must remember there is no order to completions; this is tricky for 'linked' completions.

		Some completions require immediate wake up, eg a timeout of a read, write or connect which may be coroutine-ending.
			This can be an 'always wake up on any of these operation index codes' bit mask.

		Some completions do not, eg we are waiting on two reads. Until both reads are true we do not want to wake up.
			This can be the inverse of the 'always wake up on any of these operation index codes' bit mask.

		However, we may want to also wake up when one read fails of the two both being done. Thus we want to wake up when:-
			- one or both reads fail (failure being permament but not a temporary EAGAIN like situation)
			- one or both reads timeout (ie report that they are cancelled OR the timeout reports it is succcesful)
			- both reads succeed without timeout
				- we don't want to wake up at all for 'without timeout completions'
			- but not one read succeeds and the other is still pending

		The simplest approach is to leave processing from i32 to Result _to the coroutine_, and wake up the coroutine every single time a result arrives
			- the downside is that we won't have committed our completions yet, so the completion queue and / or submission queue may still be full.
			- if the submission queue is full the coroutine will want to be woken up AGAIN as soon as it isn't at the exact point it was full.

	https://github.com/CarterLi/liburing4cpp
	Performance suggestions

		Batch syscalls
			Use io_uring_for_each_cqe to peek multiple cqes once, handle them and enqueue multiple sqes.
			After all cqes are handled, submit all sqes.
			ie Handle multiple CQEs then submit ALL SQEs ONCE

		Blocking for reads of non-disk I/O
			For operations that may block, kernel will punt them into a kernel worker called io-wq, which turns out to have a high overhead cost.
			Always make sure that the fd to read is ready to read.
			Thus use POLL_ADD then *separately* FIXED-READ
			This is changing in Linux 5.7.

		Link-With
			This is borked in Linux 5.6 and before; it forces operations after poll be executed async, that makes POLL_ADD mostly useless (See: https://lore.kernel.org/io-uring/5f09d89a-0c6d-47c2-465c-993af0c7ae71@kernel.dk/).

		Link-With for READ link-with WRITE
			A short read is an error for the link-chain which then cancels WRITE.

		Fixed file descriptors have very little performance boost for a lot of complexity and a *very* limited resource.
 */
