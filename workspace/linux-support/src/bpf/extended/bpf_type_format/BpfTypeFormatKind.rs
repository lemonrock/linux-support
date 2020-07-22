// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// `BTF_KIND_*`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub(crate) enum BpfTypeFormatKind
{
	/// `BTF_KIND_UNKN`.
	Unknown = 0,
	
	/// `BTF_KIND_INT`.
	Integer = 1,
	
	/// `BTF_KIND_PTR`.
	Pointer = 2,
	
	/// `BTF_KIND_ARRAY`.
	Array = 3,
	
	/// `BTF_KIND_STRUCT`.
	Struct = 4,
	
	/// `BTF_KIND_UNION`.
	Union = 5,
	
	/// `BTF_KIND_ENUM`.
	Enumeration = 6,
	
	/// `BTF_KIND_FWD`.
	Forward = 7,
	
	/// `BTF_KIND_TYPEDEF`.
	TypeDefinition = 8,
	
	/// `BTF_KIND_VOLATILE`.
	Volatile = 9,
	
	/// `BTF_KIND_CONST`.
	Constant = 10,
	
	/// `BTF_KIND_RESTRICT`.
	Restrict = 11,
	
	/// `BTF_KIND_FUNC`.
	Function = 12,
	
	/// `BTF_KIND_FUNC_PROTO`.
	FunctionPrototype = 13,
	
	/// `BTF_KIND_VAR`.
	Variable = 14,
	
	/// `BTF_KIND_DATASEC`.
	Section = 15,
}

impl Default for BpfTypeFormatKind
{
	#[inline(always)]
	fn default() -> Self
	{
		BpfTypeFormatKind::Unknown
	}
}

impl BpfTypeFormatKind
{
	#[allow(dead_code)]
	pub(crate) const BTF_KIND_MAX: Self = BpfTypeFormatKind::Section;
	
	#[allow(dead_code)]
	pub(crate) const NR_BTF_KINDS: u8 = (BpfTypeFormatKind::BTF_KIND_MAX as u8) + 1;
}
