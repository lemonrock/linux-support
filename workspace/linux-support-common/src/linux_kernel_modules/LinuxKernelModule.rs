// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A Linux kernel module.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LinuxKernelModule<'depends>
{
	linux_kernel_module_name: LinuxKernelModuleName,
	linux_kernel_module_file_base_name: LinuxKernelModuleFileBaseName,
	depends_on: Option<&'depends Self>,
}

impl LinuxKernelModule<'static>
{
	#[inline(always)]
	fn new_without_dependencies(name: &'static [u8], file_base_name: &'static [u8]) -> Self
	{
		Self::new(name, file_base_name, None)
	}

	#[inline(always)]
	fn new_with_dependencies(name: &'static [u8], file_base_name: &'static [u8], depends_on: &'static Self) -> Self
	{
		Self::new(name, file_base_name, Some(depends_on))
	}

	#[inline(always)]
	fn new(name: &'static [u8], file_base_name: &'static [u8], depends_on: Option<&'static Self>) -> Self
	{
		Self
		{
			linux_kernel_module_name: name.into(),
			linux_kernel_module_file_base_name: file_base_name.into(),
			depends_on,
		}
	}

	/// UIO kernel module.
	#[inline(always)]
	pub fn uio() -> &'static Self
	{
		lazy_static!
		{
    		static ref uio: LinuxKernelModule<'static> = LinuxKernelModule::new_without_dependencies(b"uio", b"uio");
    	}

		&uio
	}

	/// Intel 1GbE kernel module.
	///
	/// Only supplied with DPDK.
	#[inline(always)]
	pub fn igb_uio() -> &'static Self
	{
		lazy_static!
		{
    		static ref igb_uio: LinuxKernelModule<'static> = LinuxKernelModule::new_with_dependencies(b"igb_uio", b"igb_uio", LinuxKernelModule::<'static>::uio());
    	}

		&igb_uio
	}

	/// UIO PCI Generic kernel module.
	#[inline(always)]
	pub fn uio_pci_generic() -> &'static Self
	{
		lazy_static!
		{
    		static ref uio_pci_generic: LinuxKernelModule<'static> = LinuxKernelModule::new_with_dependencies(b"uio_pci_generic", b"uio_pci_generic", LinuxKernelModule::<'static>::uio());
    	}

		&uio_pci_generic
	}

	/// VFIO (virtual function I/O) PCI kernel module.
	///
	/// Only supplied with DPDK.
	#[inline(always)]
	pub fn vfio_pci() -> &'static Self
	{
		lazy_static!
		{
			// Note that there is a hyphen deliberately in the first argument (`vfio-pci`).
			// It would be nice if developers could be more consistent...
    		static ref vfio_pci: LinuxKernelModule<'static> = LinuxKernelModule::new_without_dependencies(b"vfio-pci", b"vfio_pci");
    	}

		&vfio_pci
	}
}

impl<'depends> LinuxKernelModule<'depends>
{
	/// Loads a Linux Kernel Module.
	///
	/// Does not use `modprobe`.
	///
	/// Returns true if loaded.
	/// Returns false if permissions error occurred (eg was not root).
	pub fn load_linux_kernel_module_from_ko_file(&self, linux_kernel_modules_path: &Path) -> Result<bool, io::Error>
	{
		self.linux_kernel_module_file_base_name.load_linux_kernel_module_from_ko_file(linux_kernel_modules_path)
	}

	/// Loads a Linux Kernel Module.
	///
	/// Uses the `modprobe` binary.
	pub fn load_linux_kernel_module_using_modprobe(&self) -> Result<(), ModProbeError>
	{
		self.linux_kernel_module_name.load_linux_kernel_module_using_modprobe()
	}

	/// Name.
	#[inline(always)]
	pub fn linux_kernel_module_name(&self) -> &LinuxKernelModuleName
	{
		&self.linux_kernel_module_name
	}

	/// File base name.
	#[inline(always)]
	pub fn linux_kernel_module_file_base_name(&self) -> &LinuxKernelModuleFileBaseName
	{
		&self.linux_kernel_module_file_base_name
	}
}
