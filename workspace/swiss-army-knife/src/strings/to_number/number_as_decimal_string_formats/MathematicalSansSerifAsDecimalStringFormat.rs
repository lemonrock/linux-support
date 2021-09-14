// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Mathematical digits 𝟢 to 𝟫 and − (unicode minus sign).
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct MathematicalSansSerifAsDecimalStringFormat;

impl NumberAsDecimalStringFormat for MathematicalSansSerifAsDecimalStringFormat
{
	const MaximumUtf8BytesPerDigit: usize = 4;
	
	const Negative: char = '−';
	
	const Zero: char = '𝟢';
	
	const One: char = '𝟣';
	
	const Two: char = '𝟤';
	
	const Three: char = '𝟥';
	
	const Four: char = '𝟦';
	
	const Five: char = '𝟧';
	
	const Six: char = '𝟨';
	
	const Seven: char = '𝟩';
	
	const Eight: char = '𝟪';
	
	const Nine: char = '𝟫';
}
