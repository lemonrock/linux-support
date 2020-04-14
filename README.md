# linux-support

[linux-support] is a Rust crate for comprehensive Linux support for namespaces, cgroups, processes, scheduling, parsing a vast number of files in `/proc` and `/sys`, signals, hyper threads, CPUS, NUMA nodes, unusual file descriptors (including pid descriptors and userfaultfd), PCI devices and much, much more.

It is intended to be a holistic, very strongly typed and properly modelled library for working with Linux's non-POSIX features, with a particular focus on being a support library for a secure root daemon running a userspace networking stack that I'm developing. As a result, it's highly opionated; it happily makes use of unsafe code, nightly features and `uninitialized()`.

File descriptors, process and vectored-io are tightly integrated.

It replaces several C and Rust libraries including:-

* From C, the horrid and effectively legacy libraries:-
    * `libnuma`.
    * `libhugetlbfs`.
    * `libcpuset` (planned).
* From Rust:-
    * [cpu-affinity](https://crates.io/crates/cpu-affinity).
    * [dpdk-unix](https://crates.io/crates/dpdk-unix).
    * [file-descriptors](https://github.com/lemonrock/file-descriptors).
    * [fs2](https://github.com/danburkert/fs2-rs).
        * Adding support for 3 kinds of file locks available since Linux 3.1.
    * [libc-extra](https://crates.io/crates/libc-extra) (partly).
    * [linux-personality](https://crates.io/crates/linux-personality).
    * [iovec](https://crates.io/crates/iovec).
    * [memfd](https://crates.io/crates/memfd).
    * [num_cpus](https://crates.io/crates/num_cpus).
    * [process_vm_io](https://crates.io/crates/process_vm_io).
    * [term-handler](https://crates.io/crates/term-handler).
    * [vm-info](https://crates.io/crates/vm-info).

It is not intended to replace [nix](https://crates.io/crates/nix), but as an alternative for Linux-specific applications.


## Features supported

* Assembly code to get current NUMA node and Hyper Thread.
* Capabilities and Privileges
* Cgroups version 2
* CPU and HyperThreads, including parsing of files in `/proc` and `/sys` to work out what's available
	* Getting the current HyperThread and NUMA node
* Clean environment variables
	* Parsing of `/proc/<N>/cmdline` and `/proc/<N>/environ`.
* File systems
* Inodes
* Parsing and validating the Linux kernel command line (boot command line)
* Working with Linux kernel modules
* Logging with Syslog
    * eg Redirecting standard error to syslog!
* Memory
	* NUMA, including a replacement for libnuma and comprehensive NUMA information and manipulation
	* Comprehensive huge and gigantic page support
    * Fast and `const`-friendly logic to get page size, ie not `sysconf(_SC_PAGE_SIZE)`!
	* vmstat, numastat and meminfo file parsing
	* Page map insight
	* Memory map insight
	* Virtual to Physical mapping and parsing of the page map
	* Read and Write process virtual memory (see Process below).
* File system mounts and mounting
* File descriptors
	* Including epoll, signalfd, eventfd, userfaultfd, pidfd, timerfd, inotify, memfd, POSIX message queues, pipes, sockets, terminals and others
	* Memory mapping.
	* Adding extended seeks for data and holes.
	* Three different kinds of advisory file locks (whole file, per-proces record and the newer and better per open file description record).
* Linux Personality.
    * Get for any process as well as current.
    * Execution domains (legacy feature).
* Namespaces
* Nice and autogroups
* PCI Express
	* Finding devices
	* Even working with registers
* Process, process groups, etc
	* Parsing of `/proc/<N>/stat`, `/proc/<N>/statm` and `/proc/<N>/status`.
	* Linux specific `get_program_name`.
	* Getting session identifier.
	* Getting process group identifier.
	* Reading and writing a process' memory.
* Resource Limits
* SecComp
* Signals
    * Including a robust, well thought out signal handler that actually accommodates the weaknesses in Linux's design and converts to strongly-typed, properly enumerated variants.
    * Support for machine check exceptions.
* Strings
    * Intended to make working with non-UTF8, byte (possibly ASCII) strings easier.
    * Support for Linux quirks, such as oddly escaped strings.
    * Support for Linux string arrays as found in `/proc/<N>/environ`.
    * Mostly intended for library internal use.
    * Support for parsing and formatting Linux's diverse ways of representing numbers in files in `/proc` and `/sys`.
* As strongly-typed as possible syscall wrappers.
* A comprehensive terminal wrapper.
* Users and groups
	* Including support for working with all 4 user identifiers and all 4 group identifiers a process can have (ie the legacy file system user and group ids)
	* Init groups
	* Checks for root where necessary with documented assertions.
* Vectors
    * Proper vector IO, including for File.
    * Integrates Vector I/O for process memory.

Currently only Linux using the musl libc has been tested, but support should be possible with minor changes for Android, Fuschia and Emscripten.

The primary architecture of the crate is 64-bit systems, with x86-64 a first tier architecture. Support is then secondary for aarch64 and riscv64 and 'best efforts' for powerpc64, mips64 and sparc64.

No support is planned for any 32-bit system.

It uses nightly because Rust still hasn't stabilized important features after 4 years.


## File Descriptors module

This is a Rust module wrapping the various kinds of file descriptors with safe abstractions, including IPv4 / IPv6 sockets, Unix domain sockets, epoll, timerfd, signalfd, eventfd, POSIX message queues, pipes, FIFOs, terminals (and serial ports), character devices, inotify, fanotify and Files.

There is a particularly extensive and safe wrapper for signals and terminals.


### Supported File Descriptors

* character devices.
* epoll.
* eventfd.
* fanotify.
* inotify.
* memfd (anonymous memory backed files).
    * See `file` module.
* POSIX message queues (<https://linux.die.net/man/7/mq_overview>).
* pipes and FIFOs (anonymous and named FIFOs), including support for splice, vmsplice and tee.
* Process File Descriptors (pidfd, new in Linux 5.2).
* sockets (TCP, UDP and the equivalent over Unix Domain Sockets; `sendfile()` with Rust's `std::file::File` supported).
* terminals (serial ports and modems).
* timerfd.
* Extensions for `std::fs::File`:-
    * `SendFile`
    * `SpliceRecipient`
    * `SpliceSender`
    * `memfd`
        * Memory mapping files.

Additionally, extensions (`SendFile`, `SpliceRecipient` and `SpliceSender`) are implemented for Rust's `File`.


### Unix Domain Sockets


#### When using file paths

* Every effort is made to create the socket file path cleanly;
* To make sure all parent folders exist;
* To make sure parent folder permissions are correctly set;
* To remove any stale files;
* To remove socket file paths on drop (close).

The above features may not work correctly after the use of `seccomp` to lock down system calls (particularly the attempt to delete a socket file path on close).


### Pipes

* The use of `splice()`, `vmsplice()` and `tee()` are supported for all file descriptors where possible (including Rust's `::std::fs::File`).
* To be able to use epoll with standard in (`stdin`), use `pipes_and_fifos::ReceivePipeFileDescriptor::standard_in()`.
* To be able to use epoll with standard out (`stdout`), use `pipes_and_fifos::SendPipeFileDescriptor::standard_out()`.
* To be able to use epoll with standard error (`stderr`), use `pipes_and_fifos::SendPipeFileDescriptor::standard_error()`.


### Unsupported for now

* Linux zero copy send (`MSG_ZEROCOPY`) and receive (`SO_ZEROCOPY`), mostly because they have a horrible, hacky API.
* `SO_BUSY_POLL` and `SO_INCOMING_CPU`.
* Unix Domain Sockets using `autobind`; setting of the `SO_PASSCRED` socket option.
* Receiving credentials over Unix Domain Sockets using `recvmsg()`.
* `mkfifo()`.
* `mknod()`.
* infiniband sockets.
* canbus (SocketCAN sockets and can4linux <http://can-wiki.info/can4linux/man/can4linux_8h_source.html> character device drivers).


## Licensing

The license for this project is MIT.

[linux-support]: https://github.com/lemonrock/linux-support "linux-support GitHub page"
