// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct btf_member
{
	pub(crate) name_off: NonZeroU32,
	
	pub(crate) type_identifier: BtfTypeIdentifier,
	
	/// Only used if `type_identifier` points to a `BtfKind::Integer`.
	/// Interpretation varies depending on setting of `kind_flag` bit 31 inside `btf_type`.
	///
	/// See `btf_member_is_reg_int()` in `kernel/bpf/btf.c`.
	///
	/// In other words: yuck.
	pub(crate) offset: u32,
}
