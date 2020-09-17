// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Used to create per-thread (Shared) sockets.
///
/// No need to hold onto an instance once all desired shared sockts have been created.
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct ShareableExpressDataPathSocket<ROTOB: ReceiveOrTransmitOrBoth, CA: ChunkAlignment>(Arc<(OwnedExpressDataPathSocket<ROTOB, CA>, BestForCompilationTargetSpinLock, BestForCompilationTargetSpinLock)>);

impl<ROTOB: ReceiveOrTransmitOrBoth, CA: ChunkAlignment> From<OwnedExpressDataPathSocket<ReceiveOrTransmitOrBoth, CA>> for ShareableExpressDataPathSocket<ROTOB, CA>
{
	#[inline(always)]
	fn from(value: OwnedExpressDataPathSocket<ReceiveOrTransmitOrBoth, CA>) -> Self
	{
		Self(Arc::new((value, BestForCompilationTargetSpinLock::new(), BestForCompilationTargetSpinLock::new())))
	}
}

impl<RingQueueDepths: CreateReceiveOrTransmitOrBoth, CA: ChunkAlignment> ShareableExpressDataPathSocket<RingQueueDepths::ReceiveOrTransmitOrBoth, CA>
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
	pub fn share(&self, receive_or_transmit_or_both_ring_queue_depths: RingQueueDepths, queue_identifier: QueueIdentifier, force_copy: bool, force_zero_copy: bool, defaults: &DefaultPageSizeAndHugePageSizes, arguments: RingQueueDepths::Arguments) -> Result<SharedExpressDataPathSocket<RingQueueDepths::ReceiveOrTransmitOrBoth, CA>, ExpressDataPathSocketCreationError>
	{
		let (common, express_data_path_socket_file_descriptor) = self.owned().shared(receive_or_transmit_or_both_ring_queue_depths, queue_identifier, force_copy, force_zero_copy, defaults, arguments)?;
		
		Ok
		(
			SharedExpressDataPathSocket
			{
				owner: Self(Arc::clone(self.owned_fill_queue_lock_and_completion_queue_lock())),
				common,
				express_data_path_socket_file_descriptor,
			}
		)
	}
}

impl<ROTOB: ReceiveOrTransmitOrBoth, CA: ChunkAlignment> ShareableExpressDataPathSocket<ROTOB, CA>
{
	#[inline(always)]
	pub(crate) fn redirect_map_and_attached_program(self) -> &RedirectMapAndAttachedProgram
	{
		&(self.owned().redirect_map_and_attached_program)
	}
	
	#[inline(always)]
	pub(crate) fn user_memory(self) -> &UserMemory<CA>
	{
		self.owned().user_memory()
	}
	
	#[inline(always)]
	fn owned(self) -> &OwnedExpressDataPathSocket<ROTOB, CA>
	{
		&(self.owned_fill_queue_lock_and_completion_queue_lock().0)
	}
	
	#[inline(always)]
	pub(crate) fn fill_queue_spin_lock(self) -> &BestForCompilationTargetSpinLock
	{
		&(self.owned_fill_queue_lock_and_completion_queue_lock().1)
	}
	
	#[inline(always)]
	pub(crate) fn completion_queue_spin_lock(self) -> &BestForCompilationTargetSpinLock
	{
		&(self.owned_fill_queue_lock_and_completion_queue_lock().2)
	}
	
	#[inline(always)]
	fn owned_fill_queue_lock_and_completion_queue_lock(self) -> &(OwnedExpressDataPathSocket<ReceiveOrTransmitOrBoth>, BestForCompilationTargetSpinLock, BestForCompilationTargetSpinLock)
	{
		self.0.deref()
	}
}
