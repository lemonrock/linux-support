// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Latin digits 🄀 to ⒐ and − (unicode minus sign).
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct FullstopLatinNumberAsDecimalStringFormat;

// 🄁

impl NumberAsDecimalStringFormat for FullstopLatinNumberAsDecimalStringFormat
{
	const MaximumUtf8BytesPerDigit: usize = 4;
	
	const Negative: char = '−';
	
	const Zero: char = '🄀';
	
	const One: char = '⒈';
	
	const Two: char = '⒉';
	
	const Three: char = '⒊';
	
	const Four: char = '⒋';
	
	const Five: char = '⒌';
	
	const Six: char = '⒍';
	
	const Seven: char = '⒎';
	
	const Eight: char = '⒏';
	
	const Nine: char = '⒐';
}
