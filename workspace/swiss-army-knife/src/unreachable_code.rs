// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Unreachable code.
///
/// A function that, in a release build, allows the compiler to optimize away the code path calling this function.
#[cfg(debug_assertions)]
#[inline(always)]
pub fn unreachable_code(explanation: fmt::Arguments) -> !
{
	unreachable!("{}", explanation)
}

/// Unreachable code.
///
/// A function that, in a release build, allows the compiler to optimize away the code path calling this function.
#[cfg(not(debug_assertions))]
#[inline(always)]
pub const fn unreachable_code(_explanation: fmt::Arguments) -> !
{
	unsafe { unreachable_unchecked() }
}
