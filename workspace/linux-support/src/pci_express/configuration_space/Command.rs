// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// See PCI Express® Base Specification Revision 3.0, Section 7.5.1.1 Command Register (Offset 04h).
	///
	/// Setting this register to the value `Command::empty()` causes the PCI device to be disconnected (configuration space access is still available).
	///
	/// The legacy bits 'Special Cycle Enable', 'Memory Write and Invalidate', 'VGA Palette Snoop', 'IDSEL Stepping/Wait Cycle Control' and 'Fast Back-to-Back Transactions Enable' are obsolete and hardwired to unset (clear).
	pub struct Command: u16
	{
		/// I/O space.
		///
		/// Controls a device's response to I/O (Port) Space accesses.
		/// A value of 0 disables the device response.
		/// A value of 1 allows the device to respond to I/O Space accesses.
		/// State after `RST#` is 0.
		const PortSpace = 1 << 0;

		/// Memory space.
		///
		/// Controls a device's response to Memory Space accesses.
		/// A value of 0 disables the device response.
		/// A value of 1 allows the device to respond to Memory Space accesses.
		/// State after `RST#` is 0
		const MemorySpace = 1 << 1;

		/// Bus Master Enable.
		///
		/// Read-write.
		///
		/// Controls the ability of a PCI Express Endpoint to issue Memory and I/O Read/Write Requests, and the ability of a Root or Switch Port to forward Memory and I/O Read/Write Requests in the Upstream direction.
		///
		/// ## Endpoints
		/// When this bit is set (`1`), the PCI Express Function is allowed to issue Memory or I/O Requests.
		/// When this bit is clear (`0`), the PCI Express Function is not allowed to issue any Memory or I/O Requests.
		/// Note that as MSI/MSI-X interrupt Messages are in-band memory writes, setting the Bus Master Enable bit to `0` disables MSI/MSI-X interrupt Messages as well.
		/// Requests other than Memory or I/O Requests are not controlled by this bit.
		/// Default value of this bit is `0`.
		/// This bit is hardwired to `0` if a Function does not generate Memory or I/O Requests.
		///
		/// ## Root and Switch Ports
		/// This bit controls forwarding of Memory or I/O Requests by a Switch or Root Port in the Upstream direction.
		/// When this bit is `0`, Memory and I/O Requests received at a Root Port or the Downstream side of a Switch Port must be handled as Unsupported Requests (UR), and for Non-Posted Requests a Completion with UR completion status must be returned.
		/// This bit does not affect forwarding of Completions in either the Upstream or Downstream direction.
		/// The forwarding of Requests other than Memory or I/O Requests is not controlled by this bit.
		/// Default value of this bit is `0`.
		const BusMasterEnable = 1 << 2;

		/// Parity Error Response.
		///
		/// Read-write.
		///
		/// This bit controls the logging of poisoned TLPs in the Master Data Parity Error bit in the Status register.
		/// A Root Complex Integrated Endpoint that is not associated with a Root Complex Event Collector is permitted to hardwire this bit to `0`.
		///
		/// Default value of this bit is `0`.
		const ParityErrorResponse = 1 << 6;

		/// SERR# Enable.
		///
		/// Read-write.
		///
		/// When Set, this bit enables reporting of Non-fatal and Fatal errors detected by the Function to the Root Complex.
		/// Note that errors are reported if enabled either through this bit or through the PCI Express specific bits in the Device Control register (see PCI Express® Base Specification Revision 3.0, Section 7.8.4).
		///
		/// In addition, for Functions with Type 1 Configuration Space headers, this bit controls transmission by the primary interface of `ERR_NONFATAL` and `ERR_FATAL` error messages forwarded from the secondary interface.
		/// This bit does not affect the transmission of forwarded `ERR_COR` messages.
		///
		/// A Root Complex Integrated Endpoint that is not associated with a Root Complex Event Collector is permitted to hardwire this bit to `0`.
		///
		/// Default value of this bit is `0`.
		const EnableSErrorNumber = 1 << 8;

		/// Interrupt Disable.
		///
		/// Read-write.
		///
		/// Controls the ability of a PCI Express Function to generate INTx interrupts.
		///
		/// When set, Functions are prevented from asserting `INTx` interrupts.
		///
		/// Any `INTx` emulation interrupts already asserted by the Function must be deasserted when this bit is set.
		///
		/// As described in PCI Express® Base Specification Revision 3.0, Section 2.2.8.1, `INTx` interrupts use virtual wires that must, if asserted, be deasserted using the appropriate `Deassert_INTx message`(s) when this bit is set.
		///
		/// Only the `INTx` virtual wire interrupt(s) associated with the Function(s) for which this bit is set are affected.
		///
		/// For Endpoints that generate `INTx` interrupts, this bit is required.
		///
		/// For Endpoints that do not generate `INTx` interrupts this bit is optional.
		/// If not implemented, this bit must be hardwired to `0`.
		///
		/// For Root Ports, Switch Ports, and Bridges that generate `INTx` interrupts on their own behalf, this bit is required.
		/// This bit has no effect on interrupts that pass through the Port from the secondary side.
		///
		/// For Root Ports, Switch Ports, and Bridges that do not generate `INTx` interrupts on their own behalf this bit is optional.
		/// If not implemented, this bit must be hardwired to `0`.
		/// Default value of this bit is `0`.
		const DisableInterrupts = 1 << 10;
	}
}
