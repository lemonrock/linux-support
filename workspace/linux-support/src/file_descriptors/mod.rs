// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


//! #file-descriptors
//!
//! file-descriptors is a Rust crate wrapping the various kinds of file descriptors with safe abstractions, including IPv4 / IPv6 sockets, Unix domain sockets, epoll, timerfd, signalfd, eventfd, POSIX message queues, pipes, FIFOs, terminals (and serial ports), character devices, inotify, fanotify and Files.
//!
//! There is a particularly extensive and safe wrapper for signals and terminals.
//!
//!
//! ## Supported File Descriptors
//!
//! * character devices.
//! * directories (with `*at()` atomic operations on files).
//! * epoll.
//! * eventfd.
//! * fanotify.
//! * inotify.
//! * POSIX message queues (<(https://linux.die.net/man/7/mq_overview>).
//! * pipes and FIFOs (anonymous and named FIFOs), including support for splice, vmsplice and tee.
//! * signalfd.
//! * sockets (TCP, UDP and the equivalent over Unix Domain Sockets; sendfile supported).
//! * terminals (serial ports and modems).
//! * timerfd.
//!
//! Additionally, extensions (`SendFile`, `SpliceRecipient` and `SpliceSender`) are implemented for Rust's `File`.
//!
//!
//! ## Unix Domain Sockets
//!
//!
//! ### When using file paths
//!
//! * Every effort is made to create the socket file path cleanly;
//! * To make sure all parent folders exist;
//! * To make sure parent folder permissions are correctly set;
//! * To remove any stale files;
//! * To remove socket file paths on drop (close).
//!
//! The above features may not work correctly after the use of `seccomp` to lock down system calls (particularly the attempt to delete a socket file path on close).
//!
//!
//! ## Pipes
//!
//! * The use of `splice()`, `vmsplice()` and `tee()` are supported for all file descriptors where possible (including Rust's `std::fs::File`).
//! * To be able to use epoll with standard in (`stdin`), use `pipes_and_fifos::ReceivePipeFileDescriptor::standard_in()`.
//! * To be able to use epoll with standard out (`stdout`), use `pipes_and_fifos::SendPipeFileDescriptor::standard_out()`.
//! * To be able to use epoll with standard error (`stderr`), use `pipes_and_fifos::SendPipeFileDescriptor::standard_error()`.
//!
//!
//! ## Unsupported for now
//!
//! * Linux zero copy send (`MSG_ZEROCOPY`) and receive (`SO_ZEROCOPY`), mostly because they have a horrible, hacky API.
//! * `SO_BUSY_POLL`.
//! * Unix Domain Sockets using `autobind`; setting of the `SO_PASSCRED` socket option.
//! * Receiving credentials over Unix Domain Sockets using `recvmsg()`.
//! * infiniband sockets.
//! * canbus (SocketCAN sockets and can4linux <http://can-wiki.info/can4linux/man/can4linux_8h_source.html> character device drivers).


