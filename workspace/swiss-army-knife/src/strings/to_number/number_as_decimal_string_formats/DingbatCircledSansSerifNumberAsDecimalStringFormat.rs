// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Circled digits 🄋 to ➈ and − (unicode minus sign).
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct DingbatCircledSansSerifNumberAsDecimalStringFormat;

impl NumberAsDecimalStringFormat for DingbatCircledSansSerifNumberAsDecimalStringFormat
{
	const MaximumUtf8BytesPerDigit: usize = 3;

	const Negative: char = '−';
	
	const Zero: char = '🄋';
	
	const One: char = '➀';
	
	const Two: char = '➁';
	
	const Three: char = '➂';
	
	const Four: char = '➃';
	
	const Five: char = '➄';
	
	const Six: char = '➅';
	
	const Seven: char = '➆';
	
	const Eight: char = '➇';
	
	const Nine: char = '➈';
}
