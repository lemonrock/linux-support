// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Grace period configuration.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RcuGracePeriodConfiguration
{
	/// Set `/sys/kernel/rcu_expedited` to `0` and `/sys/kernel/rcu_normal` to `0`.
	///
	/// This is the default
	Ordinary,
	
	/// Set `/sys/kernel/rcu_expedited` to `1` and `/sys/kernel/rcu_normal` to `0`.
	Expedited,

	/// Set `/sys/kernel/rcu_expedited` to `0` and `/sys/kernel/rcu_normal` to `1`.
	Normal,
}

impl RcuGracePeriodConfiguration
{
	/// Set.
	///
	/// Requires root.
	pub fn set(self, sys_path: &SysPath) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write to /sys/kernel/rcu_expedited and /sys/kernel/rcu_normal");
		
		use self::RcuGracePeriodConfiguration::*;
		
		let (expedited, normal) = match self
		{
			Ordinary => (false, false),
			
			Expedited => (true, false),
			
			Normal => (false, true),
		};
		
		let rcu_expedited_file_path = Self::rcu_expedited_file_path(sys_path);
		let rcu_normal_file_path = Self::rcu_normal_file_path(sys_path);
		
		if rcu_expedited_file_path.exists() && rcu_normal_file_path.exists()
		{
			rcu_expedited_file_path.write_value(expedited)?;
			rcu_normal_file_path.write_value(normal)?;
		}
		Ok(())
	}
	
	#[inline(always)]
	fn rcu_expedited_file_path(sys_path: &SysPath) -> PathBuf
	{
		sys_path.kernel_file_path("rcu_expedited")
	}
	
	#[inline(always)]
	fn rcu_normal_file_path(sys_path: &SysPath) -> PathBuf
	{
		sys_path.kernel_file_path("rcu_normal")
	}
}
