// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Strictly speaking, this is a flags bit set but not all flag combinations make sense.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u8)]
pub enum NumaZoneReclaimMode
{
	/// Off.
	Off = 0,

	/// On.
	On = 1,

	/// On and write dirty pages out.
	OnWriteDirtyPagesOut = 1 | 2,

	/// On and swap pages.
	OnSwapPages = 1 | 4,

	/// On, write dirty pages out and swap pages.
	OnWriteDirtyPagesOutAndSwapPages = 1 | 2 | 4,
}

impl Default for NumaZoneReclaimMode
{
	#[inline(always)]
	fn default() -> Self
	{
		NumaZoneReclaimMode::Off
	}
}

impl NumaZoneReclaimMode
{
	/// Set value of `/proc/sys/vm/zone_reclaim_mode` if it exists.
	pub fn set(&self, proc_path: &ProcPath) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write to `/proc/sys/vm/zone_reclaim_mode`");
		
		let file_path = Self::file_path(proc_path);
		
		if file_path.exists()
		{
			file_path.write_value(self.to_unpadded_decimal_integer())
		}
		else
		{
			Ok(())
		}
	}
	
	#[inline(always)]
	fn to_unpadded_decimal_integer(self) -> UnpaddedDecimalInteger<u8>
	{
		UnpaddedDecimalInteger(self as u8)
	}
	
	#[inline(always)]
	fn file_path(proc_path: &ProcPath) -> PathBuf
	{
		proc_path.sys_vm_file_path("zone_reclaim_mode")
	}
}
