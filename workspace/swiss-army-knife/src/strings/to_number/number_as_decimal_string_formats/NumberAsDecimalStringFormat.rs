// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A decimal string format.
pub trait NumberAsDecimalStringFormat
{
	#[doc(hidden)]
	const MaximumUtf8BytesPerDigit: usize;
	
	#[doc(hidden)]
	const Negative: char;
	
	#[doc(hidden)]
	const Zero: char;
	
	#[doc(hidden)]
	const One: char;
	
	#[doc(hidden)]
	const Two: char;
	
	#[doc(hidden)]
	const Three: char;
	
	#[doc(hidden)]
	const Four: char;
	
	#[doc(hidden)]
	const Five: char;
	
	#[doc(hidden)]
	const Six: char;
	
	#[doc(hidden)]
	const Seven: char;
	
	#[doc(hidden)]
	const Eight: char;
	
	#[doc(hidden)]
	const Nine: char;
}
