// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![deny(missing_docs)]
#![deny(unconditional_recursion)]
#![deny(unreachable_patterns)]
#![feature(alloc_layout_extra)]
#![feature(allocator_api)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(const_fn_union)]
#![feature(core_intrinsics)]
#![feature(const_ptr_offset_from)]
#![feature(const_raw_ptr_deref)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(const_transmute)]
#![feature(const_type_id)]
#![feature(internal_uninit_const)]
#![feature(llvm_asm)]
#![feature(maybe_uninit_extra)]
#![feature(move_ref_pattern)]
#![feature(never_type)]
#![feature(ptr_offset_from)]
#![feature(read_initializer)]
#![feature(thread_local)]


//! #linux-support
//! 
//! This library provides wrappers and additional functionality to make use of a panoply of miscellaneous Linux (and, sometimes, POSIX) features.
//!
//! See <https://github.com/lemonrock/linux-support> for far more detail.


use static_assertions::assert_cfg;
assert_cfg!(target_os = "linux");
assert_cfg!(target_pointer_width = "64");


use crate::cpu::HyperThread;
use crate::memory::numa::NumaNode;
#[cfg(target_arch = "x86_64")] use raw_cpuid::*;
use arrayvec::Array;
use arrayvec::ArrayVec;
use arrayvec::CapacityError;
use bitflags::bitflags;
use chrono::DateTime;
use chrono::Datelike;
use chrono::SecondsFormat;
use chrono::Timelike;
use chrono::Utc;
use const_fn_assert::cfn_assert;
use const_fn_assert::cfn_assert_eq;
use const_fn_assert::cfn_assert_ne;
use const_fn_assert::cfn_debug_assert;
use either::Either;
use either::Either::Left;
use either::Either::Right;
use errno::Errno;
use errno::errno;
use errno::set_errno;
use indexmap::indexset;
use indexmap::set::IndexSet;
use lazy_static::lazy_static;
use libc::_IOLBF;
use libc::_SC_NPROCESSORS_CONF;
use libc::AF_IB;
use libc::AF_INET6;
use libc::AF_INET;
use libc::AF_NETLINK;
use libc::AF_UNIX;
use libc::AF_UNSPEC;
use libc::AT_EACCESS;
use libc::AT_EMPTY_PATH;
use libc::AT_FDCWD;
use libc::AT_NO_AUTOMOUNT;
use libc::AT_REMOVEDIR;
use libc::AT_SYMLINK_FOLLOW;
use libc::AT_SYMLINK_NOFOLLOW;
use libc::E2BIG;
use libc::EACCES;
use libc::EADDRINUSE;
use libc::EADDRNOTAVAIL;
use libc::EAFNOSUPPORT;
use libc::EAGAIN;
use libc::EALREADY;
use libc::EBADF;
use libc::EBADR;
use libc::EBUSY;
use libc::ECANCELED;
use libc::ECONNABORTED;
use libc::ECONNREFUSED;
use libc::ECONNRESET;
use libc::EDEADLK;
use libc::EDESTADDRREQ;
use libc::EDQUOT;
use libc::EEXIST;
use libc::EFAULT;
use libc::EFBIG;
use libc::EINPROGRESS;
use libc::EINTR;
use libc::EINVAL;
use libc::EIO;
use libc::EISCONN;
use libc::EISDIR;
use libc::ELOOP;
use libc::EMFILE;
use libc::EMSGSIZE;
use libc::ENAMETOOLONG;
use libc::ENETUNREACH;
use libc::ENFILE;
use libc::ENOBUFS;
use libc::ENODATA;
use libc::ENODEV;
use libc::ENOENT;
use libc::ENOLCK;
use libc::ENOMEM;
use libc::ENOPROTOOPT;
use libc::ENOSPC;
use libc::ENOSR;
use libc::ENOSYS;
use libc::ENOTBLK;
use libc::ENOTCONN;
use libc::ENOTDIR;
use libc::ENOTSOCK;
use libc::ENOTTY;
use libc::ENXIO;
use libc::EOPNOTSUPP;
use libc::EOVERFLOW;
use libc::EPERM;
use libc::EPIPE;
use libc::EPROTO;
use libc::EPROTONOSUPPORT;
use libc::EPROTOTYPE;
use libc::EROFS;
use libc::ESOCKTNOSUPPORT;
use libc::ESPIPE;
use libc::ESRCH;
use libc::ETIME;
use libc::ETIMEDOUT;
use libc::ETXTBSY;
use libc::EWOULDBLOCK;
use libc::EXDEV;
use libc::FD_CLOEXEC;
use libc::FIONREAD;
use libc::F_ADD_SEALS;
use libc::F_DUPFD_CLOEXEC;
use libc::F_GETFD;
use libc::F_GETFL;
use libc::F_GETLEASE;
use libc::F_GETLK;
use libc::F_GETPIPE_SZ;
use libc::F_GET_SEALS;
use libc::F_OFD_GETLK;
use libc::F_OFD_SETLK;
use libc::F_OFD_SETLKW;
use libc::F_OK;
use libc::F_RDLCK;
use libc::F_SEAL_FUTURE_WRITE;
use libc::F_SEAL_GROW;
use libc::F_SEAL_SEAL;
use libc::F_SEAL_SHRINK;
use libc::F_SEAL_WRITE;
use libc::F_SETFD;
use libc::F_SETFL;
use libc::F_SETLEASE;
use libc::F_SETLK;
use libc::F_SETLKW;
use libc::F_SETPIPE_SZ;
use libc::F_UNLCK;
use libc::F_WRLCK;
use libc::FILE;
use libc::IFA_ADDRESS;
use libc::IFA_ANYCAST;
use libc::IFA_BROADCAST;
use libc::IFA_CACHEINFO;
use libc::IFA_F_DADFAILED;
use libc::IFA_F_DEPRECATED;
use libc::IFA_F_HOMEADDRESS;
use libc::IFA_F_NODAD;
use libc::IFA_F_OPTIMISTIC;
use libc::IFA_F_PERMANENT;
use libc::IFA_F_SECONDARY;
use libc::IFA_F_TENTATIVE;
use libc::IFA_LABEL;
use libc::IFA_LOCAL;
use libc::IFA_MULTICAST;
use libc::IFA_UNSPEC;
use libc::IFLA_ADDRESS;
use libc::IFLA_BROADCAST;
use libc::IFLA_IFNAME;
use libc::IFLA_LINK;
use libc::IFLA_MTU;
use libc::IFLA_QDISC;
use libc::IFLA_STATS;
use libc::IFLA_UNSPEC;
use libc::IF_NAMESIZE;
use libc::IPPROTO_IP;
use libc::IPPROTO_TCP;
use libc::IPPROTO_UDP;
use libc::LC_ALL;
use libc::LOCK_EX;
use libc::LOCK_NB;
use libc::LOCK_SH;
use libc::LOCK_UN;
use libc::LOG_NDELAY;
use libc::LOG_PERROR;
use libc::LOG_PID;
use libc::MAP_ANONYMOUS;
use libc::MAP_FAILED;
use libc::MAP_FIXED;
use libc::MAP_HUGETLB;
use libc::MAP_NORESERVE;
use libc::MAP_POPULATE;
use libc::MAP_PRIVATE;
use libc::MCL_CURRENT;
use libc::MCL_FUTURE;
use libc::MNT_DETACH;
use libc::MNT_EXPIRE;
use libc::MNT_FORCE;
use libc::MREMAP_FIXED;
use libc::MREMAP_MAYMOVE;
use libc::MS_ASYNC;
use libc::MS_BIND;
use libc::MS_DIRSYNC;
use libc::MS_MANDLOCK;
use libc::MS_MOVE;
use libc::MS_NOATIME;
use libc::MS_NODEV;
use libc::MS_NODIRATIME;
use libc::MS_NOEXEC;
use libc::MS_NOSUID;
use libc::MS_INVALIDATE;
use libc::MS_REC;
use libc::MS_RELATIME;
use libc::MS_SILENT;
use libc::MS_STRICTATIME;
use libc::MS_SYNC;
use libc::MS_SYNCHRONOUS;
use libc::NETLINK_ROUTE;
use libc::NLMSG_DONE;
use libc::NLMSG_ERROR;
use libc::NLMSG_NOOP;
use libc::NLMSG_OVERRUN;
use libc::NLM_F_ACK;
use libc::NLM_F_APPEND;
use libc::NLM_F_ATOMIC;
use libc::NLM_F_CREATE;
use libc::NLM_F_DUMP_FILTERED;
use libc::NLM_F_DUMP_INTR;
use libc::NLM_F_ECHO;
use libc::NLM_F_EXCL;
use libc::NLM_F_MATCH;
use libc::NLM_F_MULTI;
use libc::NLM_F_REPLACE;
use libc::NLM_F_REQUEST;
use libc::NLM_F_ROOT;
use libc::O_APPEND;
use libc::O_CLOEXEC;
use libc::O_CREAT;
use libc::O_DIRECT;
use libc::O_DIRECTORY;
use libc::O_DSYNC;
use libc::O_EXCL;
use libc::O_LARGEFILE;
use libc::O_NOATIME;
use libc::O_NOCTTY;
use libc::O_NOFOLLOW;
use libc::O_NONBLOCK;
use libc::O_PATH;
use libc::O_RDONLY;
use libc::O_RDWR;
use libc::O_SYNC;
use libc::O_TMPFILE;
use libc::O_TRUNC;
use libc::O_WRONLY;
use libc::POLLERR;
use libc::POLLHUP;
use libc::POLLIN;
use libc::POLLNVAL;
use libc::POLLOUT;
use libc::POLLPRI;
use libc::POLLRDBAND;
use libc::POLLRDNORM;
use libc::POSIX_FADV_DONTNEED;
use libc::POSIX_FADV_NOREUSE;
use libc::POSIX_FADV_NORMAL;
use libc::POSIX_FADV_RANDOM;
use libc::POSIX_FADV_SEQUENTIAL;
use libc::POSIX_FADV_WILLNEED;
use libc::PR_CAP_AMBIENT;
use libc::PR_CAP_AMBIENT_CLEAR_ALL;
use libc::PR_CAP_AMBIENT_IS_SET;
use libc::PR_CAP_AMBIENT_LOWER;
use libc::PR_CAP_AMBIENT_RAISE;
use libc::PR_CAPBSET_DROP;
use libc::PR_CAPBSET_READ;
use libc::PR_SET_DUMPABLE;
use libc::PR_SET_NO_NEW_PRIVS;
use libc::PR_SET_SECUREBITS;
use libc::PR_SET_THP_DISABLE;
use libc::PRIO_PGRP;
use libc::PRIO_PROCESS;
use libc::PRIO_USER;
use libc::PROT_EXEC;
use libc::PROT_GROWSDOWN;
use libc::PROT_GROWSUP;
use libc::PROT_NONE;
use libc::PROT_READ;
use libc::PROT_WRITE;
use libc::PR_GET_NAME;
use libc::PR_SET_NAME;
use libc::RLIM_INFINITY;
use libc::RLIMIT_AS;
use libc::RLIMIT_CORE;
use libc::RLIMIT_CPU;
use libc::RLIMIT_DATA;
use libc::RLIMIT_FSIZE;
use libc::RLIMIT_MEMLOCK;
use libc::RLIMIT_MSGQUEUE;
use libc::RLIMIT_NICE;
use libc::RLIMIT_NOFILE;
use libc::RLIMIT_NPROC;
use libc::RLIMIT_RSS;
use libc::RLIMIT_RTPRIO;
use libc::RLIMIT_RTTIME;
use libc::RLIMIT_SIGPENDING;
use libc::RLIMIT_STACK;
use libc::RENAME_EXCHANGE;
use libc::RENAME_NOREPLACE;
use libc::RENAME_WHITEOUT;
use libc::RTM_DELACTION;
use libc::RTM_DELADDR;
use libc::RTM_DELADDRLABEL;
use libc::RTM_DELLINK;
use libc::RTM_DELMDB;
use libc::RTM_DELNEIGH;
use libc::RTM_DELNSID;
use libc::RTM_DELQDISC;
use libc::RTM_DELROUTE;
use libc::RTM_DELRULE;
use libc::RTM_DELTCLASS;
use libc::RTM_DELTFILTER;
use libc::RTM_GETACTION;
use libc::RTM_GETADDR;
use libc::RTM_GETADDRLABEL;
use libc::RTM_GETANYCAST;
use libc::RTM_GETDCB;
use libc::RTM_GETLINK;
use libc::RTM_GETMDB;
use libc::RTM_GETMULTICAST;
use libc::RTM_GETNEIGH;
use libc::RTM_GETNEIGHTBL;
use libc::RTM_GETNETCONF;
use libc::RTM_GETNSID;
use libc::RTM_GETQDISC;
use libc::RTM_GETROUTE;
use libc::RTM_GETRULE;
use libc::RTM_GETTCLASS;
use libc::RTM_GETTFILTER;
use libc::RTM_NEWACTION;
use libc::RTM_NEWADDR;
use libc::RTM_NEWADDRLABEL;
use libc::RTM_NEWLINK;
use libc::RTM_NEWMDB;
use libc::RTM_NEWNDUSEROPT;
use libc::RTM_NEWNEIGH;
use libc::RTM_NEWNEIGHTBL;
use libc::RTM_NEWNETCONF;
use libc::RTM_NEWNSID;
use libc::RTM_NEWPREFIX;
use libc::RTM_NEWQDISC;
use libc::RTM_NEWROUTE;
use libc::RTM_NEWRULE;
use libc::RTM_NEWTCLASS;
use libc::RTM_NEWTFILTER;
use libc::RTM_SETDCB;
use libc::RTM_SETLINK;
use libc::RTM_SETNEIGHTBL;
use libc::R_OK;
use libc::SEEK_CUR;
use libc::SEEK_END;
use libc::SEEK_SET;
use libc::SIG_BLOCK;
use libc::SIG_DFL;
use libc::SIG_SETMASK;
use libc::SOCK_DGRAM;
use libc::SOCK_RAW;
use libc::SOCK_STREAM;
use libc::ST_MANDLOCK;
use libc::ST_NOATIME;
use libc::ST_NODEV;
use libc::ST_NODIRATIME;
use libc::ST_NOEXEC;
use libc::ST_NOSUID;
use libc::ST_RDONLY;
use libc::ST_SYNCHRONOUS;
use libc::SYNC_FILE_RANGE_WAIT_AFTER;
use libc::SYNC_FILE_RANGE_WAIT_BEFORE;
use libc::SYNC_FILE_RANGE_WRITE;
use libc::S_IFBLK;
use libc::S_IFCHR;
use libc::S_IFDIR;
use libc::S_IFIFO;
use libc::S_IFLNK;
use libc::S_IFMT;
use libc::S_IFREG;
use libc::S_IFSOCK;
use libc::S_IRUSR;
use libc::S_IRWXG;
use libc::S_IRWXO;
use libc::S_IRWXU;
use libc::S_IWUSR;
use libc::UTIME_NOW;
use libc::UTIME_OMIT;
use libc::W_OK;
use libc::XATTR_CREATE;
use libc::XATTR_REPLACE;
use libc::X_OK;
use libc::c_char;
use libc::c_int;
use libc::c_long;
use libc::c_longlong;
use libc::c_short;
use libc::c_uchar;
use libc::c_uint;
use libc::c_ulong;
use libc::c_ulonglong;
use libc::c_ushort;
use libc::c_void;
use libc::clearenv;
use libc::clock_t;
use libc::close;
use libc::cpu_set_t;
use libc::dev_t;
use libc::dup2;
use libc::faccessat;
use libc::fallocate;
use libc::fchdir;
use libc::fchmodat;
use libc::fchownat;
use libc::fcntl;
use libc::fdatasync;
use libc::fgetxattr;
use libc::flistxattr;
use libc::fork;
use libc::fremovexattr;
use libc::fsetxattr;
use libc::fstatat;
use libc::fstatvfs;
use libc::fsync;
use libc::getegid;
use libc::geteuid;
use libc::getgid;
#[cfg(not(any(target_arch = "powerpc64", target_arch = "riscv64", target_arch = "sparc64", target_arch = "x86_64")))] use libc_extra::unix::unistd::getpagesize;
use libc::getpgid;
use libc::getpid;
use libc::getresgid;
use libc::getresuid;
use libc::getrlimit;
use libc::getsid;
use libc::getuid;
use libc::gid_t;
use libc::if_nametoindex;
use libc::in_addr_t;
use libc::in_port_t;
use libc::ino_t;
use libc::ioctl;
use libc::iovec;
use libc::linkat;
use libc::loff_t;
use libc::lseek;
use libc::madvise;
#[allow(deprecated)] use std::mem::uninitialized;
use libc::mkdirat;
use libc::mknodat;
use libc::mlockall;
use libc::mmap;
use libc::mode_t;
use libc::mount;
use libc::mprotect;
use libc::mremap;
use libc::msync;
use libc::munlock;
use libc::munlockall;
use libc::munmap;
use libc::nlink_t;
use libc::off_t;
use libc::open;
use libc::openat;
use libc::openlog;
use libc::pid_t;
use libc::posix_fadvise;
use libc::prctl;
use libc::pread;
use libc::process_vm_readv;
use libc::process_vm_writev;
use libc::pthread_self;
use libc::pthread_setaffinity_np;
use libc::pthread_sigmask;
use libc::pthread_t;
use libc::pwrite;
use libc::readahead;
use libc::recv;
use libc::rlim_t;
use libc::rlimit;
use libc::sa_family_t;
use libc::sched_getcpu;
use libc::sched_rr_get_interval;
use libc::sched_setaffinity;
use libc::send;
use libc::sendfile;
use libc::setdomainname;
use libc::setenv;
use libc::setfsgid;
use libc::setfsuid;
use libc::setgroups;
use libc::sethostname;
use libc::setlocale;
use libc::setlogmask;
use libc::setpriority;
use libc::setresgid;
use libc::setresuid;
use libc::setrlimit;
use libc::setsid;
use libc::setvbuf;
use libc::sigaddset;
use libc::sigdelset;
use libc::sigemptyset;
use libc::sigfillset;
use libc::siginfo_t;
use libc::sigtimedwait;
use libc::sigset_t;
use libc::size_t;
use libc::socklen_t;
use libc::ssize_t;
use libc::stat;
use libc::statvfs;
use libc::strnlen;
use libc::swapoff;
use libc::symlinkat;
use libc::sync;
use libc::sync_file_range;
use libc::sysconf;
use libc::sysinfo;
use libc::time_t;
use libc::timespec;
use libc::timeval;
use libc::uid_t;
use libc::umask;
use libc::umount2;
use libc::unlink;
use libc::unlinkat;
use libc::utimensat;
use libc_extra::android_linux::linux::capability;
use libc_extra::android_linux::linux::ethtool::*;
use libc_extra::android_linux::linux::securebits::SECBIT_KEEP_CAPS;
use libc_extra::android_linux::linux::securebits::SECBIT_KEEP_CAPS_LOCKED;
use libc_extra::android_linux::linux::securebits::SECBIT_NOROOT;
use libc_extra::android_linux::linux::securebits::SECBIT_NOROOT_LOCKED;
use libc_extra::android_linux::linux::securebits::SECBIT_NO_CAP_AMBIENT_RAISE;
use libc_extra::android_linux::linux::securebits::SECBIT_NO_CAP_AMBIENT_RAISE_LOCKED;
use libc_extra::android_linux::linux::securebits::SECBIT_NO_SETUID_FIXUP;
use libc_extra::android_linux::linux::securebits::SECBIT_NO_SETUID_FIXUP_LOCKED;
use libc_extra::android_linux::linux::sockios::SIOCETHTOOL;
use libc_extra::android_linux::mntent::endmntent;
use libc_extra::android_linux::mntent::getmntent;
use libc_extra::android_linux::mntent::mntent;
use libc_extra::android_linux::mntent::setmntent;
use libc_extra::android_linux::net::if_::ifreq;
use libc_extra::android_linux::stdio::cookie_io_functions_t;
use libc_extra::android_linux::stdio::fopencookie;
use libc_extra::linux::errno::program_invocation_short_name;
use libc_extra::unix::stdio;
use libc_extra::unix::string::strsignal;
use likely::likely;
use likely::unlikely;
use maplit::btreeset;
use maplit::hashset;
use memchr::Memchr;
use memchr::memchr2;
use memchr::memchr3;
use memchr::memchr;
use memchr::memchr_iter;
use memchr::memrchr;
use memoffset::offset_of;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde::de;
use serde::de::Unexpected;
use serde::de::Visitor;
use std::any::TypeId;
#[cfg(all(target_arch = "x86_64", target_feature = "popcnt"))] use std::arch::x86_64::_mm_popcnt_u64;
use std::array::TryFromSliceError;
use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::borrow::Cow;
use std::cell::Cell;
use std::cmp::Eq;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::cmp::PartialEq;
use std::cmp::PartialOrd;
use std::cmp::max;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::AsRef;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::env::JoinPathsError;
use std::env::join_paths;
use std::env::set_current_dir;
use std::env::set_var;
use std::env::var_os;
use std::error;
use std::error::Error;
use std::ffi::CStr;
use std::ffi::CString;
use std::ffi::FromBytesWithNulError;
use std::ffi::NulError;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::fmt;
use std::fmt::Arguments;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fs::DirBuilder;
use std::fs::File;
use std::fs::OpenOptions;
use std::fs::metadata;
use std::fs::remove_dir;
use std::fs::remove_file;
use std::fs::set_permissions;
use std::hash::Hash;
use std::hash::Hasher;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::ErrorKind;
use std::io::Initializer;
use std::io::IoSlice;
use std::io::IoSliceMut;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Stderr;
use std::io::Stdin;
use std::io::Stdout;
use std::io::Write;
use std::io::stderr;
use std::io::stdin;
use std::io::stdout;
use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::mem::align_of;
use std::mem::forget;
use std::mem::size_of;
use std::mem::transmute;
use std::mem::transmute_copy;
use std::mem::zeroed;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;
use std::net::SocketAddr;
use std::net::SocketAddrV4;
use std::net::SocketAddrV6;
use std::num::NonZeroI32;
use std::num::NonZeroU16;
use std::num::NonZeroU32;
use std::num::NonZeroU64;
use std::num::NonZeroU8;
use std::num::NonZeroUsize;
use std::num::ParseIntError;
use std::num::TryFromIntError;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::BitAnd;
use std::ops::BitOr;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::Mul;
use std::ops::Not;
use std::ops::Range;
use std::ops::RangeInclusive;
use std::ops::Shl;
use std::ops::Shr;
use std::ops::Sub;
use std::ops::SubAssign;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::ffi::OsStringExt;
use std::os::unix::fs::DirBuilderExt;
use std::os::unix::fs::FileExt;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::FromRawFd;
use std::os::unix::io::IntoRawFd;
use std::os::unix::io::RawFd;
use std::panic::AssertUnwindSafe;
use std::panic::RefUnwindSafe;
use std::panic::catch_unwind;
use std::panic::resume_unwind;
use std::panic::set_hook;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use std::process::exit;
use std::ptr::NonNull;
use std::ptr::null;
use std::ptr::null_mut;
use std::ptr::read;
use std::ptr::read_volatile;
use std::ptr::write;
use std::ptr::write_bytes;
use std::ptr::write_volatile;
use std::rc::Rc;
use std::rc::Weak;
use std::slice::from_raw_parts;
use std::slice::from_raw_parts_mut;
use std::str::Utf8Error;
use std::str::from_utf8;
use std::str::from_utf8_unchecked;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Acquire;
use std::sync::atomic::Ordering::Release;
use std::sync::atomic::spin_loop_hint;
use std::thread::Builder;
use std::thread::JoinHandle;
use std::thread::Thread;
use std::thread::ThreadId;
use std::thread::current;
use std::thread::panicking;
use std::thread::park;
use std::thread::sleep;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use streaming_iterator::StreamingIterator;
use strum::EnumMessage;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use strum_macros::EnumMessage;
use strum_macros::IntoStaticStr;
use swiss_army_knife::bit_set_aware;
use swiss_army_knife::ConstArrayVec;
use swiss_army_knife::LoadNonAtomically;
use swiss_army_knife::move_to_front_of_vec;
use swiss_army_knife::StaticInitializedOnce;
use swiss_army_knife::VariablySized;
use swiss_army_knife::bit_set::BitsInAByte;
use swiss_army_knife::bit_set::BitSet;
use swiss_army_knife::bit_set::BitSetAware;
use swiss_army_knife::bit_set::BitSetAwareTryFromU16Error;
use swiss_army_knife::bit_set::BitSetIterator;
use swiss_army_knife::bit_set::IntoBitMask;
use swiss_army_knife::bit_set::IntoList;
use swiss_army_knife::bit_set::ListParseError;
use swiss_army_knife::internet_protocol::InternetProtocolAddress;
use swiss_army_knife::internet_protocol::InternetProtocolAddressWithMask;
use swiss_army_knife::path::path_bytes_without_trailing_nul;
use swiss_army_knife::path::PathBufExt;
use swiss_army_knife::split::SplitBytes;
use swiss_army_knife::strings::c_string_pointer_to_path_buf;
use swiss_army_knife::strings::ConstCStr;
use swiss_army_knife::strings::CStringExt;
use swiss_army_knife::strings::format_escaped_ascii_string;
use swiss_army_knife::strings::FromBytes;
use swiss_army_knife::strings::LinuxStringEscapeSequence;
use swiss_army_knife::strings::NonNumericDigitCase;
use swiss_army_knife::strings::NulTerminatedCStringArray;
use swiss_army_knife::strings::OsStrExtMore;
use swiss_army_knife::strings::parse_ascii_nul_string_values;
use swiss_army_knife::strings::path_to_cstring;
use swiss_army_knife::strings::Radix;
use swiss_army_knife::strings::replace;
use swiss_army_knife::strings::without_suffix;
use swiss_army_knife::strings::into_line_feed_terminated_byte_string::IntegerIntoLineFeedTerminatedByteString;
use swiss_army_knife::strings::into_line_feed_terminated_byte_string::IntoLineFeedTerminatedByteString;
use swiss_army_knife::strings::into_line_feed_terminated_byte_string::UnpaddedDecimalInteger;
use swiss_army_knife::strings::into_line_feed_terminated_byte_string::ZeroPaddedLowerCaseHexadecimalInteger;
use swiss_army_knife::strings::parse_number::ParseNumber;
use swiss_army_knife::strings::parse_number::ParseNumberError;
use swiss_army_knife::strings::parse_number::ParseNumberOption;
use swiss_army_knife::strings::to_number::NumberAsBytes;
use terminate::ParsedPanic;
use terminate::ParsedPanicErrorLogger;
use terminate::SimpleTerminate;
use terminate::Terminate;


