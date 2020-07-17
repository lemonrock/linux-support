// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Error when attaching to a raw trace point.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum RawTracePointAttachError
{
	/// Should only occur for:-
	///
	/// * `RawTracePoint` and `RawTracePointWritable`: the `trace_point_name` does not exist.
	/// * `BPF_PROG_TYPE_TRACING` with expected attach type `BPF_TRACE_RAW_TP`: `prog->aux->attach_func_name` does not exist.
	TracePointNameNotFound,
	
	/// Out of memory.
	OutOfMemory,
}

impl Display for RawTracePointAttachError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for RawTracePointAttachError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::RawTracePointAttachError::*;

		match self
		{
			&TracePointNameNotFound => None,
			&OutOfMemory => None,
		}
	}
}
