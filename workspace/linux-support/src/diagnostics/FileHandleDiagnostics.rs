// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct FileHandleDiagnostics
{
	pub allocate_free_and_maximum: DiagnosticUnobtainableResult<AllocatedFreeAndMaximumNumberOfFileHandles>,

	pub maximum: DiagnosticUnobtainableResult<NumberOfFileHandles>,
}

impl FileHandleDiagnostics
{
	#[inline(always)]
	fn gather(proc_path: &ProcPath) -> Self
	{
		Self
		{
			allocate_free_and_maximum: NumberOfFileHandles::allocated_free_and_maximum(proc_path).map_err(DiagnosticUnobtainable::from),
			
			maximum: NumberOfFileHandles::maximum(proc_path).map_err(DiagnosticUnobtainable::from),
		}
	}
}
