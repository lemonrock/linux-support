// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Support trait for sockets.
pub trait HyperThreadPerBitSetAwareData<PerBitSetAware>
{
	/// Gets the data for a particular BitSetAware; if no data for that core, gets it for `HyperThread::current()`.
	///
	/// If the BitSetAware does not exist (or does not have assigned data), returns None; this can happen on Linux if using the` SO_INCOMING_CPU` socket option, which can return an index for a CPU not assigned to the process.
	fn get_or_current(&self, hyper_thread: HyperThread) -> &PerBitSetAware;
	
	/// Gets the mutable_data for a particular BitSetAware; if no data for that core, gets it for `HyperThread::current()`.
	///
	/// If the BitSetAware does not exist (or does not have assigned data), returns None; this can happen on Linux if using the` SO_INCOMING_CPU` socket option with `PerBitSetAwareData<HyperThread>`, which can return an index for a HyperThread not assigned to the process.
	fn get_mut_or_current(&mut self, hyper_thread: HyperThread) -> &mut PerBitSetAware;
}

impl<PerBitSetAware> HyperThreadPerBitSetAwareData<PerBitSetAware> for PerBitSetAwareData<HyperThread, PerBitSetAware>
{
	#[inline(always)]
	fn get_or_current(&self, hyper_thread: HyperThread) -> &PerBitSetAware
	{
		self.get_or(hyper_thread, || HyperThread::current().1)
	}

	#[inline(always)]
	fn get_mut_or_current(&mut self, hyper_thread: HyperThread) -> &mut PerBitSetAware
	{
		self.get_mut_or(hyper_thread, || HyperThread::current().1)
	}
}
