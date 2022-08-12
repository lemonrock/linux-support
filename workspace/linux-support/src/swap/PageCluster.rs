// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Page cluster: controls the number of pages up to which consecutive pages are read in from swap in a single attempt.
///
/// This is the swap counterpart to page cache readahead.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(i32)]
pub enum PageCluster
{
	/// Off.
	Off = 0,
	
	/// 2 pages.
	#[serde(rename = "2")] TwoPages = 1,
	
	/// 4 pages.
	#[serde(rename = "4")] FourPages = 2,
	
	/// 8 pages.
	///
	/// This is the default.
	#[serde(rename = "8")] EightPages = 3,
	
	/// 16 pages.
	#[serde(rename = "16")] SixteenPages = 4,
	
	/// 32 pages.
	#[serde(rename = "32")] ThirtyTwoPages = 5,
	
	/// 64 pages.
	#[serde(rename = "64")] SixtyFourPages = 6,
	
	/// 128 pages.
	#[serde(rename = "128")] OneHundredAndTwentyEightPages = 7,
	
	/// 256 pages.
	#[serde(rename = "256")] TwoHundredAndFiftySixPages = 8,
	
	/// 512 pages (one huge page on x86_64 and most other 64-bit systems).
	#[serde(rename = "512")] FiveHundredAndTwelvePages = 9,
}

impl Default for PageCluster
{
	#[inline(always)]
	fn default() -> Self
	{
		PageCluster::EightPages
	}
}

impl PageCluster
{
	/// Read `/proc/sys/vm/page-cluster`.
	///
	/// Default is 3.
	/// 0 disables.
	#[inline(always)]
	pub fn read(proc_path: &ProcPath) -> io::Result<Either<Self, i32>>
	{
		use self::PageCluster::*;
		
		let file_path = Self::file_path(proc_path);
		let common = match file_path.read_value()?
		{
			0 => Off,
			
			1 => TwoPages,
			
			2 => FourPages,
			
			3 => EightPages,
			
			4 => SixteenPages,
			
			5 => ThirtyTwoPages,
			
			6 => SixtyFourPages,
			
			7 => OneHundredAndTwentyEightPages,
			
			8 => TwoHundredAndFiftySixPages,
			
			9 => FiveHundredAndTwelvePages,
			
			other @ _ => return Ok(Right(other)),
		};
		Ok(Left(common))
	}
	
	/// Write `/proc/sys/vm/page-cluster`.
	///
	/// Default is 3.
	/// 0 disables.
	///
	/// Requires root.
	#[inline(always)]
	pub fn write(self, proc_path: &ProcPath) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write /proc/sys/vm/page-cluster");
		
		let file_path = Self::file_path(proc_path);
		if file_path.exists()
		{
			file_path.write_value(UnpaddedDecimalInteger(self as i32))
		}
		else
		{
			Ok(())
		}
	}
	
	#[inline(always)]
	fn file_path(proc_path: &ProcPath) -> PathBuf
	{
		proc_path.sys_vm_file_path("page-cluster")
	}
}
