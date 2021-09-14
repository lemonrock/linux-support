// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Circled digits ⓿ to ❾ and − (unicode minus sign).
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct DingbatNegativeCircledSerifNumberAsDecimalStringFormat;

impl NumberAsDecimalStringFormat for DingbatNegativeCircledSerifNumberAsDecimalStringFormat
{
	const MaximumUtf8BytesPerDigit: usize = 3;

	const Negative: char = '−';
	
	const Zero: char = '⓿';
	
	const One: char = '❶';
	
	const Two: char = '❷';
	
	const Three: char = '❸';
	
	const Four: char = '❹';
	
	const Five: char = '❺';
	
	const Six: char = '❻';
	
	const Seven: char = '❼';
	
	const Eight: char = '❽';
	
	const Nine: char = '❾';
}
