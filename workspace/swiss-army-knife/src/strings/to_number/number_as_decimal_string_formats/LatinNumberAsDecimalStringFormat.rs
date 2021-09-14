// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Latin digits 0 to 9 and − (unicode minus sign).
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct LatinNumberAsDecimalStringFormat;

impl NumberAsDecimalStringFormat for LatinNumberAsDecimalStringFormat
{
	const MaximumUtf8BytesPerDigit: usize = 3;
	
	const Negative: char = '−';
	
	const Zero: char = '0';
	
	const One: char = '1';
	
	const Two: char = '2';
	
	const Three: char = '3';
	
	const Four: char = '4';
	
	const Five: char = '5';
	
	const Six: char = '6';
	
	const Seven: char = '7';
	
	const Eight: char = '8';
	
	const Nine: char = '9';
}
