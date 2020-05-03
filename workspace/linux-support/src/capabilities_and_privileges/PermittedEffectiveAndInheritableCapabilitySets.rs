// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Three of the five per-thread capability sets.
///
/// Does not include the bounding set or the ambient set.
///
/// Set <http://man7.org/linux/man-pages/man7/capabilities.7.html>.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct PermittedEffectiveAndInheritableCapabilitySets
{
	#[allow(missing_docs)]
	pub permitted: BitSet<Capability>,

	#[allow(missing_docs)]
	pub effective: BitSet<Capability>,

	#[allow(missing_docs)]
	pub inheritable: BitSet<Capability>
}

impl PermittedEffectiveAndInheritableCapabilitySets
{
	/// Get for thread.
	#[inline(always)]
	pub fn get(thread_identifier: ThreadIdentifier) -> io::Result<Self>
	{
		let mut header = __user_cap_header_struct::new(thread_identifier);
		let mut data: [__user_cap_data_struct; 2] = unsafe { zeroed() };
		let result = unsafe { capget(&mut header as *mut _ as *mut _, &mut data as *mut _ as *mut _) };
		if likely!(result == 0)
		{
			#[inline(always)]
			fn new_set(index0: u32, index1: u32) -> BitSet<Capability>
			{
				BitSet::new_from_u64((index0 as u64) << 32 | (index1 as u64))
			}

			Ok
			(
				Self
				{
					permitted: new_set(data[0].permitted, data[1].permitted),
					effective: new_set(data[0].effective, data[1].effective),
					inheritable: new_set(data[0].inheritable, data[1].inheritable)
				}
			)
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable!("Unexpected result {} from capget()", result)
		}
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn set(&self, thread_identifier: ThreadIdentifier) -> io::Result<()>
	{
		fn capability_to_index_and_bit(capability: Capability) -> (usize, u32)
		{
			let capability = capability as u8;
			let (index, bit_index) = if capability > 31
			{
				(1, capability % 32)
			}
			else
			{
				(0, capability)
			};
			(index, 1 << (bit_index as u32))
		}

		let mut data: [__user_cap_data_struct; 2] = unsafe { zeroed() };

		for capability in self.permitted.iterate()
		{
			let (index, bit) = capability_to_index_and_bit(capability);
			data[index].permitted |= bit
		}

		for capability in self.effective.iterate()
		{
			let (index, bit) = capability_to_index_and_bit(capability);
			data[index].effective |= bit
		}

		for capability in self.inheritable.iterate()
		{
			let (index, bit) = capability_to_index_and_bit(capability);
			data[index].inheritable |= bit
		}

		let mut header = __user_cap_header_struct::new(thread_identifier);
		let result = unsafe { capset(&mut header as *mut _ as *mut _, &mut data as *mut _ as *mut _) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable!("Unexpected result {} from capget()", result)
		}
	}
}
