// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Specifies a character name, or, for non-canonical mode, read time outs and minimum number of characters to be read.
#[derive(EnumIter, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(usize)]
pub enum Character
{
	#[cfg(target_os = "dragonfly")] VCHECKPT = VCHECKPT,

	/// Specifies the `DISCARD` character.
	///
	/// (Linux does not support this being the ASCII `SI` character (also known as `Control-O`)).
	///
	/// When the `MiscellaneousLocalModeFlag::ImplementationDefinedOutputProcessing` flag (`IEXTEN`) is set the `DISCARD` character will not be passed as input and input data will be discarded.
	/// Input will cease to be discarded when a second `DISCARD` character is received.
	#[cfg(any(target_os = "android", target_os = "dragonfly", target_os = "freebsd", target_os = "fuschia", target_os = "linux", target_os = "ios", target_os = "macos"))] DISCARD = VDISCARD,

	// ?linux
	/// (not in POSIX; not supported under Linux; 031, EM, Ctrl-Y) Delayed suspend character (DSUSP): send SIGTSTP signal when the character is read by the user program. Recognized when IEXTEN and ISIG are set, and the system supports job control, and then not passed as input.

	/// Specifies the delayed suspend `DSUSP` character.
	///
	/// (Linux does not support this being the ASCII `EM` character (also known as `Control-Y`)).
	///
	/// When the `MiscellaneousLocalModeFlag::ImplementationDefinedOutputProcessing` flag (`IEXTEN`) is set, `SignalRaising` is not `SignalRaising::Off`, job control is supported then the `DSUSP` character will not be passed as input and `SIGTSTP` is raised.
	#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos"))] DSUSP = VDSUSP,

	/// Specifies the end-of-file `EOF` character.
	///
	/// Usually this is the ASCII `EOT` character (also known as `Control-D`).
	///
	/// When the terminal mode is `TerminalMode::Canonical` the `EOF` character will not be passed as input and the pending tty buffer will be sent to the waiting user program without waiting for end-of-line.
	/// If it is the first character of the line, the `read()` in the user program returns 0, which signifies end-of-file.
	EOF = VEOF,

	/// Specifies an additional end-of-line character (to new line, `NF`).
	///
	/// Usually this is the ASCII `NUL` character.
	///
	/// When the terminal mode is `TerminalMode::Canonical` the `EOL2` character will not be passed as input.
	EOL2 = VEOL2,

	/// Specifies an additional end-of-line character (to new line, `NF`).
	///
	/// Usually this is the ASCII `NUL` character.
	///
	/// When the terminal mode is `TerminalMode::Canonical` the `EOL` character will not be passed as input.
	VEOL = VEOL,

	/// Specifies the `ERASE2` character.
	///
	///  When the terminal mode is `TerminalMode::Canonical` the `ERASE2` will not be passed as input and thes the previous not-yet-erased character will be erased unless it is beyond end-of-file or beginning-of-line.
	#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "openbsd"))] ERASE2 = VERASE2,

	/// Specifies the `ERASE` character (also known as 'rubout').
	///
	/// Usually this is the ASCII `DEL` character, but can also be `#`, `Control-H` and octal 10.
	///
	///  When the terminal mode is `TerminalMode::Canonical` the `ERASE` will not be passed as input and thes the previous not-yet-erased character will be erased unless it is beyond end-of-file or beginning-of-line.
	ERASE = VERASE,

	/// Specifies the interrupt `INTR` character.
	///
	/// Usually this is the ASCII `ETX` character (also known as `Control-C`).
	///
	/// When the `MiscellaneousLocalModeFlag::ImplementationDefinedOutputProcessing` flag (`IEXTEN`) is set and `SignalRaising` is not `SignalRaising::Off` then the `INTR` character will not be passed as input and `SIGINT` is raised.
	INTR = VINTR,

