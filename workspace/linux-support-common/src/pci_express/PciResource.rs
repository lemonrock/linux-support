// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A memory-mapped PCI resource.
#[derive(Debug)]
pub struct PciResource
{
	/// Technically, this isn't true; `mmap()` could return a pointer of zero.
	pointer: NonNull<u8>,
	size: usize,
}

impl Drop for PciResource
{
	#[inline(always)]
	fn drop(&mut self)
	{
		let result = unsafe { munmap(self.pointer.as_ptr() as *mut c_void, self.size) };
		if likely!(result == 0)
		{
			return
		}
		else if likely!(result == -1)
		{
			panic!("munmap returned an error of {}", errno())
		}
		else
		{
			panic!("munmap() failed with an unexpected exit code of {:?}", result)
		}
	}
}

impl PciResource
{
	#[inline(always)]
	fn pointer(&self) -> NonNull<u8>
	{
		self.pointer
	}
}
