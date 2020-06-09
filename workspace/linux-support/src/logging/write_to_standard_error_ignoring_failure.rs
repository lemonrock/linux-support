// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Does not do heap allocation.
///
/// Based on Rust standard library's internal function `dumb_print()`.
///
/// Note that internally `Stderr.write_fmt()` takes `.lock()`.
#[inline(always)]
pub fn write_to_standard_error_ignoring_failure(format_arguments: Arguments)
{
	write_to_standard_stream_ignoring_failure(format_arguments, &mut stderr())
}
