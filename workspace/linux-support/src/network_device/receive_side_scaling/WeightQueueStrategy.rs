// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A weight of `1` is one index in the `indirection_table`.
pub trait WeightQueueStrategy
{
	/// The returned value can not exceed `denominator.get()`.
	///
	/// `number_of_receive_queues` should be the value from `NetworkDeviceInputOutputControl::get_receive_ring_queue_count`.
	///
	/// It is unclear how this relates to `Channels.receive_only_channels_count`, `Channels.receive_and_transmit_channels_count` or `GetLinkMessageData.number_of_receive_queues`.
	fn weight(&self, queue_index: QueueIdentifier, number_of_receive_queues: QueueCount, denominator: NonZeroU32) -> u32;
	
	/// Allocate some remaining weight.
	fn allocate_some_remaining_weight(&self, remaining_weight: NonZeroU32, incomplete_indirection_table: &mut Vec<QueueIdentifier>, number_of_receive_queues: QueueCount)
	{
		let queue_identifier = QueueIdentifier(number_of_receive_queues.get() - 1);
		for _index in 0 .. remaining_weight.get()
		{
			incomplete_indirection_table.push(queue_identifier);
		}
	}
}
