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
//! * `SO_BUSY_POLL` and `SO_INCOMING_CPU`.
//! * Unix Domain Sockets using `autobind`; setting of the `SO_PASSCRED` socket option.
//! * Receiving credentials over Unix Domain Sockets using `recvmsg()`.
//! * `mkfifo()`.
//! * `mknod()`.
//! * `copy_file_range()`.
//! * infiniband sockets.
//! * canbus (SocketCAN sockets and can4linux <http://can-wiki.info/can4linux/man/can4linux_8h_source.html> character device drivers).


use libc::close;
use std::os::unix::io::RawFd;
include!("RawFdExt.rs");


use self::terminal::TerminalSettingsError;
use arrayvec::Array;
use arrayvec::ArrayVec;
use bitflags::bitflags;
use errno::errno;
use errno::Errno;
use libc::c_char;
use libc::c_int;
use libc::c_void;
use libc::EACCES;
use libc::EAGAIN;
use libc::EBADF;
use libc::ECANCELED;
use libc::ECONNREFUSED;
use libc::ECONNRESET;
use libc::EDESTADDRREQ;
use libc::EDQUOT;
use libc::EEXIST;
use libc::EFAULT;
use libc::EFBIG;
use libc::EISCONN;
use libc::EINTR;
use libc::EINVAL;
use libc::EIO;
use libc::EISDIR;
use libc::ELOOP;
use libc::EMFILE;
use libc::EMSGSIZE;
use libc::ENFILE;
use libc::ENODEV;
use libc::ENOBUFS;
use libc::ENOENT;
use libc::ENOMEM;
use libc::ENOSPC;
use libc::ENOSYS;
use libc::ENOTCONN;
use libc::ENOTDIR;
use libc::ENOTSOCK;
use libc::EOPNOTSUPP;
use libc::EPERM;
use libc::EPIPE;
use libc::F_GETFL;
use libc::F_SETFL;
use libc::fcntl;
use libc::getpid;
use libc::O_CLOEXEC;
use libc::O_NONBLOCK;
use libc::O_RDONLY;
use libc::O_RDWR;
use libc::O_WRONLY;
use libc::iovec;
use libc::pid_t;
use libc::read;
use libc::readv;
use libc::sigset_t;
use libc::size_t;
use libc::write;
use libc::writev;
use likely::*;
use memchr::memchr;
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::error;
use std::ffi::CStr;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fs::File;
use std::hash::Hash;
use std::hash::Hasher;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::ErrorKind;
use std::io::Initializer;
use std::io::Read;
use std::io::Write;
use std::mem::size_of;
use std::mem::transmute;
#[allow(deprecated)] use std::mem::uninitialized;
use std::mem::zeroed;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::FromRawFd;
use std::os::unix::io::IntoRawFd;
use std::path::Path;
use std::ptr::null;
use std::ptr::null_mut;
use std::str::from_utf8;
use strum_macros::EnumIter;


/// Character device file descriptors.
pub mod character_device;


/// EPoll file descriptors.
pub mod epoll;


/// Event file descriptors.
pub mod eventfd;


/// inotify file descriptors.
pub mod inotify;


/// POSIX message queue file descriptors.
pub mod posix_message_queues;


/// Anonymous and named, connected unidirectional pipes_and_fifos (act like TCP connected sockets).
///
/// Since Linux 2.6.35, the default pipe capacity is 16 pages (which are 4096 bytes on x86-64), but the capacity can be queried and set using the `fcntl()` `F_GETPIPE_SZ` and `F_SETPIPE_SZ` operations.
///
/// The unread bytes in a pipe can be obtained using the `fcntl()` operation `FIONREAD`.
///
/// The maximum capacity that can be set for a non-privileged process (one without the `CAP_SYS_RESOURCE` capability) is specified in the file `/proc/sys/fs/pipe-max-size`; it defaults to 1Mb.
///
/// Writes of less than `PIPE_BUF` bytes are atomic; `PIPE_BUF` is 4096 bytes on Linux.
///
/// Atomic writes are significant when there are many writers sharing one named pipe (FIFO).
pub mod pipes_and_fifos;


/// fanotify file descriptors.
pub mod fanotify;


/// File descriptor information obtained from `/proc`.
pub mod file_descriptor_information;


/// Additional support for using `sendfile()` with Rust's File and our sockets.
pub mod sendfile;


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


include!("AsRawFdExt.rs");
include!("CreationError.rs");
include!("InvalidPathReason.rs");
include!("path_bytes_without_trailing_nul.rs");
include!("SpecialFileOpenError.rs");
include!("StructReadError.rs");
include!("StructWriteError.rs");
include!("VectoredRead.rs");
include!("VectoredWrite.rs");
