// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Port identifier; a process identifier, where zero (`0`) is the Linux kernel.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PortIdentifier(Option<ProcessIdentifier>);

impl Display for PortIdentifier
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		match self.0
		{
			None => write!(f, "0"),
			Some(process_identifier) => write!(f, "{}", process_identifier)
		}
	}
}

impl PortIdentifier
{
	/// Kernel.
	pub const LinuxKernel: Self = Self(None);
	
	/// Possibly ***SLOW**: Makes a syscall on first call.
	///
	/// Broken if the process is forked, as will cache parent's PID.
	pub fn current_process() -> Self
	{
		static mut CurrentProcess: PortIdentifier = Self(None);
		unsafe
		{
			// Thread safe; initialization happens more than once but there are not side effects unless the process is forked on a thread whilst initialization occurs.
			if unlikely!(CurrentProcess.0.is_none())
			{
				let current = Self(Some(ProcessIdentifier::default()));
				CurrentProcess = current;
				current
			}
			else
			{
				CurrentProcess
			}
		}
	}
	
	/// Is from Linux kernel?
	#[inline(always)]
	pub fn is_from_linux_kernel(self) -> bool
	{
		self.0.is_none()
	}
	
	/// Is from user space?
	#[inline(always)]
	pub fn is_from_user_space(self) -> bool
	{
		self.0.is_some()
	}
	
	/// Is from our process?
	///
	/// Possibly ***SLOW**: Makes a syscall on first call.
	///
	/// Broken if the process is forked, as will cache parent's PID.
	#[inline(always)]
	pub fn is_from_current_process(self) -> bool
	{
		self == Self::current_process()
	}
}