/// Vectored reads and writes.
#[macro_use]
pub mod vectors;


/// Berkeley Packet Filter (BPF) and Extended Berkeley Packet Filter (eBPF).
pub mod bpf;


/// Capabilities and privileges.
///
/// * Manage capability sets for security.
/// * Disable the 'dumpable' flag for security.
/// * Lock down a process to remove privileges.
pub mod capabilities_and_privileges;


/// Coredump settings.
pub mod coredump;


/// Cgroups (containers).
pub mod cgroups;


/// Configuration.
pub mod configuration;


/// CPU.
///
/// * Cpu features wrapper.
/// * A proper CPU count that takes into account NUMA nodes, hotplugs, etc.
/// * Hyper thread (SMT) insight, status, usage, etc.
/// 	* Turn off and on
/// 	* Mappings to NUMA nodes
/// 	* And lots more
pub mod cpu;


/// Block and character device abstractions.
pub mod devices;


/// Environment variables.
///
/// * Find the original environment of a process.
/// * Find the command line of a process.
/// * Create a clean environment for a process with just essential variables set (a security and reproducibility protection).
pub mod environment;


/// Extended file attributes.
pub mod extended_attributes;


pub mod file_descriptors;


/// File handles.
pub mod file_handles;


/// File systems.
pub mod file_systems;


