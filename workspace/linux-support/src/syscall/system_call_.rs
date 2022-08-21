// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[inline(always)]
pub(crate) fn system_call_bpf<const size: u32>(cmd: bpf_cmd, attr: &mut bpf_attr) -> SystemCallResult
{
	unsafe { SystemCallNumber::bpf.system_call_3(cmd as i32 as usize, attr as *mut bpf_attr as usize, size as usize) }
}

#[inline(always)]
pub(crate) fn system_call_delete_module(name: &CStr, flags: c_long) -> SystemCallResult
{
	unsafe { SystemCallNumber::delete_module.system_call_2(name.as_ptr() as usize, flags as usize) }
}

#[inline(always)]
pub(crate) fn system_call_finit_module(file_descriptor: RawFd, options: &CStr, flags: c_int) -> SystemCallResult
{
	unsafe { SystemCallNumber::finit_module.system_call_3(file_descriptor as usize, options.as_ptr() as usize, flags as usize) }
}

/// NOTE: The manpage defines the return type of `getdents64` as `ssize_t`, but musl libc defined the return type of `getdents` as `int` and aliases it to `getdents64`.
/// NOTE: musl libc redefines `SYS_getdents` as `SYS_getdents64`.
#[inline(always)]
pub(crate) fn system_call_getdents64(file_descriptor: RawFd, dirp: *mut c_void, count: size_t) -> SystemCallResult
{
	if cfg!(debug_assertions)
	{
		if dirp.is_null()
		{
			debug_assert_eq!(count, 0, "len must be 0 if dirp is null")
		}
	}
	
	let len = min(count, INT_MAX as usize);
	unsafe { SystemCallNumber::getdents64.system_call_3(file_descriptor as usize, dirp as usize, len) }
}

/// Note:-
///
/// * `nodemask` can be `null()`; in which case, `maxnode` is `0`.
/// * When `flags` is `0`, addr must be specified as `NULL`.
///
/// See <http://man7.org/linux/man-pages/man2/get_mempolicy.2.html>.
#[inline(always)]
pub(crate) fn system_call_get_mempolicy(mode: *mut i32, nodemask: *mut usize, maxnode: usize, addr: *const c_void, flags: GetMemoryPolicyFlags) -> SystemCallResult
{
	let flags = flags.bits();
	
	if cfg!(debug_assertions)
	{
		if nodemask.is_null()
		{
			debug_assert_eq!(maxnode, 0, "maxnode must be 0 if nodemask is null")
		}
		if flags == 0
		{
			debug_assert!(addr.is_null(), "when flags is 0 addr must be null")
		}
	}
	unsafe { SystemCallNumber::get_mempolicy.system_call_5(mode as usize, nodemask as usize, maxnode, addr as usize, flags as usize) }
}

#[inline(always)]
pub(crate) fn system_call_io_uring_enter(fd: RawFd, to_submit: c_uint, min_complete: c_uint, flags: EnterFlags, sig: *mut sigset_t) -> SystemCallResult
{
	let flags = flags.bits();
	unsafe { SystemCallNumber::io_uring_enter.system_call_6(fd as usize, to_submit as usize, min_complete as usize, flags as usize, sig as usize, (_NSIG / 8) as usize) }
}

/// `fd` is the file descriptor returned by a call to `io_uring_setup()`.
/// `args` is a list with `nr_args` elements.
/// The type of `args` depends on the value of `opcode`.
///
/// If `arg` is `NULL` then `nr_args` must be `0`.
#[inline(always)]
pub(crate) fn system_call_io_uring_register(fd: RawFd, opcode: RegisterOperation, arg: *mut c_void, nr_args: c_uint) -> SystemCallResult
{
	if cfg!(debug_assertions)
	{
		if arg.is_null()
		{
			debug_assert_eq!(nr_args, 0, "nr_args must be 0 if arg is null")
		}
	}
	
	let opcode = opcode as u32;
	unsafe { SystemCallNumber::io_uring_register.system_call_4(fd as usize, opcode as usize, arg as usize, nr_args as usize) }
}

#[inline(always)]
pub(crate) fn system_call_io_uring_setup(entries: c_uint, p: &mut io_uring_params) -> SystemCallResult
{
	unsafe { SystemCallNumber::io_uring_setup.system_call_2(entries as usize, p as *mut io_uring_params as usize) }
}

