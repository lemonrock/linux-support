// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Mathematical digits 𝟘 to 𝟡 and ➖ (unicode bold minus sign).
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct MathematicalDoubleStruckAsDecimalStringFormat;

impl NumberAsDecimalStringFormat for MathematicalDoubleStruckAsDecimalStringFormat
{
	const MaximumUtf8BytesPerDigit: usize = 4;
	
	const Negative: char = '➖';
	
	const Zero: char = '𝟘';
	
	const One: char = '𝟙';
	
	const Two: char = '𝟚';
	
	const Three: char = '𝟛';
	
	const Four: char = '𝟜';
	
	const Five: char = '𝟝';
	
	const Six: char = '𝟞';
	
	const Seven: char = '𝟟';
	
	const Eight: char = '𝟠';
	
	const Nine: char = '𝟡';
}
