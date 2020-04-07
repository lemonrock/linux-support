// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Represents a character device for reading and writing to.
///
/// A character device can be a (USB) serial port.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TerminalFileDescriptor(CharacterDeviceFileDescriptor);

impl AsRawFd for TerminalFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0.as_raw_fd()
	}
}

impl AsRawFdExt for TerminalFileDescriptor
{
}

impl IntoRawFd for TerminalFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.0.into_raw_fd()
	}
}

impl FromRawFd for TerminalFileDescriptor
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(CharacterDeviceFileDescriptor::from_raw_fd(fd))
	}
}

impl SpliceRecipient for TerminalFileDescriptor
{
}

impl SpliceSender for TerminalFileDescriptor
{
}

impl VectoredRead for TerminalFileDescriptor
{
}

impl VectoredWrite for TerminalFileDescriptor
{
}

impl Read for TerminalFileDescriptor
{
	/// This particular implementation can only return an `io::ErrorKind` of:-
	///
	/// * `UnexpectedEof`
	/// * `WouldBlock`
	/// * `Interrupted`
	/// * `Other` (which is for when the kernel reports `ENOMEM`, ie it is out of memory).
	#[inline(always)]
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>
	{
		self.0.read(buf)
	}

	#[inline(always)]
	unsafe fn initializer(&self) -> Initializer
	{
		self.0.initializer()
	}
}

impl Write for TerminalFileDescriptor
{
	/// This particular implementation can only return an `io::ErrorKind` of:-
	///
	/// * `WriteZero` (implying end-of-file).
	/// * `WouldBlock`
	/// * `Interrupted`
	/// * `BrokenPipe`
	#[inline(always)]
	fn write(&mut self, buf: &[u8]) -> io::Result<usize>
	{
		self.0.write(buf)
	}

	#[inline(always)]
	fn flush(&mut self) -> io::Result<()>
	{
		let result = unsafe { tcdrain(self.as_raw_fd()) };

		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable!()
		}
	}
}

impl TerminalFileDescriptor
{
	/// Opens a terminal character device named in the file system suitable for sending data to.
	///
	/// Sadly, there is no way to atomically detect if the provided path is **not** a terminal character device.
	#[inline(always)]
	pub fn open_terminal_character_device(terminal_character_device_file_path: impl AsRef<Path>, terminal_settings: &TerminalSettings) -> Result<Self, SpecialFileOpenError>
	{
		let this = Self(CharacterDeviceFileDescriptor::open_character_device_internal(terminal_character_device_file_path, O_NOCTTY)?);

		this.change_terminal_settings(terminal_settings, WhenToChangeTerminalAttributes::Now, false).map_err(|terminal_settings_error| SpecialFileOpenError::Terminal(terminal_settings_error))?;

		Ok(this)
	}

	/// Changes terminal settings.
	///
	/// `ignore_control_flags` is only honoured on BSD systems; it is equivalent to setting `MiscellaneousControlModeFlag::Ignore` flag (`CIGNORE`).
	#[inline(always)]
	pub fn change_terminal_settings(&self, terminal_settings: &TerminalSettings, when: WhenToChangeTerminalAttributes, ignore_control_flags: bool) -> Result<(), TerminalSettingsError>
	{
		let mut terminal_options = self.get_terminal_settings()?;

		terminal_settings.change_settings(&mut terminal_options);

		Self::handle_terminal_settings_error(unsafe { tcsetattr(self.as_raw_fd(), when.flags(ignore_control_flags), &terminal_options) }, TerminalSettingsError::CouldNotSetTerminalAttributes)
	}

	/// Current terminal settings.
	#[inline(always)]
	pub fn current_terminal_settings(&self) -> Result<CurrentTerminalSettings, TerminalSettingsError>
	{
		self.get_terminal_settings().map(|terminal_options| CurrentTerminalSettings(terminal_options))
	}

