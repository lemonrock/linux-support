// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A resource file.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct ResourceFile
{
	/// Zero-based index (internally, Linux uses an i32).
	pub index: u32,

	/// Is this a write-combining (WC) resource that can be prefeteched?
	pub write_combining: bool,
}

impl ResourceFile
{
	/// File path.
	#[inline(always)]
	pub fn file_path(&self, pci_device: &PciDevice) -> PathBuf
	{
		pci_device.device_file_or_folder_path(&format!("resource{}", self.index))
	}
	
	/// Write combining file path, if supported.
	#[inline(always)]
	pub fn write_combining_file_path(&self, pci_device: &PciDevice) -> Option<PathBuf>
	{
		if self.write_combining
		{
			Some(pci_device.device_file_or_folder_path(&format!("resource{}_wc", self.index)))
		}
		else
		{
			None
		}
	}
}
