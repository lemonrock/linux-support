// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Mathematical digits 𝟎 to 𝟗 and ➖ (unicode bold minus sign).
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct MathematicalBoldAsDecimalStringFormat;

impl NumberAsDecimalStringFormat for MathematicalBoldAsDecimalStringFormat
{
	const MaximumUtf8BytesPerDigit: usize = 4;
	
	const Negative: char = '➖';
	
	const Zero: char = '𝟎';
	
	const One: char = '𝟏';
	
	const Two: char = '𝟐';
	
	const Three: char = '𝟑';
	
	const Four: char = '𝟒';
	
	const Five: char = '𝟓';
	
	const Six: char = '𝟔';
	
	const Seven: char = '𝟕';
	
	const Eight: char = '𝟖';
	
	const Nine: char = '𝟗';
}
