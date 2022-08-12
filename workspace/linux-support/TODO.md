## Unfinished code

* Busy polling
    * See <https://netdevconf.info/2.1/papers/BusyPollingNextGen.pdf>
    * Does not require driver support (part of NAPI)
* "If the NETIF_F_RXHASH flag is set, the 32-bit result of the hash function delivered in the Rx CQ descriptor is set in the received SKB." (Amazon ENA)
* Send a start up Diagnostics message over DogStatsD, or dump to the console.
* Use CLOCK_TAI instead of CLOCK_REALTIME in some Unix-epoch code, eg
    * <https://superuser.com/questions/1156693/is-there-a-way-of-getting-correct-clock-tai-on-linux> for setting the TAI offset.


## Remove lazy_static!

lazy_static! is very dangerous in environments where coroutine memory allocators are in use.
We should probably have a lazy static initializer, using a pre-allocated block of memory.
Could use new Lazy / OnceCell.


## How to use XDP

* Load maps and programs from ELF files, copy code in `ip` tool that bypasses libbpf
* Firewall
    * https://github.com/gamemann/XDP-Firewall
    * https://github.com/Barricade-FW/Firewall
* Can we specify a 'mark' with a XDP processed packet? And get it from userspace?
* Can we use XDP to redirect to a queue?
* <https://stackoverflow.com/questions/44958511/what-is-the-main-difference-between-rss-rps-and-rfs>
* <https://oxnz.github.io/2016/05/03/performance-tuning-networking/>
* Setting a per-connection value on incoming packets in XDP BPF (a mark)
    * Checking incoming packets for permission (IP + PORT + Protocol)
* <https://blog.packagecloud.io/eng/2016/06/22/monitoring-tuning-linux-networking-stack-receiving-data/#preparing-to-receive-data-from-the-network>
* IS `SO_INCOMING_NAPI_ID` any use?
    * Set via `sk_mark_napi_id` and `skb_mark_napi_id`, but ?only for busy poll?
* `SO_INCOMING_CPU` is actually broken: <https://blog.cloudflare.com/perfect-locality-and-three-epic-systemtap-scripts/>
    * Use SO_ATTACH_REUSEPORT_EBPF instead.
* Review `/proc/net/softnet_stat` file with the softnet.sh script: <https://github.com/majek/dump/blob/master/how-to-receive-a-packet/softnet.sh>
* Review the locality examples in <https://lwn.net/Articles/675043/>; includes pinning RSS queues, setting up the RSS indirection table and setting XPS (transmit packet steering)
* Review <https://www.kernel.org/doc/html/v5.8/networking/scaling.html>
* <https://docs.gz.ro/tuning-network-cards-on-linux.html>
* <https://blog.cloudflare.com/how-to-achieve-low-latency/>
* <https://blog.packagecloud.io/eng/2016/06/22/monitoring-tuning-linux-networking-stack-receiving-data/>
* `/proc/sys/net/core/netdev_budget` - defaults to 300.
* <https://xdp-project.net/>


## Medium Importance


### inet and inet6 configuration


### Changing Traffic Class (`qdisc`)

* Look at `tc.c` and `tc_qdisc.c` in iproute2.
* Get scared as it uses a lot of Route Netlink.
* Uggh.


### DogStatsD

* 8192 UDS packet size for dogstatsd (<https://docs.datadoghq.com/developers/dogstatsd/high_throughput/?tab=go#use-dogstatsd-over-uds-unix-domain-socket>)
* Send messages in batches to local UDS (Unix Domain Socket)


## Low Importance


### Unfinished Code

* NUMA distances
* Intel flow director / Network Flow Classifier (flow steering)


### `/proc/sys` sysctls remaining to consider


#### Memory (low mem)

* `vm/lowmem_reserve_ratio`.


#### Memory watermark

* `vm/min_free_kbytes`.
* `vm/watermark_boost_factor`.
* `vm/watermark_scale_factor`.


#### Memory dirtiness

* `vm/dirty_background_bytes`.
* `vm/dirty_background_ratio`.
* `vm/dirty_bytes`.
* `vm/dirty_expire_centisecs` eg 500
* `vm/dirty_ratio`.
* `vm/dirty_writeback_centisecs` eg 3000
* `vm/dirtytime_expire_seconds`.
* `vm/extfrag_threshold`.


#### inet / inet6 settings.

* Lots in sysctl.


#### Kernel miscellany in /proc/sys/kernel

* `acct`:-
    acct:
    
    high water low water frequency
    
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
* `pty/` (maximum number of pseudo-terminals)
* `sched_domain/` (far too complicated)
* `keys/` (constrains memory usage of encryption keys in the kernel)
* `usermodehelper/` ?


### Miscellaneous File system


#### Extended attributes

* Use ioctls `FS_IOC_FSGETXATTR` and `FS_IOC_FSSETXATTR`.
* Uses struct `fsxattr`.
* Definitions of `fsxattr.fsx_xflags` are in `include/uapi/linux/fs.h`.


#### `fcntl` `F_GET_RW_HINT` and `F_GET_FILE_RW_HINT` and related read-write hints

* See <http://man7.org/linux/man-pages/man2/fcntl.2.html>


#### POSIX ACLs

* See <http://man7.org/linux/man-pages/man5/acl.5.html>



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
