// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A file descriptor for a BPF map.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MapFileDescriptor(RawFd);

impl Drop for MapFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.0.close()
	}
}

impl AsRawFd for MapFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl IntoRawFd for MapFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.0
	}
}

impl FromRawFd for MapFileDescriptor
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(fd)
	}
}

impl FileDescriptor for MapFileDescriptor
{
}

impl BpfFileDescriptor for MapFileDescriptor
{
	type Information = bpf_map_info;
}

impl MemoryMappableFileDescriptor for MapFileDescriptor
{
}

impl UsedAsValueInArrayMapDescriptor for MapFileDescriptor
{
	#[inline(always)]
	fn transmute_from_file_descriptor_copies(values: &[Self]) -> &[RawFd]
	{
		unsafe { transmute(values) }
	}
}

impl ProvidesIdentifierWhenUsedAsValueInArrayMapDescriptor for MapFileDescriptor
{
	type Identifier = MapIdentifier;
}

impl MapFileDescriptor
{
	/// Gets the next key.
	///
	/// Returns `None` if the `key` is the last one (ie `capacity() - 1`).
	#[inline(always)]
	#[allow(deprecated)]
	pub fn get_next_key<K: Sized>(&self, key: *const K) -> Result<Option<K>, Errno>
	{
		debug_assert_ne!(size_of::<K>(), 0);
		
		let mut next_key = unsafe { uninitialized() };
		
		let mut attr = bpf_attr::default();
		attr.map_change = BpfCommandMapChange
		{
			map_fd: self.as_raw_fd(),
			key: AlignedU64::from(key),
			value_or_next_key: BpfCommandMapChangeValueOrNextKey
			{
				next_key: AlignedU64::from(&mut next_key),
			},
			flags: BPF_MAP_UPDATE_ELEM_flags::BPF_ANY,
		};
		
		let result = attr.syscall(bpf_cmd::BPF_MAP_GET_NEXT_KEY);
		if likely!(result == 0)
		{
			Ok(Some(next_key))
		}
		else if likely!(result == -1)
		{
			let errno = errno();
			match errno.0
			{
				// "If key is the last element, -1 is returned and errno is set to ENOENT'.
				ENOENT => Ok(None),
				
				// "Other possible errno values are ENOMEM, EFAULT, EPERM, and EINVAL".
				_ => Err(errno)
			}
		}
		else
		{
			unreachable!("Unexpected result `{}` from bpf(BPF_MAP_GET_NEXT_KEY)", result)
		}
	}
	
	#[allow(deprecated)]
	#[inline(always)]
	pub(crate) fn get<K: Sized, V: Sized>(&self, key: &K) -> Option<V>
	{
		debug_assert_ne!(size_of::<K>(), 0);
		debug_assert_ne!(size_of::<V>(), 0);
		
		let mut value: V = unsafe { uninitialized() };
		let found = self.get_internal(key, unsafe { NonNull::new_unchecked(&mut value) });
		if found
		{
			Some(value)
		}
		else
		{
			None
		}
	}
	
	#[inline(always)]
	pub(crate) fn get_variably_sized<K: Sized, V>(&self, key: &K, length: usize) -> Option<Vec<V>>
	{
		debug_assert_ne!(size_of::<K>(), 0);
		
		let mut value: Vec<V> = Vec::with_capacity(length);
		
		let found = self.get_internal(key, unsafe { NonNull::new_unchecked(value.as_mut_ptr()) });
		if found
		{
			unsafe { value.set_len(length) };
			Some(value)
		}
		else
		{
			None
		}
	}
	
	#[inline(always)]
	fn get_internal<K: Sized, V>(&self, key: &K, value: NonNull<V>) -> bool
	{
		debug_assert_ne!(size_of::<K>(), 0);
		
		let mut attr = bpf_attr::default();
		attr.map_change = BpfCommandMapChange
		{
			map_fd: self.as_raw_fd(),
			key: AlignedU64::from(key),
			value_or_next_key: BpfCommandMapChangeValueOrNextKey
			{
				value: AlignedU64::from(value),
			},
			flags: BPF_MAP_UPDATE_ELEM_flags::BPF_ANY,
		};
		
		let result = attr.syscall(bpf_cmd::BPF_MAP_LOOKUP_ELEM);
		if likely!(result == 0)
		{
			true
		}
		else if likely!(result == -1)
		{
			let errno = errno();
			match errno.0
			{
				ENOENT => false,
				ENOTSUPP | EOPNOTSUPP => panic!("Operation is unsupported"),
				_ => panic!("Unexpected error {}", errno)
			}
		}
		else
		{
			unreachable!("Unexpected result `{}` from bpf(BPF_MAP_LOOKUP_ELEM)", result)
		}
	}
	
