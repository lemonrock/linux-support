// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Thread settings.
///
/// Effectively a named tuple of values need to create a thread.
#[derive(Debug)]
pub struct ThreadSettings<'a, TF: ThreadFunction>
{
	/// Thread configuration; typically shared amongst many threads.
	pub thread_configuration: &'a ThreadConfiguration,
	
	/// Should normally be for just one HyperThread.
	pub affinity: HyperThreads,
	
	/// Thread function.
	pub thread_function: TF,
}
