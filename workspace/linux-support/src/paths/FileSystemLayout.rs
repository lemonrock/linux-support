// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// File system layout.
///
/// Defaults to `/sys`, `/proc`, `/dev` and `/etc`.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct FileSystemLayout
{
	#[allow(missing_docs)]
	pub sys_path: SysPath,
	
	#[allow(missing_docs)]
	pub proc_path: ProcPath,
	
	#[allow(missing_docs)]
	pub dev_path: DevPath,
	
	#[allow(missing_docs)]
	pub etc_path: EtcPath,
}

impl FileSystemLayout
{
	/// Paths.
	#[inline(always)]
	pub fn paths(&self) -> (&SysPath, &ProcPath, &DevPath, &EtcPath)
	{
		(
			&self.sys_path,
			&self.proc_path,
			&self.dev_path,
			&self.etc_path,
		)
	}
	
	/// Defaults for page sizes.
	#[inline(always)]
	pub fn defaults(&self) -> DefaultPageSizeAndHugePageSizes
	{
		DefaultPageSizeAndHugePageSizes::new(&self.sys_path, &self.proc_path)
	}
}