	/// Insert or set.
	///
	/// Errors if already present.
	pub(crate) fn insert_or_set<K: Sized, V: Sized>(&self, key: &K, value: &V, lock_flags: LockFlags) -> Result<(), ()>
	{
		match self.update(key, value, BPF_MAP_UPDATE_ELEM_flags::BPF_ANY, lock_flags)
		{
			Ok(()) => Ok(()),
			
			Err(errno) => match errno.0
			{
				E2BIG => panic!("MaximumCapacityReached"),
				
				EEXIST => unreachable!("Should not return `EEXIST` as flag `BPF_NOEXIST` not specified"),
				
				ENOENT => unreachable!("Should not return `ENOENT` as flag `BPF_EXIST` not specified"),
				
				_ => panic!("Unexpected error `{}`", errno),
			}
		}
	}
	
	/// Insert.
	///
	/// Errors if already present.
	pub(crate) fn insert<K: Sized, V: Sized>(&self, key: &K, value: &V, lock_flags: LockFlags) -> Result<(), InsertError>
	{
		use self::InsertError::*;
		
		match self.update(key, value, BPF_MAP_UPDATE_ELEM_flags::BPF_NOEXIST, lock_flags)
		{
			Ok(()) => Ok(()),
			
			Err(errno) => match errno.0
			{
				E2BIG => Err(MaximumCapacityReached),
				
				EEXIST => Err(AlreadyPresent),
				
				ENOENT => unreachable!("Should not return `ENOENT` as flag `BPF_EXIST` not specified"),
				
				_ => panic!("Unexpected error `{}`", errno),
			}
		}
	}
	
	/// Set.
	///
	/// For some weird reason, one needs to specify different `map_flags`.
	///
	/// See the Linux kernel function `bpf_fd_array_map_update_elem()`.
	pub(crate) fn set_for_file_descriptor_array_map<K: Sized, V: Sized>(&self, key: &K, value: &V) -> Result<(), ()>
	{
		match self.update(key, value, BPF_MAP_UPDATE_ELEM_flags::BPF_ANY, LockFlags::DoNotLock)
		{
			Ok(()) => Ok(()),
			
			Err(errno) => match errno.0
			{
				E2BIG => Err(()),
				
				EEXIST => unreachable!("Should not return `EEXIST` as flag `BPF_NOEXIST` not specified"),
				
				ENOENT => unreachable!("Should not return `ENOENT` as flag `BPF_EXIST` not specified"),
				
				_ => panic!("Unexpected error `{}`", errno),
			}
		}
	}
	
	/// Set.
	///
	/// Returns an error if the key does not exist.
	///
	/// If specifying `BPF_MAP_UPDATE_ELEM_flags::BPF_F_LOCK`:-
	///
	/// * Only valid for basic hash, basic array and cgroup local-storage when `V` has been created with a `bpf_spin_lock`.
	/// * This will panic if this array was not created with a spinlock; it is not valid on per-CPU and per-device arrays nor if the array has been memory-mapped.
	/// * Additionally, the map needs to have been created with BTF data.
	/// * Furthermore, `V` must be a struct of type `SpinLockableValue`.
	pub(crate) fn set<K: Sized, V: Sized>(&self, key: &K, value: &V, lock_flags: LockFlags) -> Result<(), ()>
	{
		match self.update(key, value, BPF_MAP_UPDATE_ELEM_flags::BPF_EXIST, lock_flags)
		{
			Ok(()) => Ok(()),
			
			Err(errno) => match errno.0
			{
				E2BIG => unreachable!("Should not return `E2BIG` as flag `BPF_NOEXIST` or `BPF_ANY` not specified"),
				
				EEXIST => unreachable!("Should not return `EEXIST` as flag `BPF_NOEXIST` not specified"),
				
				ENOENT => Err(()),
				
				_ => panic!("Unexpected error `{}`", errno),
			}
		}
	}
	