#[inline(always)]
pub(crate) fn system_call_ioprio_get(which: c_int, who: IOPRIO_WHO) -> SystemCallResult
{
	let who = who as i32;
	unsafe { SystemCallNumber::ioprio_get.system_call_2(which as usize, who as usize) }
}

#[inline(always)]
pub(crate) fn system_call_ioprio_set(which: c_int, who: IOPRIO_WHO, ioprio: u16) -> SystemCallResult
{
	let who = who as i32;
	let ioprio = ioprio as i32;
	unsafe { SystemCallNumber::ioprio_set.system_call_3(which as usize, who as usize, ioprio as usize) }
}

/// `dirfd` is either a `RawFd` or the special value `AT_FDCWD`.
#[inline(always)]
pub(crate) fn system_call_openat2(dirfd: &DirectoryFileDescriptor, pathname: *const c_char, how: &open_how, size: size_t) -> SystemCallResult
{
	unsafe { SystemCallNumber::openat2.system_call_4(dirfd.as_raw_fd() as usize, pathname as usize, how as *const open_how as usize, size as usize) }
}

/// `start` is a pointer to memory.
///
/// Note:-
///
/// * `nodemask` can be `null()`; in which case, `maxnode` is `0`.
///
/// See <http://man7.org/linux/man-pages/man2/mbind.2.html>.
#[inline(always)]
pub(crate) fn system_call_mbind(start: NonNull<c_void>, len: usize, mode: i32, nodemask: *const usize, maxnode: usize, flags: MemoryBindFlags) -> SystemCallResult
{
	if cfg!(debug_assertions)
	{
		if nodemask.is_null()
		{
			debug_assert_eq!(maxnode, 0, "maxnode must be 0 if nodemask is null")
		}
	}
	unsafe { SystemCallNumber::mbind.system_call_6(start.as_ptr() as usize, len as usize, mode as usize, nodemask as usize, maxnode, flags.bits() as usize) }
}

/// Note:-
///
/// * `count` is the size of the `pages` and `status` arrays,
/// * `count` is the size of the `nodes` array if it is *not* `NULL`.
/// * `pages` can be `null()`; in which case, `count` is `0`..
#[inline(always)]
pub(crate) fn system_call_move_pages(pid: pid_t, count: usize, pages: *const NonNull<c_void>, nodes: *const i32, status: NonNull<i32>, flags: MemoryBindFlags) -> SystemCallResult
{
	if cfg!(debug_assertions)
	{
		if pages.is_null()
		{
			debug_assert_eq!(count, 0, "count must be 0 if pages is null")
		}
	}
	unsafe { SystemCallNumber::move_pages.system_call_6(pid as usize, count, pages as usize, nodes as usize, status.as_ptr() as usize, flags.bits() as usize) }
}

/// Note:-
///
/// * `frommask` can be `null()`; in which case, `maxnode` is `0`.
/// * `tomask` can be `null()`; in which case, `maxnode` is `0`.
#[inline(always)]
pub(crate) fn system_call_migrate_pages(pid: i32, maxnode: usize, frommask: *const usize, tomask: *const usize) -> SystemCallResult
{
	if cfg!(debug_assertions)
	{
		if frommask.is_null()
		{
			debug_assert_eq!(maxnode, 0, "maxnode must be 0 if frommask is null")
		}
		if tomask.is_null()
		{
			debug_assert_eq!(maxnode, 0, "maxnode must be 0 if tomask is null")
		}
	}
	unsafe { SystemCallNumber::migrate_pages.system_call_4(pid as usize, maxnode, frommask as usize, tomask as usize) }
}

#[inline(always)]
pub(crate) fn system_call_perf_event_open(attr: &mut perf_event_attr, pid: pid_t, cpu: c_int, group_fd: RawFd, flags: c_ulong) -> SystemCallResult
{
	unsafe { SystemCallNumber::perf_event_open.system_call_5(attr as *mut perf_event_attr as usize, pid as usize, cpu as usize, group_fd as usize, flags as usize) }
}

