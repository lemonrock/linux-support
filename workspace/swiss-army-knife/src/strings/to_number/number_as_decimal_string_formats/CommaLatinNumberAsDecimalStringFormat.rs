// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Latin digits 🄁 to 🄊 and − (unicode minus sign).
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct CommaLatinNumberAsDecimalStringFormat;

impl NumberAsDecimalStringFormat for CommaLatinNumberAsDecimalStringFormat
{
	const MaximumUtf8BytesPerDigit: usize = 4;
	
	const Negative: char = '−';
	
	const Zero: char = '🄁';
	
	const One: char = '🄂';
	
	const Two: char = '🄃';
	
	const Three: char = '🄄';
	
	const Four: char = '🄅';
	
	const Five: char = '🄆';
	
	const Six: char = '🄇';
	
	const Seven: char = '🄈';
	
	const Eight: char = '🄉';
	
	const Nine: char = '🄊';
}
