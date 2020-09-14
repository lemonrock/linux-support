// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use crate::memory::mapping::MappedMemory;
use crate::network_device::NetworkInterfaceIndex;
use crate::bpf::extended::instructions::offset::memory::*;
use crate::bpf::extended::instructions::offset::*;
use crate::bpf::extended::instructions::offset::immediate::*;
use crate::file_descriptors::netlink::request::NetlinkRequestMessageBody;
use crate::file_descriptors::netlink::attributes::NetlinkAttributeType;


include!("sockaddr_xdp.rs");
include!("XDP_.SocketOptions.rs");
include!("XDP_.sxdp_flags.rs");
include!("XDP_.xdp_umem_reg.rs");
include!("xdp_action.rs");
include!("XDP_DIAG.rs");
include!("xdp_diag_msg.rs");
include!("xdp_diag_info.rs");
include!("xdp_diag_req.rs");
include!("xdp_diag_ring.rs");
include!("xdp_diag_stats.rs");
include!("xdp_diag_umem.rs");
include!("XDP_DU_F_.rs");
include!("xdp_md.rs");
include!("xdp_mmap_offsets.rs");
include!("xdp_options.rs");
include!("XDP_OPTIONS_.rs");
include!("XDP_PGOFF_.rs");
include!("XDP_RING_.rs");
include!("xdp_ring_offset.rs");
include!("XDP_SHOW_.rs");
include!("XDP_SHOW_flags.rs");
include!("xdp_statistics.rs");
include!("XDP_UMEM_PGOFF_.rs");
include!("xdp_umem_reg.rs");
include!("XdpDiagnosticUserMemoryFlags.rs");
include!("XdpOptionsFlags.rs");
include!("XdpSocketAddressFlags.rs");
include!("XdpUmemRegFlags.rs");
include!("XSK_UMEM__DEFAULT_FRAME_HEADROOM.rs");
include!("XSK_UNALIGNED_BUF_.rs");
