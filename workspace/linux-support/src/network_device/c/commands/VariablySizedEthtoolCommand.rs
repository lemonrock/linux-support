// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Forcing the variably-sized ethtool command to be `Copy` ensures `Drop` is not implemented and so we can safely back the memory allocation using a Vec.
pub(crate) trait VariablySizedEthtoolCommand: EthtoolCommand + Copy
{
	type ArrayElement: Copy;
	
	fn array_length(&self) -> u32;
	
	const HeaderSize: usize = size_of::<Self>();
	
	#[inline(always)]
	fn array_size(&self) -> usize
	{
		size_of::<Self::ArrayElement>() * (self.array_length() as usize)
	}
	
	#[inline(always)]
	fn total_size(&self) -> usize
	{
		Self::HeaderSize + self.array_size()
	}
	
	#[inline(always)]
	fn new_with_initialized_header_but_uninitialized_array(variably_sized_ethtool_command_header: Self) -> VariablySizedEthtoolCommandWrapper<Self>
	{
		Self
		{
			data:
			{
				let length = variably_sized_ethtool_command_header.total_size();
				
				let mut data = Vec::with_capacity(length);
				unsafe
				{
					data.set_len(length);
					(data.as_mut_ptr() as *mut Self).write(variably_sized_ethtool_command_header);
				};
				
				data
			}
			
		}
	}
}
