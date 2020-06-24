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
include!("Blocking.rs");
include!("BlockingDuration.rs");
include!("BusyPollMicroseconds.rs");
include!("CongestionControlAlgorithm.rs");
include!("ConnectionFailedReason.rs");
include!("ControlMessageBufferSizeInBytes.rs");
include!("Credentials.rs");
include!("DatagramClientSocketFileDescriptor.rs");
include!("DatagramServerListenerSocketFileDescriptor.rs");
include!("ErrorFlags.rs");
include!("FilePathInvalidReason.rs");
include!("FinishTimeoutSeconds.rs");
include!("IdlesBeforeKeepAliveSeconds.rs");
include!("InternetProtocolSocketSettings.rs");
include!("KeepAliveIntervalSeconds.rs");
include!("MaximumKeepAliveProbes.rs");
include!("MaximumSynAckRetransmits.rs");
include!("MaximumSynRetransmits.rs");
include!("MaximumUnixDomainSocketDatagramQueueLength.rs");
include!("MemoryPressure.rs");
include!("MessageHeadersIterator.rs");
include!("NewSocketClientError.rs");
include!("NewSocketServerListenerError.rs");
include!("NonServerSocket.rs");
include!("NotSentLowWaterInBytes.rs");
include!("NumberOfSockets.rs");
include!("PendingAcceptConnection.rs");
include!("QueuingDisciplineAlgorithm.rs");
include!("QueuingDisciplineSendPriority.rs");
include!("ReceiveBufferSizeInBytes.rs");
include!("ReceiveLowWaterMarkInBytes.rs");
include!("ReceivedMessageHelper.rs");
include!("ReceivedMessages.rs");
include!("ReceiveBufferSizeSocketOption.rs");
include!("ReceiveFlags.rs");
include!("ReceiveFileDescriptorsError.rs");
include!("ReceiveMessage.rs");
include!("ReceivePacketSteeringFlowsPerHyperThread.rs");
include!("ReordingThreshold.rs");
include!("Retries1.rs");
include!("Retries2.rs");
include!("RetriesOrphan.rs");
include!("SendBufferSizeInBytes.rs");
include!("SendBufferSizeSocketOption.rs");
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
include!("SocketDataWithLength.rs");
include!("SocketFileDescriptor.rs");
include!("SocketLingerSeconds.rs");
include!("SocketListenError.rs");
include!("StreamingServerListenerSocketFileDescriptor.rs");
include!("StreamingSocketFileDescriptor.rs");
include!("TransmissionControlProtocolSocketSettings.rs");
include!("UnixSocketAddress.rs");
