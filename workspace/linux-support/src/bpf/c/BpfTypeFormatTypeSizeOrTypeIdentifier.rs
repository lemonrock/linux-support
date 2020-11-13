// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Not an actual type.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union BpfTypeFormatTypeSizeOrTypeIdentifier
{
	/// Used by `BTF_KIND_INT`, `BTF_KIND_ENUM`, `BTF_KIND_ STRUCT`, `BTF_KIND_UNION` and `BTF_KIND_DATASEC`.
	///
	/// The size of the described type (in bytes).
	pub(crate) size: u32,
	
	/// Used by `BTF_KIND_PTR, `BTF_KIND_TYPEDEF`, `BTF_KIND_VOLATILE, `BTF_KIND_CONST, `BTF_KIND_RESTRICT, `BTF_KIND_FUNC, `BTF_KIND_FUNC_PROTO and `BTF_KIND_VAR`.
	///
	/// A reference to another type identifier.
	pub(crate) type_identifier: BpfTypeFormatTypeIdentifier,
}

impl Default for BpfTypeFormatTypeSizeOrTypeIdentifier
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe_zeroed()
	}
}

impl Debug for BpfTypeFormatTypeSizeOrTypeIdentifier
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "bpf_attr {{ union }}")
	}
}
