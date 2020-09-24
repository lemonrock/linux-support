// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Currently `Subcommand` is always `ethtool_coalesce`.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct ethtool_per_queue_op<Subcommand: Copy + EthtoolCommand>
{
	/// Always `ETHTOOL_PERQUEUE`.
	cmd: u32,
	
	/// Currently either `ETHTOOL_GCOALESCE` or `ETHTOOL_SCOALESCE`.
	sub_command: u32,
	
	queue_mask: [u32; QueueMaskSize],
	
	/// Array of commands, total count is number of queues in the `queue_mask`.
	data: __IncompleteArrayField<Subcommand>,
}

impl<Subcommand: Copy + EthtoolCommand> EthtoolCommand for ethtool_per_queue_op<Subcommand>
{
	#[inline(always)]
	fn command(&self) -> u32
	{
		self.cmd
	}
}

impl<Subcommand: Copy + EthtoolCommand> VariablySizedEthtoolCommand for ethtool_per_queue_op<Subcommand>
{
	type ArrayElement = Subcommand;
	
	#[inline(always)]
	fn array_length(&self) -> u32
	{
		self.number_of_queues() * (size_of::<Subcommand>() as u32)
	}
}

impl<Subcommand: Copy + EthtoolCommand> ethtool_per_queue_op<Subcommand>
{
	#[inline(always)]
	fn number_of_queues(&self) -> u32
	{
		let mut number_of_queues = 0;
		for word_index in 0 .. self.queue_mask.len()
		{
			let word = unsafe { *self.queue_mask.get_unchecked(word_index) };
			number_of_queues += word.count_ones();
		}
		number_of_queues
	}
}

impl VariablySizedEthtoolCommandWrapper<ethtool_per_queue_op<ethtool_coalesce>>
{
	#[inline(always)]
	pub(crate) fn as_coalesce_configurations(&self) -> Result<HashMap<QueueIdentifier, CoalesceConfiguration>, AdaptiveCoalescingError>
	{
		let mut coalesce_configurations = HashMap::with_capacity(self.number_of_queues() as usize);
		
		let subcommands = self.array_elements();
		let mut subcommands_index = 0;
		for word_index in 0 .. self.queue_mask.len()
		{
			let word = unsafe { *self.queue_mask.get_unchecked(word_index) };
			for bit_in_word in 0 .. U32SizeInBits
			{
				let bit_value = (1 << bit_in_word) as u32;
				if word & bit_value != 0
				{
					let queue_identifier = bit_in_word + (word_index * U32SizeInBits);
					let queue_identifier = QueueIdentifier::from_validated_u16(queue_identifier as u16);
					let coalesce_configuration = (unsafe { subcommands.get_unchecked(subcommands_index) }).as_coalesce_configuration()?;
					coalesce_configurations.insert(queue_identifier, coalesce_configuration);
					subcommands_index += 1;
				}
			}
		}
		
		Ok(coalesce_configurations)
	}
}

impl ethtool_per_queue_op<ethtool_coalesce>
{
	#[inline(always)]
	pub(crate) fn coalesce_get(queue_identifiers: Either<QueueCount, &HashSet<QueueIdentifier>>) -> VariablySizedEthtoolCommandWrapper<Self>
	{
		Self::new_with_initialized_header_but_uninitialized_array
		(
			Self
			{
				cmd: ETHTOOL_PERQUEUE,
				
				sub_command: ETHTOOL_GCOALESCE,
				
				queue_mask: match queue_identifiers
				{
					Left(maximum_number_of_queues) => Self::all_queue_identifiers_to_queue_mask(maximum_number_of_queues),
					
					Right(queue_identifiers) => Self::queue_identifiers_to_queue_mask(queue_identifiers.iter()),
				},
				
				data: __IncompleteArrayField::Default,
			}
		)
	}
	
	#[inline(always)]
	pub(crate) fn coalesce_set<'a>(queue_identifiers: impl 'a + Iterator<Item=&'a QueueIdentifier>) -> VariablySizedEthtoolCommandWrapper<Self>
	{
		Self::new_with_initialized_header_but_uninitialized_array
		(
			Self
			{
				cmd: ETHTOOL_PERQUEUE,
				
				sub_command: ETHTOOL_SCOALESCE,
				
				queue_mask: Self::queue_identifiers_to_queue_mask(queue_identifiers),
				
				data: __IncompleteArrayField::Default,
			}
		)
	}
	
	#[allow(deprecated)]
	#[inline(always)]
	fn all_queue_identifiers_to_queue_mask(maximum_number_of_queues: QueueCount) -> [u32; QueueMaskSize]
	{
		let maximum_number_of_queues: usize = maximum_number_of_queues.into();
		
		let mut queue_mask: [u32; QueueMaskSize] = unsafe { zeroed() };
		let number_of_wholly_populated_words = maximum_number_of_queues / U32SizeInBits;
		unsafe { queue_mask.as_mut_ptr().write_bytes(0xFF, number_of_wholly_populated_words) };
		
		let bit_set_in_last_word =  (1 << ((maximum_number_of_queues % U32SizeInBits) as u32)) - 1;
		
		let word = unsafe { queue_mask.get_unchecked_mut(number_of_wholly_populated_words) };
		*word = bit_set_in_last_word;
		
		queue_mask
	}
	
	#[inline(always)]
	fn queue_identifiers_to_queue_mask<'a>(queue_identifiers: impl 'a + Iterator<Item=&'a QueueIdentifier>) -> [u32; QueueMaskSize]
	{
		let mut queue_mask: [u32; QueueMaskSize] = unsafe { zeroed() };
		for queue_identifier in queue_identifiers
		{
			let queue_identifier: u16 = (*queue_identifier).into();
			let word_index = (queue_identifier as usize) / U32SizeInBits;
			let word = unsafe { queue_mask.get_unchecked_mut(word_index) };
			let bit_index_in_word = ((queue_identifier as usize) % U32SizeInBits) as u32;
			*word = (*word) | 1 << bit_index_in_word;
		}
		queue_mask
	}
}

const BitsInAByte: usize = 8;

const U32SizeInBits: usize = size_of::<u32>() * BitsInAByte;

#[inline(always)]
const fn divide_rounded_up(numerator: u16, denominator: usize) -> usize
{
	let numerator = numerator as usize;
	
	(numerator + denominator - 1) / denominator
}

const QueueMaskSize: usize = divide_rounded_up(MAX_NUM_QUEUE, U32SizeInBits);
