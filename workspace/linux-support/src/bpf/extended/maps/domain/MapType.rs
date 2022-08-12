// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) enum MapType<'map_of_maps_template_file_descriptor>
{
	/// A hash.
	///
	/// Hash super-type.
	///
	/// Key size is anything in the range `1 ..= MAX_BPF_STACK`.
	/// Value size is anything in the range `1 .. KMALLOC_MAX_SIZE - MAX_BPF_STACK - size_of::<htab_elem>()` but we restrict to an inclusive maximum of `1 << 21`.
	/// Max entries is non-zero.
	HashPerDevice(KeySize, ValueSizeU32, MaximumEntries, #[serde(default)] AccessPermissions, NetworkInterfaceIndex),
	
	/// A hash.
	///
	/// Hash super-type.
	///
	/// Key size is anything in the range `1 ..= MAX_BPF_STACK`.
	/// Value size is anything in the range `1 .. KMALLOC_MAX_SIZE - MAX_BPF_STACK - size_of::<htab_elem>()`.
	HashPerHyperThread(KeySize, ValueSizeU32, MaximumEntries, #[serde(default)] AccessPermissions, #[serde(default)] Preallocation),
	
	/// A hash.
	///
	/// Hash super-type.
	///
	/// Key size is anything in the range `1 ..= MAX_BPF_STACK`.
	/// Value size is anything in the range `1 .. KMALLOC_MAX_SIZE - MAX_BPF_STACK - size_of::<htab_elem>()`.
	HashSystemWide(KeySize, ValueSizeU32, MaximumEntries, #[serde(default)] AccessPermissions, #[serde(default)] Option<NumaNode>, #[serde(default)] Preallocation),
	
	/// An array.
	///
	/// Array super-type.
	///
	/// Key size is always 4 (`size_of::<u32>()`).
	/// Value size is anything in the range `1 ..= KMALLOC_MAX_SIZE`.
	/// Max entries is non-zero.
	ArrayPerDevice(ValueSizeU32, MaximumEntries, #[serde(default)] AccessPermissions, NetworkInterfaceIndex),
	
	/// An array.
	///
	/// Array super-type.
	///
	/// Key size is always 4 (`size_of::<u32>()`).
	/// Value size is anything in the range `1 ..= KMALLOC_MAX_SIZE`.
	/// Max entries is non-zero.
	ArrayPerHyperThread(ValueSizeU32, MaximumEntries, #[serde(default)] AccessPermissions),
	
	/// An array.
	///
	/// Array super-type.
	///
	/// Key size is always 4 (`size_of::<u32>()`).
	/// Value size is anything in the range `1 ..= KMALLOC_MAX_SIZE`.
	/// Max entries is non-zero.
	ArraySystemWide(ValueSizeU32, MaximumEntries, #[serde(default)] AccessPermissions, #[serde(default)] Option<NumaNode>, #[serde(default)] MemoryMap),
	
	/// Least-recently used (LRU) Hash.
	///
	/// Hash super-type.
	///
	/// Key size is anything in the range `1 ..= MAX_BPF_STACK`.
	/// Value size is anything in the range `1 ..= ?`.
	/// Max entries is non-zero.
	///
	/// Requires the capability `CAP_SYS_ADMIN`.
	LeastRecentlyUsedHashSystemWide(KeySize, ValueSizeU32, MaximumEntries, #[serde(default)] AccessPermissions, #[serde(default)] Option<NumaNode>),
	
	/// Least-recently used (LRU) Hash.
	///
	/// Hash super-type.
	///
	/// Key size is anything in the range `1 ..= MAX_BPF_STACK`.
	/// Value size is anything in the range `1 ..= ?`.
	/// Max entries is non-zero.
	///
	/// Requires the capability `CAP_SYS_ADMIN`.
	LeastRecentlyUsedHashSystemWideWithAPerHyperThreadLeastRecentlyUsedList(KeySize, ValueSizeU32, MaximumEntries, #[serde(default)] AccessPermissions),
	
	/// Least-recently used (LRU) Hash.
	///
	/// Hash super-type.
	///
	/// Key size is anything in the range `1 ..= MAX_BPF_STACK`.
	/// Value size is anything in the range `1 ..= ?`.
	/// Max entries is non-zero.
	///
	/// Requires the capability `CAP_SYS_ADMIN`.
	LeastRecentlyUsedHashPerHyperThread(KeySize, ValueSizeU32, MaximumEntries, #[serde(default)] AccessPermissions),
	
	/// Least-recently used (LRU) Hash.
	///
	/// Hash super-type.
	///
	/// Key size is anything in the range `1 ..= MAX_BPF_STACK`.
	/// Value size is anything in the range `1 ..= ?`.
	/// Max entries is non-zero.
	///
	/// Requires the capability `CAP_SYS_ADMIN`.
	LeastRecentlyUsedHashPerHyperThreadWithAPerHyperThreadLeastRecentlyUsedList(KeySize, ValueSizeU32, MaximumEntries, #[serde(default)] AccessPermissions),

	/// An array of eBPF programs to call.
	///
	/// Array super-type.
	///
	/// Key size is always 4 (`size_of::<u32>()`).
	/// Value size is always 4 (`size_of::<RawFd>()` where `RawFd` is an `ExtendedBpfFileDescriptor`).
	/// Max entries is non-zero.
	ProgramArray(MaximumEntries, #[serde(default)] KernelOnlyAccessPermissions),
	
	/// An array of perf event file descriptors
	///
	/// Array super-type.
	///
	/// Key size is always 4 (`size_of::<u32>()`).
	/// Value size is always 4 (`size_of::<RawFd>()` where `RawFd` is an `PerfEventFileDescriptor`).
	/// Max entries is non-zero.
	PerfEventArray(MaximumEntries, #[serde(default)] KernelOnlyAccessPermissions),
	
	/// Key size is always 4 (`size_of::<u32>()`).
	/// Value size can not be 0 and is always a multiple of 8 (`size_of::<AlignedU64>()`).
	/// `(Value size / 8) <= sysctl_perf_event_max_stack`; `sysctl_perf_event_max_stack` is `PERF_MAX_STACK_DEPTH` which is `127`.
	/// Max entries is non-zero.
	///
	/// Requires the capability `CAP_SYS_ADMIN`.
	///
	/// Uses structures of the type `AlignedU64` (a pointer).
	StackTraceInstructionPointerAddress(StackDepth, MaximumEntries, #[serde(default)] KernelOnlyAccessPermissions, #[serde(default)] Option<NumaNode>),
	
	/// Key size is always 4 (`size_of::<u32>()`).
	/// Value size can not be 0 and is always a multiple of 32 (`size_of::<bpf_stack_build_id>()`).
	/// `(Value size / 32) <= sysctl_perf_event_max_stack`; `sysctl_perf_event_max_stack` is `PERF_MAX_STACK_DEPTH` which is `127`.
	/// Max entries is non-zero.
	///
	/// Requires the capability `CAP_SYS_ADMIN`.
	///
	/// Uses structures of the type `bpf_stack_build_id`.
	StackTraceBuildIdentifier(StackDepth, MaximumEntries, #[serde(default)] KernelOnlyAccessPermissions, #[serde(default)] Option<NumaNode>),
	
	/// An array of cgroup file descriptors.
	///
	/// Array super-type.
	///
	/// Key size is always 4 (`size_of::<u32>()`).
	/// Value size is always 4 (`size_of::<RawFd>()` where `RawFd` is an `CgroupFileDescriptor`).
	/// Max entries is non-zero.
	CgroupArray(MaximumEntries, #[serde(default)] KernelOnlyAccessPermissions),
	
	/// Reuse port socket array.
	///
	/// Array super-type.
	///
	/// Key size is `size_of::<u32>()`.
	/// Value size is `size_of::<u32>()` (4).
	/// Max entries is non-zero.(MaximumEntries, #[serde(default)] AccessPermissions, #[serde(default)] Option<NumaNode>)
	ReusePortSocketArrayU32(MaximumEntries, #[serde(default)] AccessPermissions, #[serde(default)] Option<NumaNode>),
	
	/// Reuse port socket array.
	///
	/// Array super-type.
	///
	/// Key size is `size_of::<u32>()`.
	/// Value size is `size_of::<u64>()` (8).
	/// Max entries is non-zero.
	ReusePortSocketArrayU64(MaximumEntries, #[serde(default)] AccessPermissions, #[serde(default)] Option<NumaNode>),
	
	/// Array of Maps.
	///
	/// Array super-type.
	///
	/// Key size is always 4 (`size_of::<u32>()`).
	/// Value size is always 4 (`size_of::<RawFd>()` where `RawFd` is a `MapFileDescriptor`).
	/// Max entries is non-zero.
	#[serde(skip)] ArrayOfMaps(MaximumEntries, #[serde(default)] KernelOnlyAccessPermissions, &'map_of_maps_template_file_descriptor MapFileDescriptor),
	
	/// Hash of Maps.
	///
	/// Hash super-type.
	///
	/// Key size is anything in the range `1 ..= MAX_BPF_STACK`.
	/// Value size is always 4 (`size_of::<RawFd>()` where `RawFd` is a `MapFileDescriptor`).
	/// Max entries is non-zero.
	#[serde(skip)] HashOfMaps(KeySize, MaximumEntries, #[serde(default)] KernelOnlyAccessPermissions, &'map_of_maps_template_file_descriptor MapFileDescriptor, #[serde(default)] Option<NumaNode>, Preallocation),

	/// Longest-prefix match (LPM) trie.
	///
	/// Key size is anything in the range `(size_of::<bpf_lpm_trie_key>() + 1) ..= (size_of::<bpf_lpm_trie_key>() + 256)`.
	/// But in practice key size is either `(size_of::<bpf_lpm_trie_key>() + 4)` for Internet Protocol version 4 or `(size_of::<bpf_lpm_trie_key>() + 16)` for Internet Protocol version 6; `size_of::<bpf_lpm_trie_key>()` is 4 bytes and holds the prefix length.
	/// Value size is anything in the range `1 ..= (KMALLOC_MAX_SIZE - 256 - size_of::<lpm_trie_node>())`.
	/// Max entries is non-zero.
	LongestPrefixMatchTrieInternetProtocolVersion4(ValueSizeU32, MaximumEntries, #[serde(default)] AccessPermissions, #[serde(default)] Option<NumaNode>),

	/// Longest-prefix match (LPM) trie.
	///
	/// Key size is anything in the range `(size_of::<bpf_lpm_trie_key>() + 1) ..= (size_of::<bpf_lpm_trie_key>() + 256)`.
	/// But in practice key size is either `(size_of::<bpf_lpm_trie_key>() + 4)` for Internet Protocol version 4 or `(size_of::<bpf_lpm_trie_key>() + 16)` for Internet Protocol version 6; `size_of::<bpf_lpm_trie_key>()` is 4 bytes and holds the prefix length.
	/// Value size is anything in the range `1 ..= (KMALLOC_MAX_SIZE - 256 - size_of::<lpm_trie_node>())`.
	/// Max entries is non-zero.
	LongestPrefixMatchTrieInternetProtocolVersion6(ValueSizeU32, MaximumEntries, #[serde(default)] AccessPermissions, #[serde(default)] Option<NumaNode>),
	
	/// Key size is `size_of::<bpf_cgroup_storage_key>()`.
	/// Value size is anything in the range `1 ..= PAGE_SIZE`.
	/// Max entries is always zero as it is unused.
	///
	/// Weirdly, unlike all other per-HyperThread maps, this one supports allocating on a NumaNode.
	CgroupStoragePerHyperThread(ValueSizeU16, #[serde(default)] AccessPermissions, #[serde(default)] Option<NumaNode>),
	
	/// Key size is `size_of::<bpf_cgroup_storage_key>()`.
	/// Value size is anything in the range `1 ..= PAGE_SIZE`.
	/// Max entries is always zero as it is unused.
	CgroupStorageSystemWide(ValueSizeU16, #[serde(default)] AccessPermissions, #[serde(default)] Option<NumaNode>),
	
	/// Key size is `size_of::<u32>()`.
	/// Value size is the size of the type referred to by `VMLinuxValueBpfTypeFormatTypeIdentifier`.
	/// Max entries is always one.
	///
	/// Requires the capability `CAP_SYS_ADMIN`.
	///
	/// `btf_key_type_id` and `btf_value_type_id` are not valid and must be `Void`.
	///
	/// No `AccessPermissions` are permitted.
	StructOps(ValueSizeU32, VMLinuxValueBpfTypeFormatTypeIdentifier),
	
	/// Netdev array map.
	///
	/// Primary use is a map for XDP BPF helper call `bpf_redirect_map()` to redirect to another network device.
	///
	/// Key size is `size_of::<u32>()`.
	/// Value size is `size_of::<Option<NetworkInterfaceIndex>>()` (4).
	/// Max entries is non-zero.
	///
	/// Requires the capability `CAP_NET_ADMIN`.
	///
	/// Does not support the user space access permission flags `BPF_F_RDONLY_PROG` and `BPF_F_WRONLY_PROG`.
	ExpressDataPathRedirectToNetworkDeviceArray(MaximumEntries, #[serde(default)] ExpressDataPathAccessPermissions, #[serde(default)] Option<NumaNode>),
	
	/// Netdev hash map.
	///
	/// Primary use is a map for XDP BPF helper call `bpf_redirect_map()` to redirect to another network device.
	///
	/// Key size is `size_of::<u32>()`.
	/// Value size is `size_of::<Option<NetworkInterfaceIndex>>()` (4).
	/// Max entries is non-zero.
	///
	/// Requires the capability `CAP_NET_ADMIN`.
	ExpressDataPathRedirectToNetworkDeviceHash(MaximumEntries, #[serde(default)] ExpressDataPathAccessPermissions, #[serde(default)] Option<NumaNode>),
	
	/// HyperThread array map.
	///
	/// Primary use is a map for XDP BPF helper call `bpf_redirect_map()` to redirect to another HyperThread.
	/// This allows for 10G wire speed pre-filtering using BPF!
	///
	/// Key size is `size_of::<u32>()`.
	/// Value size is `size_of::<u32>()` (4) but is actually a `NumaNode`.
	/// Max entries is in the range `1 ..= NR_CPUS`.
	///
	/// Requires the capability `CAP_SYS_ADMIN`.
	ExpressDataPathRedirectToHyperThreadArray(MaximumEntries, #[serde(default)] Option<NumaNode>),
	
	/// XDP socket map.
	///
	/// Primary use is a map for XDP BPF helper call `bpf_redirect_map()` to redirect to another XDP socket (XSK).
	///
	/// Key size is `size_of::<u32>()`.
	/// Value size is `size_of::<RawFd>()` (4) which is a reference to a XDP socket.
	/// Max entries is non-zero.
	///
	/// Requires the capability `CAP_NET_ADMIN`.
	///
	/// Does not support the user space access permission flags `BPF_F_RDONLY_PROG` and `BPF_F_WRONLY_PROG`.
	ExpressDataPathRedirectToSocketArray(MaximumEntries, #[serde(default)] ExpressDataPathAccessPermissions, #[serde(default)] Option<NumaNode>),
	
	/// Key size is `size_of::<u32>()`.
	/// Value size is `size_of::<u32>()` (4).
	/// Max entries is non-zero.
	///
	/// Requires the capability `CAP_NET_ADMIN`.
	SocketArrayU32(MaximumEntries, #[serde(default)] KernelOnlyAccessPermissions, #[serde(default)] Option<NumaNode>),
	
	/// Key size is `size_of::<u32>()`.
	/// Value size is `size_of::<u64>()` (8).
	/// Max entries is non-zero.
	///
	/// Requires the capability `CAP_NET_ADMIN`.
	SocketArrayU64(MaximumEntries, #[serde(default)] KernelOnlyAccessPermissions, #[serde(default)] Option<NumaNode>),
	
	/// Key size is in the range `1 ..= MAX_BPF_STACK`.
	/// Value size is `size_of::<u32>()`.
	/// Max entries is non-zero.
	///
	/// Requires the capability `CAP_NET_ADMIN`.
	SocketHashU32(KeySize, MaximumEntries, #[serde(default)] KernelOnlyAccessPermissions, #[serde(default)] Option<NumaNode>),
	
	/// Key size is in the range `1 ..= MAX_BPF_STACK`.
	/// Value size is`size_of::<u64>()` (8).
	/// Max entries is non-zero.
	///
	/// Requires the capability `CAP_NET_ADMIN`.
	SocketHashU64(KeySize, MaximumEntries, #[serde(default)] KernelOnlyAccessPermissions, #[serde(default)] Option<NumaNode>),
	
	/// Queue.
	///
	/// Key size is unused but must be non-zero.
	/// Value size is in the range `1 ..= KMALLOC_MAX_SIZE`.
	/// Max entries is non-zero.
	///
	/// Requires the capability `CAP_SYS_ADMIN`.
	Queue(ValueSizeU32, MaximumEntries, #[serde(default)] AccessPermissions, #[serde(default)] Option<NumaNode>),
	
	/// Stack.
	///
	/// Key size is unused but must be non-zero.
	/// Value size is in the range `1 ..= KMALLOC_MAX_SIZE`.
	/// Max entries is non-zero.
	///
	/// Requires the capability `CAP_SYS_ADMIN`.
	Stack(ValueSizeU32, MaximumEntries, #[serde(default)] AccessPermissions, #[serde(default)] Option<NumaNode>),
	
	/// Key size is `size_of::<RawFd>()` (4).
	/// Value size is in the range `1 ..= MAX_VALUE_SIZE` where `MAX_VALUE_SIZE` is the smaller of either `KMALLOC_MAX_SIZE - MAX_BPF_STACK - size_of::<bpf_sk_storage_elem>()` or `u16::MAX - size_of::<bpf_sk_storage_elem>()`; in practice this means the later value, as it is much smaller.
	/// Max entries is non-zero.
	///
	/// Must have BTF key type identifier and value type identifier.
	///
	/// Requires the capability `CAP_SYS_ADMIN`.
	SocketStorage(ValueSizeU16, MaximumEntries, CloneFromListener),
}

impl<'map_of_maps_template_file_descriptor> MapType<'map_of_maps_template_file_descriptor>
{
	#[inline(always)]
	pub(crate) fn to_values(&self, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>) -> Result<(bpf_map_type, BPF_MAP_CREATE_flags, (RawFd, BpfTypeFormatTypeIdentifier, BpfTypeFormatTypeIdentifier, BpfTypeFormatTypeIdentifier), Option<NetworkInterfaceIndex>, u32, RawFd, NonZeroU32, NonZeroU32, u32), MapCreationError>
	{
		use self::bpf_map_type::*;
		use self::MapType::*;
		
		const UnusedKeySize: NonZeroU32 = new_non_zero_u32(1);
		const KeySizeOfU32: NonZeroU32 = new_non_zero_u32(size_of::<u32>() as u32);
		const KeySizeOfRawFd: NonZeroU32 = new_non_zero_u32(size_of::<RawFd>() as u32);
		const KeySizeOfAlignedU64: NonZeroU32 = new_non_zero_u32(size_of::<AlignedU64>() as u32);
		const ValueSizeOfU32: NonZeroU32 = KeySizeOfU32;
		const ValueSizeOfU64: NonZeroU32 = new_non_zero_u32(size_of::<u64>() as u32);
		const ValueSizeOfRawFd: NonZeroU32 = new_non_zero_u32(size_of::<RawFd>() as u32);
		const ValueSizeOfBpfCgroupStorageKey: NonZeroU32 = new_non_zero_u32(size_of::<bpf_cgroup_storage_key>() as u32);
		const ValueSizeOptionNetworkInterfaceIndex: NonZeroU32 = new_non_zero_u32(size_of::<Option<NetworkInterfaceIndex>>() as u32);
		const ValueSizeOfLongestPrefixMatchTrieInternetProtocolVersion4: NonZeroU32 = new_non_zero_u32(size_of::<bpf_lpm_trie_key>() as u32 + 4);
		const ValueSizeOfLongestPrefixMatchTrieInternetProtocolVersion6: NonZeroU32 = new_non_zero_u32(size_of::<bpf_lpm_trie_key>() as u32 + 16);
		const NoNumaNode: u32 = 0;
		const NoInnerMapFileDescriptor: RawFd = 0;
		
		let values = match self
		{
			&HashPerDevice(key_size, value_size, maximum_entries, access_permissions, network_interface_index) => (BPF_MAP_TYPE_HASH, access_permissions.to_map_flags(), Self::no_btf(parsed_bpf_type_format_map_data, BpfTypeFormatTypeIdentifier::Void)?, Some(network_interface_index), NoNumaNode, NoInnerMapFileDescriptor, key_size.to_non_zero_u32(), value_size.to_non_zero_u32(), maximum_entries.to_u32()),
			&HashPerHyperThread(key_size, value_size, maximum_entries, access_permissions, preallocation) => (BPF_MAP_TYPE_HASH, access_permissions.to_map_flags() | preallocation.to_map_flags(), Self::btf(parsed_bpf_type_format_map_data), None, NoNumaNode, NoInnerMapFileDescriptor, key_size.to_non_zero_u32(), value_size.to_non_zero_u32(), maximum_entries.to_u32()),
			&HashSystemWide(key_size, value_size, maximum_entries, access_permissions, None, preallocation) => (BPF_MAP_TYPE_PERCPU_HASH, access_permissions.to_map_flags() | preallocation.to_map_flags(), Self::btf(parsed_bpf_type_format_map_data), None, NoNumaNode, NoInnerMapFileDescriptor, key_size.to_non_zero_u32(), value_size.to_non_zero_u32(), maximum_entries.to_u32()),
			&HashSystemWide(key_size, value_size, maximum_entries, access_permissions, Some(numa_node), preallocation) => (BPF_MAP_TYPE_PERCPU_HASH, access_permissions.to_map_flags() | preallocation.to_map_flags() | BPF_MAP_CREATE_flags::BPF_F_NUMA_NODE, Self::btf(parsed_bpf_type_format_map_data), None, numa_node.into(), NoInnerMapFileDescriptor, key_size.to_non_zero_u32(), value_size.to_non_zero_u32(), maximum_entries.to_u32()),
			&ArrayPerDevice(value_size, maximum_entries, access_permissions, network_interface_index) => (BPF_MAP_TYPE_ARRAY, access_permissions.to_map_flags(), Self::no_btf(parsed_bpf_type_format_map_data, BpfTypeFormatTypeIdentifier::Void)?, Some(network_interface_index), NoNumaNode, NoInnerMapFileDescriptor, KeySizeOfU32, value_size.to_non_zero_u32(), maximum_entries.to_u32()),
			&ArrayPerHyperThread(value_size, maximum_entries, access_permissions) => (BPF_MAP_TYPE_PERCPU_ARRAY, access_permissions.to_map_flags(), Self::btf(parsed_bpf_type_format_map_data), None, NoNumaNode, NoInnerMapFileDescriptor, KeySizeOfU32, value_size.to_non_zero_u32(), maximum_entries.to_u32()),
			&ArraySystemWide(value_size, maximum_entries, access_permissions, None, memory_map) => (BPF_MAP_TYPE_ARRAY, access_permissions.to_map_flags() | memory_map.to_flags(), Self::btf(parsed_bpf_type_format_map_data), None, NoNumaNode, NoInnerMapFileDescriptor, KeySizeOfU32, value_size.to_non_zero_u32(), maximum_entries.to_u32()),
			&ArraySystemWide(value_size, maximum_entries, access_permissions, Some(numa_node), memory_map) => (BPF_MAP_TYPE_ARRAY, access_permissions.to_map_flags() | memory_map.to_flags() | BPF_MAP_CREATE_flags::BPF_F_NUMA_NODE, Self::btf(parsed_bpf_type_format_map_data), None, numa_node.into(), NoInnerMapFileDescriptor, KeySizeOfU32, value_size.to_non_zero_u32(), maximum_entries.to_u32()),
			&LeastRecentlyUsedHashSystemWide(key_size, value_size, maximum_entries, access_permissions, None) => (BPF_MAP_TYPE_LRU_HASH, access_permissions.to_map_flags(), Self::btf(parsed_bpf_type_format_map_data), None, NoNumaNode, NoInnerMapFileDescriptor, key_size.to_non_zero_u32(), value_size.to_non_zero_u32(), maximum_entries.to_u32()),
			&LeastRecentlyUsedHashSystemWide(key_size, value_size, maximum_entries, access_permissions, Some(numa_node)) => (BPF_MAP_TYPE_LRU_HASH, access_permissions.to_map_flags(), Self::btf(parsed_bpf_type_format_map_data), None, numa_node.into(), NoInnerMapFileDescriptor, key_size.to_non_zero_u32(), value_size.to_non_zero_u32(), maximum_entries.to_u32()),
			&LeastRecentlyUsedHashSystemWideWithAPerHyperThreadLeastRecentlyUsedList(key_size, value_size, maximum_entries, access_permissions) => (BPF_MAP_TYPE_LRU_HASH, access_permissions.to_map_flags() | BPF_MAP_CREATE_flags::BPF_F_NO_COMMON_LRU, Self::btf(parsed_bpf_type_format_map_data), None, NoNumaNode, NoInnerMapFileDescriptor, key_size.to_non_zero_u32(), value_size.to_non_zero_u32(), maximum_entries.to_u32()),
			&LeastRecentlyUsedHashPerHyperThread(key_size, value_size, maximum_entries, access_permissions) => (BPF_MAP_TYPE_LRU_PERCPU_HASH, access_permissions.to_map_flags(), Self::btf(parsed_bpf_type_format_map_data), None, NoNumaNode, NoInnerMapFileDescriptor, key_size.to_non_zero_u32(), value_size.to_non_zero_u32(), maximum_entries.to_u32()),
			&LeastRecentlyUsedHashPerHyperThreadWithAPerHyperThreadLeastRecentlyUsedList(key_size, value_size, maximum_entries, access_permissions) => (BPF_MAP_TYPE_LRU_PERCPU_HASH, access_permissions.to_map_flags() | BPF_MAP_CREATE_flags::BPF_F_NO_COMMON_LRU, Self::btf(parsed_bpf_type_format_map_data), None, NoNumaNode, NoInnerMapFileDescriptor, key_size.to_non_zero_u32(), value_size.to_non_zero_u32(), maximum_entries.to_u32()),
			&ProgramArray(maximum_entries, access_permissions) => (BPF_MAP_TYPE_PROG_ARRAY, access_permissions.to_map_flags(), Self::btf(parsed_bpf_type_format_map_data), None, NoNumaNode, NoInnerMapFileDescriptor, KeySizeOfU32, ValueSizeOfRawFd, maximum_entries.to_u32()),
			&PerfEventArray(maximum_entries, access_permissions) => (BPF_MAP_TYPE_PERF_EVENT_ARRAY, access_permissions.to_map_flags(), Self::btf(parsed_bpf_type_format_map_data), None, NoNumaNode, NoInnerMapFileDescriptor, KeySizeOfU32, ValueSizeOfRawFd, maximum_entries.to_u32()),
			&StackTraceInstructionPointerAddress(stack_depth, maximum_entries, access_permissions, None) => (BPF_MAP_TYPE_STACK_TRACE, access_permissions.to_map_flags(), Self::btf(parsed_bpf_type_format_map_data), None, NoNumaNode, NoInnerMapFileDescriptor, KeySizeOfAlignedU64, stack_depth.to_non_zero_u32::<AlignedU64>(), maximum_entries.to_u32()),
			&StackTraceInstructionPointerAddress(stack_depth, maximum_entries, access_permissions, Some(numa_node)) => (BPF_MAP_TYPE_STACK_TRACE, access_permissions.to_map_flags() | BPF_MAP_CREATE_flags::BPF_F_NUMA_NODE, Self::btf(parsed_bpf_type_format_map_data), None, numa_node.into(), NoInnerMapFileDescriptor, KeySizeOfAlignedU64, stack_depth.to_non_zero_u32::<AlignedU64>(), maximum_entries.to_u32()),
			&StackTraceBuildIdentifier(stack_depth, maximum_entries, access_permissions, None) => (BPF_MAP_TYPE_STACK_TRACE, access_permissions.to_map_flags() | BPF_MAP_CREATE_flags::BPF_F_STACK_BUILD_ID, Self::btf(parsed_bpf_type_format_map_data), None, NoNumaNode, NoInnerMapFileDescriptor, KeySizeOfAlignedU64, stack_depth.to_non_zero_u32::<bpf_stack_build_id>(), maximum_entries.to_u32()),
			&StackTraceBuildIdentifier(stack_depth, maximum_entries, access_permissions, Some(numa_node)) => (BPF_MAP_TYPE_STACK_TRACE, access_permissions.to_map_flags() | BPF_MAP_CREATE_flags::BPF_F_STACK_BUILD_ID | BPF_MAP_CREATE_flags::BPF_F_NUMA_NODE, Self::btf(parsed_bpf_type_format_map_data), None, numa_node.into(), NoInnerMapFileDescriptor, KeySizeOfAlignedU64, stack_depth.to_non_zero_u32::<bpf_stack_build_id>(), maximum_entries.to_u32()),
			&CgroupArray(maximum_entries, access_permissions) => (BPF_MAP_TYPE_CGROUP_ARRAY, access_permissions.to_map_flags(), Self::btf(parsed_bpf_type_format_map_data), None, NoNumaNode, NoInnerMapFileDescriptor, KeySizeOfU32, ValueSizeOfRawFd, maximum_entries.to_u32()),
			&ReusePortSocketArrayU32(maximum_entries, access_permissions, None) => (BPF_MAP_TYPE_REUSEPORT_SOCKARRAY, access_permissions.to_map_flags(), Self::btf(parsed_bpf_type_format_map_data), None, NoNumaNode, NoInnerMapFileDescriptor, KeySizeOfU32, ValueSizeOfU32, maximum_entries.to_u32()),
			&ReusePortSocketArrayU32(maximum_entries, access_permissions, Some(numa_node)) => (BPF_MAP_TYPE_REUSEPORT_SOCKARRAY, access_permissions.to_map_flags() | BPF_MAP_CREATE_flags::BPF_F_NUMA_NODE, Self::btf(parsed_bpf_type_format_map_data), None, numa_node.into(), NoInnerMapFileDescriptor, KeySizeOfU32, ValueSizeOfU32, maximum_entries.to_u32()),
			&ReusePortSocketArrayU64(maximum_entries, access_permissions, None) => (BPF_MAP_TYPE_REUSEPORT_SOCKARRAY, access_permissions.to_map_flags(), Self::btf(parsed_bpf_type_format_map_data), None, NoNumaNode, NoInnerMapFileDescriptor, KeySizeOfU32, ValueSizeOfU64, maximum_entries.to_u32()),
			&ReusePortSocketArrayU64(maximum_entries, access_permissions, Some(numa_node)) => (BPF_MAP_TYPE_REUSEPORT_SOCKARRAY, access_permissions.to_map_flags() | BPF_MAP_CREATE_flags::BPF_F_NUMA_NODE, Self::btf(parsed_bpf_type_format_map_data), None, numa_node.into(), NoInnerMapFileDescriptor, KeySizeOfU32, ValueSizeOfU64, maximum_entries.to_u32()),
			&ArrayOfMaps(maximum_entries, access_permissions, template_map_file_descriptor) => (BPF_MAP_TYPE_ARRAY_OF_MAPS, access_permissions.to_map_flags(), Self::btf(parsed_bpf_type_format_map_data), None, NoNumaNode, template_map_file_descriptor.as_raw_fd(), KeySizeOfU32, ValueSizeOfRawFd, maximum_entries.to_u32()),
			&HashOfMaps(key_size, maximum_entries, access_permissions, template_map_file_descriptor, None, preallocation) => (BPF_MAP_TYPE_HASH_OF_MAPS, access_permissions.to_map_flags() | preallocation.to_map_flags(), Self::btf(parsed_bpf_type_format_map_data), None, NoNumaNode, template_map_file_descriptor.as_raw_fd(), key_size.to_non_zero_u32(), ValueSizeOfRawFd, maximum_entries.to_u32()),
			&HashOfMaps(key_size, maximum_entries, access_permissions, template_map_file_descriptor, Some(numa_node), preallocation) => (BPF_MAP_TYPE_HASH_OF_MAPS, access_permissions.to_map_flags() | preallocation.to_map_flags() | BPF_MAP_CREATE_flags::BPF_F_NUMA_NODE, Self::btf(parsed_bpf_type_format_map_data), None, numa_node.into(), template_map_file_descriptor.as_raw_fd(), key_size.to_non_zero_u32(), ValueSizeOfRawFd, maximum_entries.to_u32()),
			&LongestPrefixMatchTrieInternetProtocolVersion4(value_size, maximum_entries, access_permissions, None) => (BPF_MAP_TYPE_LPM_TRIE, access_permissions.to_map_flags() | BPF_MAP_CREATE_flags::BPF_F_NO_PREALLOC, Self::btf(parsed_bpf_type_format_map_data), None, NoNumaNode, NoInnerMapFileDescriptor, ValueSizeOfLongestPrefixMatchTrieInternetProtocolVersion4, value_size.to_non_zero_u32(), maximum_entries.to_u32()),
			&LongestPrefixMatchTrieInternetProtocolVersion4(value_size, maximum_entries, access_permissions, Some(numa_node)) => (BPF_MAP_TYPE_LPM_TRIE, access_permissions.to_map_flags() | BPF_MAP_CREATE_flags::BPF_F_NO_PREALLOC | BPF_MAP_CREATE_flags::BPF_F_NUMA_NODE, Self::btf(parsed_bpf_type_format_map_data), None, numa_node.into(), NoInnerMapFileDescriptor, ValueSizeOfLongestPrefixMatchTrieInternetProtocolVersion4, value_size.to_non_zero_u32(), maximum_entries.to_u32()),
			&LongestPrefixMatchTrieInternetProtocolVersion6(value_size, maximum_entries, access_permissions, None) => (BPF_MAP_TYPE_LPM_TRIE, access_permissions.to_map_flags() | BPF_MAP_CREATE_flags::BPF_F_NO_PREALLOC, Self::btf(parsed_bpf_type_format_map_data), None, NoNumaNode, NoInnerMapFileDescriptor, ValueSizeOfLongestPrefixMatchTrieInternetProtocolVersion6, value_size.to_non_zero_u32(), maximum_entries.to_u32()),
			&LongestPrefixMatchTrieInternetProtocolVersion6(value_size, maximum_entries, access_permissions, Some(numa_node)) => (BPF_MAP_TYPE_LPM_TRIE, access_permissions.to_map_flags() | BPF_MAP_CREATE_flags::BPF_F_NO_PREALLOC | BPF_MAP_CREATE_flags::BPF_F_NUMA_NODE, Self::btf(parsed_bpf_type_format_map_data), None, numa_node.into(), NoInnerMapFileDescriptor, ValueSizeOfLongestPrefixMatchTrieInternetProtocolVersion6, value_size.to_non_zero_u32(), maximum_entries.to_u32()),
			&CgroupStoragePerHyperThread(value_size, access_permissions, None) => (BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE, access_permissions.to_map_flags(), Self::btf(parsed_bpf_type_format_map_data), None, NoNumaNode, NoInnerMapFileDescriptor, ValueSizeOfBpfCgroupStorageKey, value_size.to_non_zero_u32(), 0),
			&CgroupStoragePerHyperThread(value_size, access_permissions, Some(numa_node)) => (BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE, access_permissions.to_map_flags() | BPF_MAP_CREATE_flags::BPF_F_NUMA_NODE, Self::btf(parsed_bpf_type_format_map_data), None, numa_node.into(), NoInnerMapFileDescriptor, ValueSizeOfBpfCgroupStorageKey, value_size.to_non_zero_u32(), 0),
			&CgroupStorageSystemWide(value_size, access_permissions, None) => (BPF_MAP_TYPE_CGROUP_STORAGE, access_permissions.to_map_flags(), Self::btf(parsed_bpf_type_format_map_data), None, NoNumaNode, NoInnerMapFileDescriptor, ValueSizeOfBpfCgroupStorageKey, value_size.to_non_zero_u32(), 0),
			&CgroupStorageSystemWide(value_size, access_permissions, Some(numa_node)) => (BPF_MAP_TYPE_CGROUP_STORAGE, access_permissions.to_map_flags() | BPF_MAP_CREATE_flags::BPF_F_NUMA_NODE, Self::btf(parsed_bpf_type_format_map_data), None, numa_node.into(), NoInnerMapFileDescriptor, ValueSizeOfBpfCgroupStorageKey, value_size.to_non_zero_u32(), 0),
			&StructOps(value_size, vmlinux_value_type_identifier) => (BPF_MAP_TYPE_STRUCT_OPS, BPF_MAP_CREATE_flags::empty(), Self::no_btf(parsed_bpf_type_format_map_data, BpfTypeFormatTypeIdentifier::new(vmlinux_value_type_identifier))?, None, NoNumaNode, NoInnerMapFileDescriptor, KeySizeOfU32, value_size.to_non_zero_u32(), 1),
			&ExpressDataPathRedirectToNetworkDeviceArray(maximum_entries, access_permissions, None) => (BPF_MAP_TYPE_DEVMAP, access_permissions.to_map_flags(), Self::btf(parsed_bpf_type_format_map_data), None, NoNumaNode, NoInnerMapFileDescriptor, KeySizeOfU32, ValueSizeOptionNetworkInterfaceIndex, maximum_entries.to_u32()),
			&ExpressDataPathRedirectToNetworkDeviceArray(maximum_entries, access_permissions, Some(numa_node)) => (BPF_MAP_TYPE_DEVMAP, access_permissions.to_map_flags() | BPF_MAP_CREATE_flags::BPF_F_NUMA_NODE, Self::btf(parsed_bpf_type_format_map_data), None, numa_node.into(), NoInnerMapFileDescriptor, KeySizeOfU32, ValueSizeOptionNetworkInterfaceIndex, maximum_entries.to_u32()),
			&ExpressDataPathRedirectToNetworkDeviceHash(maximum_entries, access_permissions, None) => (BPF_MAP_TYPE_DEVMAP_HASH, access_permissions.to_map_flags(), Self::btf(parsed_bpf_type_format_map_data), None, NoNumaNode, NoInnerMapFileDescriptor, KeySizeOfU32, ValueSizeOptionNetworkInterfaceIndex, maximum_entries.to_u32()),
			&ExpressDataPathRedirectToNetworkDeviceHash(maximum_entries, access_permissions, Some(numa_node)) => (BPF_MAP_TYPE_DEVMAP_HASH, access_permissions.to_map_flags() | BPF_MAP_CREATE_flags::BPF_F_NUMA_NODE, Self::btf(parsed_bpf_type_format_map_data), None, numa_node.into(), NoInnerMapFileDescriptor, KeySizeOfU32, ValueSizeOptionNetworkInterfaceIndex, maximum_entries.to_u32()),
			&ExpressDataPathRedirectToHyperThreadArray(maximum_entries, None) => (BPF_MAP_TYPE_CPUMAP, BPF_MAP_CREATE_flags::empty(), Self::btf(parsed_bpf_type_format_map_data), None, NoNumaNode, NoInnerMapFileDescriptor, KeySizeOfU32, ValueSizeOfU32, maximum_entries.to_u32()),
			&ExpressDataPathRedirectToHyperThreadArray(maximum_entries, Some(numa_node)) => (BPF_MAP_TYPE_CPUMAP, BPF_MAP_CREATE_flags::BPF_F_NUMA_NODE, Self::btf(parsed_bpf_type_format_map_data), None, numa_node.into(), NoInnerMapFileDescriptor, KeySizeOfU32, ValueSizeOfU32, maximum_entries.to_u32()),
			&ExpressDataPathRedirectToSocketArray(maximum_entries, access_permissions, None) => (BPF_MAP_TYPE_XSKMAP, access_permissions.to_map_flags(), Self::btf(parsed_bpf_type_format_map_data), None, NoNumaNode, NoInnerMapFileDescriptor, KeySizeOfU32, ValueSizeOfRawFd, maximum_entries.to_u32()),
			&ExpressDataPathRedirectToSocketArray(maximum_entries, access_permissions, Some(numa_node)) => (BPF_MAP_TYPE_XSKMAP, access_permissions.to_map_flags() | BPF_MAP_CREATE_flags::BPF_F_NUMA_NODE, Self::btf(parsed_bpf_type_format_map_data), None, numa_node.into(), NoInnerMapFileDescriptor, KeySizeOfU32, ValueSizeOfRawFd, maximum_entries.to_u32()),
			&SocketArrayU32(maximum_entries, access_permissions, None) => (BPF_MAP_TYPE_SOCKMAP, access_permissions.to_map_flags(), Self::btf(parsed_bpf_type_format_map_data), None, NoNumaNode, NoInnerMapFileDescriptor, KeySizeOfU32, ValueSizeOfU32, maximum_entries.to_u32()),
			&SocketArrayU32(maximum_entries, access_permissions, Some(numa_node)) => (BPF_MAP_TYPE_SOCKMAP, access_permissions.to_map_flags() | BPF_MAP_CREATE_flags::BPF_F_NUMA_NODE, Self::btf(parsed_bpf_type_format_map_data), None, numa_node.into(), NoInnerMapFileDescriptor, KeySizeOfU32, ValueSizeOfU32, maximum_entries.to_u32()),
			&SocketArrayU64(maximum_entries, access_permissions, None) => (BPF_MAP_TYPE_SOCKMAP, access_permissions.to_map_flags(), Self::btf(parsed_bpf_type_format_map_data), None, NoNumaNode, NoInnerMapFileDescriptor, KeySizeOfU32, ValueSizeOfU64, maximum_entries.to_u32()),
			&SocketArrayU64(maximum_entries, access_permissions, Some(numa_node)) => (BPF_MAP_TYPE_SOCKMAP, access_permissions.to_map_flags() | BPF_MAP_CREATE_flags::BPF_F_NUMA_NODE, Self::btf(parsed_bpf_type_format_map_data), None, numa_node.into(), NoInnerMapFileDescriptor, KeySizeOfU32, ValueSizeOfU64, maximum_entries.to_u32()),
			&SocketHashU32(key_size, maximum_entries, access_permissions, None) => (BPF_MAP_TYPE_SOCKHASH, access_permissions.to_map_flags(), Self::btf(parsed_bpf_type_format_map_data), None, NoNumaNode, NoInnerMapFileDescriptor, key_size.to_non_zero_u32(), ValueSizeOfU32, maximum_entries.to_u32()),
			&SocketHashU32(key_size, maximum_entries, access_permissions, Some(numa_node)) => (BPF_MAP_TYPE_SOCKHASH, access_permissions.to_map_flags() | BPF_MAP_CREATE_flags::BPF_F_NUMA_NODE, Self::btf(parsed_bpf_type_format_map_data), None, numa_node.into(), NoInnerMapFileDescriptor, key_size.to_non_zero_u32(), ValueSizeOfU32, maximum_entries.to_u32()),
			&SocketHashU64(key_size, maximum_entries, access_permissions, None) => (BPF_MAP_TYPE_SOCKHASH, access_permissions.to_map_flags(), Self::btf(parsed_bpf_type_format_map_data), None, NoNumaNode, NoInnerMapFileDescriptor, key_size.to_non_zero_u32(), ValueSizeOfU64, maximum_entries.to_u32()),
			&SocketHashU64(key_size, maximum_entries, access_permissions, Some(numa_node)) => (BPF_MAP_TYPE_SOCKHASH, access_permissions.to_map_flags() | BPF_MAP_CREATE_flags::BPF_F_NUMA_NODE, Self::btf(parsed_bpf_type_format_map_data), None, numa_node.into(), NoInnerMapFileDescriptor, key_size.to_non_zero_u32(), ValueSizeOfU64, maximum_entries.to_u32()),
			&Queue(value_size, maximum_entries, access_permissions, None) => (BPF_MAP_TYPE_QUEUE, access_permissions.to_map_flags(), Self::btf(parsed_bpf_type_format_map_data), None, NoNumaNode, NoInnerMapFileDescriptor, UnusedKeySize, value_size.to_non_zero_u32(), maximum_entries.to_u32()),
			&Queue(value_size, maximum_entries, access_permissions, Some(numa_node)) => (BPF_MAP_TYPE_QUEUE, access_permissions.to_map_flags() | BPF_MAP_CREATE_flags::BPF_F_NUMA_NODE, Self::btf(parsed_bpf_type_format_map_data), None, numa_node.into(), NoInnerMapFileDescriptor, UnusedKeySize, value_size.to_non_zero_u32(), maximum_entries.to_u32()),
			&Stack(value_size, maximum_entries, access_permissions, None) => (BPF_MAP_TYPE_STACK, access_permissions.to_map_flags(), Self::btf(parsed_bpf_type_format_map_data), None, NoNumaNode, NoInnerMapFileDescriptor, UnusedKeySize, value_size.to_non_zero_u32(), maximum_entries.to_u32()),
			&Stack(value_size, maximum_entries, access_permissions, Some(numa_node)) => (BPF_MAP_TYPE_REUSEPORT_SOCKARRAY, access_permissions.to_map_flags() | BPF_MAP_CREATE_flags::BPF_F_NUMA_NODE, Self::btf(parsed_bpf_type_format_map_data), None, numa_node.into(), NoInnerMapFileDescriptor, UnusedKeySize, value_size.to_non_zero_u32(), maximum_entries.to_u32()),
			&SocketStorage(value_size, maximum_entries, false) => (BPF_MAP_TYPE_SK_STORAGE, BPF_MAP_CREATE_flags::BPF_F_NO_PREALLOC, Self::mandatory_btf(parsed_bpf_type_format_map_data)?, None, NoNumaNode, NoInnerMapFileDescriptor, KeySizeOfRawFd, value_size.to_non_zero_u32(), maximum_entries.to_u32()),
			&SocketStorage(value_size, maximum_entries, true) => (BPF_MAP_TYPE_SK_STORAGE, BPF_MAP_CREATE_flags::BPF_F_NO_PREALLOC | BPF_MAP_CREATE_flags::BPF_F_CLONE, Self::mandatory_btf(parsed_bpf_type_format_map_data)?, None, NoNumaNode, NoInnerMapFileDescriptor, KeySizeOfRawFd, value_size.to_non_zero_u32(), maximum_entries.to_u32()),
		};
		Ok(values)
	}
	
	#[inline(always)]
	fn btf(parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>) -> (RawFd, BpfTypeFormatTypeIdentifier, BpfTypeFormatTypeIdentifier, BpfTypeFormatTypeIdentifier)
	{
		match parsed_bpf_type_format_map_data
		{
			None => ((ParsedBpfTypeFormatData::NoBpfTypeFormatFileDescriptor, BpfTypeFormatTypeIdentifier::Void, BpfTypeFormatTypeIdentifier::Void, BpfTypeFormatTypeIdentifier::Void)),
			
			Some
			(
				&ParsedBpfTypeFormatMapData
				{
					data,
					bpf_type_format_key_value_type_identifiers: BpfTypeFormatKeyValueTypeIdentifiers
					{
						key_type_identifier,
						value_type_identifier
					}
				}
			) => (data.to_raw_file_descriptor(), BpfTypeFormatTypeIdentifier::new(key_type_identifier), BpfTypeFormatTypeIdentifier::new(value_type_identifier), BpfTypeFormatTypeIdentifier::Void),
		}
		
	}
	
	#[inline(always)]
	fn no_btf(parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, vmlinux_value_type_identifier: BpfTypeFormatTypeIdentifier) -> Result<(RawFd, BpfTypeFormatTypeIdentifier, BpfTypeFormatTypeIdentifier, BpfTypeFormatTypeIdentifier), MapCreationError>
	{
		if parsed_bpf_type_format_map_data.is_some()
		{
			Err(MapCreationError::HashByDeviceArrayByDeviceAndStructOpsMandatesThatThereAreNotKeyOrValueTypeIdentifiers)
		}
		else
		{
			Ok((ParsedBpfTypeFormatData::NoBpfTypeFormatFileDescriptor, BpfTypeFormatTypeIdentifier::Void, BpfTypeFormatTypeIdentifier::Void, vmlinux_value_type_identifier))
		}
	}
	
	#[inline(always)]
	fn mandatory_btf(parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>) -> Result<(RawFd, BpfTypeFormatTypeIdentifier, BpfTypeFormatTypeIdentifier, BpfTypeFormatTypeIdentifier), MapCreationError>
	{
		match parsed_bpf_type_format_map_data
		{
			None => Err(MapCreationError::SocketStorageMandatesBpfTypeFormatTypeIdentifiersForKeyAndValue),
			
			Some
			(
				&ParsedBpfTypeFormatMapData
				{
					data,
					bpf_type_format_key_value_type_identifiers: BpfTypeFormatKeyValueTypeIdentifiers
					{
						key_type_identifier,
						value_type_identifier
					}
				}
			) => Ok((data.to_raw_file_descriptor(), BpfTypeFormatTypeIdentifier::new(key_type_identifier), BpfTypeFormatTypeIdentifier::new(value_type_identifier), BpfTypeFormatTypeIdentifier::Void))
		}
	}
}
