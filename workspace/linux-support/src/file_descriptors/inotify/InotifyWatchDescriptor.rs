// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// A watch descriptor.
#[derive(Debug)]
pub struct InotifyWatchDescriptor
{
	parent: Weak<InotifyFileDescriptor>,
	watch_descriptor: c_int,
}

impl Drop for InotifyWatchDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if let Some(parent) = self.parent.upgrade()
		{
			let result = unsafe { inotify_rm_watch(parent.as_raw_fd(), self.watch_descriptor) };
			if likely!(result == 0)
			{
				return
			}
			else if likely!(result == -1)
			{
				match errno().0
				{
					EBADF => panic!("`fd` is not a valid file descriptor"),
					EINVAL => panic!("The watch descriptor `wd` is not valid; or `fd` is not an inotify file descriptor"),
					_ => unreachable_code(format_args!("")),
				}
			}
			else
			{
				unreachable_code(format_args!(""))
			}
		}

	}
}

impl PartialEq<c_int> for InotifyWatchDescriptor
{
	#[inline(always)]
	fn eq(&self, other: &c_int) -> bool
	{
		self.watch_descriptor == *other
	}
}

impl PartialOrd<c_int> for InotifyWatchDescriptor
{
	#[inline(always)]
	fn partial_cmp(&self, other: &c_int) -> Option<Ordering>
	{
		self.watch_descriptor.partial_cmp(other)
	}
}

impl Hash for InotifyWatchDescriptor
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, hasher: &mut H)
	{
		self.watch_descriptor.hash(hasher)
	}
}

impl InotifyWatchDescriptor
{
	/// Is this watch descriptor the same?
	#[inline(always)]
	pub fn is(&self, watch_descriptor: c_int) -> bool
	{
		self.watch_descriptor == watch_descriptor
	}
}

