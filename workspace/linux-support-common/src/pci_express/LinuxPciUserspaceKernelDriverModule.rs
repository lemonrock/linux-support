// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Linux PCI userspace kernel driver module.
///
/// Defaults to `uio_pci_generic`.
#[derive(Serialize, Deserialize)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LinuxPciUserspaceKernelDriverModule
{
	/// Generic Userspace Input/Output PCI driver.
	uio_pci_generic,

	/// Intel specific Userspace Input/Output PCI driver for their 1GbE controllers ('igb'); only shipped with DPDK.
	igb_uio,

	/// Intel specifc virtual function (VF) Userspace Input/Output PCI driver; only shipped with DPDK.
	vfio_pci,
}

impl Default for LinuxPciUserspaceKernelDriverModule
{
	#[inline(always)]
	fn default() -> Self
	{
		LinuxPciUserspaceKernelDriverModule::uio_pci_generic
	}
}

impl LinuxPciUserspaceKernelDriverModule
{
	#[inline(always)]
	pub(crate) fn linux_kernel_module_name(self) -> &'static LinuxKernelModuleName
	{
		self.linux_kernel_module().linux_kernel_module_name()
	}

	#[inline(always)]
	pub(crate) fn linux_kernel_module(self) -> &'static LinuxKernelModule<'static>
	{
		use self::LinuxPciUserspaceKernelDriverModule::*;

		match self
		{
			uio_pci_generic => LinuxKernelModule::uio_pci_generic(),
			igb_uio => LinuxKernelModule::igb_uio(),
			vfio_pci => LinuxKernelModule::vfio_pci(),
		}
	}
}
