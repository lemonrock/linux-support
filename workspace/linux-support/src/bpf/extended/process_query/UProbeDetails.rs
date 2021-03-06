// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// UProbe details.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UProbeDetails
{
	/// File name.
	pub file_name: CString,
	
	/// Offset.
	pub offset: u64,
}

impl UProbeDetails
{
	#[inline(always)]
	pub(crate) fn construct(file_name: CString, task_fd_query: &BpfCommandTaskFileDescriptorQuery) -> Self
	{
		Self
		{
			file_name,
			offset: task_fd_query.probe_offset,
		}
	}
}
