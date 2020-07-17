// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A raw trace point type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RawTracePointType
{
	/// `BPF_PROG_TYPE_TRACING` with expected attach type `BPF_TRACE_RAW_TP`.
	TracingOfRawTracePoint,
	
	/// `BPF_PROG_TYPE_RAW_TRACEPOINT`, any expected attach type.
	RawTracePoint(TracePointDetails),
	
	/// `BPF_PROG_TYPE_RAW_TRACEPOINT_WRITABLE`, any expected attach type.
	RawTracePointWritable(TracePointDetails),
}
