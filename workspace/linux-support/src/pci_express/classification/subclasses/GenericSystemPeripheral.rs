// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum GenericSystemPeripheral
{
	/// Programmable Interrupt Controller (PIC).
	ProgrammableInterruptController(ProgrammableInterruptControllerGenericSystemPeripheralProgrammingInterface),
	
	/// Direct Memory Access (DMA) Controller.
	DirectMemoryAccessController(DirectMemoryAccessControllerGenericSystemPeripheralProgrammingInterface),

	/// System timer.
	SystemTimer(SystemTimerGenericSystemPeripheralProgrammingInterface),
	
	/// Real Time Clock (RTC) controller.
	RealTimeClockController(RealTimeClockControllerGenericSystemPeripheralProgrammingInterface),

	PciHotPlugController,
	
	/// SD HostController.
	SdHostController,
	
	/// IOMMU.
	Iommu,

	/// Root complex event collector.
	///
	/// # Note
	/// Some versions of the PCI Express Base Specification defined Root Complex Event Collectors to use Sub-class 06h.
	/// Implementations are permitted to use Sub-class 06h for this purpose, but this practice is strongly discouraged.
	/// The Device/Port Type field value can be used to accurately identify all Root Complex Event Collectors.
	RootComplexEventCollector,
	
	/// Other.
	Other,
}

impl GenericSystemPeripheral
{
	#[inline(always)]
	pub(crate) fn programming_interface(self) -> u8
	{
		use self::GenericSystemPeripheral::*;

		match self
		{
			ProgrammableInterruptController(programming_interface) => programming_interface as u8,
			DirectMemoryAccessController(programming_interface) => programming_interface as u8,
			SystemTimer(programming_interface) => programming_interface as u8,
			RealTimeClockController(programming_interface) => programming_interface as u8,

			_ => 0x00,
		}
	}

	#[inline(always)]
	pub(crate) fn parse(value: u8, programming_interface: u8) -> Option<Self>
	{
		use self::GenericSystemPeripheral::*;

		match value
		{
			0x00 => programming_interface!(ProgrammableInterruptController, programming_interface, ProgrammableInterruptControllerGenericSystemPeripheralProgrammingInterface),
			0x01 => programming_interface!(DirectMemoryAccessController, programming_interface, DirectMemoryAccessControllerGenericSystemPeripheralProgrammingInterface),
			0x02 => programming_interface!(SystemTimer, programming_interface, SystemTimerGenericSystemPeripheralProgrammingInterface),
			0x03 => programming_interface!(RealTimeClockController, programming_interface, RealTimeClockControllerGenericSystemPeripheralProgrammingInterface),
			0x04 => zero_programming_interface!(PciHotPlugController, programming_interface),
			0x05 => zero_programming_interface!(SdHostController, programming_interface),
			0x06 => zero_programming_interface!(Iommu, programming_interface),
			0x07 => zero_programming_interface!(RootComplexEventCollector, programming_interface),
			0x80 => zero_programming_interface!(Other, programming_interface),
			_ => None,
		}
	}

	#[inline(always)]
	pub(crate) fn minor(self) -> u8
	{
		use self::GenericSystemPeripheral::*;

		match self
		{
			ProgrammableInterruptController(_) => 0x00,
			DirectMemoryAccessController(_) => 0x01,
			SystemTimer(_) => 0x02,
			RealTimeClockController(_) => 0x03,
			PciHotPlugController => 0x04,
			SdHostController => 0x05,
			Iommu => 0x06,
			RootComplexEventCollector => 0x07,
			Other => 0x80,
		}
	}
}
