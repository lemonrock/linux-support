// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use self::c::*;
use self::descriptors::*;
use self::ring_queues::*;
use super::*;
use crate::bpf::c::bpf_func_id;
use crate::bpf::c::bpf_prog_type;
use crate::bpf::c::ValidateAttachModeError;
use crate::bpf::extended::identifiers::*;
use crate::bpf::extended::instructions::*;
use crate::bpf::extended::instructions::file_descriptors_map::*;
use crate::bpf::extended::instructions::offset::memory::VariableSlotU64;
use crate::bpf::extended::maps::domain::*;
use crate::bpf::extended::maps::domain::access_permissions::ExpressDataPathAccessPermissions;
use crate::bpf::extended::maps::express_data_path_redirect::*;
use crate::bpf::extended::programs::*;
use crate::bpf::extended::programs::program_type::*;
use crate::file_descriptors::*;
use crate::file_descriptors::bpf::*;
use crate::file_descriptors::netlink::NetlinkSocketFileDescriptor;
use crate::file_descriptors::netlink::route::RouteNetlinkProtocol;
use crate::file_descriptors::netlink::route::get_link::GetLinkMessageData;
use crate::file_descriptors::socket::*;
use crate::file_descriptors::socket::c::*;
use crate::memory::huge_pages::DefaultPageSizeAndHugePageSizes;
use crate::memory::huge_pages::HugePageSize;
use crate::memory::mapping::*;
use crate::network_device::*;


pub(crate) mod c;


/// Descriptors.
pub mod descriptors;


/// Ring queues.
pub mod ring_queues;


include!("AttachMode.rs");
include!("AttachProgramError.rs");
include!("FrameHeadroom.rs");
include!("FrameSize.rs");
include!("OwnedReceiveTransmitMemoryRingQueues.rs");
include!("OwnedRedirectMapAndAttachedProgramSettings.rs");
include!("QueueIdentifier.rs");
include!("ReceiveOrTransmitOrBoth.rs");
include!("ReceiveTransmitMemoryRingQueues.rs");
include!("RedirectMapAndAttachedProgram.rs");
include!("RingQueueDepth.rs");
include!("SharedReceiveTransmitMemoryRingQueues.rs");
include!("UpdateMode.rs");
include!("UserMemory.rs");
include!("UserMemoryArea.rs");
include!("UserMemoryAreaRelativeAddress.rs");
