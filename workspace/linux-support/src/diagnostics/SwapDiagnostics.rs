// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct SwapDiagnostics
{
	pub page_cluster: DiagnosticUnobtainableResult<Either<PageCluster, i32>>,
	
	pub swappiness: DiagnosticUnobtainableResult<Swappiness>,
	
	pub swaps: DiagnosticUnobtainableResult<Swaps>,
}

impl SwapDiagnostics
{
	fn gather(proc_path: &ProcPath) -> Self
	{
		Self
		{
			page_cluster: PageCluster::read(proc_path).map_err(DiagnosticUnobtainable::from),
			swappiness: Swappiness::read(proc_path).map_err(DiagnosticUnobtainable::from),
			swaps: Swaps::parse(proc_path).map_err(DiagnosticUnobtainable::from)
		}
	}
}
