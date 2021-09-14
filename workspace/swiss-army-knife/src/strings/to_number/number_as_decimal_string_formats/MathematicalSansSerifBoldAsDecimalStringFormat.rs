// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Mathematical digits 𝟬 to 𝟵 and ➖ (unicode bold minus sign).
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct MathematicalSansSerifBoldAsDecimalStringFormat;

impl NumberAsDecimalStringFormat for MathematicalSansSerifBoldAsDecimalStringFormat
{
	const MaximumUtf8BytesPerDigit: usize = 4;
	
	const Negative: char = '➖';
	
	const Zero: char = '𝟬';
	
	const One: char = '𝟭';
	
	const Two: char = '𝟮';
	
	const Three: char = '𝟯';
	
	const Four: char = '𝟰';
	
	const Five: char = '𝟱';
	
	const Six: char = '𝟲';
	
	const Seven: char = '𝟳';
	
	const Eight: char = '𝟴';
	
	const Nine: char = '𝟵';
}