/// Inode.
///
/// A wrapper type for Inodes.
pub mod inode;


/// Interrupt requests in `/proc`.
pub mod interrupt_request;


/// `ioprio` and scheduling.
///
/// Also known as `ionice`.
pub mod io_priority;


/// io_uring.
pub mod io_uring;


/// Basic (for security) access io I/O ports on mip64, powerpc64 and x86_64.
#[cfg(any(target_arch = "mips64", target_arch = "powerpc64", target_arch = "x86_64"))] pub mod ioports;


/// Linux kernel command line.
pub mod linux_kernel_command_line;


/// Also known as `KAIO`.
///
/// Support for functions such as `io_submit()` in `linuxaio.h`.
///
/// This is *NOT* POSIX AIO.
///
/// Very basic support.
pub mod linux_kernel_asynchronous_io;


/// Linux kernel modules.
pub mod linux_kernel_modules;


/// Linux kernel panic.
pub mod linux_kernel_panic;


/// Linux kernel version.
pub mod linux_kernel_version;


/// Logging.
///
/// Miscellany support for using syslog with a Rust process, including:-
///
/// * Redirecting standard out and standard error to syslog;
/// * Logging process terminating signals to syslog.
/// * Logging panics to syslog.
/// * Configuring syslog.
pub mod logging;


