// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Returns error `EBUSY` if a XDP extended BPF program is already attached.
pub(crate) const XDP_FLAGS_UPDATE_IF_NOEXIST: u32 = 1 << 0;

/// Known as generic mode.
///
/// The XDP program is loaded into the kernel as part of the normal network path.
/// This mode does not impart the same performance benefits as Native or Offloaded XDP, but works generically on kernels since 4.12 and is great for testing XDP programs or running them on generic hardware.
///
/// `net_attach_type::NET_ATTACH_TYPE_XDP_GENERIC`.
pub(crate) const XDP_FLAGS_SKB_MODE: u32 = 1 << 1;

/// Known as native mode.
///
/// The XDP program is loaded by the NICs driver into its early receive path.
/// This requires support by the NIC driver.
///
/// `net_attach_type::NET_ATTACH_TYPE_XDP_DRIVER`.
pub(crate) const XDP_FLAGS_DRV_MODE: u32 = 1 << 2;

/// Known as offloaded mode.
///
/// The XDP program is loaded onto the NIC itself, executing entirely off of the CPU.
/// This requires support by the NIC device itself.
///
/// Such support is rare, but Netronome drivers (`nfp`) are known to support it for some devices.
///
/// `net_attach_type::NET_ATTACH_TYPE_XDP_OFFLOAD`.
pub(crate) const XDP_FLAGS_HW_MODE: u32 = 1 << 3;

/// Returns error `EEXIST` if a XDP extended BPF program is already attached with a different file descriptor to that expected.
pub(crate) const XDP_FLAGS_REPLACE: u32 = 1 << 4;

#[allow(dead_code)]
pub(crate) const XDP_FLAGS_MODES: u32 = XDP_FLAGS_SKB_MODE | XDP_FLAGS_DRV_MODE | XDP_FLAGS_HW_MODE;

#[allow(dead_code)]
pub(crate) const XDP_FLAGS_MASK: u32 = XDP_FLAGS_UPDATE_IF_NOEXIST | XDP_FLAGS_MODES | XDP_FLAGS_REPLACE;
