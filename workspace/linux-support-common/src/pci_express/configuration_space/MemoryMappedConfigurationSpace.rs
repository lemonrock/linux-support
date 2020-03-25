// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A memory-mapped PCI configuration space.
///
/// A more sophisticated model is available in 'stormmq-os'.
///
/// Size is either `linux/pci.h:PCI_CFG_SPACE_SIZE` or `linux/pci.h:PCI_HEADER_TYPE_CARDBUS` (although it can be overridden for broken chips to be much smaller).
#[derive(Debug)]
pub struct MemoryMappedConfigurationSpace(pub(crate) MemoryMappedFile);

impl Deref for MemoryMappedConfigurationSpace
{
	type Target = MemoryMappedFile;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl MemoryMappedConfigurationSpace
{
	const CommandRegister: usize = 0x04;

	/// Overwrite command register bits.
	#[inline(always)]
	pub fn enable_direct_memory_acess_bus_mastering(&self)
	{
		self.overwrite_command_register_bits(Command::BusMasterEnable)
	}

	/// Overwrite command register bits.
	#[inline(always)]
	pub fn overwrite_command_register_bits(&self, bits_to_set: Command)
	{
		self.write_command_register(self.read_command_register() | bits_to_set)
	}

	/// Read command register.
	#[inline(always)]
	pub fn read_command_register(&self) -> Command
	{
		self.get(Self::CommandRegister)
	}

	/// Write command register.
	#[inline(always)]
	pub fn write_command_register(&self, command: Command)
	{
		self.set(Self::CommandRegister, command)
	}
}
