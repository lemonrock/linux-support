// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct AcceptCoroutine<SA: SocketAddress, CoroutineHeapSize: 'static + MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, AC: AccessControl<SA::SD, AccessControlValue>>(PhantomData<(SA, CoroutineHeapSize, GTACSA, AC)>);

impl<SA: SocketAddress, CoroutineHeapSize: 'static + MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, AC: AccessControl<SA::SD, AccessControlValue>> Coroutine for AcceptCoroutine<SA, CoroutineHeapSize, GTACSA, AC>
{
	type StartArguments = AcceptStartArguments<SA, CoroutineHeapSize, GTACSA, AC>;

	type ResumeArguments = SimpleIoUringResumeArguments;

	type Yields = SimpleIoUringYields;

	type Complete = UnusedComplete;
	
	const LifetimeHint: LifetimeHint = LifetimeHint::LongLived;
	
	const HeapMemoryAllocatorBlockSizeHint: NonZeroUsize = new_non_zero_usize(64);
	
	#[inline(always)]
	fn coroutine(coroutine_instance_handle: CoroutineInstanceHandle, yielder: Yielder<Self::ResumeArguments, Self::Yields, Self::Complete>, start_arguments: Self::StartArguments) -> Self::Complete
	{
		let mut accept = Accept::new(coroutine_instance_handle, yielder, start_arguments);
		
		loop
		{
			// Must be pinned until `yield_awaiting_accept()` returns.
			let mut pending_accept_connection = PendingAcceptConnection::new();
			
			let killed = accept.yield_submit_accept(&mut pending_accept_connection);
			if unlikely!(killed)
			{
				return ()
			}
			
			let completion_response = match accept.yield_awaiting_accept()
			{
				Ok(completion_response) => completion_response,
				Err(()) => return (),
			};
			
			let killed = accept.process_accept(completion_response, pending_accept_connection);
			if unlikely!(killed)
			{
				return ()
			}
		}
	}
}
