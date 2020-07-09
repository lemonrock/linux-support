// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum MapType
{
	/// A hash.
	Hash(#[serde(default)] PerDevice),

	/// An array.
	Array(#[serde(default)] PerDevice),
	
	LeastRecentlyUsedHash(#[serde(default)] PerCpu, #[serde(default)] LeastRecentlyUsedLists),
	PerCpuLeastRecentlyUsedHash,

	/// An array of eBPF programs to call.
	ProgramArray,
	
	/// An array of perf event ?file descriptors?
	PerfEventArray,
	
	StackTrace,
	
	/// An array of ?cgroup?
	CgroupArray,

	LongestPrefixMatchTrie,
	
	CgroupStorage(#[serde(default)] PerCpu),
	
	/// `vmlinux_value_type_identifier`.
	StructOps(#[serde(default)] BtfTypeIdentifier),
	
	ArrayOfMaps,
	HashOfMaps,
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
	pub fn to_values(&self) -> (bpf_map_type, BPF_MAP_CREATE_flags, BtfTypeIdentifier, Option<NetworkInterfaceIndex>)
	{
		use self::bpf_map_type::*;
		use self::LeastRecentlyUsedLists::*;
		use self::MapType::*;
		use self::PerCpu::*;
		use self::PerDevice::*;
		
		match self
		{
			&Hash(IsPerDevice(network_interface_index)) => (BPF_MAP_TYPE_HASH, BPF_MAP_CREATE_flags::empty(), BtfTypeIdentifier::Void, Some(network_interface_index)),
			&Hash(Cpu(SystemWide)) => (BPF_MAP_TYPE_HASH, BPF_MAP_CREATE_flags::empty(), BtfTypeIdentifier::Void, None),
			&Hash(Cpu(IsPerCpu)) => (BPF_MAP_TYPE_PERCPU_HASH, BPF_MAP_CREATE_flags::empty(), BtfTypeIdentifier::Void, None),
			&Array(IsPerDevice(network_interface_index)) => (BPF_MAP_TYPE_ARRAY, BPF_MAP_CREATE_flags::empty(), BtfTypeIdentifier::Void, Some(network_interface_index)),
			&Array(Cpu(SystemWide)) => (BPF_MAP_TYPE_ARRAY, BPF_MAP_CREATE_flags::empty(), BtfTypeIdentifier::Void, None),
			&Array(Cpu(IsPerCpu)) => (BPF_MAP_TYPE_PERCPU_ARRAY, BPF_MAP_CREATE_flags::empty(), BtfTypeIdentifier::Void, None),
			
			LeastRecentlyUsedHash(SystemWide, Common) => (BPF_MAP_TYPE_LRU_HASH, BPF_MAP_CREATE_flags::empty(), BtfTypeIdentifier::Void, None),
			LeastRecentlyUsedHash(SystemWide, OnePerCpu) => (BPF_MAP_TYPE_LRU_HASH, BPF_MAP_CREATE_flags::BPF_F_NO_COMMON_LRU, BtfTypeIdentifier::Void, None),
			LeastRecentlyUsedHash(IsPerCpu, Common) => (BPF_MAP_TYPE_LRU_PERCPU_HASH, BPF_MAP_CREATE_flags::empty(), BtfTypeIdentifier::Void, None),
			LeastRecentlyUsedHash(IsPerCpu, OnePerCpu) => (BPF_MAP_TYPE_LRU_PERCPU_HASH, BPF_MAP_CREATE_flags::BPF_F_NO_COMMON_LRU, BtfTypeIdentifier::Void, None),
			&ProgramArray => (BPF_MAP_TYPE_PROG_ARRAY, BPF_MAP_CREATE_flags::empty(), BtfTypeIdentifier::Void, None),
			&PerfEventArray => (BPF_MAP_TYPE_PERF_ARRAY, BPF_MAP_CREATE_flags::empty(), BtfTypeIdentifier::Void, None),
			&StackTrace => (BPF_MAP_TYPE_STACK_TRACE, BPF_MAP_CREATE_flags::empty(), BtfTypeIdentifier::Void, None),
			&CgroupArray => (BPF_MAP_TYPE_CGROUP_ARRAY, BPF_MAP_CREATE_flags::empty(), BtfTypeIdentifier::Void, None),
			&LongestPrefixMatchTrie => (BPF_MAP_TYPE_LPM_TRIE, BPF_MAP_CREATE_flags::empty(), BtfTypeIdentifier::Void, None),
			&CgroupStorage(SystemWide) => (BPF_MAP_TYPE_CGROUP_STORAGE, BPF_MAP_CREATE_flags::empty(), BtfTypeIdentifier::Void, None),
			&CgroupStorage(IsPerCpu) => (BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE, BPF_MAP_CREATE_flags::empty(), BtfTypeIdentifier::Void, None),
			&StructOps(vmlinux_value_type_identifier) => (BPF_MAP_TYPE_STRUCT_OPS, BPF_MAP_CREATE_flags::empty(), vmlinux_value_type_identifier, None)
		}
	}
}