use crate::extended_attributes::*;
use crate::file_systems::FileSystemMetadata;
use crate::inode::InodeFlags;
use crate::inode::c::FS_IOC_GETFLAGS;
use crate::inode::c::FS_IOC_SETFLAGS;
use crate::terminal::TerminalSettingsError;
#[allow(deprecated)] use std::mem::uninitialized;
use arrayvec::Array;
use arrayvec::ArrayVec;
use bitflags::bitflags;
use errno::Errno;
use errno::errno;
use libc::AF_IB;
use libc::AF_INET6;
use libc::AF_INET;
use libc::AF_UNIX;
use libc::AT_EACCESS;
use libc::AT_EMPTY_PATH;
use libc::AT_FDCWD;
use libc::AT_NO_AUTOMOUNT;
use libc::AT_REMOVEDIR;
use libc::AT_SYMLINK_FOLLOW;
use libc::AT_SYMLINK_NOFOLLOW;
use libc::close;
use libc::c_char;
use libc::c_int;
use libc::c_long;
use libc::c_short;
use libc::c_uchar;
use libc::c_uint;
use libc::c_ulong;
use libc::c_ulonglong;
use libc::c_ushort;
use libc::c_void;
use libc::dev_t;
use libc::EACCES;
use libc::EADDRINUSE;
use libc::EADDRNOTAVAIL;
use libc::EAFNOSUPPORT;
use libc::EAGAIN;
use libc::EALREADY;
use libc::EBADF;
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
use libc::ENOTCONN;
use libc::ENOTDIR;
use libc::ENOTSOCK;
use libc::ENXIO;
use libc::EOPNOTSUPP;
use libc::EOVERFLOW;
use libc::EPERM;
use libc::EPIPE;
use libc::EPROTO;
use libc::EPROTONOSUPPORT;
use libc::EROFS;
use libc::ESOCKTNOSUPPORT;
use libc::ESPIPE;
use libc::ESRCH;
use libc::ETIMEDOUT;
use libc::ETXTBSY;
use libc::EWOULDBLOCK;
use libc::EXDEV;
use libc::F_ADD_SEALS;
use libc::F_DUPFD_CLOEXEC;
use libc::F_GET_SEALS;
use libc::F_GETFD;
use libc::F_GETFL;
use libc::F_GETLEASE;
use libc::F_GETLK;
use libc::F_GETPIPE_SZ;
use libc::F_OFD_GETLK;
use libc::F_OFD_SETLK;
use libc::F_OFD_SETLKW;
use libc::F_OK;
use libc::F_RDLCK;
use libc::F_SETFD;
use libc::F_SETFL;
use libc::F_SEAL_FUTURE_WRITE;
use libc::F_SEAL_GROW;
use libc::F_SEAL_SEAL;
use libc::F_SEAL_SHRINK;
use libc::F_SEAL_WRITE;
use libc::F_SETLEASE;
use libc::F_SETLK;
use libc::F_SETLKW;
use libc::F_SETPIPE_SZ;
use libc::F_WRLCK;
use libc::F_UNLCK;
use libc::faccessat;
use libc::fallocate;
use libc::fchdir;
use libc::fchmodat;
use libc::fchownat;
use libc::fcntl;
use libc::FD_CLOEXEC;
use libc::fdatasync;
use libc::fgetxattr;
use libc::flistxattr;
use libc::fremovexattr;
use libc::fsetxattr;
use libc::fstatat;
use libc::fstatvfs;
use libc::fsync;
use libc::getpid;
use libc::gid_t;
use libc::in_addr_t;
use libc::in_port_t;
use libc::ino_t;
use libc::ioctl;
use libc::iovec;
use libc::IPPROTO_TCP;
use libc::IPPROTO_UDP;
use libc::linkat;
use libc::LOCK_EX;
use libc::LOCK_NB;
use libc::LOCK_SH;
use libc::LOCK_UN;
use libc::loff_t;
use libc::lseek;
use libc::mode_t;
use libc::mkdirat;
use libc::mknodat;
use libc::nlink_t;
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
use libc::off_t;
use libc::open;
use libc::openat;
use libc::pid_t;
use libc::POSIX_FADV_DONTNEED;
use libc::POSIX_FADV_NOREUSE;
use libc::POSIX_FADV_NORMAL;
use libc::POSIX_FADV_RANDOM;
use libc::POSIX_FADV_SEQUENTIAL;
use libc::POSIX_FADV_WILLNEED;
use libc::posix_fadvise;
use libc::pread;
use libc::pwrite;
use libc::R_OK;
use libc::read;
use libc::readahead;
use libc::RENAME_EXCHANGE;
use libc::RENAME_NOREPLACE;
use libc::RENAME_WHITEOUT;
use libc::S_IFBLK;
use libc::S_IFCHR;
use libc::S_IFDIR;
use libc::S_IFMT;
use libc::S_IFIFO;
use libc::S_IFLNK;
use libc::S_IFREG;
use libc::S_IFSOCK;
use libc::S_IRUSR;
use libc::S_IRWXU;
use libc::S_IRWXG;
use libc::S_IRWXO;
use libc::S_IWUSR;
use libc::sa_family_t; // Typically u16.
use libc::SEEK_CUR;
use libc::SEEK_END;
use libc::SEEK_SET;
use libc::send;
use libc::sendfile;
use libc::siginfo_t;
use libc::sigset_t;
use libc::size_t;
use libc::socklen_t; // Typically u32.
use libc::SOCK_DGRAM;
use libc::SOCK_STREAM;
use libc::ssize_t;
use libc::stat;
use libc::statvfs;
use libc::symlinkat;
use libc::sync;
use libc::SYNC_FILE_RANGE_WAIT_AFTER;
use libc::SYNC_FILE_RANGE_WAIT_BEFORE;
use libc::SYNC_FILE_RANGE_WRITE;
use libc::sync_file_range;
use libc::time_t;
use libc::timespec;
use libc::uid_t;
use libc::unlink;
use libc::unlinkat;
use libc::UTIME_NOW;
use libc::UTIME_OMIT;
use libc::utimensat;
use libc::W_OK;
use libc::write;
use libc::XATTR_CREATE;
use libc::XATTR_REPLACE;
use libc::X_OK;
use likely::*;
use memchr::memchr;
use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::cmp::max;
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::error;
use std::ffi::CStr;
use std::ffi::CString;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;
use std::fs::DirBuilder;
use std::fs::File;
use std::fs::remove_dir;
use std::fs::remove_file;
use std::fs::set_permissions;
use std::hash::Hash;
use std::hash::Hasher;
use std::io::BufRead;
use std::io::BufReader;
use std::io::ErrorKind;
use std::io::Initializer;
use std::io::IoSlice;
use std::io::IoSliceMut;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::io;
use std::marker::PhantomData;
use std::mem::forget;
use std::mem::size_of;
use std::mem::transmute;
use std::mem::transmute_copy;
use std::mem::zeroed;
use std::net::SocketAddr;
use std::net::SocketAddrV4;
use std::net::SocketAddrV6;
use std::num::NonZeroI32;
use std::num::NonZeroU32;
use std::num::NonZeroU64;
use std::ops::Deref;
use std::ops::DerefMut;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::DirBuilderExt;
use std::os::unix::fs::FileExt;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::FromRawFd;
use std::os::unix::io::IntoRawFd;
use std::os::unix::io::RawFd;
use std::path::Path;
use std::ptr::NonNull;
use std::ptr::null;
use std::ptr::null_mut;
use std::rc::Rc;
use std::rc::Weak;
use std::slice::from_raw_parts;
use std::slice::from_raw_parts_mut;
use std::str::from_utf8;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;


