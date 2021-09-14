// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Mathematical digits 𝟶 to 𝟿 and − (unicode minus sign).
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct MathematicalMonospaceAsDecimalStringFormat;

impl NumberAsDecimalStringFormat for MathematicalMonospaceAsDecimalStringFormat
{
	const MaximumUtf8BytesPerDigit: usize = 4;
	
	const Negative: char = '−';
	
	const Zero: char = '𝟶';
	
	const One: char = '𝟷';
	
	const Two: char = '𝟸';
	
	const Three: char = '𝟹';
	
	const Four: char = '𝟺';
	
	const Five: char = '𝟻';
	
	const Six: char = '𝟼';
	
	const Seven: char = '𝟽';
	
	const Eight: char = '𝟾';
	
	const Nine: char = '𝟿';
}
