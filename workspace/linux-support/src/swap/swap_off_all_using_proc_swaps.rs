// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// `swapoff -a`.
pub fn swap_off_all_using_proc_swaps(proc_path: &ProcPath) -> io::Result<()>
{
	let mut bytes = proc_path.file_path("swaps").read_raw()?;
	let mut lines = bytes.split_mut(|byte| *byte == b'\n');
	let _header = lines.next().unwrap();
	for line in lines
	{
		let index = memchr(b' ', line).ok_or(io::Error::new(ErrorKind::Other, "Path field not terminated by spaces"))?;
		unsafe { *line.get_unchecked_mut(index) = b'\0' };
		swap_off(line)?
	}
	Ok(())
}