/// Block device file descriptors.
pub mod block_device;


/// Character device file descriptors.
pub mod character_device;


/// Directory file descriptors.
pub mod directory;


/// EPoll file descriptors.
pub mod epoll;


/// Event file descriptors.
pub mod eventfd;


/// fanotify file descriptors.
pub mod fanotify;


/// Additional support for `std::fs::File` including:-
///
/// * `sendfile()` with Rust's File and our sockets.
/// * `SpliceRecipient`
/// * `SpliceSender`
/// * `memfd` anonymous memory backed files.
pub mod file;


/// File descriptor information obtained from `/proc`.
pub mod file_descriptor_information;


/// inotify file descriptor.
pub mod inotify;


/// Memory file descriptor.
pub mod memfd;


/// Path file descriptor (created using `O_PATH`).
pub mod path;


/// POSIX message queue file descriptors.
pub mod posix_message_queues;


/// Anonymous and named, connected unidirectional pipes_and_fifos (act like TCP connected sockets).
///
/// Since Linux 2.6.35, the default pipe capacity is 16 pages (which are 4096 bytes on x86_64), but the capacity can be queried and set using the `fcntl()` `F_GETPIPE_SZ` and `F_SETPIPE_SZ` operations.
///
/// The unread bytes in a pipe can be obtained using the `fcntl()` operation `FIONREAD`.
///
/// The maximum capacity that can be set for a non-privileged process (one without the `CAP_SYS_RESOURCE` capability) is specified in the file `/proc/sys/fs/pipe-max-size`; it defaults to 1Mb.
///
/// Writes of less than `PIPE_BUF` bytes are atomic; `PIPE_BUF` is 4096 bytes on Linux.
///
/// Atomic writes are significant when there are many writers sharing one named pipe (FIFO).
pub mod pipes_and_fifos;


/// Process Identifier file descriptors, `pidfd`.
///
/// Since Linux 5.1.
pub mod process_identifier;


/// Signal file descriptors.
pub mod signalfd;


/// Socket file descriptors.
pub mod socket;


/// Terminal (serial port and modem) file descriptors.
///
/// Create an instance of `TerminalFileDescriptor` to get started with a terminal or serial port.
pub mod terminal;


/// Timer file descriptors.
pub mod timerfd;


include!("CreationError.rs");
include!("FileDescriptor.rs");
include!("InvalidPathReason.rs");
include!("OnDiskFileDescriptor.rs");
include!("path_bytes_without_trailing_nul.rs");
include!("RawFdExt.rs");
include!("SpecialFileOpenError.rs");
include!("StructReadError.rs");
include!("StructWriteError.rs");
include!("TimestampUpdate.rs");
