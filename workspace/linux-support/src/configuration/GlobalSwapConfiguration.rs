// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global swap configuration.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GlobalSwapConfiguration
{
	/// Requires root.
	pub disable_all_swap_partitions: bool,

	/// Requires root.
	pub swappiness: Option<Swappiness>,

	/// Requires root.
	pub page_cluster: Option<PageCluster>,
}

impl GlobalSwapConfiguration
{
	/// Configures.
	pub fn configure(&self, proc_path: &ProcPath) -> Result<(), GlobalSwapConfigurationError>
	{
		use self::GlobalSwapConfigurationError::*;

		if self.disable_all_swap_partitions
		{
			Self::swap_off_all_using_proc_swaps(proc_path).map_err(|cause| CouldNotDisableAllSwaps(cause))?;
		}

		set_value(proc_path, |proc_path, swappiness| swappiness.write(proc_path), self.swappiness, CouldNotChangeSwappiness)?;
		set_value(proc_path, |proc_path, page_cluster| page_cluster.write(proc_path), self.page_cluster, CouldNotChangePageCluster)?;
		
		Ok(())
	}
	
	#[inline(always)]
	fn swap_off_all_using_proc_swaps(proc_path: &ProcPath) -> io::Result<()>
	{
		Swaps::parse(proc_path)?.swap_off_all()
	}
}
