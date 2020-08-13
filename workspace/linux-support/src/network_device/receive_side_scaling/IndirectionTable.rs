// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Is never empty.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndirectionTable(pub(crate) Vec<QueueIdentifier>);

impl IndirectionTable
{
	/// Calculate an indirection table using a `WeightQueueStrategy`.
	///
	/// Will panic if  `weight_queue_strategy.weight()` returns more weight than remains (or if it exceeds the size of an `u32`).
	///
	/// `number_of_receive_queues` should be the value from `NetworkDeviceInputOutputControl::get_receive_ring_queue_count`.
	///
	/// It is unclear how this relates to `Channels.receive_only_channels_count`, `Channels.receive_and_transmit_channels_count` or `GetLinkMessageData.number_of_receive_queues`.
	pub fn calculate_indirection_table(number_of_receive_queues: QueueCount, indirection_table_size: NonZeroU32, weight_queue_strategy: impl WeightQueueStrategy) -> Self
	{
		let denominator = indirection_table_size;
		let indirection_table_size = indirection_table_size.get();
		let mut indirection_table = Vec::with_capacity(number_of_receive_queues.get() as usize);
		
		let mut hash_index = 0u32;
		for queue_identifier in 0 .. number_of_receive_queues.get()
		{
			let queue_identifier = QueueIdentifier(queue_identifier);
			let weight = weight_queue_strategy.weight(queue_identifier, number_of_receive_queues, denominator);
			let next_hash_index = hash_index.checked_add(weight).expect("Far too much weight");
			assert!(next_hash_index <= indirection_table_size, "Asked for too much weight ({:?}) for `{:?}` when only a weight of {:?} remains to allocate", weight, queue_identifier, indirection_table_size - hash_index);
			
			for add_hash_index in hash_index .. next_hash_index
			{
				indirection_table.push(queue_identifier);
			}
			
			if next_hash_index == indirection_table_size
			{
				break
			}
			hash_index = next_hash_index
		}
		
		if hash_index < indirection_table_size
		{
			let remaining_weight = unsafe { NonZeroU32::new_unchecked(indirection_table_size - hash_index) };
			weight_queue_strategy.allocate_some_remaining_weight(remaining_weight, &mut indirection_table, number_of_receive_queues)
		}
		
		Self(indirection_table)
	}
}
