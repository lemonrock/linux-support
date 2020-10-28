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
		
		let result = attr.syscall(Self::Next);
		if likely!(result == 0)
		{
			Some(Self::from(unsafe { attr.get_identifier.next_id }))
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				ENOENT => None,
				unexpected @ _ => panic!("Unexpected errror {}", unexpected),
			}
		}
		else
		{
			unreachable_code(format_args!("Unexpected result `{}` from bpf({:?})", result, Self::Next))
		}
	}
}
