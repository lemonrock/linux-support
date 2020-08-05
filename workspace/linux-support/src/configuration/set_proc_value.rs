// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[inline(always)]
fn set_proc_value<'a, E: error::Error>(file_function: impl FnOnce(&ProcPath, &str) -> PathBuf, proc_path: &ProcPath, file_name: &'static str, value: Option<impl IntoLineFeedTerminatedByteString<'a>>, error: impl FnOnce(io::Error) -> E) -> Result<(), E>
{
	assert_effective_user_id_is_root("write to `/proc/sys`");
	
	if let Some(value) = value
	{
		let file_path = file_function(proc_path, file_name);
		if file_path.exists()
		{
			return file_path.write_value(value).map_err(error)
		}
	}
	
	Ok(())
}
