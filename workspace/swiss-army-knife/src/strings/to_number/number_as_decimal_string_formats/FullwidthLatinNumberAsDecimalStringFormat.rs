// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Fullwidth Latin digits ０ to ９ and － (fullwidth hyphen minus).
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct FullwidthLatinNumberAsDecimalStringFormat;

impl NumberAsDecimalStringFormat for FullwidthLatinNumberAsDecimalStringFormat
{
	const MaximumUtf8BytesPerDigit: usize = 3;
	
	const Negative: char = '－';
	
	const Zero: char = '０';
	
	const One: char = '１';
	
	const Two: char = '２';
	
	const Three: char = '３';
	
	const Four: char = '４';
	
	const Five: char = '５';
	
	const Six: char = '６';
	
	const Seven: char = '７';
	
	const Eight: char = '８';
	
	const Nine: char = '９';
}
