// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// `BitSet` of `NumaNode`.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct NumaNodes(pub BitSet<NumaNode>);

impl Deref for NumaNodes
{
	type Target = BitSet<NumaNode>;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl DerefMut for NumaNodes
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.0
	}
}

impl NumaNodes
{
	/// Returns an intersection to best calculate the valid set.
	///
	/// This will return `None` if the Linux kernel wasn't configured with `CONFIG_NUMA`.
	#[inline(always)]
	pub fn valid(sys_path: &SysPath, proc_path: &ProcPath) -> Option<Self>
	{
		let mut valid = Self::has_a_folder_path(sys_path)?;
		valid.intersection(&Self::is_in_proc_self_status(proc_path));
		valid.intersection(&Self::have_at_least_one_cpu(sys_path).unwrap());
		valid.intersection(&Self::possible(sys_path).unwrap());
		valid.intersection(&Self::online(sys_path).unwrap());
		valid.intersection(&Self::have_normal_memory(sys_path).unwrap());

		assert!(!valid.is_empty());

		valid.shrink_to_fit();

		Some(valid)
	}

	/// Sets affinity.
	///
	/// Does a cursory check that the `path` exists (but is subject to a TOCTOU flaw).
	#[inline(always)]
	pub fn set_affinity(&self, path: impl AsRef<Path>) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write affinity to path");
		
		let path = path.as_ref();
		if path.exists()
		{
			let mask = IntoBitMask(self);
			path.write_value(mask)
		}
		else
		{
			Ok(())
		}
	}

	/// Sets affinity.
	///
	/// Does a cursory check that the `path` exists (but is subject to a TOCTOU flaw).
	#[inline(always)]
	pub fn set_affinity_list(&self, path: impl AsRef<Path>) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write affinity to path");
		
		let path = path.as_ref();
		if path.exists()
		{
			let list = IntoList(&self.0);
			path.write_value(list)
		}
		else
		{
			Ok(())
		}
	}
	
	/// NUMA nodes that exist in the file system.
	///
	/// This will return `None` if the Linux kernel wasn't configured with `CONFIG_NUMA`.
	#[inline(always)]
	fn has_a_folder_path(sys_path: &SysPath) -> Option<Self>
	{
		sys_path.numa_nodes_folder_path().entries_in_folder_path().unwrap().map(|bit_set| Self(bit_set))
	}

	/// NUMA nodes that could possibly be online at some point.
	///
	/// This will be empty if the Linux kernel wasn't configured with `CONFIG_NUMA`.
	#[inline(always)]
	fn is_in_proc_self_status(proc_path: &ProcPath) -> Self
	{
		let process_status_statistics = Status::self_status(proc_path).unwrap();
		process_status_statistics.numa_nodes_allowed
	}

	/// NUMA nodes that have a CPU.
	///
	/// NUMA nodes without a CPU are effectively fake NUMA nodes.
	///
	/// This will return `None` if the Linux kernel wasn't configured with `CONFIG_NUMA`.
	#[inline(always)]
	fn have_at_least_one_cpu(sys_path: &SysPath) -> Option<Self>
	{
		Self::read_numa_node_list(sys_path, "has_cpu")
	}

	/// NUMA nodes that could possibly be online at some point.
	///
	/// This will return `None` if the Linux kernel wasn't configured with `CONFIG_NUMA`.
	///
	/// Not reliable, as includes NUMA nodes that can never be brought online; simply reports the number that could be used by the Linux kernel upto the `CONFIG_` number of NUMA nodes.
	#[inline(always)]
	pub(crate) fn possible(sys_path: &SysPath) -> Option<Self>
	{
		Self::read_numa_node_list(sys_path, "possible")
	}

	/// NUMA nodes that are online at some point.
	///
	/// This will return `None` if the Linux kernel wasn't configured with `CONFIG_NUMA`.
	#[inline(always)]
	fn online(sys_path: &SysPath) -> Option<Self>
	{
		Self::read_numa_node_list(sys_path, "online")
	}

	/// NUMA nodes that have normal memory (as opposed to what was high memory, `has_high_memory`, which only exists if Linux is compiled with `CONFIG_HIGHMEM` which is an ancient setting).
	///
	/// I suspect this value is effectively useless.
	///
	/// This will return `None` if the Linux kernel wasn't configured with `CONFIG_NUMA`.
	#[inline(always)]
	fn have_normal_memory(sys_path: &SysPath) -> Option<Self>
	{
		Self::read_numa_node_list(sys_path, "has_normal_memory")
	}

	/// NUMA nodes that have hot-pluggable memory.
	///
	/// Intended to support hot-plugging of memory.
	///
	/// This will return `None` if the Linux kernel wasn't configured with `CONFIG_NUMA`.
	#[inline(always)]
	pub fn have_movable_memory(sys_path: &SysPath) -> Option<Self>
	{
		Self::read_numa_node_list(sys_path, "has_memory")
	}

	/// NUMA nodes that have high memory.
	///
	/// Obsolete.
	///
	/// This will return `None` if the Linux kernel wasn't configured with `CONFIG_NUMA`.
	#[inline(always)]
	pub fn have_high_memory(sys_path: &SysPath) -> Option<Self>
	{
		Self::read_numa_node_list(sys_path, "has_high_memory")
	}

	#[inline(always)]
	fn read_numa_node_list(sys_path: &SysPath, file_name: &str) -> Option<Self>
	{
		let file_path = sys_path.numa_nodes_path(file_name);
		file_path.read_hyper_thread_or_numa_node_list_if_exists().unwrap().map(Self)
	}
}
