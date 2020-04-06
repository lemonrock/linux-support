// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Clone flags.
	#[allow(missing_docs)]
	#[derive(Deserialize, Serialize)]
	pub struct CloneFlags: i32
	{
		/// Clear (zero) the child thread ID at the location `ctid` in child memory when the child exits, and do a wakeup on the futex at that address.
		/// The address involved may be changed by the `man 2 set_tid_address` system call.
		/// This is used by threading libraries.
		///
		/// Since Linux 2.5.49.
		const ZeroChildThreadIdentifierInChildMemoryWhenChildExits = CLONE_CHILD_CLEARTID;

		/// Store the child thread ID at the location `ctid` in the child's memory.
		/// The store operation completes before clone() returns control to user space in the child process.
		/// (Note that the store operation may not have completed before `clone()` returns in the parent process, which will be relevant if the `CLONE_VM` flag is also employed).
		///
		/// Since Linux 2.5.49.
		const StoreChildThreadIdentifierInChildMemory = CLONE_CHILD_SETTID;

		/// If `CLONE_FILES` is set, the calling process and the child process share the same file descriptor table.
		/// Any file descriptor created by the calling process or by the child process is also valid in the other process.
		/// Similarly, if one of the processes closes a file descriptor, or changes its associated flags (using the `man 2 fcntl F_SETFD` operation), the other process is also affected.
		/// If a process sharing a file descriptor table calls `man 2 execve`, its file descriptor table is duplicated (unshared).
		///
		/// If `CLONE_FILES` is not set, the child process inherits a copy of all file descriptors opened in the calling process at the time of `clone()`.
		/// Subsequent operations that open or close file descriptors, or change file descriptor flags, performed by either the calling process or the child process do not affect the other process.
		/// Note, however, that the duplicated file descriptors in the child refer to the same open file descriptions as the corresponding file descriptors in the calling process, and thus share file offsets and file status flags (see `man 2 open`).
		///
		/// Since Linux 2.0.
		const ShareFileDescriptorTable = CLONE_FILES;

		/// If `CLONE_FS` is set, the caller and the child process share the same filesystem information.
		/// This includes the root of the filesystem, the current working directory, and the umask.
		/// Any call to `chroot()`, `chdir()`, or `umask()` performed by the calling process or the child process also affects the other process.
		///
 		/// If `CLONE_FS` is not set, the child process works on a copy of the filesystem information of the calling process at the time of the `clone()` call.
 		/// Calls to `chroot()`, `chdir()`, or `umask()` performed later by one of the processes do not affect the other process.
		///
		/// Since Linux 2.0.
		const ShareFilesystemContext = CLONE_FS;

		/// If `CLONE_IO` is set, then the new process shares an I/O context with the calling process.
		/// If this flag is not set, then (as with `fork()`) the new process has its own I/O context.
		///
		/// The I/O context is the I/O scope of the disk scheduler (ie what the I/O scheduler uses to model scheduling of a process's I/O).
		/// If processes share the same I/O context, they are treated as one by the I/O scheduler.
		/// As a consequence, they get to share disk time.
		/// For some I/O schedulers, if two processes share an I/O context, they will be allowed to interleave their disk access.
		/// If several threads are doing I/O on behalf of the same process (for instance, see `man 3 aio_read()`), they should employ `CLONE_IO` to get better I/O performance.
		///
		/// If the kernel is not configured with the `CONFIG_BLOCK` option, this flag is a no-op.
		///
		/// Since Linux 2.6.25.
		const ShareIoContext = CLONE_IO;

		/// Create the process in a new cgroup namespace.
		///
		/// If this flag is not set, then (as with `fork()`) the process is created in the same cgroup namespaces as the calling process.
		/// This flag is intended for the implementation of containers.
		///
		/// For further information on cgroup namespaces, see `man 7 cgroup_namespaces`.
		///
		/// Only a privileged process (`CAP_SYS_ADMIN`) can employ `CLONE_NEWCGROUP`.
		///
		/// Since Linux 4.6.
		const NewCgroupNamespace = CLONE_NEWCGROUP;

		/// If `CLONE_NEWIPC` is set, then create the process in a new IPC namespace.
		/// If this flag is not set, then (as with `fork()`), the process is created in the same IPC (inter-process communication) namespace as the calling process.
		/// This flag is intended for the implementation of containers.
		///
		/// An IPC namespace provides an isolated view of System V IPC objects (see `man 7 sysvipc`) and (since Linux 2.6.30) POSIX message queues (see `man 7 mq_overview`).
		/// The common characteristic of these IPC mechanisms is that IPC objects are identified by mechanisms other than filesystem pathnames.
		///
		/// Objects created in an IPC namespace are visible to all other processes that are members of that namespace, but are not visible to processes in other IPC namespaces.
		///
		/// When an IPC namespace is destroyed (ie, when the last process that is a member of the namespace terminates), all IPC objects in the namespace are automatically destroyed.
		///
		/// Only a privileged process (`CAP_SYS_ADMIN`) can employ `CLONE_NEWIPC`.
		/// This flag can't be specified in conjunction with `CLONE_SYSVSEM`.
		///
		/// For further information on IPC namespaces, see `man 7 namespaces`.
		///
		/// Since Linux 2.6.19.
		const NewInterProcessCommunicationNamespace = CLONE_NEWIPC;

		/// If `CLONE_NEWNET` is set, then create the process in a new network namespace.
		/// If this flag is not set, then (as with `fork()`) the process is created in the same network namespace as the calling process.
		/// This flag is intended for the implementation of containers.
		///
		/// A network namespace provides an isolated view of the networking stack (network device interfaces, IPv4 and IPv6 protocol stacks, IP routing tables, firewall rules, the `/proc/net` and `/sys/class/net` directory trees, sockets, etc).
		/// A physical network device can live in exactly one network namespace.
		/// A virtual network (`man 4 veth`) device pair provides a pipe-like abstraction that can be used to create tunnels between network namespaces, and can be used to create a bridge to a physical network device in another namespace.
		///
		/// When a network namespace is freed (ie, when the last process in the namespace terminates), its physical network devices are moved back to the initial network namespace (not to the parent of the process).
		/// For further information on network namespaces, see `man 7 namespaces`.
		///
		/// Only a privileged process (`CAP_SYS_ADMIN`) can employ `CLONE_NEWNET`.
		///
		/// Since Linux 2.6.29.
		const NewNetworkNamespace = CLONE_NEWNET;

		/// If `CLONE_NEWNS` is set, the cloned child is started in a new mount namespace, initialized with a copy of the namespace of the parent.
		/// If `CLONE_NEWNS` is not set, the child lives in the same mount namespace as the parent.
		///
		/// Only a privileged process (`CAP_SYS_ADMIN`) can employ `CLONE_NEWNS`.
		/// It is not permitted to specify both `CLONE_NEWNS` and `CLONE_FS` in the same `clone()` call.
		///
		/// For further information on mount namespaces, see `man 7 namespaces` and `man 7 mount_namespaces`.
		///
		/// Since Linux 2.6.19.
		const NewMountNamespace = CLONE_NEWNS;

		/// If `CLONE_NEWPID` is set, then create the process in a new PID namespace.
		/// If this flag is not set, then (as with `fork()`) the process is created in the same PID namespace as the calling process.
		/// This flag is intended for the implementation of containers.
		///
		/// For further information on PID namespaces, see `man 7 namespaces` and `man 7 pid_namespaces`.
		///
		/// Only a privileged process (`CAP_SYS_ADMIN`) can employ `CLONE_NEWPID`.
		/// This flag can't be specified in conjunction with `CLONE_THREAD` or `CLONE_PARENT`.
		///
		/// Since Linux 2.6.24.
		const NewProcessIdentifierNamespace = CLONE_NEWPID;

		/// If `CLONE_NEWUSER` is set, then create the process in a new user namespace.
		/// If this flag is not set, then (as with `fork()`) the process is created in the same user namespace as the calling process.
		///
		/// This flag can't be specified in conjunction with `CLONE_THREAD` or `CLONE_PARENT`.
		/// For security reasons, `CLONE_NEWUSER` cannot be specified in conjunction with `CLONE_FS`.
		///
		/// For further information on user namespaces, see `man 7 namespaces` and `man 7 user_namespaces`.
		///
		/// Since Linux 3.8.
		const NewUserNamespace = CLONE_NEWUSER;

		/// If `CLONE_NEWUTS` is set, then create the process in a new UTS namespace, whose identifiers are initialized by duplicating the identifiers from the UTS namespace of the calling process.
		/// If this flag is not set, then (as with `fork()`) the process is created in the same UTS namespace as the calling process.
		/// This flag is intended for the implementation of containers.
		///
		/// A UTS namespace is the set of identifiers returned by `uname()`; among these, the domain name and the hostname can be modified by `setdomainname()` and `sethostname()`, respectively.
		/// Changes made to the identifiers in a UTS namespace are visible to all other processes in the same namespace, but are not visible to processes in other UTS namespaces.
		///
		/// Only a privileged process (`CAP_SYS_ADMIN`) can employ `CLONE_NEWUTS`.
		///
		/// For further information on UTS namespaces, see `man 7 namespaces`.
		///
		/// Since Linux 2.6.19.
		const NewUtsNamespace = CLONE_NEWUTS;

		/// If `CLONE_PARENT` is set, then the parent of the new child (as returned by `getppid()`) will be the same as that of the calling process.
		///
		/// If `CLONE_PARENT` is not set, then (as with `fork()`) the child's parent is the calling process.
		///
		/// Note that it is the parent process, as returned by `getppid()`, which is signaled when the child terminates, so that if `CLONE_PARENT` is set, then the parent of the calling process, rather than the calling process itself, will be signaled.
		///
		/// Since Linux 2.3.12.
		const ParentOfChildIsParentOfParent = CLONE_PARENT;

		/// Store the child thread identifier at the location `ptid` in the parent's memory.
		///
		/// The store operation completes before `clone()` returns control to user space.
		///
		/// Since Linux 2.5.49.
		const StoreChildThreadIdentifierInParentMemory = CLONE_PARENT_SETTID;

		/// If `CLONE_PTRACE` is specified, and the calling process is being traced, then trace the child also (see `man 2 ptrace()`).
		///
		/// Since Linux 2.2.
		const Trace = CLONE_PTRACE;

		/// The TLS (Thread Local Storage) descriptor is set to `newtls`.
		///
		/// The interpretation of `newtls` and the resulting effect is architecture dependent.
		/// On x86_64 it is the new value to be set for the %fs base register (see the `ARCH_SET_FS` argument to `man 2 arch_prctl()`).
		///
		/// Since Linux 2.5.32.
		const SetThreadLocalStorageDescriptor = CLONE_SETTLS;

		/// If `CLONE_SIGHAND` is set, the calling process and the child process share the same table of signal handlers.
		/// If the calling process or child process calls `sigaction()` to change the behavior associated with a signal, the behavior is changed in the other process as well.
		/// However, the calling process and child processes still have distinct signal masks and sets of pending signals.
		/// So, one of them may block or unblock signals using `sigprocmask()` without affecting the other process.
		///
		/// If `CLONE_SIGHAND` is not set, the child process inherits a copy of the signal handlers of the calling process at the time `clone()` is called.
		/// Calls to `sigaction()` performed later by one of the processes have no effect on the other process.
		///
		/// Flags must also include `CLONE_VM` if `CLONE_SIGHAND` is specified.
		///
		/// Since Linux 2.6.0.
		const ShareSignalHandlersTable = CLONE_SIGHAND;

		/// If `CLONE_SYSVSEM` is set, then the child and the calling process share a single list of System V semaphore adjustment (`semadj`) values (see `man 2 semop()`).
		/// In this case, the shared list accumulates `semadj` values across all processes sharing the list, and semaphore adjustments are performed only when the last process that is sharing the list terminates (or ceases sharing the list using `unshare()`).
		/// If this flag is not set, then the child has a separate `semadj` list that is initially empty.
		///
		/// Since Linux 2.5.10.
		const ShareSysVSemaphoreAdjustmentList = CLONE_SYSVSEM;

		/// If `CLONE_THREAD `is set, the child is placed in the same thread group as the calling process.
		/// To make the remainder of the discussion of `CLONE_THREAD` more readable, the term "thread" is used to refer to the processes within a thread group.
		///
		/// Thread groups were a feature added in Linux 2.4 to support the POSIX threads notion of a set of threads that share a single PID.
		/// Internally, this shared PID is the so-called thread group identifier (`TGID`) for the thread group.
		/// Since Linux 2.4, calls to `getpid()` return the `TGID` of the caller.
		///
		/// The threads within a group can be distinguished by their (system-wide) unique thread IDs (`TID`).
		/// A new thread's `TID` is available as the function result returned to the caller of `clone()`, and a thread can obtain its own `TID` using `gettid()`.
		///
		/// When a call is made to `clone()` without specifying `CLONE_THREAD`, then the resulting thread is placed in a new thread group whose `TGID` is the same as the thread's `TID`.
		/// This thread is the leader of the new thread group.
		///
		/// A new thread created with `CLONE_THREAD` has the same parent process as the caller of `clone()` (ie, like `CLONE_PARENT`), so that calls to `getppid()` return the same value for all of the threads in a thread group.
		/// When a `CLONE_THREAD` thread terminates, the thread that created it using `clone()` is not sent a `SIGCHLD` (or other termination) signal; nor can the status of such a thread be obtained using `wait()`.
		/// (The thread is said to be detached).
		///
		/// After all of the threads in a thread group terminate the parent process of the thread group is sent a `SIGCHLD` (or other termination) signal.
		///
		/// If any of the threads in a thread group performs an `execve()`, then all threads other than the thread group leader are terminated, and the new program is executed in the thread group leader.
		///
		/// If one of the threads in a thread group creates a child using `fork()`, then any thread in the group can `wait()` for that child.
		///
		/// Since Linux 2.5.35, flags must also include `CLONE_SIGHAND` if `CLONE_THREAD` is specified (and note that, since Linux 2.6.0, `CLONE_SIGHAND` also requires `CLONE_VM` to be included).
		///
		/// Signal dispositions and actions are process-wide: if an unhandled signal is delivered to a thread, then it will affect (terminate, stop, continue, be ignored in) all members of the thread group.
		///
		/// Each thread has its own signal mask, as set by `sigprocmask()`.
		///
		/// A signal may be process-directed or thread-directed.
		/// A process-directed signal is targeted at a thread group (ie, a `TGID`), and is delivered to an arbitrarily selected thread from among those that are not blocking the signal.
		/// A signal may be process directed because it was generated by the kernel for reasons other than a hardware exception, or because it was sent using `kill()` or `sigqueue()`.
		/// A thread-directed signal is targeted at (ie, delivered to) a specific thread.
		/// A signal may be thread directed because it was sent using `tgkill()` or `pthread_sigqueue()`, or because the thread executed a machine language instruction that triggered a hardware exception (eg, invalid memory access triggering `SIGSEGV` or a floating-point exception triggering `SIGFPE`).
		///
		/// A call to `sigpending()` returns a signal set that is the union of the pending process-directed signals and the signals that are pending for the calling thread.
		///
		/// If a process-directed signal is delivered to a thread group, and the thread group has installed a handler for the signal, then the handler will be invoked in exactly one, arbitrarily selected member of the thread group that has not blocked the signal.
		/// If multiple threads in a group are waiting to accept the same signal using sigwaitinfo(2), the kernel will arbitrarily select one of these threads to receive the signal.
		///
		/// Since Linux 2.4.0.
		const Thread = CLONE_THREAD;

		/// If `CLONE_UNTRACED` is specified, then a tracing process cannot force `CLONE_PTRACE` on this child process.
		///
		/// Since Linux 2.5.46.
		const Untraced = CLONE_UNTRACED;

		/// If CLONE_VFORK is set, the execution of the calling process is suspended until the child releases its virtual memory resources via a call to `execve()` or `_exit()` (as with `vfork()`).
		///
		/// If `CLONE_VFORK` is not set, then both the calling process and the child are schedulable after the call, and an application should not rely on execution occurring in any particular order.
		///
		/// Since Linux 2.2.
		const VFork = CLONE_VFORK;

		/// If `CLONE_VM is set`, the calling process and the child process run in the same memory space.
		/// In particular, memory writes performed by the calling process or by the child process are also visible in the other process.
		/// Moreover, any memory mapping or unmapping performed with `mmap()` or `munmap()` by the child or calling process also affects the other process.
		///
		/// If `CLONE_VM` is not set, the child process runs in a separate copy of the memory space of the calling process at the time of `clone()`.
		/// Memory writes or file mappings or file unmappings performed by one of the processes do not affect the other, as with `fork()`.
		const VirtualMachine = CLONE_VM;
	}
}

