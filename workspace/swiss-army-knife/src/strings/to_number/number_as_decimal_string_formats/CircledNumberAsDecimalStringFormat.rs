// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Circled digits ⓪ to ⑨ and − (unicode minus sign).
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct CircledNumberAsDecimalStringFormat;

impl NumberAsDecimalStringFormat for CircledNumberAsDecimalStringFormat
{
	const MaximumUtf8BytesPerDigit: usize = 3;
	
	const Negative: char = '−';
	
	const Zero: char = '⓪';
	
	const One: char = '①';
	
	const Two: char = '②';
	
	const Three: char = '③';
	
	const Four: char = '④';
	
	const Five: char = '⑤';
	
	const Six: char = '⑥';
	
	const Seven: char = '⑦';
	
	const Eight: char = '⑧';
	
	const Nine: char = '⑨';
}
