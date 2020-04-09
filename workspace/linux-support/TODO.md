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


/*
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

/proc/pid/map
address                  perms offset   dev   inode                      pathname

eg
55c0f5fd4000-55c0f5fe0000 r--p 00000000 08:03 1048666                    /bin/busybox
55c0f5fe0000-55c0f607c000 r-xp 0000c000 08:03 1048666                    /bin/busybox
55c0f607c000-55c0f609d000 r--p 000a8000 08:03 1048666                    /bin/busybox
55c0f609e000-55c0f60a2000 r--p 000c9000 08:03 1048666                    /bin/busybox
55c0f60a2000-55c0f60a3000 rw-p 000cd000 08:03 1048666                    /bin/busybox
55c0f6864000-55c0f6887000 rw-p 00000000 00:00 0                          [heap]
7f0951b20000-7f0951b35000 r--p 00000000 08:03 2097277                    /lib/ld-musl-x86_64.so.1
7f0951b35000-7f0951b7c000 r-xp 00015000 08:03 2097277                    /lib/ld-musl-x86_64.so.1
7f0951b7c000-7f0951bb0000 r--p 0005c000 08:03 2097277                    /lib/ld-musl-x86_64.so.1
7f0951bb1000-7f0951bb2000 r--p 00090000 08:03 2097277                    /lib/ld-musl-x86_64.so.1
7f0951bb2000-7f0951bb3000 rw-p 00091000 08:03 2097277                    /lib/ld-musl-x86_64.so.1
7f0951bb3000-7f0951bb6000 rw-p 00000000 00:00 0
7ffc4c759000-7ffc4c77a000 rw-p 00000000 00:00 0                          [stack]
7ffc4c796000-7ffc4c799000 r--p 00000000 00:00 0                          [vvar]
7ffc4c799000-7ffc4c79a000 r-xp 00000000 00:00 0                          [vdso]

The perms field is a set of permissions:
                  r = read
                  w = write
                  x = execute
                  s = shared
                  p = private (copy on write)
Map to a mixture of enum Sharing and enum Protection

offset is u32 hexadecimal and is always zero if not a file.
inode is the inode on that device.
              0 indicates that no inode is associated with the memory
              region, as would be the case with BSS (uninitialized data).

pathname:-
[stack]
	  The initial process's (also known as the main
	  thread's) stack.
	Must be 'rw-p'
	One entry only, always present

[stack:<tid>] (from Linux 3.4 to 4.4)
	  A thread's stack (where the <tid> is a thread ID).
	  It corresponds to the /proc/[pid]/task/[tid]/
	  path.  This field was removed in Linux 4.5, since
	  providing this information for a process with
	  large numbers of threads is expensive.

[vdso] The virtual dynamically linked shared object.  See
	  vdso(7).
	Must be 'r-xp'
	One entry only, always present

[vvar]: https://lwn.net/Articles/615809/
	Must be 'r--p'
	One entry only, always present

[heap] The process's heap.
	Must be 'rw-p'
	One entry only, always present

If the pathname field is blank, this is an anonymous mapping
              as obtained via mmap(2).  There is no easy way to coordinate
              this back to a process's source, short of running it through
              gdb(1), strace(1), or similar.

pathname is shown unescaped except for newline characters,
              which are replaced with an octal escape sequence.  As a
              result, it is not possible to determine whether the original
              pathname contained a newline character or the literal \e012
              character sequence.
If the mapping is file-backed and the file has been deleted,
              the string " (deleted)" is appended to the pathname.  Note
              that this is ambiguous too.
A mapping under /proc/<N>/map_files/symlink will exist.
Mappings also exist here for MAP_ANON | MAPSHARED


/proc/pid/numa_maps
file=
    - one of
        - mapped= mapmax=, OR
        - anon= dirty=
    - always have NX= kernelpagesize_kB=
heap
    - anon= dirty=
    - always have NX= kernelpagesize_kB=
    Does not have to present (eg missing for init)
stack
    - anon= dirty=
    - always have NX= kernelpagesize_kB=
private (starts anon=)
    - anon= dirty=
    - always have NX= kernelpagesize_kB=
blank (probably vdso / vddr; no value of N<X>=)

alpine:/proc$ cat /proc/2442/numa_maps
55c0f5fd4000 default file=/bin/busybox mapped=12 mapmax=10 N0=12 kernelpagesize_kB=4
55c0f5fe0000 default file=/bin/busybox mapped=90 mapmax=14 N0=90 kernelpagesize_kB=4
55c0f607c000 default file=/bin/busybox mapped=33 mapmax=14 N0=33 kernelpagesize_kB=4
55c0f609e000 default file=/bin/busybox anon=4 dirty=4 N0=4 kernelpagesize_kB=4
55c0f60a2000 default file=/bin/busybox anon=1 dirty=1 N0=1 kernelpagesize_kB=4
55c0f6864000 default heap anon=35 dirty=35 N0=35 kernelpagesize_kB=4
7f0951b20000 default file=/lib/ld-musl-x86_64.so.1 mapped=20 mapmax=11 N0=20 kernelpagesize_kB=4
7f0951b35000 default file=/lib/ld-musl-x86_64.so.1 mapped=60 mapmax=18 N0=60 kernelpagesize_kB=4
7f0951b7c000 default file=/lib/ld-musl-x86_64.so.1 mapped=32 mapmax=18 N0=32 kernelpagesize_kB=4
7f0951bb1000 default file=/lib/ld-musl-x86_64.so.1 anon=1 dirty=1 N0=1 kernelpagesize_kB=4
7f0951bb2000 default file=/lib/ld-musl-x86_64.so.1 anon=1 dirty=1 N0=1 kernelpagesize_kB=4
7f0951bb3000 default anon=3 dirty=3 N0=3 kernelpagesize_kB=4
7ffc4c759000 default stack anon=17 dirty=17 N0=17 kernelpagesize_kB=4
7ffc4c796000 default
7ffc4c799000 default
alpine:/proc$ cat /proc/2442/maps



From-To Perms Offset Maj:Min Inode FilePath/Psuedo/Blank
55c0f5fd4000-55c0f5fe0000 r--p 00000000 08:03 1048666                    /bin/busybox
55c0f5fe0000-55c0f607c000 r-xp 0000c000 08:03 1048666                    /bin/busybox
55c0f607c000-55c0f609d000 r--p 000a8000 08:03 1048666                    /bin/busybox
55c0f609e000-55c0f60a2000 r--p 000c9000 08:03 1048666                    /bin/busybox
55c0f60a2000-55c0f60a3000 rw-p 000cd000 08:03 1048666                    /bin/busybox
55c0f6864000-55c0f6887000 rw-p 00000000 00:00 0                          [heap]
7f0951b20000-7f0951b35000 r--p 00000000 08:03 2097277                    /lib/ld-musl-x86_64.so.1
7f0951b35000-7f0951b7c000 r-xp 00015000 08:03 2097277                    /lib/ld-musl-x86_64.so.1
7f0951b7c000-7f0951bb0000 r--p 0005c000 08:03 2097277                    /lib/ld-musl-x86_64.so.1
7f0951bb1000-7f0951bb2000 r--p 00090000 08:03 2097277                    /lib/ld-musl-x86_64.so.1
7f0951bb2000-7f0951bb3000 rw-p 00091000 08:03 2097277                    /lib/ld-musl-x86_64.so.1
7f0951bb3000-7f0951bb6000 rw-p 00000000 00:00 0 
7ffc4c759000-7ffc4c77a000 rw-p 00000000 00:00 0                          [stack]
7ffc4c796000-7ffc4c799000 r--p 00000000 00:00 0                          [vvar]
7ffc4c799000-7ffc4c79a000 r-xp 00000000 00:00 0                          [vdso]


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


NOTE: There is also /proc/[pid]/smaps_rollup, with a similar but only-one-entry structure to /proc/[pid]/smaps , with slightly different entries.


/proc/[pid]/smaps (since Linux 2.6.14)
              This file shows memory consumption for each of the process's
              mappings.  (The pmap(1) command displays similar information,
              in a form that may be easier for parsing.)  For each mapping
              there is a series of lines such as the following:
              
55c0f5fd4000-55c0f5fe0000 r--p 00000000 08:03 1048666                    /bin/busybox
Size:                 48 kB
KernelPageSize:        4 kB
MMUPageSize:           4 kB
Rss:                  48 kB
Pss:                   4 kB
Shared_Clean:         48 kB
Shared_Dirty:          0 kB
Private_Clean:         0 kB
Private_Dirty:         0 kB
Referenced:           48 kB
Anonymous:             0 kB
LazyFree:              0 kB
AnonHugePages:         0 kB
ShmemPmdMapped:        0 kB
FilePmdMapped:        0 kB
Shared_Hugetlb:        0 kB
Private_Hugetlb:       0 kB
Swap:                  0 kB
SwapPss:               0 kB
Locked:                0 kB
THPeligible:		0
VmFlags: rd mr mw me dw
55c0f5fe0000-55c0f607c000 r-xp 0000c000 08:03 1048666                    /bin/busybox
Size:                624 kB
KernelPageSize:        4 kB
MMUPageSize:           4 kB
Rss:                 360 kB
Pss:                 143 kB
Shared_Clean:        288 kB
Shared_Dirty:          0 kB
Private_Clean:        72 kB
Private_Dirty:         0 kB
Referenced:          360 kB
Anonymous:             0 kB
LazyFree:              0 kB
AnonHugePages:         0 kB
ShmemPmdMapped:        0 kB
FilePmdMapped:        0 kB
Shared_Hugetlb:        0 kB
Private_Hugetlb:       0 kB
Swap:                  0 kB
SwapPss:               0 kB
Locked:                0 kB
THPeligible:		0
VmFlags: rd ex mr mw me dw
55c0f607c000-55c0f609d000 r--p 000a8000 08:03 1048666                    /bin/busybox
Size:                132 kB
KernelPageSize:        4 kB
MMUPageSize:           4 kB
Rss:                 132 kB
Pss:                   9 kB
Shared_Clean:        132 kB
Shared_Dirty:          0 kB
Private_Clean:         0 kB
Private_Dirty:         0 kB
Referenced:          132 kB
Anonymous:             0 kB
LazyFree:              0 kB
AnonHugePages:         0 kB
ShmemPmdMapped:        0 kB
FilePmdMapped:        0 kB
Shared_Hugetlb:        0 kB
Private_Hugetlb:       0 kB
Swap:                  0 kB
SwapPss:               0 kB
Locked:                0 kB
THPeligible:		0
VmFlags: rd mr mw me dw
55c0f609e000-55c0f60a2000 r--p 000c9000 08:03 1048666                    /bin/busybox
Size:                 16 kB
KernelPageSize:        4 kB
MMUPageSize:           4 kB
Rss:                  16 kB
Pss:                  16 kB
Shared_Clean:          0 kB
Shared_Dirty:          0 kB
Private_Clean:         0 kB
Private_Dirty:        16 kB
Referenced:           16 kB
Anonymous:            16 kB
LazyFree:              0 kB
AnonHugePages:         0 kB
ShmemPmdMapped:        0 kB
FilePmdMapped:        0 kB
Shared_Hugetlb:        0 kB
Private_Hugetlb:       0 kB
Swap:                  0 kB
SwapPss:               0 kB
Locked:                0 kB
THPeligible:		0
VmFlags: rd mr mw me dw ac
55c0f60a2000-55c0f60a3000 rw-p 000cd000 08:03 1048666                    /bin/busybox
Size:                  4 kB
KernelPageSize:        4 kB
MMUPageSize:           4 kB
Rss:                   4 kB
Pss:                   4 kB
Shared_Clean:          0 kB
Shared_Dirty:          0 kB
Private_Clean:         0 kB
Private_Dirty:         4 kB
Referenced:            4 kB
Anonymous:             4 kB
LazyFree:              0 kB
AnonHugePages:         0 kB
ShmemPmdMapped:        0 kB
FilePmdMapped:        0 kB
Shared_Hugetlb:        0 kB
Private_Hugetlb:       0 kB
Swap:                  0 kB
SwapPss:               0 kB
Locked:                0 kB
THPeligible:		0
VmFlags: rd wr mr mw me dw ac
55c0f6864000-55c0f6887000 rw-p 00000000 00:00 0                          [heap]
Size:                140 kB
KernelPageSize:        4 kB
MMUPageSize:           4 kB
Rss:                 140 kB
Pss:                 140 kB
Shared_Clean:          0 kB
Shared_Dirty:          0 kB
Private_Clean:         0 kB
Private_Dirty:       140 kB
Referenced:          140 kB
Anonymous:           140 kB
LazyFree:              0 kB
AnonHugePages:         0 kB
ShmemPmdMapped:        0 kB
FilePmdMapped:        0 kB
Shared_Hugetlb:        0 kB
Private_Hugetlb:       0 kB
Swap:                  0 kB
SwapPss:               0 kB
Locked:                0 kB
THPeligible:		0
VmFlags: rd wr mr mw me ac
7f0951b20000-7f0951b35000 r--p 00000000 08:03 2097277                    /lib/ld-musl-x86_64.so.1
Size:                 84 kB
KernelPageSize:        4 kB
MMUPageSize:           4 kB
Rss:                  80 kB
Pss:                   7 kB
Shared_Clean:         80 kB
Shared_Dirty:          0 kB
Private_Clean:         0 kB
Private_Dirty:         0 kB
Referenced:           80 kB
Anonymous:             0 kB
LazyFree:              0 kB
AnonHugePages:         0 kB
ShmemPmdMapped:        0 kB
FilePmdMapped:        0 kB
Shared_Hugetlb:        0 kB
Private_Hugetlb:       0 kB
Swap:                  0 kB
SwapPss:               0 kB
Locked:                0 kB
THPeligible:		0
VmFlags: rd mr mw me dw
7f0951b35000-7f0951b7c000 r-xp 00015000 08:03 2097277                    /lib/ld-musl-x86_64.so.1
Size:                284 kB
KernelPageSize:        4 kB
MMUPageSize:           4 kB
Rss:                 240 kB
Pss:                  13 kB
Shared_Clean:        240 kB
Shared_Dirty:          0 kB
Private_Clean:         0 kB
Private_Dirty:         0 kB
Referenced:          240 kB
Anonymous:             0 kB
LazyFree:              0 kB
AnonHugePages:         0 kB
ShmemPmdMapped:        0 kB
FilePmdMapped:        0 kB
Shared_Hugetlb:        0 kB
Private_Hugetlb:       0 kB
Swap:                  0 kB
SwapPss:               0 kB
Locked:                0 kB
THPeligible:		0
VmFlags: rd ex mr mw me dw
7f0951b7c000-7f0951bb0000 r--p 0005c000 08:03 2097277                    /lib/ld-musl-x86_64.so.1
Size:                208 kB
KernelPageSize:        4 kB
MMUPageSize:           4 kB
Rss:                 128 kB
Pss:                  10 kB
Shared_Clean:        128 kB
Shared_Dirty:          0 kB
Private_Clean:         0 kB
Private_Dirty:         0 kB
Referenced:          128 kB
Anonymous:             0 kB
LazyFree:              0 kB
AnonHugePages:         0 kB
ShmemPmdMapped:        0 kB
FilePmdMapped:        0 kB
Shared_Hugetlb:        0 kB
Private_Hugetlb:       0 kB
Swap:                  0 kB
SwapPss:               0 kB
Locked:                0 kB
THPeligible:		0
VmFlags: rd mr mw me dw
7f0951bb1000-7f0951bb2000 r--p 00090000 08:03 2097277                    /lib/ld-musl-x86_64.so.1
Size:                  4 kB
KernelPageSize:        4 kB
MMUPageSize:           4 kB
Rss:                   4 kB
Pss:                   4 kB
Shared_Clean:          0 kB
Shared_Dirty:          0 kB
Private_Clean:         0 kB
Private_Dirty:         4 kB
Referenced:            4 kB
Anonymous:             4 kB
LazyFree:              0 kB
AnonHugePages:         0 kB
ShmemPmdMapped:        0 kB
FilePmdMapped:        0 kB
Shared_Hugetlb:        0 kB
Private_Hugetlb:       0 kB
Swap:                  0 kB
SwapPss:               0 kB
Locked:                0 kB
THPeligible:		0
VmFlags: rd mr mw me dw ac
7f0951bb2000-7f0951bb3000 rw-p 00091000 08:03 2097277                    /lib/ld-musl-x86_64.so.1
Size:                  4 kB
KernelPageSize:        4 kB
MMUPageSize:           4 kB
Rss:                   4 kB
Pss:                   4 kB
Shared_Clean:          0 kB
Shared_Dirty:          0 kB
Private_Clean:         0 kB
Private_Dirty:         4 kB
Referenced:            4 kB
Anonymous:             4 kB
LazyFree:              0 kB
AnonHugePages:         0 kB
ShmemPmdMapped:        0 kB
FilePmdMapped:        0 kB
Shared_Hugetlb:        0 kB
Private_Hugetlb:       0 kB
Swap:                  0 kB
SwapPss:               0 kB
Locked:                0 kB
THPeligible:		0
VmFlags: rd wr mr mw me dw ac
7f0951bb3000-7f0951bb6000 rw-p 00000000 00:00 0
Size:                 12 kB
KernelPageSize:        4 kB
MMUPageSize:           4 kB
Rss:                  12 kB
Pss:                  12 kB
Shared_Clean:          0 kB
Shared_Dirty:          0 kB
Private_Clean:         0 kB
Private_Dirty:        12 kB
Referenced:           12 kB
Anonymous:            12 kB
LazyFree:              0 kB
AnonHugePages:         0 kB
ShmemPmdMapped:        0 kB
FilePmdMapped:        0 kB
Shared_Hugetlb:        0 kB
Private_Hugetlb:       0 kB
Swap:                  0 kB
SwapPss:               0 kB
Locked:                0 kB
THPeligible:		0
VmFlags: rd wr mr mw me ac
7ffc4c759000-7ffc4c77a000 rw-p 00000000 00:00 0                          [stack]
Size:                132 kB
KernelPageSize:        4 kB
MMUPageSize:           4 kB
Rss:                  68 kB
Pss:                  68 kB
Shared_Clean:          0 kB
Shared_Dirty:          0 kB
Private_Clean:         0 kB
Private_Dirty:        68 kB
Referenced:           68 kB
Anonymous:            68 kB
LazyFree:              0 kB
AnonHugePages:         0 kB
ShmemPmdMapped:        0 kB
FilePmdMapped:        0 kB
Shared_Hugetlb:        0 kB
Private_Hugetlb:       0 kB
Swap:                  0 kB
SwapPss:               0 kB
Locked:                0 kB
THPeligible:		0
VmFlags: rd wr mr mw me gd ac
7ffc4c796000-7ffc4c799000 r--p 00000000 00:00 0                          [vvar]
Size:                 12 kB
KernelPageSize:        4 kB
MMUPageSize:           4 kB
Rss:                   0 kB
Pss:                   0 kB
Shared_Clean:          0 kB
Shared_Dirty:          0 kB
Private_Clean:         0 kB
Private_Dirty:         0 kB
Referenced:            0 kB
Anonymous:             0 kB
LazyFree:              0 kB
AnonHugePages:         0 kB
ShmemPmdMapped:        0 kB
FilePmdMapped:        0 kB
Shared_Hugetlb:        0 kB
Private_Hugetlb:       0 kB
Swap:                  0 kB
SwapPss:               0 kB
Locked:                0 kB
THPeligible:		0
VmFlags: rd mr pf io de dd
7ffc4c799000-7ffc4c79a000 r-xp 00000000 00:00 0                          [vdso]
Size:                  4 kB
KernelPageSize:        4 kB
MMUPageSize:           4 kB
Rss:                   4 kB
Pss:                   0 kB
Shared_Clean:          4 kB
Shared_Dirty:          0 kB
Private_Clean:         0 kB
Private_Dirty:         0 kB
Referenced:            4 kB
Anonymous:             0 kB
LazyFree:              0 kB
AnonHugePages:         0 kB
ShmemPmdMapped:        0 kB
FilePmdMapped:        0 kB
Shared_Hugetlb:        0 kB
Private_Hugetlb:       0 kB
Swap:                  0 kB
SwapPss:               0 kB
Locked:                0 kB
THPeligible:		0
VmFlags: rd ex mr mw me de

*/



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
