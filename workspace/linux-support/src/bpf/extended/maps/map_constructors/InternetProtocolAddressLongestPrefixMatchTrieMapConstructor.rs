// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Support trait implemented only for `in_addr` and `in6_addr`.
pub trait InternetProtocolAddressLongestPrefixMatchTrieMapConstructor<V: Copy>: InternetProtocolAddress
{
	#[doc(hidden)]
	fn new_longest_prefix_match_trie_map(map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_btf_map_data: Option<&ParsedBtfMapData>, maximum_entries: MaximumEntries, access_permissions: AccessPermissions, numa_node: Option<NumaNode>) -> Result<LongestPrefixMatchTrieMap<Self, V>, MapCreationError>;
}

impl<V: Copy> InternetProtocolAddressLongestPrefixMatchTrieMapConstructor<V> for in_addr
{
	#[inline(always)]
	fn new_longest_prefix_match_trie_map(map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_btf_map_data: Option<&ParsedBtfMapData>, maximum_entries: MaximumEntries, access_permissions: AccessPermissions, numa_node: Option<NumaNode>) -> Result<LongestPrefixMatchTrieMap<in_addr, V>, MapCreationError>
	{
		LongestPrefixMatchTrieMap::new_internet_protocol_version_4(map_file_descriptors, map_name, parsed_btf_map_data, maximum_entries, access_permissions, numa_node)
	}
}

impl<V: Copy> InternetProtocolAddressLongestPrefixMatchTrieMapConstructor<V> for in6_addr
{
	#[inline(always)]
	fn new_longest_prefix_match_trie_map(map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_btf_map_data: Option<&ParsedBtfMapData>, maximum_entries: MaximumEntries, access_permissions: AccessPermissions, numa_node: Option<NumaNode>) -> Result<LongestPrefixMatchTrieMap<in6_addr, V>, MapCreationError>
	{
		LongestPrefixMatchTrieMap::new_internet_protocol_version_6(map_file_descriptors, map_name, parsed_btf_map_data, maximum_entries, access_permissions, numa_node)
	}
}
