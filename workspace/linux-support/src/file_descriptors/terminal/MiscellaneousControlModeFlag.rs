// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Miscellaneous control mode flags.
#[derive(EnumIter, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(not(any(target_os = "ios", target_os = "macos")), repr(u32))]
#[cfg_attr(all(any(target_os = "ios", target_os = "macos"), target_pointer_width = "32"), repr(u32))]
#[cfg_attr(all(any(target_os = "ios", target_os = "macos"), target_pointer_width = "64"), repr(u64))]
pub enum MiscellaneousControlModeFlag
{
	/// Enable receiver.
	///
	/// If set, the receiver is enabled, and characters can be received.
	EnableReceiver = CREAD,

	/// Hang up on last close.
	///
	/// If set, the modem control lines are lowered (ie, the modem connection is broken) when the last process closes the device.
	HangUpOnLastClose = HUPCL,

	/// Ignore modem status lines (actually, ignores only the `CD` signal).
	///
	/// This usually means that the device is directly attached.
	/// When this flag is not set, an open of a terminal device usually blocks until the modem answers a call and establishes a connection, for example.
	IgnoreModemStatusLines = CLOCAL,

	/// Ignore control flags.
	#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] Ignore = CIGNORE,

	/// Enable hardware flow control of the input and output, using the Clear-To-Send (`CTS`) and Request-To-Send (`RTS`) RS-232 signals.
	///
	/// Can be separated into `RequestToSendFlowControlOfInput` and `ClearToSendFlowControlOfOutput` on some platforms.
	RequestToSendClearToSendFlowControlOfInputAndOutput = CRTSCTS,

	/// Enable hardware flow control of the output using the Clear-To-Send (`CTS`) RS-232 signal.
	#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos"))] ClearToSendFlowControlOfOutput = CCTS_OFLOW,

	/// Enable hardware flow control of the input using the Request-To-Send (`RTS`) RS-232 signal.
	#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos"))] RequestToSendFlowControlOfInput = CRTS_IFLOW,

	/// Enable hardware flow control of the input according to the Data-Terminal-Ready (`DTR`) RS-232 signal.
	#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos"))] DataTerminalReadyFlowControlOfInput = CDTR_IFLOW,

	/// Enable hardware flow control of the output according to the Data-Set-Ready (`DSR`) RS-232 signal.
	#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos"))] DataSetReadyFlowControlOfOutput = CDSR_OFLOW,

	/// Enable hardware flow control of the output using the Data-Carrier-Detect (`DCD`, also known as `CD`) RS-232 modem carrier signal.
	#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos"))] DataCarrierDetectFlowControlOfOutput = CCAR_OFLOW,
	#[cfg(target_os = "openbsd")] DataCarrierDetectFlowControlOfOutput = MDMBUF,
}

impl MiscellaneousControlModeFlag
{
	/// Enable hardware flow control of the output using the Clear-To-Send (`CTS`) RS-232 signal.
	#[cfg(target_os = "openbsd")] pub const ClearToSendFlowControlOfOutput: Self = MiscellaneousControlModeFlag::CRTSCTS;

	/// Enable hardware flow control of the input using the Request-To-Send (`RTS`) RS-232 signal.
	#[cfg(target_os = "openbsd")] pub const RequestToSendFlowControlOfInput: Self = MiscellaneousControlModeFlag::CRTSCTS;
}
