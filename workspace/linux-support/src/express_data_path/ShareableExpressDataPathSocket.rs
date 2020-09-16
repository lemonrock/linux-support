// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Used to create per-thread (Shared) sockets.
///
/// No need to hold onto an instance once all desired shared sockts have been created.
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct ShareableExpressDataPathSocket<ROTOB: ReceiveOrTransmitOrBoth<RingQueueDepth, RingQueueDepth> + MapReceiveOrTransmitOrBoth, RP: ReceivePoll, CA: ChunkAlignment>
{
	owned_fill_queue_lock_and_completion_queue_lock: Arc<(OwnedExpressDataPathSocket<ROTOB>, BestForCompilationTargetSpinLock, BestForCompilationTargetSpinLock)>,
}

impl<ROTOB: ReceiveOrTransmitOrBoth<RingQueueDepth, RingQueueDepth> + MapReceiveOrTransmitOrBoth, RP: ReceivePoll, CA: ChunkAlignment> From<OwnedExpressDataPathSocket<ROTOB, RP, CA>> for ShareableExpressDataPathSocket<ROTOB, RP, CA>
{
	#[inline(always)]
	fn from(value: OwnedExpressDataPathSocket<ROTOB, RP, CA>) -> Self<>
	{
		Self
		{
			owned_fill_queue_lock_and_completion_queue_lock: Arc::new((value, BestForCompilationTargetSpinLock::new(), BestForCompilationTargetSpinLock::new())),
		}
	}
}

impl<ROTOB: ReceiveOrTransmitOrBoth<RingQueueDepth, RingQueueDepth> + MapReceiveOrTransmitOrBoth, RP: ReceivePoll, CA: ChunkAlignment> ShareableExpressDataPathSocket<ROTOB, RP, CA>
{
	/// The `express_data_path_extended_bpf_program` in use with `self` must be suitable for use with shared user memory; if not an error of `Err(AttachProgramError::AttachedXdpProgramNotSuitableForSharing)` is returned.
	///
	/// A potential bug: ***`queue_identifier` is not checked to see if it used by another instance of `SharedReceiveTransmitMemoryRingQueues`***.
	/// Adding such a check is possible but is tedious.
	pub fn share(&self, ring_queue_depths: ROTOB, receive_poll_creator: impl FnOnce(&ExpressDataPathSocketFileDescriptor) -> RP, queue_identifier: QueueIdentifier, force_copy: bool, force_zero_copy: bool, defaults: &DefaultPageSizeAndHugePageSizes) -> Result<SharedExpressDataPathSocket<ROTOB, RP, CA>, ExpressDataPathSocketCreationError>
	{
		let owned_fill_queue_lock_and_completion_queue_lock = &self.owned_fill_queue_lock_and_completion_queue_lock;
		let (common, express_data_path_socket_file_descriptor) = owned_fill_queue_lock_and_completion_queue_lock.deref().0.share(ring_queue_depths, receive_poll_creator, queue_identifier, force_copy, force_zero_copy, defaults)?;
		Ok
		(
			SharedExpressDataPathSocket
			{
				owner: Self
				{
					owned_fill_queue_lock_and_completion_queue_lock: Arc::clone(owned_fill_queue_lock_and_completion_queue_lock),
				},
				common,
				express_data_path_socket_file_descriptor,
			}
		)
	}
	
	#[inline(always)]
	pub(crate) fn fill_queue_spin_lock(self) -> &SpinLock
	{
		&(self.owned_fill_queue_lock_and_completion_queue_lock.deref().1)
	}
	
	#[inline(always)]
	pub(crate) fn completion_queue_spin_lock(self) -> &SpinLock
	{
		&(self.owned_fill_queue_lock_and_completion_queue_lock.deref().2)
	}
}
