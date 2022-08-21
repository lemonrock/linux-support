// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Unreachable code with an explanation, typically from the result of a syscall or intrinsic.
///
/// A macro that, in a release build, allows the compiler to optimize away the code path calling this function
#[macro_export]
macro_rules! unexpected_result
{
	($function_name: tt, $variant_name: literal, $unexpected: expr) =>
	{
		{
			const function_name: &'static str = stringify!($function_name);
			unreachable_code(format_args!("Unexpected result `{}` from {} ({})", $unexpected, function_name, $variant_name))
		}
	};
	
	($function_name: tt, $variant_name: tt, $unexpected: expr) =>
	{
		{
			const function_name: &'static str = stringify!($function_name);
			const variant_name: &'static str = stringify!($variant_name);
			unreachable_code(format_args!("Unexpected result `{}` from {} ({})", $unexpected, function_name, variant_name))
		}
	};
	
	($function_name: tt, $unexpected: expr) =>
	{
		{
			const function_name: &'static str = stringify!($function_name);
			unreachable_code(format_args!("Unexpected result `{}` from {}", $unexpected, function_name))
		}
	};
	
	($function_name: literal, $unexpected: expr) =>
	{
		unreachable_code(format_args!("Unexpected result `{}` from {}", $unexpected, $function_name))
	};
}
