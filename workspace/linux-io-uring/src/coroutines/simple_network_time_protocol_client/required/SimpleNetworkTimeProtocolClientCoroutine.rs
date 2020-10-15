// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct SimpleNetworkTimeProtocolClientCoroutine<SA: SocketAddress, CoroutineHeapSize: 'static + MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>>(PhantomData<(SA, CoroutineHeapSize, GTACSA)>);

impl<SA: SocketAddress, CoroutineHeapSize: 'static + MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>> Coroutine for SimpleNetworkTimeProtocolClientCoroutine<SA, CoroutineHeapSize, GTACSA>
{
	type StartArguments = SimpleNetworkTimeProtocolClientStartArguments<SA::SD, CoroutineHeapSize, GTACSA>;

	type ResumeArguments = SimpleIoUringResumeArguments;

	type Yields = SimpleIoUringYields;

	type Complete = UnusedComplete;
	
	const LifetimeHint: LifetimeHint = LifetimeHint::LongLived;
	
	const HeapMemoryAllocatorBlockSizeHint: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(64) };
	
	#[inline(always)]
	fn coroutine(coroutine_instance_handle: CoroutineInstanceHandle, yielder: Yielder<Self::ResumeArguments, Self::Yields, Self::Complete>, start_arguments: Self::StartArguments) -> Self::Complete
	{
		let mut simple_network_time_protocol_client = SimpleNetworkTimeProtocolClient::new(coroutine_instance_handle, yielder, start_arguments);
		
		loop
		{
			// Send a packet buffer.
			
			
			
			// Receive a packet buffer.
		}
	}
}
