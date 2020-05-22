// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This structure and its functions are designed to be straightforward to turn into runtime assembler.
struct Point<Value>
{
	matches: [MatchEntry<Value>; PermutationsOfAByte],
}

impl<Value> Point<Value>
{
	fn longest_match_next<'a, 'b>(&'a self, remaining_bytes: &'b [u8]) -> Option<&'a Value>
	{
		debug_assert_ne!(remaining_bytes.len(), 0);
		
		let index = (unsafe { *remaining_bytes.get_unchecked(0) }) as usize;
		
		let match_entry = unsafe { self.matches.get_unchecked(index) };
		
		if let Some(ref more_specific) = match_entry.more_specific
		{
			debug_assert!(remaining_bytes.len() > 1);
			
			if let Some(result) = more_specific.longest_match_next(&remaining_bytes[1 .. ])
			{
				return Some(result)
			}
		}
		
		match match_entry.partial
		{
			None => None,
			Some(ref value) => Some(value.deref())
		}
	}
	
	fn add_from_root<'a, 'b, IPA: InternetProtocolAddress>(&'a mut self, internet_protocol_address_with_mask: &'b InternetProtocolAddressWithMask<IPA>, value: &'b Rc<Value>)
	{
		let remaining_mask_length_in_bits = internet_protocol_address_with_mask.mask_length_in_bits;
		
		let remaining_bytes = internet_protocol_address_with_mask.internet_protocol_address.bytes();
		
		self.add(remaining_bytes, remaining_mask_length_in_bits, value)
	}
	
	// We must add in sort order, with remaining_mask_bits from 1 to 32, say (least specific to most specific).
	// Sort order is ordered by `remaining_mask_length_in_bits` then `remaining_bytes`.
	fn add<'a, 'b>(&'a mut self, remaining_bytes: &[u8], remaining_mask_length_in_bits: NonZeroU8, value: &'b Rc<Value>)
	{
		debug_assert_ne!(remaining_bytes.len(), 0);
		
		let byte = unsafe { *remaining_bytes.get_unchecked(0) };
		
		let remaining_mask_length_in_bits = remaining_mask_length_in_bits.get();
		if remaining_mask_length_in_bits > BitsInAByte
		{
			let match_entry = unsafe { self.matches.get_unchecked_mut(byte as usize) };
			if match_entry.more_specific.is_none()
			{
				match_entry.more_specific = Some(Box::new(Self::new()))
			}
			match_entry.more_specific.as_mut().unwrap().add(&remaining_bytes[1 .. ], unsafe { NonZeroU8::new_unchecked(remaining_mask_length_in_bits - BitsInAByte) }, value)
		}
		else
		{
			let shift = BitsInAByte - remaining_mask_length_in_bits;
			
			let lower_bits_mask = (1 << remaining_mask_length_in_bits) - 1;
			let upper_bits_mask = lower_bits_mask << shift;
			
			let must_match_upper = byte & upper_bits_mask;
			
			for can_match_lower in 0 .. (1 << shift)
			{
				let can_match_byte = must_match_upper + can_match_lower;
				
				let match_entry = unsafe { self.matches.get_unchecked_mut(can_match_byte as usize) };
				match_entry.partial = Some(value.clone())
			}
		}
	}
	
	#[inline(always)]
	fn new() -> Self
	{
		Self
		{
			matches:
			[
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
				MatchEntry::default(),
			],
		}
	}
}
