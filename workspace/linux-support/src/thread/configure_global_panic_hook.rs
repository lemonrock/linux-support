// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Should only ever be invoked *once* from the main thread that started the process.
///
/// Does not terminate the process unless the panic hook itself panicked (a double panic), in which case the exit code is `71` (BSD exit code `EX_OSERR`) and the proces *immediately exits* without clean up.
#[inline(always)]
pub fn configure_global_panic_hook(terminate: &Arc<impl Terminate + 'static>)
{
	const EX_OSERR: i32 = 71;

	let terminate = terminate.clone();
	set_hook(Box::new(move |panic_info|
	{
		if thread::panicking()
		{
			process::exit(EX_OSERR)
		}

		terminate.begin_termination_due_to_panic(panic_info)
	}));
}
