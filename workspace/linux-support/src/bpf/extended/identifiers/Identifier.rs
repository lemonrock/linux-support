// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An identifier.
///
/// Use `BfFileDescriptor::from_identifier` to use.
pub trait Identifier: From<u32> + Into<u32> + Into<BpfCommandGetIdentifierValueOfIdentifier> + Default + Debug + Copy + Clone + PartialEq + Eq + PartialOrd + Ord + Hash + Sized
{
	#[doc(hidden)]
	const Next: bpf_cmd;
	
	/// First identifier.
	#[inline(always)]
	fn first() -> Option<Self>
	{
		Self::from(0).next()
	}
	
	/// Next identifier.
	#[inline(always)]
	fn next(self) -> Option<Self>
	{
		let mut attr = bpf_attr::default();
		attr.get_identifier = BpfCommandGetIdentifier
		{
			value_of_identifier: BpfCommandGetIdentifierValueOfIdentifier
			{
				start_id: self.into(),
			},
			next_id: 0,
			open_flags: 0,
		};
		
		match attr.syscall(Self::Next).as_usize()
		{
			0 => Some(Self::from(unsafe { attr.get_identifier.next_id })),
			
			SystemCallResult::ENOENT_usize => None,
			unexpected_error @ SystemCallResult::InclusiveErrorRangeStartsFrom_usize ..= SystemCallResult::InclusiveErrorRangeEndsAt_usize => unexpected_error!(bpf, Self::Next, SystemCallResult::usize_to_system_call_error_number(unexpected_error)),
			
			unexpected @ _ => unexpected_result!(bpf, Self::Next, unexpected),
			
		}
	}
}
