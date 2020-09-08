// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) enum bpf_map_type
{
	#[allow(dead_code)]
	#[doc(hidden)]
	#[serde(rename = "unspec")] BPF_MAP_TYPE_UNSPEC = 0,
	
	#[serde(rename = "hash")] BPF_MAP_TYPE_HASH = 1,
	#[serde(rename = "array")] BPF_MAP_TYPE_ARRAY = 2,
	#[serde(rename = "prog_array")] BPF_MAP_TYPE_PROG_ARRAY = 3,
	#[serde(rename = "perf_event_array")] BPF_MAP_TYPE_PERF_EVENT_ARRAY = 4,
	#[serde(rename = "percpu_hash")] BPF_MAP_TYPE_PERCPU_HASH = 5,
	#[serde(rename = "percpu_array")] BPF_MAP_TYPE_PERCPU_ARRAY = 6,
	#[serde(rename = "stack_trace")] BPF_MAP_TYPE_STACK_TRACE = 7,
	#[serde(rename = "cgroup_array")] BPF_MAP_TYPE_CGROUP_ARRAY = 8,
	#[serde(rename = "lru_hash")] BPF_MAP_TYPE_LRU_HASH = 9,
	#[serde(rename = "lru_percpu_hash")] BPF_MAP_TYPE_LRU_PERCPU_HASH = 10,
	#[serde(rename = "lpm_trie")] BPF_MAP_TYPE_LPM_TRIE = 11,
	#[serde(rename = "array_of_maps")] BPF_MAP_TYPE_ARRAY_OF_MAPS = 12,
	#[serde(rename = "hash_of_maps")] BPF_MAP_TYPE_HASH_OF_MAPS = 13,
	#[serde(rename = "devmap")] BPF_MAP_TYPE_DEVMAP = 14,
	#[serde(rename = "sockmap")] BPF_MAP_TYPE_SOCKMAP = 15,
	#[serde(rename = "cpumap")] BPF_MAP_TYPE_CPUMAP = 16,
	#[serde(rename = "xskmap")] BPF_MAP_TYPE_XSKMAP = 17,
	#[serde(rename = "sockhash")] BPF_MAP_TYPE_SOCKHASH = 18,
	#[serde(rename = "cgroup_storage")] BPF_MAP_TYPE_CGROUP_STORAGE = 19,
	#[serde(rename = "reuseport_sockarray")] BPF_MAP_TYPE_REUSEPORT_SOCKARRAY = 20,
	#[serde(rename = "percpu_cgroup_storage")] BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE = 21,
	#[serde(rename = "queue")] BPF_MAP_TYPE_QUEUE = 22,
	#[serde(rename = "stack")] BPF_MAP_TYPE_STACK = 23,
	#[serde(rename = "sk_storage")] BPF_MAP_TYPE_SK_STORAGE = 24,
	#[serde(rename = "devmap_hash")] BPF_MAP_TYPE_DEVMAP_HASH = 25,
	#[serde(rename = "struct_ops")] BPF_MAP_TYPE_STRUCT_OPS = 26,
}
