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


use super::*;
use crate::extended_attributes::*;
use crate::file_systems::FileSystemMetadata;
use crate::inode::InodeFlags;
use crate::inode::InodeGenerationNumber;
use crate::inode::c::FS_IOC_GETFLAGS;
use crate::inode::c::FS_IOC_SETFLAGS;
use crate::inode::c::FS_IOC_GETVERSION;
use crate::inode::c::FS_IOC_SETVERSION;
use crate::paths::ProcPath;
use crate::process::ProcessIdentifierChoice;
use crate::vectors::VectoredRead;
use crate::vectors::VectoredWrite;
use crate::terminal::TerminalSettingsError;


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


/// Netlink.
pub mod netlink;


/// Path file descriptor (created using `O_PATH`).
pub mod path;


/// POSIX message queue file descriptors.
pub mod posix_message_queues;


/// Anonymous and named, connected unidirectional pipes_and_fifos (act like TCP connected sockets).
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


/// Seccomp user notifications.
pub mod seccomp_user_notification;


/// Standard (eg standard in) file descriptors.
pub mod standard;


/// Terminal (serial port and modem) file descriptors.
///
/// Create an instance of `TerminalFileDescriptor` to get started with a terminal or serial port.
pub mod terminal;


/// Timer file descriptors.
pub mod timerfd;


include!("close_all_open_file_descriptors_apart_from_standard.rs");
include!("CreationError.rs");
include!("FileDescriptor.rs");
include!("MemoryMappableFileDescriptor.rs");
include!("InvalidPathReason.rs");
include!("OnDiskFileDescriptor.rs");
include!("PipeLikeFileDescriptor.rs");
include!("RawFdExt.rs");
include!("SeekableFileDescriptor.rs");
include!("SpecialFileOpenError.rs");
include!("StructReadError.rs");
include!("StructWriteError.rs");
include!("TimestampUpdate.rs");
