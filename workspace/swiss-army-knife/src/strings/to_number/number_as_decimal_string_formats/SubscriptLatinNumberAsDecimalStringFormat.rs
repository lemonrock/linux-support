// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Latin digits ⁰ to ⁹ and ₋ (subscript minus sign).
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct SubscriptLatinNumberAsDecimalStringFormat;

impl NumberAsDecimalStringFormat for SubscriptLatinNumberAsDecimalStringFormat
{
	const MaximumUtf8BytesPerDigit: usize = 3;
	
	const Negative: char = '₋';
	
	const Zero: char = '₀';
	
	const One: char = '₁';
	
	const Two: char = '₂';
	
	const Three: char = '₃';
	
	const Four: char = '₄';
	
	const Five: char = '₅';
	
	const Six: char = '₆';
	
	const Seven: char = '₇';
	
	const Eight: char = '₈';
	
	const Nine: char = '₉';
}
