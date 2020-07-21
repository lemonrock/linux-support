// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An array map specialized for use with XDP.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct XdpRedirectHyperThreadArrayMap
{
	map_file_descriptor: Rc<MapFileDescriptor>,
	maximum_entries: MaximumEntries,
}

impl CanBeInnerMap for XdpRedirectHyperThreadArrayMap
{
	#[inline(always)]
	fn map_file_descriptor(&self) -> &MapFileDescriptor
	{
		&self.map_file_descriptor
	}
}

impl XdpRedirectHyperThreadArrayMap
{
	/// New u32 array.
	///
	/// Needs the capability `CAP_SYS_ADMIN`.
	#[inline(always)]
	pub fn new_xdp_redirect_hyper_thread_array(map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_btf_map_data: Option<&ParsedBtfMapData>, maximum_entries: MaximumEntries, numa_node: Option<NumaNode>) -> Result<Self, MapCreationError>
	{
		Self::create(map_file_descriptors, map_name, parsed_btf_map_data, MapType::XdpRedirectToHyperThreadArray(maximum_entries, numa_node), maximum_entries)
	}
	
	/// Length.
	#[inline(always)]
	pub fn capacity(&self) -> NonZeroU32
	{
		self.maximum_entries.0
	}
	
	/// Freeze.
	#[inline(always)]
	pub fn freeze(&self) -> Result<(), Errno>
	{
		self.map_file_descriptor.freeze()
	}
	
	/// Indices.
	#[inline(always)]
	pub fn indices(&self) -> RangeInclusive<u32>
	{
		0 ..= self.maximum_entries.0.get()
	}
	
	/// Gets value.
	#[inline(always)]
	pub fn get(&self, hyper_thread: HyperThread) -> QueueDepth
	{
		let index: u32 = hyper_thread.into();
		
		self.guard_index(index);
		
		let value: u32 = self.map_file_descriptor.get(&index).expect("index should always be present");
		QueueDepth::try_from(value).unwrap()
	}
	
	/// Insert or update existing.
	///
	/// (A `queue_depth` of `0` is supported; it is equivalent to `self.delete()`).
	pub fn insert_or_set(&self, hyper_thread: HyperThread, queue_depth: QueueDepth) -> Result<(), ()>
	{
		let index: u32 = hyper_thread.into();
		
		self.guard_index(index);
		
		let queue_depth: u32 = queue_depth.into();
		
		self.map_file_descriptor.insert_or_set(&index, &queue_depth, LockFlags::DoNotLock)
	}
	
	/// Insert.
	///
	/// (A `queue_depth` of `0` is supported; it is equivalent to `self.delete()`).
	pub fn set(&self, hyper_thread: HyperThread, queue_depth: QueueDepth) -> Result<(), ()>
	{
		let index: u32 = hyper_thread.into();
		
		self.guard_index(index);
		
		let queue_depth: u32 = queue_depth.into();
		
		self.map_file_descriptor.set(&index, &queue_depth, LockFlags::DoNotLock)
	}
	
	/// Removes a queue (sets its queue depth to zero).
	#[inline(always)]
	pub fn delete(&self, hyper_thread: HyperThread) -> Result<bool, Errno>
	{
		let index: u32 = hyper_thread.into();
		
		self.map_file_descriptor.delete(&index)
	}
	
	#[inline(always)]
	fn guard_index(&self, index: u32)
	{
		debug_assert!(index < self.maximum_entries.to_u32());
	}
	
	#[inline(always)]
	fn create(map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_name: &MapName, parsed_btf_map_data: Option<&ParsedBtfMapData>, map_type: MapType, maximum_entries: MaximumEntries) -> Result<Self, MapCreationError>
	{
		MapFileDescriptor::create(map_file_descriptors, map_type, map_name, parsed_btf_map_data).map(|map_file_descriptor| Self::new(map_file_descriptor, maximum_entries))
	}
	
	#[inline(always)]
	const fn new(map_file_descriptor: Rc<MapFileDescriptor>, maximum_entries: MaximumEntries) -> Self
	{
		Self
		{
			map_file_descriptor,
			maximum_entries,
		}
	}
}
