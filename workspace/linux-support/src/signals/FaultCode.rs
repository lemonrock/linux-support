// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Signal-specific fault code.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FaultCode
{
	/// `SIGILL`.
	SIGILL(IllegalInstructionCode),

	/// `SIGFPE`.
	SIGFPE(ArithmeticErrorCode),

	/// `SIGSEGV`.
	SIGSEGV(SegmentationFaultCode),

	/// `SIGTRAP`.
	SIGTRAP(TrapCode),

	/// `SIGEMT`.
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] SIGEMT(EmulatorTrapCode),

	/// `SIGBUS`.
	SIGBUS(BusCode),
}
