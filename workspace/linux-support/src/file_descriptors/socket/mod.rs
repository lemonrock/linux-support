// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


use super::*;
use super::pipes_and_fifos::SpliceRecipient;
use super::pipes_and_fifos::SpliceSender;
use super::sendfile::SendFile;
use self::syscall::*;
use libc::AF_IB;
use libc::AF_INET;
use libc::AF_INET6;
use libc::AF_UNIX;
use libc::EADDRINUSE;
use libc::EADDRNOTAVAIL;
use libc::EALREADY;
use libc::EAFNOSUPPORT;
use libc::ECONNABORTED;
use libc::EINPROGRESS;
use libc::ENETUNREACH;
use libc::ENOPROTOOPT;
use libc::ENOSR;
use libc::ENOTDIR;
use libc::EPROTO;
use libc::EPROTONOSUPPORT;
use libc::EROFS;
use libc::ESOCKTNOSUPPORT;
use libc::ETIMEDOUT;
use libc::gid_t;
use libc::iovec;
use libc::IPPROTO_TCP;
use libc::IPPROTO_UDP;
use libc::sa_family_t; // Typically u16.
use libc::SOCK_DGRAM;
use libc::SOCK_STREAM;
use libc::send;
use libc::sendfile;
use libc::socklen_t; // Typically u32.
use libc::uid_t;
use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::cmp::max;
use std::cmp::Ordering;
use std::hash::Hash;
use std::hash::Hasher;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::net::SocketAddrV4;
use std::net::SocketAddrV6;
use std::ops::Deref;
use std::ops::DerefMut;
use std::slice::from_raw_parts;
use std::slice::from_raw_parts_mut;
use std::fs::DirBuilder;
use std::fs::remove_dir;
use std::fs::remove_file;
use std::fs::set_permissions;
use std::os::unix::fs::DirBuilderExt;
use std::os::unix::fs::PermissionsExt;
use crate::process::{ProcessIdentifier, UserIdentifier, GroupIdentifier};


/// Contains essential data structures.
pub mod syscall;


include!("AcceptedConnection.rs");
include!("AcceptedConnectionEnum.rs");
include!("ConnectionFailedReason.rs");
include!("Credentials.rs");
include!("DatagramClientSocketFileDescriptor.rs");
include!("DatagramClientSocketFileDescriptorEnum.rs");
include!("DatagramClientSocketInternetProtocolVersion4FileDescriptor.rs");
include!("DatagramClientSocketInternetProtocolVersion6FileDescriptor.rs");
include!("DatagramClientSocketUnixDomainFileDescriptor.rs");
include!("DatagramServerListenerSocketFileDescriptor.rs");
include!("DatagramServerListenerSocketFileDescriptorEnum.rs");
include!("DatagramServerListenerSocketInternetProtocolVersion4FileDescriptor.rs");
include!("DatagramServerListenerSocketInternetProtocolVersion6FileDescriptor.rs");
include!("DatagramServerListenerSocketUnixDomainFileDescriptor.rs");
include!("ErrorFlags.rs");
include!("FilePathInvalidReason.rs");
include!("InfinibandSocketAddress.rs");
include!("MessageHeadersIterator.rs");
include!("NewSocketClientError.rs");
include!("NewSocketServerListenerError.rs");
include!("ReceivedMessageHelper.rs");
include!("ReceivedMessages.rs");
include!("ReceiveFlags.rs");
include!("ReceiveFileDescriptorsError.rs");
include!("SendFlags.rs");
include!("SocketAcceptError.rs");
include!("SocketAddress.rs");
include!("SocketBindError.rs");
include!("SocketConnectError.rs");
include!("SocketData.rs");
include!("SocketFileDescriptor.rs");
include!("SocketListenError.rs");
include!("StreamingServerListenerSocketFileDescriptor.rs");
include!("StreamingServerListenerSocketFileDescriptorEnum.rs");
include!("StreamingServerListenerSocketInternetProtocolVersion4FileDescriptor.rs");
include!("StreamingServerListenerSocketInternetProtocolVersion6FileDescriptor.rs");
include!("StreamingServerListenerSocketUnixDomainFileDescriptor.rs");
include!("StreamingSocketFileDescriptor.rs");
include!("StreamingSocketFileDescriptorEnum.rs");
include!("StreamingSocketInternetProtocolVersion4FileDescriptor.rs");
include!("StreamingSocketInternetProtocolVersion6FileDescriptor.rs");
include!("StreamingSocketUnixDomainFileDescriptor.rs");
include!("UnixSocketAddress.rs");

