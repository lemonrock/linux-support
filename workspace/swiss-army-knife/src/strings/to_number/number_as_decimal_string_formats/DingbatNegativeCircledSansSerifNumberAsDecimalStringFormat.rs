// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Circled digits 🄌 to ➒ and − (unicode minus sign).
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct DingbatNegativeCircledSansSerifNumberAsDecimalStringFormat;

impl NumberAsDecimalStringFormat for DingbatNegativeCircledSansSerifNumberAsDecimalStringFormat
{
	const MaximumUtf8BytesPerDigit: usize = 3;

	const Negative: char = '−';
	
	const Zero: char = '🄌';
	
	const One: char = '➊';
	
	const Two: char = '➋';
	
	const Three: char = '➌';
	
	const Four: char = '➍';
	
	const Five: char = '➎';
	
	const Six: char = '➏';
	
	const Seven: char = '➐';
	
	const Eight: char = '➑';
	
	const Nine: char = '➒';
}
