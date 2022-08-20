// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// An error that can in a terminal.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TerminalSettingsError
{
	/// Not a terminal.
	NotATerminal(SystemCallErrorNumber),

	/// Could not set terminal attributes.
	CouldNotSetTerminalAttributes(SystemCallErrorNumber),
}

impl Display for TerminalSettingsError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<TerminalSettingsError as Debug>::fmt(self, f)
	}
}

impl error::Error for TerminalSettingsError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(dyn error::Error + 'static)>
	{
		use self::TerminalSettingsError::*;

		match self
		{
			NotATerminal(_) => None,

			CouldNotSetTerminalAttributes(_) => None,
		}
	}
}