	/// Specifies the interrupt `KILL` character.
	///
	/// Usually this is the ASCII `NAK` character.
	///
	/// When the terminal mode is `TerminalMode::Canonical` then the `KILL` character will not be passed as input and the input is erased since the last end-of-file or beginning-of-line
	KILL = VKILL,

	/// Specifies the literal next `LNEXT` character.
	///
	/// Usually this is the ASCII `SYN` character.
	///
	/// When the `MiscellaneousLocalModeFlag::ImplementationDefinedOutputProcessing` flag (`IEXTEN`) is set then the `LNEXT` character will not be passed as input and the next character will be 'quoted' (escaped) to remove any special meaning.
	/// This probably can be used to pass data that otherwise be interpreted as `ERASE`, `KILL`, etc.
	LNEXT = VLNEXT,

	/// Specifies the `QUIT` character.
	///
	/// Usually this is the ASCII `FS` character (also known as `Control-\`).
	///
	/// When the `MiscellaneousLocalModeFlag::ImplementationDefinedOutputProcessing` flag (`IEXTEN`) is set and `SignalRaising` is not `SignalRaising::Off` then the `QUIT` character will not be passed as input and `SIGQUIT` is raised.
	QUIT = VQUIT,

	/// Specifies the `REPRINT` character.
	///
	/// When the `MiscellaneousLocalModeFlag::ImplementationDefinedOutputProcessing` flag (`IEXTEN`) is set and when the terminal mode is `TerminalMode::NonCanonical` then the `REPRINT` character will not be passed as input and unread characters will be reprinted.
	///
	/// See also the documentation of `MiscellaneousLocalModeFlag::ReprintUnreadInput`.
	REPRINT = VREPRINT,

	/// Specifies the `START` character for software flow control.
	///
	/// When the `InputModeFlag::EnableXOnXOffFlowControlOnOutput` flag (`IXON`) is set then the `START` character will not be passed as input and output will be restarted.
	START = VSTART,

	/// Specified the status request `STATUS` character.
	///
	/// The `STATUS` character will not be passed as input and:-
	///
	/// * Status information will displayed on the terminal, including state of foreground process and amount of CPU time it has consumed (unless the `MiscellaneousLocalModeFlag::PreventStatusCharacterFromPrintingInformation` flag is set).
	/// * On BSD systems, a `SIGINFO` signal will be sent to the foreground process group.
	#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos"))] STATUS = VSTATUS,

	/// Specifies the `STOP` character for software flow control.
	///
	/// When the `InputModeFlag::EnableXOnXOffFlowControlOnOutput` flag (`IXON`) is set then the `STOP` character will not be passed as input and output will be stopped.
	STOP = VSTOP,

	/// Specifies the suspend `SUSP` character.
	///
	/// Usually this is the ASCII `SUB` character (also known as `Control-Z`).
	///
	/// When the `MiscellaneousLocalModeFlag::ImplementationDefinedOutputProcessing` flag (`IEXTEN`) is set and `SignalRaising` is not `SignalRaising::Off` then the `SUSP` character will not be passed as input and `SIGSUSP` is raised.
	SUSP = VSUSP,

	/// Specifies the word erase `SWITCH` character.
	///
	/// This is a legacy feature.
	#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] SWITCH = VSWTC,

	/// Specifies the word erase `WERASE` character.
	///
	/// Usually this is the ASCII `ETB` character (also known as `Control-W`).
	///
	/// When the `MiscellaneousLocalModeFlag::ImplementationDefinedOutputProcessing` flag (`IEXTEN`) is set and when the terminal mode is `TerminalMode::NonCanonical` then the `WERASE` character will not be passed as input and a word will be erased.
	WERASE = VWERASE,

	/// When the terminal mode is `TerminalMode::NonCanonical` then this specifies the minimum number of characters for a read.
	ReadMinimumNumberOfCharacters = VMIN,

	/// When the terminal mode is `TerminalMode::NonCanonical` then this specifies the timeout in deciseconds for a read.
	ReadTimeOut = VTIME,
}
