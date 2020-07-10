// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum MapType
{
	/// A hash.
	///
	/// Hash super-type (these do not support mmap).
	///
	/// Key size is anything in the range `1 ..= MAX_BPF_STACK`.
	/// Value size is anything in the range `1 .. KMALLOC_MAX_SIZE - MAX_BPF_STACK - sizeof(struct htab_elem)`.
	/// Max entries is non-zero.
	HashPerDevice(NetworkInterfaceIndex),
	
	/// A hash.
	///
	/// Hash super-type (these do not support mmap).
	/// Does not support having a NUMA node specified; this may also be the case for `HashPerDevice`.
	HashPerCpu(#[serde(default)] Preallocation),
	
	/// A hash.
	///
	/// Hash super-type (these do not support mmap).
	HashSystemWide(#[serde(default)] Option<NumaNode>, #[serde(default)] Preallocation),
	
	/// An array.
	///
	/// Array super-type.
	/// Key size is always 4 (`size_of::<u32>()`).
	/// Value size is anything in the range `1 ..= KMALLOC_MAX_SIZE`.
	/// Max entries is non-zero.
	ArrayPerDevice(NetworkInterfaceIndex),
	
	/// An array.
	///
	/// Array super-type.
	/// Key size is always 4 (`size_of::<u32>()`).
	/// Value size is anything in the range `1 ..= KMALLOC_MAX_SIZE`.
	/// Max entries is non-zero.
	ArrayPerCpu,
	
	/// An array.
	///
	/// Array super-type.
	/// Key size is always 4 (`size_of::<u32>()`).
	/// Value size is anything in the range `1 ..= KMALLOC_MAX_SIZE`.
	/// Max entries is non-zero.
	ArraySystemWide(#[serde(default)] Option<NumaNode>, #[serde(default)] MemoryMap),
	
	/// Least-recently used (LRU) Hash.
	///
	/// Hash super-type (these do not support mmap).
	///
	/// Key size is anything in the range `1 ..= MAX_BPF_STACK`.
	/// Value size is anything in the range `1 ..= ?`.
	/// Max entries is non-zero.
	///
	/// Requires the capability `CAP_SYS_ADMIN`.
	/// Does not support `BPF_MAP_CREATE_flags::BPF_F_NO_PREALLOC`.
	/// `LeastRecentlyUsedHash(PerCpu, _)` not support having a NUMA node specified.
	/// `LeastRecentlyUsedHash(_, OnePerCpu)` not support having a NUMA node specified.
	LeastRecentlyUsedHash(#[serde(default)] PerCpu, #[serde(default)] LeastRecentlyUsedLists),
	
	LeastRecentlyUsedHashPerCpuListPerCpu,
	LeastRecentlyUsedHashPerCpuListSystemWide,
	LeastRecentlyUsedHashSystemWideListPerCpu,
	LeastRecentlyUsedHashSystemWideListSystemWide(#[serde(default)] Option<NumaNode>),

	/// An array of eBPF programs to call.
	///
	/// Array super-type.
	/// Key size is always 4 (`size_of::<u32>()`).
	/// Value size is always 4 (`size_of::<RawFd>()` where `RawFd` is an `ExtendedBpfFileDescriptor`).
	/// Max entries is non-zero.
	ProgramArray,
	
	/// An array of perf event ?file descriptors?
	///
	/// Array super-type.
	/// Key size is always 4 (`size_of::<u32>()`).
	/// Value size is always 4 (`size_of::<RawFd>()`).
	/// Max entries is non-zero.
	PerfEventArray,
	
	/// Key size is always 4 (`size_of::<u32>()`).
	/// Value size can not be 0 and is always a multiple of 8 (`size_of::<AlignedU64>()`).
	/// `(Value size / 8) <= sysctl_perf_event_max_stack`.
	/// Max entries is non-zero.
	/// Requires the capability `CAP_SYS_ADMIN`.
	///
	/// Uses structures of the type `AlignedU64` (a pointer).
	///
	/// Does not support the user space access permission flags `BPF_F_RDONLY_PROG` and `BPF_F_WRONLY_PROG`.
	StackTraceAlignedU64(Option<NumaNode>),
	
	/// Key size is always 4 (`size_of::<u32>()`).
	/// Value size can not be 0 and is always a multiple of 32 (`size_of::<bpf_stack_build_id>()`).
	/// `(Value size / 32) <= sysctl_perf_event_max_stack`.
	/// Max entries is non-zero.
	/// Requires the capability `CAP_SYS_ADMIN`.
	///
	/// Uses structures of the type `bpf_stack_build_id`.
	///
	/// Does not support the user space access permission flags `BPF_F_RDONLY_PROG` and `BPF_F_WRONLY_PROG`.
	StackTraceBuildIdentifier(Option<NumaNode>),
	
	/// An array of ?cgroup? file descriptors.
	///
	/// Array super-type.
	/// Key size is always 4 (`size_of::<u32>()`).
	/// Value size is always 4 (`size_of::<RawFd>()`).
	/// Max entries is non-zero.
	CgroupArray,

	LongestPrefixMatchTrie,
	
	CgroupStorage(#[serde(default)] PerCpu),
	
	/// `vmlinux_value_type_identifier`.
	StructOps(#[serde(default)] BtfTypeIdentifier),
	
	/// Array of Maps.
	///
	/// Array super-type.
	/// Key size is always 4 (`size_of::<u32>()`).
	/// Value size is always 4 (`size_of::<RawFd>()` where `RawFd` is a `MapFileDescriptor`).
	/// Max entries is non-zero.
	ArrayOfMaps,
	
	/// Hash of Maps.
	///
	/// Hash super-type (these do not support mmap).
	/// Key size is anything in the range `1 ..= MAX_BPF_STACK`.
	/// Value size is always 4 (`size_of::<RawFd>()` where `RawFd` is a `MapFileDescriptor`).
	/// Max entries is non-zero.
	/// Supports `BPF_MAP_CREATE_flags::BPF_F_NO_PREALLOC`.
	HashOfMaps(#[serde(default)] Option<NumaNode>, Preallocation),
	
	DevMap,
	SockMap,
	CpuMap,
	XSkMap,
	SockHash,
	ReusePortSockArray,
	Queue,
	Stack(#[serde(default)] XXX),
	SkStorage,
	DevMapHash,
}

impl MapType
{
	#[inline(always)]
	pub fn to_values(&self) -> (bpf_map_type, BPF_MAP_CREATE_flags, BtfTypeIdentifier, Option<NetworkInterfaceIndex>, Option<NumaNode>)
	{
		use self::bpf_map_type::*;
		use self::LeastRecentlyUsedLists::*;
		use self::MapType::*;
		use self::PerCpu::*;
		use self::PerDevice::*;
		use self::MemoryMap::*;
		
		match self
		{
			&HashPerDevice(network_interface_index) => (BPF_MAP_TYPE_HASH, BPF_MAP_CREATE_flags::empty(), BtfTypeIdentifier::Void, Some(network_interface_index), None),
			&HashPerCpu(preallocation) => (BPF_MAP_TYPE_HASH, preallocation.to_flags(), BtfTypeIdentifier::Void, None, None),
			&HashSystemWide(None, preallocation) => (BPF_MAP_TYPE_PERCPU_HASH, preallocation.to_flags(), BtfTypeIdentifier::Void, None, None),
			&HashSystemWide(Some(numa_node), preallocation) => (BPF_MAP_TYPE_PERCPU_HASH, preallocation.to_flags() |BPF_MAP_CREATE_flags::BPF_F_NUMA_NODE, BtfTypeIdentifier::Void, None, Some(numa_node)),
			
			&ArrayPerDevice(network_interface_index) => (BPF_MAP_TYPE_ARRAY, BPF_MAP_CREATE_flags::empty(), BtfTypeIdentifier::Void, Some(network_interface_index), None),
			&ArrayPerCpu => (BPF_MAP_TYPE_PERCPU_ARRAY, BPF_MAP_CREATE_flags::empty(), BtfTypeIdentifier::Void, None, None),
			&ArraySystemWide(None, memory_map) => (BPF_MAP_TYPE_ARRAY, memory_map.to_flags(), BtfTypeIdentifier::Void, None, None),
			&ArraySystemWide(Some(numa_node), memory_map) => (BPF_MAP_TYPE_ARRAY, memory_map.to_flags() | BPF_MAP_CREATE_flags::BPF_F_NUMA_NODE, BtfTypeIdentifier::Void, None, Some(numa_node)),
			
			&LeastRecentlyUsedHashPerCpuListPerCpu => (BPF_MAP_TYPE_LRU_HASH, BPF_MAP_CREATE_flags::empty(), BtfTypeIdentifier::Void, None, None),
			&LeastRecentlyUsedHashPerCpuListSystemWide => (BPF_MAP_TYPE_LRU_HASH, BPF_MAP_CREATE_flags::BPF_F_NO_COMMON_LRU, BtfTypeIdentifier::Void, None, None),
			&LeastRecentlyUsedHashSystemWideListPerCpu => (BPF_MAP_TYPE_LRU_PERCPU_HASH, BPF_MAP_CREATE_flags::empty(), BtfTypeIdentifier::Void, None, None),
			&LeastRecentlyUsedHashSystemWideListSystemWide(None) => (BPF_MAP_TYPE_LRU_PERCPU_HASH, BPF_MAP_CREATE_flags::BPF_F_NO_COMMON_LRU, BtfTypeIdentifier::Void, None, None),
			&LeastRecentlyUsedHashSystemWideListSystemWide(Some(numa_node)) => (BPF_MAP_TYPE_LRU_PERCPU_HASH, BPF_MAP_CREATE_flags::BPF_F_NO_COMMON_LRU | BPF_MAP_CREATE_flags::BPF_F_NUMA_NODE, BtfTypeIdentifier::Void, None, Some(numa_node)),
			
			&ProgramArray => (BPF_MAP_TYPE_PROG_ARRAY, BPF_MAP_CREATE_flags::empty(), BtfTypeIdentifier::Void, None, None),
			&PerfEventArray => (BPF_MAP_TYPE_PERF_ARRAY, BPF_MAP_CREATE_flags::empty(), BtfTypeIdentifier::Void, None, None),
			
			&StackTraceAlignedU64(None) => (BPF_MAP_TYPE_STACK_TRACE, BPF_MAP_CREATE_flags::empty(), BtfTypeIdentifier::Void, None, None),
			&StackTraceAlignedU64(Some(numa_node)) => (BPF_MAP_TYPE_STACK_TRACE, BPF_MAP_CREATE_flags::BPF_F_NUMA_NODE, BtfTypeIdentifier::Void, None, Some(numa_node)),
			&StackTraceBuildIdentifier(None) => (BPF_MAP_TYPE_STACK_TRACE, BPF_MAP_CREATE_flags::BPF_F_STACK_BUILD_ID, BtfTypeIdentifier::Void, None, None),
			&StackTraceBuildIdentifier(Some(numa_node)) => (BPF_MAP_TYPE_STACK_TRACE, BPF_MAP_CREATE_flags::BPF_F_STACK_BUILD_ID | BPF_MAP_CREATE_flags::BPF_F_NUMA_NODE, BtfTypeIdentifier::Void, None, Some(numa_node)),
			
			&CgroupArray => (BPF_MAP_TYPE_CGROUP_ARRAY, BPF_MAP_CREATE_flags::empty(), BtfTypeIdentifier::Void, None, None),
			&LongestPrefixMatchTrie => (BPF_MAP_TYPE_LPM_TRIE, BPF_MAP_CREATE_flags::empty(), BtfTypeIdentifier::Void, None, None),
			&CgroupStorage(SystemWide) => (BPF_MAP_TYPE_CGROUP_STORAGE, BPF_MAP_CREATE_flags::empty(), BtfTypeIdentifier::Void, None, None),
			&CgroupStorage(IsPerCpu) => (BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE, BPF_MAP_CREATE_flags::empty(), BtfTypeIdentifier::Void, None, None),
			&StructOps(vmlinux_value_type_identifier) => (BPF_MAP_TYPE_STRUCT_OPS, BPF_MAP_CREATE_flags::empty(), vmlinux_value_type_identifier, None, None),
			
			&ArrayOfMaps => (BPF_MAP_TYPE_ARRAY_OF_MAPS, BPF_MAP_CREATE_flags::empty(), BtfTypeIdentifier::Void, None, None),
			
			&HashOfMaps(None, preallocation) => (BPF_MAP_TYPE_HASH_OF_MAPS, preallocation.to_flags(), BtfTypeIdentifier::Void, None),
			&HashOfMaps(Some(numa_node), preallocation) => (BPF_MAP_TYPE_HASH_OF_MAPS, preallocation.to_flags() | BPF_MAP_CREATE_flags::BPF_F_NUMA_NODE, BtfTypeIdentifier::Void, None, Some(numa_node)),
		}
	}
}
