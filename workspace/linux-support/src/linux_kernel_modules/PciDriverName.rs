// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A PCI driver name.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct PciDriverName(DriverName);

impl From<DriverName> for PciDriverName
{
	#[inline(always)]
	fn from(value: DriverName) -> Self
	{
		Self(value)
	}
}

impl<'a> IntoLineFeedTerminatedByteString<'a> for &'a PciDriverName
{
	#[inline(always)]
	fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>
	{
		self.0.into_line_feed_terminated_byte_string()
	}
}

impl PciDriverName
{
	/// Force a driver to bind to a particular device.
	#[inline(always)]
	pub(crate) fn bind(&self, sys_path: &SysPath, pci_device_address: PciDeviceAddress)
	{
		self.driver_file_path(sys_path, "bind").write_value(pci_device_address).unwrap()
	}

	/// Force a driver to unbind from a particular device.
	#[inline(always)]
	pub(crate) fn unbind(&self, sys_path: &SysPath, pci_device_address: PciDeviceAddress)
	{
		self.driver_file_path(sys_path, "unbind").write_value(pci_device_address).unwrap()
	}

	#[inline(always)]
	fn driver_file_path(&self, sys_path: &SysPath, file_name: &str) -> PathBuf
	{
		let os_string: OsString = (&self.0).into();
		sys_path.pci_driver_folder_path(os_string).append(file_name)
	}
}
