# linux-support

[linux-support] is a Rust crate for comprehensive Linux support for namespaces, cgroups, processes, scheduling, parsing a vast number of files in `/proc` and `/sys`, signals, hyper threads, CPUS, NUMA nodes, unusual file descriptors (including pid descriptors and userfaultfd), PCI devices and much, much more.

It is intended to be a holistic, very strongly typed and properly modelled library for working with Linux's non-POSIX features, with a particular focus on being a support library for a secure root daemon running a userspace networking stack that I'm developing. As a result, it's highly opionated; it happily makes use of unsafe code, nightly features and `uninitialized()`.

It replaces several C and Rust libraries including:-

* From C, the horrid and effectively legacy libraries:-
    * `libnuma`.
    * `libhugetlbfs`.
    * `libcpuset` (planned).
* From Rust:-
    * [cpu-affinity](https://crates.io/crates/cpu-affinity).
    * [dpdk-unix](https://crates.io/crates/dpdk-unix).
    * [file-descriptors](https://github.com/lemonrock/file-descriptors).
    * [libc-extra](https://crates.io/crates/libc-extra) (partly).
    * [linux-personality](https://crates.io/crates/linux-personality).
    * [memfd](https://crates.io/crates/memfd) (in progress).
    * [num_cpus](https://crates.io/crates/num_cpus).
    * [term-handler](https://crates.io/crates/term-handler).
    * [vm-info](https://crates.io/crates/vm-info).

It is not intended to replace [nix](https://crates.io/crates/nix), but as a partial complement.


## Features supported

* Capabilities and Privileges
* Cgroups version 2
* CPU and HyperThreads, including parsing of files in `/proc` and `/sys` to work out what's available
	* Getting the current HyperThread and NUMA node
* Clean environment variables
* File systems
* Inodes
* Parsing and validating the Linux kernel command line (boot command line)
* Working with Linux kernel modules
* Logging with Syslog
* Memory
    * Fast and const-friendly logic to get page size, ie not `_SC_PAGE_SIZE`!
	* Comprehensive huge and gigantic page support
	* NUMA, including a replacement for libnuma and comprehensive NUMA information and manipulation
	* vmstat, numastat and meminfo file parsing
	* Virtual to Physical mapping and parsing of the page map
* File system mounts and mounting
* File descriptors
	* Including epoll, signalfd, eventfd and others
	* Memory mapping.
* Namespaces
* Nice and autogroups
* PCI Express
	* Finding devices
	* Even working with registers
* Process statistics
	* Parsing of `/proc/<N>/stat` and `/proc/<N>/status`.
* Linux Personality
* Resource Limits
* SecComp
* Signals
    * Including a robust, well thought out signal handler that actually accommodates the weaknesses in Linux's design.
* A comprehensive terminal wrapper.
* Users and groups
	* Including support for working with all 4 user identifiers and all 4 group identifiers a process can have

Currently only Linux using the musl libc has been tested, but support should be possible with minor changes for Android, Fuschia and Emscripten.

The primary architecture of the crate is 64-bit systems, with x86-64 a first tier architecture. Support is then secondary for aarch64 and riscv64 and 'best efforts' for powerpc64. mips64 is a mess, rare in the developers' use cases and so not supported. sparc64 is increasingly obscure with Oracle's discontinuance of development and frustrating in its subtle differences to x86-64, aarch64 and riscv64.

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
