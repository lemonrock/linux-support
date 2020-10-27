// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A cache of recently randomly generated message identifiers to
///
/// * (a) minimize the risk that any upstream server can deduce a pattern to our requests.
/// * (b) ensure a high likelihood that a message reply is for our request.
pub struct RecentMessageIdentifiers
{
	recently_generated_random: HashSet<MessageIdentifier>,
	recently_generated_ring_queue: Box<[MessageIdentifier]>,
	capacity_power_of_two: usize,
	capacity_mask: usize,
	recently_generated_ring_queue_head: usize,
	recently_generated_ring_queue_tail: usize,
}

impl RecentMessageIdentifiers
{
	#[inline(always)]
	pub fn new(capacity_power_of_two: usize) -> Self
	{
		debug_assert_eq!(capacity_power_of_two, capacity_power_of_two.next_power_of_two(), "capacity was not a power of two");
		
		Self
		{
			recently_generated_random: HashSet::with_capacity(capacity_power_of_two),
			recently_generated_ring_queue:
			{
				let mut most_recently_used = Vec::with_capacity(capacity_power_of_two);
				unsafe { most_recently_used.set_len(capacity_power_of_two) };
				most_recently_used.into_boxed_slice()
			},
			capacity_power_of_two,
			capacity_mask: capacity_power_of_two - 1,
			recently_generated_ring_queue_head: 0,
			recently_generated_ring_queue_tail: 0,
		}
	}
	
	pub(crate) fn next(&mut self) -> MessageIdentifier
	{
		let recently_generated = loop
		{
			let random = MessageIdentifier::random();
			if self.recently_generated_random.contains(&random)
			{
				continue
			}
			break random
		};
		
		if likely!(self.recently_generatd_ring_queue_depth() == self.capacity_power_of_two)
		{
			self.remove_least_recently_generated()
		}
		self.add_most_recently_generated(recently_generated);
		recently_generated
	}
	
	#[inline(always)]
	fn recently_generatd_ring_queue_depth(&self) -> usize
	{
		self.recently_generated_ring_queue_head - self.recently_generated_ring_queue_tail
	}
	
	#[inline(always)]
	fn add_most_recently_generated(&mut self, message_identifier: MessageIdentifier)
	{
		let head_index = self.recently_generated_ring_queue_head & self.capacity_mask;
		unsafe { *self.recently_generated_ring_queue.get_unchecked_mut(head_index) = message_identifier };
		self.recently_generated_ring_queue_head += 1;
		
		let was_not_present = self.recently_generated_random.insert(message_identifier);
		debug_assert!(was_not_present);
	}
	
	#[inline(always)]
	fn remove_least_recently_generated(&mut self)
	{
		debug_assert_ne!(self.recently_generated_ring_queue_tail, self.recently_generated_ring_queue_head);
		
		let tail_index = self.recently_generated_ring_queue_tail & self.capacity_mask;
		let message_identifier = unsafe { self.recently_generated_ring_queue.get_unchecked(tail_index) };
		self.recently_generated_ring_queue_tail += 1;
		
		let was_present = self.recently_generated_random.remove(message_identifier);
		debug_assert!(was_present)
	}
}
