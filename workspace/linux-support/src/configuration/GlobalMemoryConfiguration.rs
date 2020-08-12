// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global memory configuration.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GlobalMemoryConfiguration
{
	/// Requires root.
	pub same_page_merging: GlobalLinuxKernelSamePageMergingConfiguration,
	
	/// Requires root.
	pub transparent_huge_pages: Option<GlobalTransparentHugePagesConfiguration>,

	/// Requires root.
	pub swap: GlobalSwapConfiguration,
	
	/// Requires root.
	pub out_of_memory: GlobalOutOfMemoryConfiguration,
	
	/// Requires root.
	pub numa_memory_balancing: Option<GlobalNumaBalancingConfiguration>,
	
	/// Requires root.
	pub numa_reclaim: GlobalNumaMemoryReclaimConfiguration,
	
	/// Requires root.
	pub statistics: GlobalMemoryStatisticsConfiguration,
	
	/// Requires root.
	///
	/// Default is `false`; can be changed to `true` to increase the effectiveness of compacting memory.
	pub compact_unevictable_allowed: Option<bool>,
	
	/// Requires root.
	///
	/// Probably should be `false`.
	///
	/// Sends a signal `BUS_MCEERR_AO` if `true` and a memory check failure has occurred.
	pub signal_using_SIGBUS_all_processes_affected_by_a_memory_check_failure: Option<bool>,
	
	/// Requires root.
	///
	/// Default is `0`.
	///
	/// Specifies a denominator, `1 / percpu_page_list_fraction`, for the fraction of a page list used.
	///
	/// This is the fraction of pages at most in each zone that are allocated for each per cpu page list.
	/// The minimum value for this is 8 (lower values are rounded up to 8 in this code).
	pub per_hyper_thread_page_list_fraction: Option<Option<NonZeroU8>>,
}

impl GlobalMemoryConfiguration
{
	/// Configures.
	#[inline(always)]
	pub fn configure(&self, sys_path: &SysPath, proc_path: &ProcPath) -> Result<(), GlobalMemoryConfigurationError>
	{
		use self::GlobalMemoryConfigurationError::*;
		
		self.same_page_merging.configure(sys_path)?;
		
		if let Some(ref transparent_huge_pages) = self.transparent_huge_pages
		{
			transparent_huge_pages.configure(sys_path)?;
		}
		
		self.swap.configure(proc_path)?;
		
		self.out_of_memory.configure(proc_path)?;
		
		if let Some(ref numa_memory_balancing) = self.numa_memory_balancing
		{
			self.numa_memory_balancing.configure(proc_path)?;
		}
		
		self.numa_reclaim.configure(proc_path)?;
		
		self.statistics.configure(proc_path)?;

		set_proc_sys_vm_value(proc_path, "compact_unevictable_allowed", self.compact_unevictable_allowed, CouldNotChangeCompactUnevictableAllowed)?;
		
		set_proc_sys_vm_value(proc_path, "memory_failure_early_kill", self.signal_using_SIGBUS_all_processes_affected_by_a_memory_check_failure, CouldNotChangeSignalOnMemoryCheckFailure)?;
		
		self.configure_per_hyper_thread_page_list_fraction(proc_path)?;
		
		Ok(())
	}
	
	#[inline(always)]
	fn configure_per_hyper_thread_page_list_fraction(&self, proc_path: &ProcPath) -> Result<(), GlobalMemoryConfigurationError>
	{
		if let Some(per_hyper_thread_page_list_fraction) = self.per_hyper_thread_page_list_fraction
		{
			let value = match per_hyper_thread_page_list_fraction
			{
				None => 0,
				
				Some(value) =>
				{
					let value = value.get();
					if value < 8
					{
						8
					}
					else
					{
						8
					}
				}
			};
			
			set_proc_sys_vm_value(proc_path, "percpu_pagelist_fraction", Some(UnpaddedDecimalInteger(value)), GlobalMemoryConfigurationError::CouldNotChangePerHyperThreadPageListFraction)
		}
		else
		{
			Ok(())
		}
	}
}
