// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


use self::c::*;
use super::*;
use super::directory::AccessPermissions;
use crate::cpu::HyperThread;
use super::file::SendFile;
use crate::paths::PathExt;
use super::pipes_and_fifos::SpliceRecipient;
use super::pipes_and_fifos::SpliceSender;
use crate::process::*;
use crate::user_and_groups::{UserIdentifier, assert_effective_user_id_is_root};
use crate::user_and_groups::GroupIdentifier;
use crate::memory::NumberOfPages;


/// Contains essential data structures.
pub mod c;


include!("AcceptedConnection.rs");
include!("AcceptedConnectionEnum.rs");
include!("BackLog.rs");
include!("CongestionControlAlgorithm.rs");
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
include!("EnumSocketAddress.rs");
include!("ErrorFlags.rs");
include!("FilePathInvalidReason.rs");
include!("FinishTimeoutSeconds.rs");
include!("from_raw_socket_file_descriptor.rs");
include!("IdlesBeforeKeepAliveSeconds.rs");
include!("KeepAliveIntervalSeconds.rs");
include!("MaximumKeepAliveProbes.rs");
include!("MaximumSynAckRetransmits.rs");
include!("MaximumSynRetransmits.rs");
include!("MemoryPressure.rs");
include!("MessageHeadersIterator.rs");
include!("NewSocketClientError.rs");
include!("NewSocketServerListenerError.rs");
include!("NonServerSocket.rs");
include!("NotSentLowWaterInBytes.rs");
include!("NumberOfSockets.rs");
include!("PendingAcceptConnection.rs");
include!("QueuingDisciplineAlgorithm.rs");
include!("ReceiveBufferSizeInBytes.rs");
include!("ReceivedMessageHelper.rs");
include!("ReceivedMessages.rs");
include!("ReceiveFlags.rs");
include!("ReceiveFileDescriptorsError.rs");
include!("ReceiveMessage.rs");
include!("ReceivePacketSteeringFlowsPerHyperThread.rs");
include!("ReordingThreshold.rs");
include!("Retries1.rs");
include!("Retries2.rs");
include!("RetriesOrphan.rs");
include!("SendBufferSizeInBytes.rs");
include!("SendFlags.rs");
include!("SendMessage.rs");
include!("set_auto_corking.rs");
include!("SocketAccept.rs");
include!("SocketAcceptError.rs");
include!("SocketAddress.rs");
include!("SocketBindError.rs");
include!("SocketConnect.rs");
include!("SocketConnectError.rs");
include!("SocketData.rs");
include!("SocketFileDescriptor.rs");
include!("SocketLingerSeconds.rs");
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
include!("TransmissionControlProtocolServerListenerConfiguration.rs");
include!("UnixSocketAddress.rs");
