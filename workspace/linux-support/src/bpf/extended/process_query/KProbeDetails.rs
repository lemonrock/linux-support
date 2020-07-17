// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// KProbe details.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KProbeDetails
{
	/// Function.
	Function
	{
		/// Symbol.
		symbol: CString,
		
		/// Offset.
		offset: u64,
	},
	
	/// Address.
	Address(u64),
}

impl KProbeDetails
{
	#[inline(always)]
	fn construct(symbol: CString, task_fd_query: &BpfCommandTaskFileDescriptorQuery) -> Self
	{
		use self::KProbeDetails::*;
		
		if symbol.as_bytes().is_empty()
		{
			Address(task_fd_query.probe_addr)
		}
		else
		{
			Function
			{
				symbol,
				offset: task_fd_query.probe_offset,
			}
		}
	}
}
