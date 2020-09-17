// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Used to create per-thread (Shared) sockets.
///
/// No need to hold onto an instance once all desired shared sockts have been created.
#[derive(Debug)]
#[repr(transparent)]
pub struct ShareableExpressDataPathInstance<ROTOB: ReceiveOrTransmitOrBoth, CA: ChunkAlignment>(Arc<(ExpressDataPathInstance<CA>, BestForCompilationTargetSpinLock, BestForCompilationTargetSpinLock, Mutex<HashSet<QueueIdentifier>>)>);

unsafe impl<ROTOB: ReceiveOrTransmitOrBoth, CA: ChunkAlignment> Sync for ShareableExpressDataPathInstance<ROTOB, CA>
{
}

unsafe impl<ROTOB: ReceiveOrTransmitOrBoth, CA: ChunkAlignment> Send for ShareableExpressDataPathInstance<ROTOB, CA>
{
}

impl<ROTOB: ReceiveOrTransmitOrBoth, CA: ChunkAlignment> Clone for ShareableExpressDataPathInstance<ROTOB, CA>
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		Self(self.0.clone())
	}
}

impl<RingQueueDepths: CreateReceiveOrTransmitOrBoth, CA: ChunkAlignment> ShareableExpressDataPathInstance<RingQueueDepths::ReceiveOrTransmitOrBoth, CA>
{
	/// Treats `self` as master; returns a slave.
	///
	/// When all slaves have been dropped the master is dropped.
	/// This ensures the correct ordering of close for socket file descriptors.
	///
	/// The `express_data_path_extended_bpf_program` in use with `self` must be suitable for use with shared user memory; if not an error of `Err(ExpressDataPathSocketCreationError::AttachedExpressDataPathProgramNotSuitableForSharing)` is returned.
	/// This check works on eXpress Data Path program names, so is fallible (it is exceedingly unlikely to return a false positive, however).
	///
	/// A potential bug: ***`queue_identifier` is not checked to see if it used by another instance of `SharedReceiveTransmitMemoryRingQueues`***.
	/// Adding such a check is possible but is tedious.
	pub fn share(&self, receive_or_transmit_or_both_ring_queue_depths: RingQueueDepths, queue_identifier: QueueIdentifier, defaults: &DefaultPageSizeAndHugePageSizes, arguments: RingQueueDepths::Arguments) -> Result<SharedExpressDataPathSocket<RingQueueDepths::ReceiveOrTransmitOrBoth, CA>, ExpressDataPathSocketCreationError>
	{
		let is_first =
		{
			let queue_identifiers = self.queue_identifiers();
			
			let is_first = queue_identifiers.is_empty();
			
			let inserted = queue_identifiers.insert(queue_identifier);
			debug_assert!(inserted, "queue_identifier `{:?}` is already in use", queue_identifier);
			is_first
		};
		
		let express_data_path_socket_file_descriptor = if is_first
		{
			let user_memory_socket_file_descriptor = self.user_memory().user_memory_socket_file_descriptor.duplicate_with_close_on_exec_non_blocking();
			loop
			{
				match user_memory_socket_file_descriptor.duplicate_with_close_on_exec_non_blocking()
				{
					Ok(Some(express_data_path_socket_file_descriptor)) => break express_data_path_socket_file_descriptor,
					
					Ok(None) | Err(CreationError::KernelWouldBeOutOfMemory) => continue,
					
					Err(error) => panic!(error),
				}
			};
		}
		else
		{
			ExpressDataPathSocketFileDescriptor::new().map_err(ExpressDataPathSocketCreationError::CouldNotCreateUserMemorySocketFileDescriptor)?
		};
		
		self.express_data_path_instance().shared(receive_or_transmit_or_both_ring_queue_depths, queue_identifier, defaults, arguments, self, express_data_path_socket_file_descriptor)
	}
}

impl<ROTOB: ReceiveOrTransmitOrBoth, CA: ChunkAlignment> ShareableExpressDataPathInstance<ROTOB, CA>
{
	#[inline(always)]
	fn redirect_map_and_attached_program(self) -> &RedirectMapAndAttachedProgram
	{
		&self.express_data_path_instance().redirect_map_and_attached_program
	}
	
	#[inline(always)]
	fn user_memory(self) -> &UserMemory<CA>
	{
		&self.express_data_path_instance().user_memory
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn express_data_path_instance(self) -> &ExpressDataPathInstance<ROTOB, CA>
	{
		&(self.fields().0)
	}
	
	#[inline(always)]
	fn fill_queue_spin_lock(self) -> &BestForCompilationTargetSpinLock
	{
		&(self.fields().1)
	}
	
	#[inline(always)]
	fn completion_queue_spin_lock(self) -> &BestForCompilationTargetSpinLock
	{
		&(self.fields().2)
	}
	
	#[inline(always)]
	fn queue_identifiers(self) -> MutexGuard<HashSet<QueueIdentifier>>
	{
		(&(self.fields().3)).lock().unwrap()
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn fields(self) -> &(OwnedExpressDataPathSocket<ReceiveOrTransmitOrBoth>, BestForCompilationTargetSpinLock, BestForCompilationTargetSpinLock)
	{
		self.0.deref()
	}
}