#[inline(always)]
pub(crate) fn system_call_pidfd_getfd(process_file_descriptor: RawFd, other_process_file_descriptor: RawFd, flags: u32) -> SystemCallResult
{
	unsafe { SystemCallNumber::pidfd_getfd.system_call_3(process_file_descriptor as usize, other_process_file_descriptor as usize, flags as usize) }
}

#[inline(always)]
pub(crate) fn system_call_pidfd_open(pid: pid_t, flags: u32) -> SystemCallResult
{
	unsafe { SystemCallNumber::pidfd_open.system_call_2(pid as usize, flags as usize) }
}

#[inline(always)]
pub(crate) fn system_call_pidfd_send_signal(process_file_descriptor: RawFd, signo: c_int, information: *const siginfo_t, flags: u32) -> SystemCallResult
{
	unsafe { SystemCallNumber::pidfd_send_signal.system_call_4(process_file_descriptor as usize, signo as usize, information as usize, flags as usize) }
}

#[inline(always)]
pub(crate) fn system_call_renameat2(oldfd: c_int, old: &CStr, newfd: c_int, new: &CStr, flags: c_int) -> SystemCallResult
{
	unsafe { SystemCallNumber::renameat2.system_call_5(oldfd as usize, old.as_ptr() as usize, newfd as usize, new.as_ptr() as usize, flags as usize) }
}

/// On failure, the memory pointer to be `attr` may not have been initialized.
///
/// The `flags` argument is provided to allow for future extensions to the interface; in the current implementation it must be specified as `0`.
#[inline(always)]
pub(crate) fn system_call_sched_getattr(pid: pid_t, attr: NonNull<sched_attr>, size: u32, flags: c_uint) -> SystemCallResult
{
	debug_assert_eq!(flags, 0);
	
	unsafe { SystemCallNumber::sched_getattr.system_call_4(pid as usize, attr.as_ptr() as usize, size as usize, flags as usize) }
}

/// The `flags` argument is provided to allow for future extensions to the interface; in the current implementation it must be specified as `0`.
#[inline(always)]
pub(crate) fn system_call_sched_setattr(pid: pid_t, attr: &sched_attr, flags: c_uint) -> SystemCallResult
{
	debug_assert_eq!(flags, 0);
	
	unsafe { SystemCallNumber::sched_setattr.system_call_3(pid as usize, attr as *const sched_attr as usize, flags as usize) }
}

#[inline(always)]
pub(crate) fn system_call_seccomp(op: u32, flags: u32, args: *mut c_void) -> SystemCallResult
{
	unsafe { SystemCallNumber::seccomp.system_call_3(op as usize, flags as usize, args as usize) }
}

/// Note:-
///
/// * `nodemask` can be `null()`; in which case, `maxnode` is `0`.
///
/// See <http://man7.org/linux/man-pages/man2/set_mempolicy.2.html>.
#[inline(always)]
pub(crate) fn system_call_set_mempolicy(mode: i32, nodemask: *const usize, maxnode: usize) -> SystemCallResult
{
	if cfg!(debug_assertions)
	{
		if nodemask.is_null()
		{
			debug_assert_eq!(maxnode, 0, "maxnode must be 0 if nodemask is null")
		}
	}
	unsafe { SystemCallNumber::set_mempolicy.system_call_3(mode as usize, nodemask as usize, maxnode) }
}

/// On failure, the memory pointer to be `buffer` may not have been initialized.
#[inline(always)]
pub(crate) fn system_call_statx(dirfd: &DirectoryFileDescriptor, filename: &CStr, flags: c_uint, mask: c_uint, buffer: NonNull<statx>) -> SystemCallResult
{
	unsafe { SystemCallNumber::statx.system_call_5(dirfd.as_raw_fd() as usize, filename.as_ptr() as usize, flags as usize, mask as usize, buffer.as_ptr() as usize) }
}

/// `flags` is a combination of `O_CLOEXEC` (`UFFD_CLOEXEC`), `O_NONBLOCK` (`UFFD_NONBLOCK`) and `UFFD_USER_MODE_ONLY`.
/// `UFFD_SHARED_FCNTL_FLAGS`, used internally, is `UFFD_CLOEXEC | UFFD_NONBLOCK`.
#[inline(always)]
pub(crate) fn system_call_userfaultfd(flags: c_int) -> SystemCallResult
{
	unsafe { SystemCallNumber::userfaultfd.system_call_1(flags as usize) }
}
