// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A per-thread memory allocator instantiator.
pub trait PerThreadMemoryAllocatorInstantiator: Default + std::marker::Sync + std::marker::Send
{
	/// Arguments to pass to instantiate.
	type InstantiationArguments: std::marker::Sync + std::marker::Send;
	
	/// Dropped when the thead is finished, even after a panic.
	///
	/// Ensures that any thread local memory is then dropped.
	type ThreadDropGuard: Sized;
	
	/// Instantiate.
	fn instantiate(thread_local_allocator_configuration: Arc<ThreadLocalAllocatorConfiguration>, instantiation_arguments: Arc<Self::InstantiationArguments>) -> Result<Self::ThreadDropGuard, MemoryMapError>;
}