	#[inline(always)]
	fn update<K: Sized, V: Sized>(&self, key: &K, value: &V, flags: BPF_MAP_UPDATE_ELEM_flags, lock_flags: LockFlags) -> Result<(), Errno>
	{
		debug_assert_ne!(size_of::<K>(), 0);
		debug_assert_ne!(size_of::<V>(), 0);
		
		let mut attr = bpf_attr::default();
		attr.map_change = BpfCommandMapChange
		{
			map_fd: self.as_raw_fd(),
			key: AlignedU64::from(key),
			value_or_next_key: BpfCommandMapChangeValueOrNextKey
			{
				value: AlignedU64::from(value),
			},
			flags: flags | lock_flags.to_update_flags(),
		};
		
		let result = attr.syscall(bpf_cmd::BPF_MAP_UPDATE_ELEM);
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			Err(errno())
		}
		else
		{
			unreachable!("Unexpected result `{}` from bpf(BPF_MAP_UPDATE)", result)
		}
	}
	
	#[allow(deprecated)]
	#[inline(always)]
	pub(crate) fn lookup_and_delete<K: Sized, V: Sized>(&self, key: &K) -> Result<Option<V>, Errno>
	{
		debug_assert_ne!(size_of::<K>(), 0);
		debug_assert_ne!(size_of::<V>(), 0);
		
		let mut value: V = unsafe { uninitialized() };
		
		let mut attr = bpf_attr::default();
		attr.map_change = BpfCommandMapChange
		{
			map_fd: self.as_raw_fd(),
			key: AlignedU64::from(key),
			value_or_next_key: BpfCommandMapChangeValueOrNextKey
			{
				value: AlignedU64::from(&mut value),
			},
			flags: BPF_MAP_UPDATE_ELEM_flags::empty(),
		};
		
		let result = attr.syscall(bpf_cmd::BPF_MAP_LOOKUP_AND_DELETE_ELEM);
		if likely!(result == 0)
		{
			Ok(Some(value))
		}
		else if likely!(result == -1)
		{
			let errno = errno();
			match errno.0
			{
				ENOENT => Ok(None),
				_ => Err(errno)
			}
		}
		else
		{
			unreachable!("Unexpected result `{}` from bpf(BPF_MAP_LOOKUP_AND_DELETE_ELEM)", result)
		}
	}
	
	/// Returns `Ok(true)` if `key` was present.
	#[inline(always)]
	#[allow(deprecated)]
	pub(crate) fn delete<K: Sized>(&self, key: &K) -> Result<bool, Errno>
	{
		debug_assert_ne!(size_of::<K>(), 0);
		
		let mut attr = bpf_attr::default();
		attr.map_change = BpfCommandMapChange
		{
			map_fd: self.as_raw_fd(),
			key: AlignedU64::from(key),
			value_or_next_key: BpfCommandMapChangeValueOrNextKey
			{
				value: AlignedU64::Null,
			},
			flags: BPF_MAP_UPDATE_ELEM_flags::empty(),
		};
		
		let result = attr.syscall(bpf_cmd::BPF_MAP_DELETE_ELEM);
		if likely!(result == 0)
		{
			Ok(true)
		}
		else if likely!(result == -1)
		{
			let errno = errno();
			match errno.0
			{
				ENOENT => Ok(false),
				_ => Err(errno)
			}
		}
		else
		{
			unreachable!("Unexpected result `{}` from bpf(BPF_MAP_DELETE_ELEM)", result)
		}
	}
	
	/// The map can no longer be updated from user space.
	///
	/// Not supported by `BPF_MAP_TYPE_STRUCT_OPS`.
	///
	/// Requires the capability `CAP_SYS_ADMIN`.
	/// Can only be called once.
	#[inline(always)]
	#[allow(deprecated)]
	pub(crate) fn freeze(&self) -> Result<(), Errno>
	{
		let mut attr = bpf_attr::default();
		attr.map_change = BpfCommandMapChange
		{
			map_fd: self.as_raw_fd(),
			key: AlignedU64::Null,
			value_or_next_key: BpfCommandMapChangeValueOrNextKey
			{
				value: AlignedU64::Null,
			},
			flags: BPF_MAP_UPDATE_ELEM_flags::empty(),
		};
		
		let result = attr.syscall(bpf_cmd::BPF_MAP_FREEZE);
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EINVAL => panic!("Invalid attr"),
				ENOTSUPP => panic!("Not supported for maps of type `BPF_MAP_TYPE_STRUCT_OPS`"),
				EBUSY => panic!("Already frozen"),
				EPERM => panic!("Permission denied"),
				errno @ _ => panic!("Unexpected error `{}`", errno),
			}
		}
		else
		{
			unreachable!("Unexpected result `{}` from bpf(BPF_MAP_FREEZE)", result)
		}
	}
	
	/// Returns `(values_filled, start of next batch position, more_entries_to_return)`.
	/// `values_filled.len()` may be less than `keys.len()`.
	#[allow(deprecated)]
	pub(crate) fn get_batch<K: Sized, V: Sized>(&self, batch_position: Option<&OpaqueBatchPosition<K>>, keys: &[K]) -> Result<(Vec<V>, OpaqueBatchPosition<K>, bool), Errno>
	{
		let keys_length = keys.len();
		debug_assert!(keys_length <= u32::MAX as usize);
		
		let mut out_batch: OpaqueBatchPosition<K> = unsafe { uninitialized() };
		let mut values = Vec::with_capacity(keys_length);
		
		let mut attr = bpf_attr::default();
		attr.batch = BpfCommandMapBatch
		{
			in_batch: match batch_position
			{
				None => AlignedU64::Null,
				Some(batch_position) => AlignedU64::from(batch_position),
			},
			out_batch: AlignedU64::from(&mut out_batch),
			keys: AlignedU64::from(keys),
			values: AlignedU64::from(values.as_mut_ptr()),
			count: keys_length as u32,
			map_fd: self.as_raw_fd(),
			elem_flags: elem_flags::empty(),
			flags: 0,
		};
		
		let result = attr.syscall(bpf_cmd::BPF_MAP_LOOKUP_BATCH);
		let count = unsafe { attr.batch.count };
		if likely!(result == 0)
		{
			unsafe { values.set_len(count as usize) };
			Ok((values, out_batch, true))
		}
		else if likely!(result == -1)
		{
			let errno = errno();
			match errno.0
			{
				ENOENT =>
				{
					unsafe { values.set_len(count as usize) };
					Ok((values, out_batch, false))
				},
				
				_ => Err(errno),
			}
		}
		else
		{
			unreachable!("Unexpected result `{}` from bpf(BPF_MAP_LOOKUP_BATCH)", result)
		}
	}
	
	/// `keys` and `values` must be the same length.
	pub(crate) fn set_batch<K: Sized, V: Sized>(&self, keys: &[K], values: &[V], lock_flags: LockFlags) -> Result<usize, Errno>
	{
		let keys_length = keys.len();
		let values_length = values.len();
		debug_assert!(keys_length == values_length);
		debug_assert!(keys_length <= u32::MAX as usize);
		
		let mut attr = bpf_attr::default();
		attr.batch = BpfCommandMapBatch
		{
			in_batch: AlignedU64::Null,
			out_batch: AlignedU64::Null,
			keys: AlignedU64::from(keys),
			values: AlignedU64::from(values),
			count: keys_length as u32,
			map_fd: self.as_raw_fd(),
			elem_flags: lock_flags.to_elem_flags(),
			flags: 0,
		};
		
		let result = attr.syscall(bpf_cmd::BPF_MAP_UPDATE_BATCH);
		let count = unsafe { attr.batch.count };
		if likely!(result == 0)
		{
			Ok(count as usize)
		}
		else if likely!(result == -1)
		{
			Err(errno())
		}
		else
		{
			unreachable!("Unexpected result `{}` from bpf(BPF_MAP_UPDATE_BATCH)", result)
		}
	}
	
	/// Returns `(values_filled, start of next batch position, more_entries_to_return)`.
	/// `values_filled.len()` may be less than `keys.len()`.
	#[allow(deprecated)]
	pub(crate) fn lookup_and_delete_batch<K: Sized, V: Sized>(&self, batch_position: Option<&OpaqueBatchPosition<K>>, keys: &[K]) -> Result<(Vec<V>, OpaqueBatchPosition<K>, bool), Errno>
	{
		let keys_length = keys.len();
		debug_assert!(keys_length <= u32::MAX as usize);
		
		let mut out_batch: OpaqueBatchPosition<K> = unsafe { uninitialized() };
		let mut values = Vec::with_capacity(keys_length);
		
		let mut attr = bpf_attr::default();
		attr.batch = BpfCommandMapBatch
		{
			in_batch: match batch_position
			{
				None => AlignedU64::Null,
				Some(batch_position) => AlignedU64::from(batch_position),
			},
			out_batch: AlignedU64::from(&mut out_batch),
			keys: AlignedU64::from(keys),
			values: AlignedU64::from(values.as_mut_ptr()),
			count: keys_length as u32,
			map_fd: self.as_raw_fd(),
			elem_flags: elem_flags::empty(),
			flags: 0,
		};
		
		let result = attr.syscall(bpf_cmd::BPF_MAP_LOOKUP_AND_DELETE_BATCH);
		let count = unsafe { attr.batch.count };
		if likely!(result == 0)
		{
			unsafe { values.set_len(count as usize) };
			Ok((values, out_batch, true))
		}
		else if likely!(result == -1)
		{
			let errno = errno();
			match errno.0
			{
				ENOENT =>
				{
					unsafe { values.set_len(count as usize) };
					Ok((values, out_batch, false))
				},
				
				_ => Err(errno),
			}
		}
		else
		{
			unreachable!("Unexpected result `{}` from bpf(BPF_MAP_LOOKUP_AND_DELETE_BATCH)", result)
		}
	}
	
	/// Returns `(count_of_entries_filled, more_entries_to_return)`.
	pub(crate) fn delete_batch<K: Sized>(&self, keys: &[K]) -> Result<(usize, bool), Errno>
	{
		let keys_length = keys.len();
		debug_assert!(keys_length <= u32::MAX as usize);
	
		let mut attr = bpf_attr::default();
		attr.batch = BpfCommandMapBatch
		{
			in_batch: AlignedU64::Null,
			out_batch: AlignedU64::Null,
			keys: AlignedU64::from(keys),
			values: AlignedU64::Null,
			count: keys_length as u32,
			map_fd: self.as_raw_fd(),
			elem_flags: elem_flags::empty(),
			flags: 0,
		};
		
		let result = attr.syscall(bpf_cmd::BPF_MAP_DELETE_BATCH);
		let count = unsafe { attr.batch.count };
		if likely!(result == 0)
		{
			Ok((count as usize, true))
		}
		else if likely!(result == -1)
		{
			let errno = errno();
			match errno.0
			{
				ENOENT => Ok((count as usize, false)),
				
				_ => Err(errno),
			}
		}
		else
		{
			unreachable!("Unexpected result `{}` from bpf(BPF_MAP_DELETE_BATCH)", result)
		}
	}
	
	/// `parsed_btf_map_data` must be `None` for `map_type` when it is:-
	///
	/// * `MapType::HashPerDevice`.
	/// * `MapType::ArrayPerDevice`.
	/// * `MapType::StructOps`.
	///
	/// `parsed_btf_map_data` must be `Some` for `map_type` when it is:-
	///
	/// * `MapType::SocketStorage`.
	pub(crate) fn create(map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, map_type: MapType, map_name: &MapName, parsed_btf_map_data: Option<&ParsedBtfMapData>) -> Result<Rc<Self>, MapCreationError>
	{
		let (map_type, map_flags, (btf_fd, btf_key_type_id, btf_value_type_id, btf_vmlinux_value_type_id), map_ifindex, numa_node, inner_map_fd, key_size, value_size, max_entries) = map_type.to_values(parsed_btf_map_data)?;
		
		let mut attributes = bpf_attr
		{
			map_create: BpfCommandMapCreate
			{
				map_type,
				key_size,
				value_size,
				max_entries,
				map_flags,
				inner_map_fd,
				numa_node,
				map_name: map_name.to_bpf_object_name(),
				map_ifindex,
				
				btf_fd,
				btf_key_type_id,
				btf_value_type_id,
				btf_vmlinux_value_type_id,
			}
		};
		
		let result = attributes.syscall(bpf_cmd::BPF_MAP_CREATE);
		if likely!(result >= 0)
		{
			let file_descriptor_label = FileDescriptorLabel::from(map_name);
			let map_file_descriptor = Self(result);
			Ok(map_file_descriptors.add(file_descriptor_label, map_file_descriptor)?)
		}
		else if likely!(result == -1)
		{
			Err(MapCreationError::CreateFailed(errno()))
		}
		else
		{
			unreachable!("Unexpected result `{}` from bpf(BPF_MAP_CREATE)", result)
		}
	}
}
