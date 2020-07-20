// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Iterator.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyIterator<'map_file_descriptor, K: Sized>
{
	map_file_descriptor: &'map_file_descriptor MapFileDescriptor,
	current_key: Option<K>,
}

impl<'map_file_descriptor, K: Sized> Iterator for KeyIterator<'map_file_descriptor, K>
{
	type Item = Result<K, Errno>;
	
	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		let current_key = match self.current_key.take()
		{
			None => return None,
			Some(current_key) => current_key,
		};
		
		let next_key = match self.map_file_descriptor.get_next_key(&current_key)
		{
			Err(errno) => return Some(Err(errno)),
			Ok(next_key) => next_key,
		};
		self.current_key = next_key;
		Some(Ok(current_key))
	}
}

impl<'map_file_descriptor, K: Sized> KeyIterator<'map_file_descriptor, K>
{
	#[inline(always)]
	fn new(map_file_descriptor: &'map_file_descriptor MapFileDescriptor) -> Result<Self, Errno>
	{
		let first_key = map_file_descriptor.get_next_key(null())?;
		
		Ok
		(
			Self
			{
				map_file_descriptor,
				current_key: first_key,
			}
		)
	}
}
