// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Latin digits ⁰ to ⁹ and ⁻ (superscript minus sign).
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct SuperscriptLatinNumberAsDecimalStringFormat;

impl NumberAsDecimalStringFormat for SuperscriptLatinNumberAsDecimalStringFormat
{
	const MaximumUtf8BytesPerDigit: usize = 3;
	
	const Negative: char = '⁻';
	
	const Zero: char = '⁰';
	
	const One: char = '¹';
	
	const Two: char = '²';
	
	const Three: char = '³';
	
	const Four: char = '⁴';
	
	const Five: char = '⁵';
	
	const Six: char = '⁶';
	
	const Seven: char = '⁷';
	
	const Eight: char = '⁸';
	
	const Nine: char = '⁹';
}
