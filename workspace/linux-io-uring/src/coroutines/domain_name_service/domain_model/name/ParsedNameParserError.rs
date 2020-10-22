// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Error when parsing a name.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ParsedNameParserError
{
	/// The name occupies no bytes at all.
	NameIsEmpty,
	
	/// There is not a terminal root label in a Name.
	NoTerminalRootLabel,
	
	/// The extended name labels are unused.
	ExtendedNameLabelsAreUnused,
	
	/// The unallocated name labels are unused.
	UnallocatedNameLabelsAreUnused,
	
	/// Technically, this is permitted by RFC 1035 but in practice it is almost certainly either misconfiguration or an attempt to attack a code vulnerability.
	LabelContainsPeriod,
	
	/// When finishing a name combined from uncompressed labels and pointers, it creates a name longer than 255 bytes (including periods, including the trailing root period).
	LabelPointerCreatesADnsNameLongerThan255Bytes,
	
	/// When finishing a name combined from uncompressed labels and pointers, it creates a name longer than 127 labels.
	LabelPointerCreatesADnsNameLongerThan127Labels,
	
	/// When finishing a name combined from compressed labels and pointers, it creates a name longer than 255 bytes (including periods, including the trailing root period).
	CompressedLabelPointerCreatesADnsNameLongerThan255Bytes,
	
	/// When finishing a name combined from compressed labels and pointers, it creates a name longer than 127 labels.
	CompressedLabelPointerCreatesADnsNameLongerThan127Labels,
	
	/// A label length would cause overflow (ie it is too long).
	LabelLengthOverflows,
	
	/// A label pointer overflows (ie there isn't another byte for bottom 8 bits).
	LabelPointerOverflows,
	
	/// A label pointer points to data that is after the start of the currently being parsed name.
	LabelPointerPointsToDataAfterTheStartOfTheCurrentlyBeingParsedName,
	
	/// The label pointer offset does not point to a previously parsed label.
	///
	/// Note that this includes pointers to pointers.
	///
	/// The tuple contains the offset.
	LabelPointerPointsToALabelThatWasNotPreviouslyParsed(CompressedPointerOffset),
	
	/// Compressed name labels are disallowed in the query section.
	CompressedNameLabelsAreDisallowedInQuerySection,
	
	/// Compressed name labels are disallowed in this resource record.
	///
	/// See RFC 3597, Section 4 for some confusing rules.
	CompressedNameLabelsAreDisallowedInThisResourceRecord(DataType),
	
	/// The name was not long enough.
	///
	/// Typically this occurs when a name is shorter than the `RLEN/RDATA` space allocated for it in, say, a `CNAME` resource record.
	NameWasNotLongEnough,
}

impl Display for ParsedNameParserError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for ParsedNameParserError
{
}
