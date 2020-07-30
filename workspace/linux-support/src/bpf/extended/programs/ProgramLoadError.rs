// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An error when loading a program.
#[derive(Debug)]
pub enum ProgramLoadError
{
	/// Parse error.
	Parse(ParseError),
	
	/// Linux can not support more than `u32::MAX` entries in the function information array.
	BpfTypeFormatFunctionInformationArrayIsLargerThanU32Max(TryFromIntError),
	
	/// Linux can not support more than `u32::MAX` entries in the line information array.
	BpfTypeFormatLineInformationArrayIsLargerThanU32Max(TryFromIntError),

	/// Invalid program.
	///
	/// Contains verifier log messages if a verifier log was used.
	InvalidProgram(Option<CString>),

	/// Not enough space for verifier log messages.
	NotEnoughSpaceForVerifierLogMessages,

	/// Not enough memory.
	OutOfMemoryOrResources,

	/// Caller lacks necessary capability.
	PermissionDenied,
	
	/// Could not register program.
	CouldNotRegisterProgram(FileDescriptorsMapAddError),
}

impl Display for ProgramLoadError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for ProgramLoadError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::ProgramLoadError::*;

		match self
		{
			&Parse(ref error) => Some(error),

			&BpfTypeFormatFunctionInformationArrayIsLargerThanU32Max(ref error) => Some(error),

			&BpfTypeFormatLineInformationArrayIsLargerThanU32Max(ref error) => Some(error),

			&CouldNotRegisterProgram(ref error) => Some(error),
			
			_ => None,
		}
	}
}

impl From<ParseError> for ProgramLoadError
{
	#[inline(always)]
	fn from(error: ParseError) -> Self
	{
		ProgramLoadError::Parse(error)
	}
}

impl From<FileDescriptorsMapAddError> for ProgramLoadError
{
	#[inline(always)]
	fn from(error: FileDescriptorsMapAddError) -> Self
	{
		ProgramLoadError::CouldNotRegisterProgram(error)
	}
}
