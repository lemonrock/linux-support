// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Memory policy.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum MemoryPolicy
{
	/// `MPOL_DEFAULT`.
	MPOL_DEFAULT = 0,

	/// `MPOL_PREFERRED`.
	MPOL_PREFERRED = 1,

	/// `MPOL_BIND`.
	MPOL_BIND =  2,

	/// `MPOL_INTERLEAVE`.
	MPOL_INTERLEAVE = 3,

	/// `MPOL_LOCAL`.
	MPOL_LOCAL = 4,
}

impl Default for MemoryPolicy
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::MPOL_DEFAULT
	}
}

impl MemoryPolicy
{
	/// Get the current thread's memory policy.
	#[inline(always)]
	pub fn get_current_thread_memory_policy() -> (Self, MemoryPolicyDynamism)
	{
		#[allow(deprecated)]
		let mut mode: i32 = unsafe { uninitialized() };
		Self::guard_result(get_mempolicy(&mut mode, null_mut(), 0, null(), GetMemoryPolicyFlags::empty()));

		Self::mode_to_flags(mode)
	}

	/// Valid NUMA nodes for set syscalls.
	#[inline(always)]
	pub fn get_current_thread_valid_numa_nodes_for_set_memory_policy_and_mbind() -> BitSet<NumaNode>
	{
		let mut bit_set = unsafe { BitSet::new_uninitialized() };
		let (pointer, length) = bit_set.to_raw_parts_mut();

		Self::guard_result(get_mempolicy(null_mut(), pointer, length, null(), GetMemoryPolicyFlags::MPOL_F_MEMS_ALLOWED));

		bit_set.shrink_to_fit();
		bit_set
	}

	/// Valid policy and NUMA nodes for memory.
	///
	/// ?The BitSet returned contains all the NUMA nodes to which the memory policy applies?
	#[inline(always)]
	pub fn get_memory_policy_for_memory(address: NonNull<u8>) -> (Self, MemoryPolicyDynamism, BitSet<NumaNode>)
	{
		#[allow(deprecated)]
		let mut mode: i32 = unsafe { uninitialized() };
		let mut bit_set = unsafe { BitSet::new_uninitialized() };
		let (pointer, length) = bit_set.to_raw_parts_mut();

		Self::guard_result(get_mempolicy(&mut mode, pointer, length, address.as_ptr() as *mut c_void, GetMemoryPolicyFlags::MPOL_F_ADDR));

		let (memory_policy, memory_policy_dynamism) = Self::mode_to_flags(mode);
		(memory_policy, memory_policy_dynamism, bit_set)
	}

	/// NUMA node that is used for memory.
	///
	/// Seems to be brittle.
	#[inline(always)]
	pub fn get_numa_node_for_memory(address: NonNull<u8>) -> NumaNode
	{
		#[allow(deprecated)]
		let mut mode: i32 = unsafe { uninitialized() };
		Self::guard_result(get_mempolicy(&mut mode, null_mut(), 0, address.as_ptr() as *mut c_void, GetMemoryPolicyFlags::MPOL_F_NODE | GetMemoryPolicyFlags::MPOL_F_ADDR));

		NumaNode::from(mode as u8)
	}

	/// Valid policy and NUMA nodes for memory if the current thread's policy is interleaved.
	///
	/// Seems to bve very brittle.
	#[inline(always)]
	pub fn get_current_thread_numa_node_for_next_interleaved_internal_kernel_page() -> Option<NumaNode>
	{
		let (memory_policy, _) = Self::get_current_thread_memory_policy();
		if memory_policy != MemoryPolicy::MPOL_INTERLEAVE
		{
			return None
		}

		#[allow(deprecated)]
		let mut mode: i32 = unsafe { uninitialized() };
		Self::guard_result(get_mempolicy(&mut mode, null_mut(), 0, null(), GetMemoryPolicyFlags::MPOL_F_NODE));

		Some(NumaNode::from(mode as u8))
	}

	#[inline(always)]
	fn mode_to_flags(mode: i32) -> (Self, MemoryPolicyDynamism)
	{
		let value = mode & 0b0000_0111;
		if unlikely!(value > MemoryPolicy::MPOL_LOCAL as i32)
		{
			panic!("invalid mode");
		}
		let policy = unsafe { transmute(value) };

		use self::MemoryPolicyDynamism::*;
		let dynamism = match unsafe { transmute(mode - value) }
		{
			0 => NoDynamism,
			0x4000 => Relative,
			0x8000 => Static,
			_ => panic!("Invalid dynamism"),
		};

		(policy, dynamism)
	}

	#[inline(always)]
	fn guard_result(result: isize)
	{
		if likely!(result == 0)
		{
			return
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EFAULT => panic!("part of or all of the memory range specified by nodemask and maxnode points outside your accessible address space."),
				EINVAL => panic!("The value specified by maxnode is less than the number of node IDs supported by the system. Or flags specified values other than MPOL_F_NODE or MPOL_F_ADDR; or flags specified MPOL_F_ADDR and addr is NULL, or flags did not specify MPOL_F_ADDR and addr is not NULL. Or, flags specified MPOL_F_NODE but not MPOL_F_ADDR and the current thread policy is not MPOL_INTERLEAVE. Or, flags specified MPOL_F_MEMS_ALLOWED with either MPOL_F_ADDR or MPOL_F_NODE. (And there are other EINVAL cases.)"),

				unknown @ _ => panic!("unknown error number {}", unknown),
			}
		}
		else
		{
			panic!("unexpected result code {}", result)
		}
	}
}
