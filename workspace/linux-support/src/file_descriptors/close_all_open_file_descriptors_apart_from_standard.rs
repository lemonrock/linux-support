// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Closes all open file descriptors apart from standard in, standard out and standard error.
#[inline(always)]
pub fn close_all_open_file_descriptors_apart_from_standard(proc_path: &ProcPath) -> io::Result<()>
{
	let folder_path = proc_path.process_file_path(ProcessIdentifierChoice::Current, "fd");
	for file in folder_path.read_dir().unwrap()
	{
		let file = file?;
		// NOTE: We do not parse as `RawFd::from_bytes()` because `RawFd` is an `i32` yet file descriptors can only be positive!
		let raw_file_descriptor = u32::from_bytes(file.file_name().as_bytes()).map_err(|parse_number_error| io::Error::new(ErrorKind::Other, parse_number_error))? as RawFd;
		if raw_file_descriptor > 2
		{
			unsafe { close(raw_file_descriptor) };
		}
	}
	Ok(())
}
