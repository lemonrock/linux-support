// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Adjusts TCP auto corking.
///
/// By default, auto corking is usually on (true).
///
/// Writes to `/proc/sys/net/ipv4/tcp_autocorking`.
#[inline(always)]
pub fn set_auto_corking(proc_path: &ProcPath, enable: bool) -> io::Result<()>
{
	assert_effective_user_id_is_root("write to /proc/sys/net/ipv4/tcp_autocorking");
	
	let file_path = proc_path.sys_net_ipv4_file_path("tcp_autocorking");
	if file_path.exists()
	{
		file_path.write_value(enable)
	}
	else
	{
		Ok(())
	}
}