	#[inline(always)]
	fn get_terminal_settings(&self) -> Result<termios, TerminalSettingsError>
	{
		#[allow(deprecated)]
		let mut terminal_options: termios = unsafe { uninitialized() };

		Self::handle_terminal_settings_error(unsafe { tcgetattr(self.as_raw_fd(), &mut terminal_options) }, TerminalSettingsError::NotATerminal)?;

		Ok(terminal_options)
	}

	/// Discard input.
	#[inline(always)]
	pub fn discard_input_received_but_not_read(&self) -> io::Result<()>
	{
		Self::handle_generic_io_error(unsafe { tcflush(self.as_raw_fd(), TCIFLUSH) })
	}

	/// Discard output.
	#[inline(always)]
	pub fn discard_output_written_but_not_transmitted(&self) -> io::Result<()>
	{
		Self::handle_generic_io_error(unsafe { tcflush(self.as_raw_fd(), TCOFLUSH) })
	}

	/// Discard input and output.
	#[inline(always)]
	pub fn discard_input_received_but_not_read_and_output_written_but_not_transmitted(&self) -> io::Result<()>
	{
		Self::handle_generic_io_error(unsafe { tcflush(self.as_raw_fd(), TCIOFLUSH) })
	}

	/// Sends a break (a stream of zero bits) for between 0.25 and 0.5 seconds.
	#[inline(always)]
	pub fn send_break(&self) -> io::Result<()>
	{
		// Note that Linux does not support an implementation-defined (non-zero) value; see the inner logic of musl at <https://github.com/bminor/musl/blob/05ac345f895098657cf44d419b5d572161ebaf43/src/termios/tcsendbreak.c>.
		Self::handle_generic_io_error(unsafe { tcsendbreak(self.as_raw_fd(), 0) })
	}

	/// Suspends output.
	///
	/// Does not use a guard and move self as in practice a file descriptor will be held in an arena, Vec, etc and so can not be moved.
	#[inline(always)]
	pub fn suspend_output(&self) -> io::Result<()>
	{
		Self::handle_generic_io_error(unsafe { tcflow(self.as_raw_fd(), TCOOFF) })
	}

	/// Resumes output.
	///
	/// Should not be used unless `suspend_output()` has been called exactly once before.
	#[inline(always)]
	pub fn resume_output(&self) -> io::Result<()>
	{
		Self::handle_generic_io_error(unsafe { tcflow(self.as_raw_fd(), TCOON) })
	}

	/// Transmits a STOP character, which stops the terminal device from transmitting data to the system, ie suspends input.
	#[inline(always)]
	pub fn stop(&self) -> io::Result<()>
	{
		Self::handle_generic_io_error(unsafe { tcflow(self.as_raw_fd(), TCIOFF) })
	}

	/// Transmits a START character, which starts the terminal device transmitting data to the system, ie resumes input.
	///
	/// Should not be used unless `start()` has been called exactly once before.
	#[inline(always)]
	pub fn start(&self) -> io::Result<()>
	{
		Self::handle_generic_io_error(unsafe { tcflow(self.as_raw_fd(), TCION) })
	}

	/// Obtains the session identifier ('sid'); not normally available for serial ports.
	#[inline(always)]
	pub fn session_identifier(&self) -> io::Result<ProcessGroupIdentifier>
	{
		let result = unsafe { tcgetsid(self.0.as_raw_fd()) };
		if unlikely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			ProcessGroupIdentifier::try_from(result).map_err(|error| io::Error::new(ErrorKind::Other, error))
		}
	}

	#[inline(always)]
	fn handle_terminal_settings_error(result: c_int, constructor: impl FnOnce(Errno) -> TerminalSettingsError) -> Result<(), TerminalSettingsError>
	{
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			Err(constructor(errno()))
		}
		else
		{
			unreachable!()
		}
	}

	#[inline(always)]
	fn handle_generic_io_error(result: c_int) -> io::Result<()>
	{
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable!()
		}
	}
}
