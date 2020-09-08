// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[doc(hidden)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum bpf_prog_type
{
	#[doc(hidden)]
	#[serde(rename = "unspec")] BPF_PROG_TYPE_UNSPEC = 0,
	
	#[serde(rename = "socket_filter")] BPF_PROG_TYPE_SOCKET_FILTER = 1,
	#[serde(rename = "kprobe")] BPF_PROG_TYPE_KPROBE = 2,
	#[serde(rename = "sched_cls")] BPF_PROG_TYPE_SCHED_CLS = 3,
	#[serde(rename = "sched_act")] BPF_PROG_TYPE_SCHED_ACT = 4,
	#[serde(rename = "tracepoint")] BPF_PROG_TYPE_TRACEPOINT = 5,
	#[serde(rename = "xdp")] BPF_PROG_TYPE_XDP = 6,
	#[serde(rename = "perf_event")] BPF_PROG_TYPE_PERF_EVENT = 7,
	#[serde(rename = "cgroup_skb")] BPF_PROG_TYPE_CGROUP_SKB = 8,
	#[serde(rename = "cgroup_sock")] BPF_PROG_TYPE_CGROUP_SOCK = 9,
	#[serde(rename = "lwt_in")] BPF_PROG_TYPE_LWT_IN = 10,
	#[serde(rename = "lwt_out")] BPF_PROG_TYPE_LWT_OUT = 11,
	#[serde(rename = "lwt_xmit")] BPF_PROG_TYPE_LWT_XMIT = 12,
	#[serde(rename = "sock_ops")] BPF_PROG_TYPE_SOCK_OPS = 13,
	#[serde(rename = "sk_skb")] BPF_PROG_TYPE_SK_SKB = 14,
	#[serde(rename = "cgroup_device")] BPF_PROG_TYPE_CGROUP_DEVICE = 15,
	#[serde(rename = "sk_msg")] BPF_PROG_TYPE_SK_MSG = 16,
	#[serde(rename = "raw_tracepoint")] BPF_PROG_TYPE_RAW_TRACEPOINT = 17,
	#[serde(rename = "cgroup_sock_addr")] BPF_PROG_TYPE_CGROUP_SOCK_ADDR = 18,
	#[serde(rename = "lwt_seg6local")] BPF_PROG_TYPE_LWT_SEG6LOCAL = 19,
	#[serde(rename = "lirc_mode2")] BPF_PROG_TYPE_LIRC_MODE2 = 20,
	#[serde(rename = "sk_reuseport")] BPF_PROG_TYPE_SK_REUSEPORT = 21,
	#[serde(rename = "flow_dissector")] BPF_PROG_TYPE_FLOW_DISSECTOR = 22,
	#[serde(rename = "cgroup_sysctl")] BPF_PROG_TYPE_CGROUP_SYSCTL = 23,
	#[serde(rename = "raw_tracepoint_writable")] BPF_PROG_TYPE_RAW_TRACEPOINT_WRITABLE = 24,
	#[serde(rename = "cgroup_sockopt")] BPF_PROG_TYPE_CGROUP_SOCKOPT = 25,
	#[serde(rename = "tracing")] BPF_PROG_TYPE_TRACING = 26,
	#[serde(rename = "struct_ops")] BPF_PROG_TYPE_STRUCT_OPS = 27,
	#[serde(rename = "ext")] BPF_PROG_TYPE_EXT = 28,
	#[serde(rename = "lsm")] BPF_PROG_TYPE_LSM = 29,
}
