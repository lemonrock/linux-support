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
	type Identifier = MapIdentifier;
	
	type Information = bpf_map_info;
	
	type Access = KernelOnlyAccessPermissions;
	
	const GetFileDescriptor: bpf_cmd = bpf_cmd::BPF_MAP_GET_FD_BY_ID;
	
	const DefaultAccess: Self::Access = KernelOnlyAccessPermissions::KernelReadAndWriteUserspaceReadWrite;
	
	#[inline(always)]
	fn access_permissions_to_open_flags(access: Self::Access) -> u32
	{
		access.to_map_flags().bits() as u32
	}
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
	pub fn get_next_key<K: Copy>(&self, key: *const K) -> Result<Option<K>, SystemCallErrorNumber>
	{
		debug_assert_ne!(size_of::<K>(), 0);
		
		let mut next_key = unsafe_uninitialized();
		
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
		
		match attr.syscall(bpf_cmd::BPF_MAP_GET_NEXT_KEY).as_usize()
		{
			0 => Ok(Some(next_key)),
			
			// "If key is the last element, -1 is returned and SystemCallErrorNumber is set to ENOENT'.
			SystemCallResult::ENOENT_usize => Ok(None),
			
			// "Other possible SystemCallErrorNumber values are ENOMEM, EFAULT, EPERM, and EINVAL".
			error @ SystemCallResult::InclusiveErrorRangeStartsFrom_usize ..= SystemCallResult::InclusiveErrorRangeEndsAt_usize => Self::error(error),
			
			unexpected @ _ => unexpected_result!(bpf, BPF_MAP_GET_NEXT_KEY, unexpected),
		}
	}
	
	#[inline(always)]
	pub(crate) fn get<K: Copy, V: Copy>(&self, key: &K) -> Option<V>
	{
		debug_assert_ne!(size_of::<V>(), 0);
		
		let mut value: V = unsafe_uninitialized();
		let found = self.get_internal(key, new_non_null(&mut value));
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
	pub(crate) fn get_variably_sized<K: Copy, V: Copy>(&self, key: &K, length: usize) -> Option<Vec<V>>
	{
		let mut allocate_values: Vec<V> = Vec::with_capacity(length);
		
		let found = self.get_variably_sized_vector(key, &mut allocate_values);
		if found
		{
			unsafe { allocate_values.set_len(length) };
			Some(allocate_values)
		}
		else
		{
			None
		}
	}
	
	#[inline(always)]
	pub(crate) fn get_variably_sized_vector<K: Copy, V: Copy>(&self, key: &K, allocate_values: &mut Vec<V>) -> bool
	{
		self.get_internal(key, new_non_null(allocate_values.as_mut_ptr()))
	}
	
	#[inline(always)]
	fn get_internal<K: Copy, V: Copy>(&self, key: &K, value: NonNull<V>) -> bool
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
		
		match attr.syscall(bpf_cmd::BPF_MAP_LOOKUP_ELEM).as_usize()
		{
			0 => true,
			
			SystemCallResult::ENOENT_usize => false,
			
			SystemCallResult::ENOTSUPP_usize | SystemCallResult::EOPNOTSUPP_usize => panic!("Operation is unsupported"),
			
			unexpected_error @ SystemCallResult::InclusiveErrorRangeStartsFrom_usize ..= SystemCallResult::InclusiveErrorRangeEndsAt_usize => unexpected_error!(bpf, BPF_MAP_LOOKUP_ELEM, SystemCallResult::usize_to_system_call_error_number(unexpected_error)),
			
			unexpected @ _ => unexpected_result!(bpf, BPF_MAP_LOOKUP_ELEM, unexpected),
		}
	}
	
	/// Insert or set.
	///
	/// Errors if already present.
	pub(crate) fn insert_or_set<K: Copy, V: Copy>(&self, key: &K, value: &V, lock_flags: LockFlags) -> Result<(), ()>
	{
		match self.update(key, value, BPF_MAP_UPDATE_ELEM_flags::BPF_ANY, lock_flags)
		{
			Ok(()) => Ok(()),
			
			Err(error_number) => match error_number
			{
				E2BIG => Err(()),
				
				EEXIST => unreachable_code(format_args!("Should not return `EEXIST` as flag `BPF_NOEXIST` not specified")),
				
				ENOENT => unreachable_code(format_args!("Should not return `ENOENT` as flag `BPF_EXIST` not specified")),
				
				ENOMEM => Err(()),
				
				_ => unexpected_error!(bpf, BPF_MAP_UPDATE_ELEM, error_number),
			}
		}
	}
	
	/// Insert or set.
	///
	/// Errors if already present.
	pub(crate) fn insert_or_set_variably_sized<K: Copy, V: Copy>(&self, key: &K, values: &[V]) -> Result<(), ()>
	{
		match self.update(key, values.as_ptr(), BPF_MAP_UPDATE_ELEM_flags::BPF_ANY, LockFlags::DoNotLock)
		{
			Ok(()) => Ok(()),
			
			Err(error_number) => match error_number
			{
				E2BIG => Err(()),
				
				EEXIST => unreachable_code(format_args!("Should not return `EEXIST` as flag `BPF_NOEXIST` not specified")),
				
				ENOENT => unreachable_code(format_args!("Should not return `ENOENT` as flag `BPF_EXIST` not specified")),
				
				ENOMEM => Err(()),
				
				_ => unexpected_error!(bpf, BPF_MAP_UPDATE_ELEM, error_number),
			}
		}
	}
	
	/// Insert.
	///
	/// Errors if already present.
	pub(crate) fn insert<K: Copy, V: Copy>(&self, key: &K, value: &V, lock_flags: LockFlags) -> Result<(), InsertError>
	{
		use self::InsertError::*;
		
		match self.update(key, value, BPF_MAP_UPDATE_ELEM_flags::BPF_NOEXIST, lock_flags)
		{
			Ok(()) => Ok(()),
			
			Err(error_number) => match error_number
			{
				E2BIG => Err(MaximumCapacityReached),
				
				EEXIST => Err(AlreadyPresent),
				
				ENOENT => unreachable_code(format_args!("Should not return `ENOENT` as flag `BPF_EXIST` not specified")),
				
				ENOMEM => Err(OutOfMemory),
				
				unexpected_error @ _ => unexpected_error!(bpf, BPF_MAP_UPDATE_ELEM, unexpected_error),
			}
		}
	}
	
	/// Insert.
	///
	/// Errors if already present.
	pub(crate) fn insert_variably_sized<K: Copy, V: Copy>(&self, key: &K, values: &[V]) -> Result<(), InsertError>
	{
		use self::InsertError::*;
		
		match self.update(key, values.as_ptr(), BPF_MAP_UPDATE_ELEM_flags::BPF_NOEXIST, LockFlags::DoNotLock)
		{
			Ok(()) => Ok(()),
			
			Err(error_number) => match error_number
			{
				E2BIG => Err(MaximumCapacityReached),
				
				EEXIST => Err(AlreadyPresent),
				
				ENOENT => unreachable_code(format_args!("Should not return `ENOENT` as flag `BPF_EXIST` not specified")),
				
				ENOMEM => Err(OutOfMemory),
				
				_ => unexpected_error!(bpf, BPF_MAP_UPDATE_ELEM, error_number),
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
	pub(crate) fn set<K: Copy, V: Copy>(&self, key: &K, value: &V, lock_flags: LockFlags) -> Result<(), ()>
	{
		match self.update(key, value, BPF_MAP_UPDATE_ELEM_flags::BPF_EXIST, lock_flags)
		{
			Ok(()) => Ok(()),
			
			Err(error_number) => match error_number
			{
				E2BIG => unreachable_code(format_args!("Should not return `E2BIG` as flag `BPF_NOEXIST` or `BPF_ANY` not specified")),
				
				EEXIST => unreachable_code(format_args!("Should not return `EEXIST` as flag `BPF_NOEXIST` not specified")),
				
				ENOENT => Err(()),
				
				unexpected_error @ _ => unexpected_error!(bpf, BPF_MAP_UPDATE_ELEM, unexpected_error),
			}
		}
	}
	
	/// Set.
	///
	/// Returns an error if the key does not exist.
	pub(crate) fn set_variably_sized<K: Copy, V: Copy>(&self, key: &K, values: &[V]) -> Result<(), ()>
	{
		match self.update(key, values.as_ptr(), BPF_MAP_UPDATE_ELEM_flags::BPF_EXIST, LockFlags::DoNotLock)
		{
			Ok(()) => Ok(()),
			
			Err(error_number) => match error_number
			{
				E2BIG => unreachable_code(format_args!("Should not return `E2BIG` as flag `BPF_NOEXIST` or `BPF_ANY` not specified")),
				
				EEXIST => unreachable_code(format_args!("Should not return `EEXIST` as flag `BPF_NOEXIST` not specified")),
				
				ENOENT => Err(()),
				
				_ => unexpected_error!(bpf, BPF_MAP_UPDATE_ELEM, error_number),
			}
		}
	}
	
	/// Set.
	///
	/// For some weird reason, one needs to specify different `map_flags`.
	///
	/// See the Linux kernel function `bpf_fd_array_map_update_elem()`.
	pub(crate) fn set_for_file_descriptor_array_map<K: Copy, V: Copy>(&self, key: &K, value: &V) -> Result<(), ()>
	{
		match self.update(key, value, BPF_MAP_UPDATE_ELEM_flags::BPF_ANY, LockFlags::DoNotLock)
		{
			Ok(()) => Ok(()),
			
			Err(error_number) => match error_number
			{
				E2BIG => Err(()),
				
				EEXIST => unreachable_code(format_args!("Should not return `EEXIST` as flag `BPF_NOEXIST` not specified")),
				
				ENOENT => unreachable_code(format_args!("Should not return `ENOENT` as flag `BPF_EXIST` not specified")),
				
				ENOMEM => Err(()),
				
				unexpected_error @ _ => unexpected_error!(bpf, BPF_MAP_UPDATE_ELEM, unexpected_error),
			}
		}
	}
	
	#[inline(always)]
	fn update<K: Copy, V: Copy>(&self, key: &K, value: *const V, flags: BPF_MAP_UPDATE_ELEM_flags, lock_flags: LockFlags) -> Result<(), SystemCallErrorNumber>
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
		
		match attr.syscall(bpf_cmd::BPF_MAP_UPDATE_ELEM).as_usize()
		{
			0 => Ok(()),
			
			SystemCallResult::ENOTSUPP_usize | SystemCallResult::EOPNOTSUPP_usize => panic!("Operation is unsupported"),
			error @ SystemCallResult::InclusiveErrorRangeStartsFrom_usize ..= SystemCallResult::InclusiveErrorRangeEndsAt_usize => Self::error(error),
			
			unexpected @ _ => unexpected_result!(bpf, BPF_MAP_UPDATE, unexpected),
		}
	}
	
	#[inline(always)]
	pub(crate) fn lookup_and_delete<K: Copy, V: Copy>(&self, key: &K) -> Result<Option<V>, SystemCallErrorNumber>
	{
		debug_assert_ne!(size_of::<K>(), 0);
		debug_assert_ne!(size_of::<V>(), 0);
		
		let mut value: V = unsafe_uninitialized();
		
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
		
		match attr.syscall(bpf_cmd::BPF_MAP_LOOKUP_AND_DELETE_ELEM).as_usize()
		{
			0 => Ok(Some(value)),
			
			SystemCallResult::ENOENT_usize => Ok(None),
			error @ SystemCallResult::InclusiveErrorRangeStartsFrom_usize ..= SystemCallResult::InclusiveErrorRangeEndsAt_usize => Self::error(error),
			
			unexpected @ _ => unexpected_result!(bpf, BPF_MAP_LOOKUP_AND_DELETE_ELEM, unexpected),
		}
	}
	
	/// Returns `Ok(true)` if `key` was present.
	#[inline(always)]
	pub(crate) fn delete<K: Copy>(&self, key: &K) -> Result<bool, SystemCallErrorNumber>
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
		
		match attr.syscall(bpf_cmd::BPF_MAP_DELETE_ELEM).as_usize()
		{
			0 => Ok(true),
			
			SystemCallResult::ENOENT_usize => Ok(false),
			error @ SystemCallResult::InclusiveErrorRangeStartsFrom_usize ..= SystemCallResult::InclusiveErrorRangeEndsAt_usize => Self::error(error),
			
			unexpected @ _ => unexpected_result!(bpf, BPF_MAP_DELETE_ELEM, unexpected),
		}
	}
	
	/// The map can no longer be updated from user space.
	///
	/// Not supported by `BPF_MAP_TYPE_STRUCT_OPS`.
	///
	/// Requires the capability `CAP_SYS_ADMIN`.
	/// Can only be called once.
	#[inline(always)]
	pub(crate) fn freeze(&self) -> Result<(), SystemCallErrorNumber>
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
		
		match attr.syscall(bpf_cmd::BPF_MAP_FREEZE).as_usize()
		{
			0 => Ok(()),
			
			SystemCallResult::EINVAL_usize => panic!("Invalid attr"),
			SystemCallResult::ENOTSUPP_usize => panic!("Not supported for maps of type `BPF_MAP_TYPE_STRUCT_OPS`"),
			SystemCallResult::EBUSY_usize => panic!("Already frozen"),
			SystemCallResult::EPERM_usize => panic!("Permission denied"),
			error @ SystemCallResult::InclusiveErrorRangeStartsFrom_usize ..= SystemCallResult::InclusiveErrorRangeEndsAt_usize => unexpected_error!(bpf, BPF_MAP_FREEZE, SystemCallResult::usize_to_system_call_error_number(error)),
			
			unexpected @ _ => unexpected_result!(bpf, BPF_MAP_FREEZE, unexpected),
		}
	}
	
	/// Returns `(values_filled, start of next batch position, more_entries_to_return)`.
	/// `values_filled.len()` may be less than `keys.len()`.
	#[inline(always)]
	pub(crate) fn get_batch<K: Copy, V: Copy>(&self, batch_position: Option<&OpaqueBatchPosition<K>>, keys: &[K]) -> Result<(Vec<V>, OpaqueBatchPosition<K>, bool), SystemCallErrorNumber>
	{
		let keys_length = keys.len();
		let mut values = Vec::with_capacity(keys_length);
		
		let (count, out_batch, more) = self.lookup_batch(batch_position, keys, AlignedU64::from(values.as_mut_ptr()))?;
		unsafe { values.set_len(count as usize) };
		Ok((values, out_batch, more))
	}
	
	#[inline(always)]
	pub(crate) fn get_batch_variably_sized<K: Copy>(&self, batch_position: Option<&OpaqueBatchPosition<K>>, keys: &[K], values: &mut [u8]) -> Result<(usize, OpaqueBatchPosition<K>, bool), SystemCallErrorNumber>
	{
		let (count, out_batch, more) = self.lookup_batch(batch_position, keys, AlignedU64::from(values.as_mut_ptr()))?;
		Ok((count as usize, out_batch, more))
	}
	
	/// Returns `(values_filled, start of next batch position, more_entries_to_return)`.
	/// `values_filled.len()` may be less than `keys.len()`.
	#[inline(always)]
	pub(crate) fn lookup_batch<K: Copy>(&self, batch_position: Option<&OpaqueBatchPosition<K>>, keys: &[K], values: AlignedU64) -> Result<(u32, OpaqueBatchPosition<K>, bool), SystemCallErrorNumber>
	{
		let keys_length = keys.len();
		assert_ne!(keys_length, 0, "There must be at least one key");
		debug_assert!(keys_length <= u32::MAX as usize);
		
		let mut out_batch: OpaqueBatchPosition<K> = unsafe_uninitialized();
		
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
			values,
			count: keys_length as u32,
			map_fd: self.as_raw_fd(),
			elem_flags: elem_flags::empty(),
			flags: 0,
		};
		
		match attr.syscall(bpf_cmd::BPF_MAP_LOOKUP_BATCH).as_usize()
		{
			0 => Ok((Self::count(attr), out_batch, true)),
			
			SystemCallResult::ENOENT_usize => Ok((Self::count(attr), out_batch, false)),
			
			error @ SystemCallResult::InclusiveErrorRangeStartsFrom_usize ..= SystemCallResult::InclusiveErrorRangeEndsAt_usize => Self::error(error),
			
			unexpected @ _ => unexpected_result!(bpf, BPF_MAP_LOOKUP_BATCH, unexpected),
		}
	}
	
	/// Returns `(values_filled, start of next batch position, more_entries_to_return)`.
	/// `values_filled.len()` may be less than `keys.len()`.
	#[inline(always)]
	pub(crate) fn get_and_delete_batch<K: Copy, V: Copy>(&self, batch_position: Option<&OpaqueBatchPosition<K>>, keys: &[K]) -> Result<(Vec<V>, OpaqueBatchPosition<K>, bool), SystemCallErrorNumber>
	{
		let keys_length = keys.len();
		let mut values = Vec::with_capacity(keys_length);
		
		let (count, out_batch, more) = self.lookup_and_delete_batch(batch_position, keys, AlignedU64::from(values.as_mut_ptr()))?;
		unsafe { values.set_len(count as usize) };
		Ok((values, out_batch, more))
	}
	
	#[inline(always)]
	pub(crate) fn get_and_delete_batch_variably_sized<K: Copy>(&self, batch_position: Option<&OpaqueBatchPosition<K>>, keys: &[K], values: &mut [u8]) -> Result<(usize, OpaqueBatchPosition<K>, bool), SystemCallErrorNumber>
	{
		let (count, out_batch, more) = self.lookup_and_delete_batch(batch_position, keys, AlignedU64::from(values.as_mut_ptr()))?;
		Ok((count as usize, out_batch, more))
	}
	
	/// Returns `(values_filled, start of next batch position, more_entries_to_return)`.
	/// `values_filled.len()` may be less than `keys.len()`.
	#[inline(always)]
	fn lookup_and_delete_batch<K: Copy>(&self, batch_position: Option<&OpaqueBatchPosition<K>>, keys: &[K], values: AlignedU64) -> Result<(u32, OpaqueBatchPosition<K>, bool), SystemCallErrorNumber>
	{
		let keys_length = keys.len();
		assert_ne!(keys_length, 0, "There must be at least one key");
		debug_assert!(keys_length <= u32::MAX as usize);
		
		let mut out_batch: OpaqueBatchPosition<K> = unsafe_uninitialized();
		
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
			values,
			count: keys_length as u32,
			map_fd: self.as_raw_fd(),
			elem_flags: elem_flags::empty(),
			flags: 0,
		};
		
		match attr.syscall(bpf_cmd::BPF_MAP_LOOKUP_AND_DELETE_BATCH).as_usize()
		{
			0 => Ok((Self::count(attr), out_batch, true)),
			
			SystemCallResult::ENOENT_usize => Ok((Self::count(attr), out_batch, false)),
			
			error @ SystemCallResult::InclusiveErrorRangeStartsFrom_usize ..= SystemCallResult::InclusiveErrorRangeEndsAt_usize => Self::error(error),
			
			unexpected @ _ => unexpected_result!(bpf, BPF_MAP_LOOKUP_AND_DELETE_BATCH, unexpected),
		}
	}
	
	/// `keys` and `values` must be the same length.
	#[inline(always)]
	pub(crate) fn set_batch<K: Copy, V: Copy>(&self, keys: &[K], values: &[V], lock_flags: LockFlags) -> Result<usize, SystemCallErrorNumber>
	{
		self.update_batch(keys, AlignedU64::from(values), lock_flags)
	}
	
	#[inline(always)]
	pub(crate) fn set_batch_variably_sized<K: Copy>(&self, keys: &[K], values: &[u8]) -> Result<usize, SystemCallErrorNumber>
	{
		self.update_batch(keys, AlignedU64::from(values), LockFlags::DoNotLock)
	}
	
	/// `keys` and `values` must be the same length.
	#[inline(always)]
	fn update_batch<K: Copy>(&self, keys: &[K], values: AlignedU64, lock_flags: LockFlags) -> Result<usize, SystemCallErrorNumber>
	{
		let keys_length = keys.len();
		assert_ne!(keys_length, 0, "There must be at least one key");
		debug_assert!(keys_length <= u32::MAX as usize);
		
		let mut attr = bpf_attr::default();
		attr.batch = BpfCommandMapBatch
		{
			in_batch: AlignedU64::Null,
			out_batch: AlignedU64::Null,
			keys: AlignedU64::from(keys),
			values,
			count: keys_length as u32,
			map_fd: self.as_raw_fd(),
			elem_flags: lock_flags.to_elem_flags(),
			flags: 0,
		};
		
		match attr.syscall(bpf_cmd::BPF_MAP_UPDATE_BATCH).as_usize()
		{
			0 => Ok(Self::count_usize(attr)),
			
			error @ SystemCallResult::InclusiveErrorRangeStartsFrom_usize ..= SystemCallResult::InclusiveErrorRangeEndsAt_usize => Self::error(error),
			
			unexpected @ _ => unexpected_result!(bpf, BPF_MAP_UPDATE_BATCH, unexpected),
		}
	}
	
	/// Returns `(count_of_entries_filled, more_entries_to_return)`.
	pub(crate) fn delete_batch<K: Copy>(&self, keys: &[K]) -> Result<(usize, bool), SystemCallErrorNumber>
	{
		let keys_length = keys.len();
		assert_ne!(keys_length, 0, "There must be at least one key");
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
		
		match attr.syscall(bpf_cmd::BPF_MAP_DELETE_BATCH).as_usize()
		{
			0 => Ok((Self::count_usize(attr), true)),
			
			SystemCallResult::ENOENT_usize => Ok((Self::count_usize(attr), false)),
			
			error @ SystemCallResult::InclusiveErrorRangeStartsFrom_usize ..= SystemCallResult::InclusiveErrorRangeEndsAt_usize => Self::error(error),
			
			unexpected @ _ => unexpected_result!(bpf, BPF_MAP_DELETE_BATCH, unexpected),
		}
	}
	
	/// `parsed_bpf_type_format_map_data` must be `None` for `map_type` when it is:-
	///
	/// * `MapType::HashPerDevice`.
	/// * `MapType::ArrayPerDevice`.
	/// * `MapType::StructOps`.
	///
	/// `parsed_bpf_type_format_map_data` must be `Some` for `map_type` when it is:-
	///
	/// * `MapType::SocketStorage`.
	pub(crate) fn create(map_file_descriptors: &mut FileDescriptorsMap<MapFileDescriptor>, map_type: MapType, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>) -> Result<Rc<Self>, MapCreationError>
	{
		let (map_type, map_flags, (btf_fd, btf_key_type_id, btf_value_type_id, btf_vmlinux_value_type_id), map_ifindex, numa_node, inner_map_fd, key_size, value_size, max_entries) = map_type.to_values(parsed_bpf_type_format_map_data)?;
		
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
				map_name: map_name.into(),
				map_ifindex,
				
				btf_fd,
				btf_key_type_id,
				btf_value_type_id,
				btf_vmlinux_value_type_id,
			}
		};
		
		match attributes.syscall(bpf_cmd::BPF_MAP_CREATE).as_usize()
		{
			raw_file_descriptor @ SystemCallResult::InclusiveMinimumRawFileDescriptor_usize ..= SystemCallResult::InclusiveMaximumRawFileDescriptor_usize =>
			{
				let map_file_descriptor = Self(raw_file_descriptor as RawFd);
				Ok(map_file_descriptors.add(map_name.clone(), map_file_descriptor)?)
			}
			
			error @ SystemCallResult::InclusiveErrorRangeStartsFrom_usize ..= SystemCallResult::InclusiveErrorRangeEndsAt_usize => Err(MapCreationError::CreateFailed(SystemCallResult::usize_to_system_call_error_number(error))),
			
			unexpected @ _ => unexpected_result!(bpf, BPF_MAP_CREATE, unexpected),
		}
	}
	
	#[inline(always)]
	const fn error<X>(error: usize) -> Result<X, SystemCallErrorNumber>
	{
		Err(SystemCallResult::usize_to_system_call_error_number(error))
	}
	
	#[inline(always)]
	const fn count_usize(attr: bpf_attr) -> usize
	{
		Self::count(attr) as usize
	}
	
	#[inline(always)]
	const fn count(attr: bpf_attr) -> u32
	{
		unsafe { attr.batch.count }
	}
}
