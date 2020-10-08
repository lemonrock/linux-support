// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub(crate) struct InterruptRequestAffinities(HashMap<InterruptRequestActionName, HyperThreads>);

impl InterruptRequestAffinities
{
	#[inline(always)]
	pub(crate) fn add_interrupt_request_affinity(&mut self, interrupt_name: InterruptRequestActionName, hyper_thread: HyperThread)
	{
		self.0.entry(interrupt_name).or_insert_with(|| HyperThreads(BitSet::empty())).add(hyper_thread);
	}
	
	#[inline(always)]
	pub(crate) fn set_hyper_thread_affinities(self, sys_path: &SysPath, proc_path: &ProcPath) -> io::Result<()>
	{
		for interrupt_request in InterruptRequest::all(sys_path)?
		{
			for action in interrupt_request.sysfs_actions(sys_path)?
			{
				if let Some(affinity) = self.0.get(&action)
				{
					interrupt_request.set_smp_affinity(proc_path, affinity)?
				}
			}
		}
		Ok(())
	}
}
