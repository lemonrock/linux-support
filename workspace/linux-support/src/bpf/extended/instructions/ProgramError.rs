// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An error when processing a list of `ProgramLine`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgramError
{
	/// More than 2^22 program lines.
	TooManyProgramLines,
	
	/// A line's column number exceeds the maximum.
	LineColumnNumberExceedsMaximum,
	
	/// Could not resolve a memory offset or an immediate.
	CouldNotResolveOffset,
	
	/// Not all offsets have resolved.
	NotAllOffsetsHaveBeenResolved(Vec<String>),
	
	/// Could not resolve a map file descriptor label.
	CouldNotResolveFileDescriptorLabel,
	
	/// Not all map file descriptor labels have been resolved.
	NotAllFileDescriptorLabelsHaveBeenResolved(Vec<String>),
	
	/// A jump or relative function jump offset can not be -1 as this creates a jump to the jump statement itself, thus creating an infinite loop.
	JumpOrRelativeFunctionOffsetOfNegativeOneCreatesAnInfiniteLoop,
	
	/// A jump offset must fit in an `i16`.
	JumpOffsetIsTooLargeForI16,
	
	/// Label names are referenced by relative function calls but no such name is ever defined in the program.
	SomeJumpLabelsAreUnresolved,
	
	/// A relative function jump offset must fit in an `i32`.
	RelativeFunctionOffsetIsTooLargeForI32,

	/// Relative function names are referenced by relative function calls but no such name is ever defined in the program.
	SomeRelativeFunctionNamesAreUnresolved,
	
	/// Modern Linux supports 1 million instructions.
	MaximumNumberOfInstructionsUsed,
	
	/// There must be at least one instruction.
	ThereAreNoInstructions,

	/// Linux can not support more than `u32::MAX` instructions.
	ThereAreMoreThanU32MaxInstructions,

	/// No type identifiers and no strings.
	NoBpfTypeFormatData,
	
	/// Too much BTF data to load.
	MaximumBpfTypeFormatDataSizeExceeded,
	
	/// Too much BTF data to load.
	CouldNotLoadBpfTypeFormatData(Errno),
	
	/// Invalid BTF.
	BpfTypeFormat(BpfTypeFormatError),
	
	/// Invalid program size.
	InvalidBpfTypeFormatDataSize(TryFromIntError),
	
	/// Could not resolve file descriptor.
	FileDescriptorMap(FileDescriptorsMapError),
}

impl Display for ProgramError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for ProgramError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(dyn error::Error + 'static)>
	{
		use self::ProgramError::*;
		
		match self
		{
			&BpfTypeFormat(ref error) => Some(error),
			
			&InvalidBpfTypeFormatDataSize(ref error) => Some(error),
			
			&FileDescriptorMap(ref error) => Some(error),
			
			_ => None,
		}
	}
}

impl From<BpfTypeFormatError> for ProgramError
{
	#[inline(always)]
	fn from(value: BpfTypeFormatError) -> Self
	{
		ProgramError::BpfTypeFormat(value)
	}
}

impl From<TryFromIntError> for ProgramError
{
	#[inline(always)]
	fn from(value: TryFromIntError) -> Self
	{
		ProgramError::InvalidBpfTypeFormatDataSize(value)
	}
}

impl From<FileDescriptorsMapError> for ProgramError
{
	#[inline(always)]
	fn from(value: FileDescriptorsMapError) -> Self
	{
		ProgramError::FileDescriptorMap(value)
	}
}
