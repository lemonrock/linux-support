// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// When an array is created, all its elements are zeroed.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpinLockableArrayMap<'map_file_descriptor_label_map, V: Sized + HasReflectionInformation>(ArrayMap<'map_file_descriptor_label_map, SpinLockableValue<V>>);

impl<'map_file_descriptor_label_map, V: Sized + HasReflectionInformation> SpinLockableArrayMap<'map_file_descriptor_label_map, V>
{
	/// Length.
	#[inline(always)]
	pub fn length(&self) -> NonZeroU32
	{
		self.0.length()
	}
	
	/// Gets the next index (key).
	///
	/// Returns `None` if the `index` is the last one (ie `length() - 1`).
	///
	/// Does not make a syscall.
	#[inline(always)]
	pub fn get_next_index(&self, index: u32) -> Option<u32>
	{
		if unlikely!(index >= self.maximum_entries.get())
		{
			Some(0)
		}
		else if unlikely!(index == self.maximum_entries.get() - 1)
		{
			None
		}
		else
		{
			Some(index + 1)
		}
	}
	
	/// Looks up an index; should always succeed.
	pub fn get(&self, index: u32) -> V
	{
		let spin_lockable_value = self.0.get(index);
		spin_lockable_value.value
	}
	
	/// Update existing.
	#[allow(deprecated)]
	pub fn set(&self, index: u32, value: V)
	{
		self.0.set_locked(index, SpinLockableValue { spin_lock: unsafe { uninitialized() }, value })
	}
}
