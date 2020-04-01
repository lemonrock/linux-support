// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Redirect standard out and standard error to syslog.
#[inline(always)]
pub fn redirect_standard_out_and_standard_error_to_syslog()
{
	redirect_to_syslog(unsafe { &mut stdout }, write_standard_out_to_syslog);
	redirect_to_syslog(unsafe { &mut stderr }, write_standard_error_to_syslog);
}
