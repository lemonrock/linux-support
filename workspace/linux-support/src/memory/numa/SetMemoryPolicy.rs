// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Used to set a memory policy.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum SetMemoryPolicy
{
	/// Default.
	Default,

	/// Local.
	Local,

	/// Bind.
	Bind
	{
		/// At least one NUMA node in this must be online.
		numa_node_bit_set: BitSet<NumaNode>,
	},

	/// Interleave.
	Interleave
	{
		/// At least one NUMA node in this must be online.
		numa_node_bit_set: BitSet<NumaNode>,
	},

	/// Preferred.
	Preferred(NumaNode),
}

impl SetMemoryPolicy
{
	/// A `Bind` policy for the current NumaNode.
	#[inline(always)]
	pub fn BindCurrent() -> Self
	{
		SetMemoryPolicy::Bind { numa_node_bit_set: NumaNode::current().0.into_bit_set() }
	}
	
	/// Set thread policy.
	#[inline(always)]
	pub fn set_thread_policy(&self)
	{
		let mut preferred_numa_node_bitmask: Option<BitSet<NumaNode>> = None;

		use self::SetMemoryPolicy::*;
		use self::MemoryPolicy::*;
		let (set_memory_policy, (nodemask, maxnode)) = match self
		{
			&Default => (MPOL_DEFAULT as i32, (null(), 0)),

			&Local => (MPOL_LOCAL as i32, (null(), 0)),

			&Bind { ref numa_node_bit_set } => (MPOL_BIND as i32, numa_node_bit_set.to_raw_parts()),

			&Interleave { ref numa_node_bit_set } => (MPOL_INTERLEAVE as i32, numa_node_bit_set.to_raw_parts()),

			&Preferred(numa_node) =>
			{
				preferred_numa_node_bitmask.replace(numa_node.into_bit_set());
				(MPOL_PREFERRED as i32, preferred_numa_node_bitmask.as_ref().unwrap().to_raw_parts())
			}
		};

		match system_call_set_mempolicy(set_memory_policy, nodemask, maxnode).as_usize()
		{
			0 => return,
			
			SystemCallResult::EFAULT_usize => panic!("part of or all of the memory range specified by nodemask and maxnode points outside your accessible address space."),
			SystemCallResult::EINVAL_usize => panic!("mode is invalid. Or, mode is MPOL_DEFAULT and nodemask is nonempty, or mode is MPOL_BIND or MPOL_INTERLEAVE and nodemask is empty. Or, maxnode specifies more than a page worth of bits. Or, nodemask specifies one or more node IDs that are greater than the maximum supported node ID. Or, none of the node IDs specified by nodemask are on-line and allowed by the process's current cpuset context, or none of the specified nodes contain memory. Or, the mode argument specified both MPOL_F_STATIC_NODES and MPOL_F_RELATIVE_NODES."),
			SystemCallResult::ENOMEM_usize => panic!("insufficient kernel memory was available."),
			unexpected_error @ SystemCallResult::InclusiveErrorRangeStartsFrom_usize ..= SystemCallResult::InclusiveErrorRangeEndsAt_usize => unexpected_error!(set_mempolicy, SystemCallResult::usize_to_system_call_error_number(unexpected_error)),
			
			unexpected @ _ => unexpected_result!(set_mempolicy, unexpected),
		}
	}

	/// Sets address policy.
	#[inline(always)]
	pub fn set_address_policy(&self, address: NonNull<u8>, length: usize)
	{
		match self.set_address_policy_(address, length, MemoryBindFlags::empty())
		{
			Ok(()) => return,
			Err(EIO) => panic!("EIO should not occur"),
			Err(EPERM) => panic!("EPERM should not occur"),
			Err(unexpected_error) => unexpected_error!(mbind, unexpected_error),
		}
	}

	/// Can not validate if the memory policy is `Default`.
	#[inline(always)]
	pub fn validate_address_policy(&self, address: NonNull<u8>, length: usize) -> Option<bool>
	{
		if self == &SetMemoryPolicy::Default
		{
			return None
		}

		match self.set_address_policy_(address, length, MemoryBindFlags::MPOL_MF_STRICT)
		{
			Ok(()) => Some(true),
			Err(EIO) => Some(false),
			Err(EPERM) => panic!("EPERM should not occur"),
			Err(unexpected_error) => unexpected_error!(mbind, unexpected_error),
		}
	}

	/// Sets and moves pages, if possible.
	///
	/// Does not attempt to move pages shared with other processes.
	#[inline(always)]
	pub fn set_address_policy_and_move_pages_if_possible(&self, address: NonNull<u8>, length: usize)
	{
		match self.set_address_policy_(address, length, MemoryBindFlags::MPOL_MF_MOVE)
		{
			Ok(()) => return,
			Err(EIO) => panic!("EIO should not occur"),
			Err(EPERM) => panic!("EPERM should not occur"),
			Err(unexpected_error) => unexpected_error!(mbind, unexpected_error),
		}
	}