/// Memory.
///
/// * Detailed, comprehensive and insightful NUMA node level information.
/// * Proper, modern Linux support for huge pages and mapping huge pages into memory.
/// * Memory usage and insight.
/// * A Linux-specific wrapper for mmap and related functionality that makes it *much* harder to misconfigure.
/// * Wrapper types for virtual and physical addreses.
/// * Wrapper types for number of pages.
/// * Efficient enums for page size and huge page sizes.
/// * Insight into memory maps
/// 	* For finding physical addresses from virtual memory addresses
pub mod memory;


/// Mounts.
pub mod mounts;


/// Namespaces.
pub mod namespaces;


/// Very basic `poll` support.
pub mod poll;


/// Nice.
pub mod scheduling;


/// Paths.
pub mod paths;


/// Linux personality.
///
/// A mostly broken and discarded concept, but we should check we're running as a standard Linux process.
pub mod personality;


/// PCI Express (PCIe).
pub mod pci_express;


/// Process.
pub mod process;


/// Resource limits.
pub mod resource_limits;


/// Seccomp.
pub mod seccomp;


/// Signals.
pub mod signals;


/// Swap.
pub mod swap;


/// Support for raw syscalls.
pub mod syscall;


/// Support for terminals.
pub mod terminal;


/// Support for time and clocks.
pub mod time;


/// Support for threads.
pub mod thread;


/// User and groups.
pub mod user_and_groups;


include!("current_numa_node_and_hyper_thread.rs");
include!("ENOTSUPP.rs");
