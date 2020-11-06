// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A protocol or resolution service parse error.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProtocolOrResolutionServiceParseError
{
	#[allow(missing_docs)]
	CanNotBeEmpty,
	
	#[allow(missing_docs)]
	CanNotExceed32Bytes(usize),
	
	#[allow(missing_docs)]
	FirstByteCanNotBeNumeric(u8),
	
	#[allow(missing_docs)]
	FirstByteOutOfRange(u8),
	
	#[allow(missing_docs)]
	SubsequentByteOutOfRange(u8, NonZeroU8),
	
	/// There was more than one resolution service of the same name once case-folded.
	DuplicateResolutionService,
	
	/// There was no protocol but a terminal flag was specified.
	ProtocolMustBeSpecifiedIfATerminalFlagIsSpecified,
}
