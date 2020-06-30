// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Used for commands `BPF_PROG_GET_NEXT_ID`, `BPF_MAP_GET_NEXT_ID`, `BPF_PROG_GET_FD_BY_ID`, `BPF_MAP_GET_FD_BY_ID`, `BPF_BTF_GET_FD_BY_ID` and `BPF_BTF_GET_NEXT_ID`.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct BpfCommandGetIdentifier
{
	pub(crate) value_of_identifier: BpfCommandGetIdentifierValueOfIdentifier,
	pub(crate) next_id: u32,
	pub(crate) open_flags: u32,
}

impl Default for BpfCommandGetIdentifier
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for BpfCommandGetIdentifier
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "BpfCommandGetIdentifier {{ value_of_identifier: {:?}, next_id: {:?}, open_flags: {:?} }}", self.value_of_identifier, self.next_id, self.open_flags)
	}
}
