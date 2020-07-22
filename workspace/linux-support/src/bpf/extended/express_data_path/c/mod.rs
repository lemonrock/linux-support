// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;


include!("XDP_.sxdp_flags.rs");
include!("XDP_.xsk_umem_config.rs");
include!("XDP_RING_.rs");



struct sockaddr_xdp {
	__u16 sxdp_family;
	__u16 sxdp_flags;
	__u32 sxdp_ifindex;
	__u32 sxdp_queue_id;
	__u32 sxdp_shared_umem_fd;
};


struct xdp_ring_offset {
	__u64 producer;
	__u64 consumer;
	__u64 desc;
	__u64 flags;
};

struct xdp_mmap_offsets {
	struct xdp_ring_offset rx;
	struct xdp_ring_offset tx;
	struct xdp_ring_offset fr; /* Fill */
	struct xdp_ring_offset cr; /* Completion */
};

/* XDP socket options */
#define XDP_MMAP_OFFSETS		1
#define XDP_RX_RING			2
#define XDP_TX_RING			3
#define XDP_UMEM_REG			4
#define XDP_UMEM_FILL_RING		5
#define XDP_UMEM_COMPLETION_RING	6
#define XDP_STATISTICS			7
#define XDP_OPTIONS			8

struct xdp_umem_reg {
	__u64 addr; /* Start of packet data area */
	__u64 len; /* Length of packet data area */
	__u32 chunk_size;
	__u32 headroom;
	__u32 flags;
};

struct xdp_statistics {
	__u64 rx_dropped; /* Dropped for reasons other than invalid desc */
	__u64 rx_invalid_descs; /* Dropped due to invalid descriptor */
	__u64 tx_invalid_descs; /* Dropped due to invalid descriptor */
};

struct xdp_options {
	__u32 flags;
};

/* Flags for the flags field of struct xdp_options */
#define XDP_OPTIONS_ZEROCOPY (1 << 0)

/* Pgoff for mmaping the rings */
#define XDP_PGOFF_RX_RING			  0
#define XDP_PGOFF_TX_RING		 0x80000000
#define XDP_UMEM_PGOFF_FILL_RING	0x100000000ULL
#define XDP_UMEM_PGOFF_COMPLETION_RING	0x180000000ULL

/* Masks for unaligned chunks mode */
#define XSK_UNALIGNED_BUF_OFFSET_SHIFT 48
#define XSK_UNALIGNED_BUF_ADDR_MASK \
	((1ULL << XSK_UNALIGNED_BUF_OFFSET_SHIFT) - 1)

/* Rx/Tx descriptor */
struct xdp_desc {
	__u64 addr;
	__u32 len;
	__u32 options;
};

/* UMEM descriptor is __u64 */