	/// Sets and moves pages; returns `Err(())` if page move failed.
	///
	/// Does not attempt to move pages shared with other processes.
	#[inline(always)]
	pub fn set_address_policy_and_move_pages(&self, address: NonNull<u8>, length: usize) -> Result<(), ()>
	{
		match self.set_address_policy_(address, length, MemoryBindFlags::MPOL_MF_MOVE | MemoryBindFlags::MPOL_MF_STRICT)
		{
			Ok(()) => Ok(()),
			Err(EIO) => Err(()),
			Err(EPERM) => panic!("EPERM should not occur"),
			Err(unexpected_error) => unexpected_error!(mbind, unexpected_error),
		}
	}

	/// Sets and moves pages, if possible.
	///
	/// Attempts to move pages shared with other processes as well.
	///
	/// Panics if the current thread does not have the capability `CAP_SYS_NICE`.
	#[inline(always)]
	pub fn set_address_policy_and_move_pages_including_those_shared_with_other_processes_if_possible(&self, address: NonNull<u8>, length: usize)
	{
		match self.set_address_policy_(address, length, MemoryBindFlags::MPOL_MF_MOVE_ALL)
		{
			Ok(()) => return,
			Err(EIO) => panic!("EIO should not occur"),
			Err(EPERM) => panic!("Current thread does not have the capability `CAP_SYS_NICE`"),
			Err(unexpected_error) => unexpected_error!(mbind, unexpected_error),
		}
	}

	/// Sets and moves pages, if possible.
	///
	/// Attempts to move pages shared with other processes as well.
	///
	/// Panics if the current thread does not have the capability `CAP_SYS_NICE`.
	#[inline(always)]
	pub fn set_address_policy_and_move_pages_including_those_shared_with_other_processes(&self, address: NonNull<u8>, length: usize) -> Result<(), ()>
	{
		match self.set_address_policy_(address, length, MemoryBindFlags::MPOL_MF_MOVE_ALL | MemoryBindFlags::MPOL_MF_STRICT)
		{
			Ok(()) => Ok(()),
			Err(EIO) => Err(()),
			Err(EPERM) => panic!("Current thread does not have the capability `CAP_SYS_NICE`"),
			Err(unexpected_error) => unexpected_error!(mbind, unexpected_error),
		}
	}

	#[inline(always)]
	fn set_address_policy_(&self, address: NonNull<u8>, length: usize, memory_bind_flags: MemoryBindFlags) -> Result<(), SystemCallErrorNumber>
	{
		debug_assert_eq!((address.as_ptr() as u64) % PageSize::default() as u64, 0, "address is not a multiple of the system page size");

		let mut preferred_numa_node_bitmask: Option<BitSet<NumaNode>> = None;

		use self::SetMemoryPolicy::*;
		use self::MemoryPolicy::*;
		let (policy, (nodemask, maxnode)) = match self
		{
			&Default => (MPOL_DEFAULT as i32, (null(), 0)),

			&Local => (MPOL_LOCAL as i32, (null(), 0)),

			&Bind { ref numa_node_bit_set } => (MPOL_BIND as i32, numa_node_bit_set.to_raw_parts()),

			&Interleave { ref numa_node_bit_set } => (MPOL_INTERLEAVE as i32, numa_node_bit_set.to_raw_parts()),

			&Preferred(numa_node) =>
			{
				preferred_numa_node_bitmask.replace(numa_node.into_bit_set());
				(MPOL_PREFERRED as i32, preferred_numa_node_bitmask.as_ref().unwrap().to_raw_parts())
			}
		};

		let mode = policy;
		match system_call_mbind(address.as_ptr() as *mut c_void, length, mode, nodemask, maxnode, memory_bind_flags).as_usize()
		{
			0 => Ok(()),
			
			SystemCallResult::EFAULT_usize => panic!("part of or all of the memory range specified by nodemask and maxnode points outside your accessible address space."),
			SystemCallResult::EINVAL_usize => panic!("An invalid value was specified for flags or mode; or addr + len was less than addr; or addr is not a multiple of the system page size. Or, mode is MPOL_DEFAULT and nodemask specified a nonempty set; or mode is MPOL_BIND or MPOL_INTERLEAVE and nodemask is empty. Or, maxnode exceeds a kernel-imposed limit. Or, nodemask specifies one or more node IDs that are greater than the maximum supported node ID. Or, none of the node IDs specified by nodemask are on-line and allowed by the thread's current cpuset context, or none of the specified nodes contain memory. Or, the mode argument specified both MPOL_F_STATIC_NODES and MPOL_F_RELATIVE_NODES."),
			SystemCallResult::ENOMEM_usize => panic!("insufficient kernel memory was available."),
			SystemCallResult::EIO_usize => Err(EIO),
			SystemCallResult::EPERM_usize => Err(EPERM),
			error @ SystemCallResult::InclusiveErrorRangeStartsFrom_usize ..= SystemCallResult::InclusiveErrorRangeEndsAt_usize => unexpected_error!(mbind, SystemCallResult::usize_to_system_call_error_number(error)),
			
			unexpected @ _ => unexpected_result!(mbind, unexpected),
		}
	}
}
