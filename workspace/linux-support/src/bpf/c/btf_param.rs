// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub(crate) struct btf_param
{
	/// Zero for a final varargs argument (`...`).
	pub(crate) name_off: u32,
	
	/// Void (zero) for a final varargs argument (`...`).
	pub(crate) type_: BpfTypeFormatTypeIdentifier,
}

impl btf_param
{
	#[allow(dead_code)]
	pub(crate) const FinalVarArgsParameter: Self = Self
	{
		name_off: 0,
		type_: BpfTypeFormatTypeIdentifier::Void,
	};
}
