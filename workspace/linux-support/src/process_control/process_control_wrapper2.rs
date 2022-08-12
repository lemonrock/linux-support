// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[inline(always)]
pub(crate) fn process_control_wrapper2<V, E>(operation: i32, arg2: usize, ok_handler: impl FnOnce(i32) -> Result<V, E>, err_handler: impl FnOnce(SystemCallErrorNumber) -> Result<V, E>) -> Result<V, E>
{
	process_control_wrapper(operation, arg2, 0, 0, 0, ok_handler, err_handler)
}
