// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A variably-sized type.
///
/// See <https://www.kernel.org/doc/html/latest/bpf/btf.html>.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub(crate) struct btf_type<ExtraData: Sized>
{
	/// Maximum value is `2^24 - 1`.
	name_off: u32,
	
	/// `info` is a bit set:-
	///
	/// * bits 0-15: `v_length` (eg number of `BTF_KIND_STRUCT`'s members).
	/// * bits 16-23: unused.
	/// * bits 24-27: `kind` (eg `BTF_KIND_INT`, `BTF_KIND_PTR`, `BTF_KIND_ARRAY`, etc).
	/// * bits 28-30: unused.
	/// * bit     31: `kind_flag`, currently used by `BTF_KIND_STRUCT`, `BTF_KIND_UNION` and `BTF_KIND_FWD`.
	info: u32,
	
	btf_type_size_or_type: BpfTypeFormatTypeSizeOrTypeIdentifier,
	
	extra_data: ExtraData,
}

impl<ExtraData: Sized> btf_type<ExtraData>
{
	#[allow(dead_code)]
	#[inline(always)]
	fn kind(&self) -> BpfTypeFormatKind
	{
		unsafe { transmute(Self::BTF_INFO_KIND(self.info) as u8) }
	}
	
	#[allow(dead_code)]
	#[inline(always)]
	fn v_length(&self) -> u16
	{
		Self::BTF_INFO_VLEN(self.info)
	}
	
	#[allow(dead_code)]
	#[inline(always)]
	fn kind_flag(&self) -> bool
	{
		Self::BTF_INFO_KFLAG(self.info)
	}
	
	#[inline(always)]
	const fn BTF_INFO_KIND(info: u32) -> u32
	{
		(info >> 24) & 0x0F
	}
	
	#[inline(always)]
	const fn BTF_INFO_VLEN(info: u32) -> u16
	{
		(info & 0xFFFF) as u16
	}
	
	#[inline(always)]
	const fn BTF_INFO_KFLAG(info: u32) -> bool
	{
		(info >> 31) != 0
	}
	
	#[inline(always)]
	const fn info(kind: BpfTypeFormatKind, v_length: u16, kind_flag: bool) -> u32
	{
		((kind_flag as u32) << 31) | ((kind as u8 as u32) << 24) | (v_length as u32)
	}
	
	#[inline(always)]
	fn guard_offset_of_name_into_string_section(offset_of_name_into_string_section: Option<NonZeroU32>) -> Result<u32, BpfTypeFormatError>
	{
		match offset_of_name_into_string_section
		{
			None => Ok(0),
			
			Some(offset_of_name_into_string_section) =>
			{
				let offset_of_name_into_string_section = offset_of_name_into_string_section.get();
				if unlikely!(offset_of_name_into_string_section >= BTF_MAX_NAME_OFFSET)
				{
					Err(BpfTypeFormatError::StringTableOffsetIsTooLarge)
				}
				else
				{
					Ok(offset_of_name_into_string_section)
				}
			}
		}
		
	}
}

impl btf_type<__IncompleteArrayField<btf_param>>
{
	/// Maximum value of `offset_of_name_into_string_section` is `2^24 - 1`.
	///
	/// Assumes `kind_flag` is `true`.
	#[inline(always)]
	pub(crate) fn function_prototype(v_length: u16, return_type: BpfTypeFormatTypeIdentifier) -> Self
	{
		Self
		{
			name_off: 0,
			info: Self::info(BpfTypeFormatKind::FunctionPrototype, v_length, false),
			btf_type_size_or_type: BpfTypeFormatTypeSizeOrTypeIdentifier
			{
				type_identifier: return_type,
			},
			extra_data: __IncompleteArrayField::new(),
		}
	}
}

impl btf_type<__IncompleteArrayField<btf_enum>>
{
	/// Maximum value of `offset_of_name_into_string_section` is `2^24 - 1`.
	///
	/// Assumes `kind_flag` is `true`.
	#[inline(always)]
	pub(crate) fn r#enum(offset_of_name_into_string_section: Option<NonZeroU32>, v_length: u16) -> Result<Self, BpfTypeFormatError>
	{
		Ok
		(
			Self
			{
				name_off: Self::guard_offset_of_name_into_string_section(offset_of_name_into_string_section)?,
				info: Self::info(BpfTypeFormatKind::Enumeration, v_length, false),
				btf_type_size_or_type: BpfTypeFormatTypeSizeOrTypeIdentifier
				{
					size: size_of::<i32>() as u32,
				},
				extra_data: __IncompleteArrayField::new(),
			}
		)
	}
}

impl btf_type<__IncompleteArrayField<btf_member>>
{
	#[inline(always)]
	pub(crate) fn struct_or_union(offset_of_name_into_string_section: Option<NonZeroU32>, v_length: u16, size: u32, is_union: bool) -> Result<Self, BpfTypeFormatError>
	{
		let kind = if is_union
		{
			BpfTypeFormatKind::Union
		}
		else
		{
			BpfTypeFormatKind::Struct
		};
		Ok
		(
			Self
			{
				name_off: Self::guard_offset_of_name_into_string_section(offset_of_name_into_string_section)?,
				info: Self::info(kind, v_length, true),
				btf_type_size_or_type: BpfTypeFormatTypeSizeOrTypeIdentifier
				{
					size,
				},
				extra_data: __IncompleteArrayField::new(),
			}
		)
	}
}

impl btf_type<()>
{
	#[inline(always)]
	pub(crate) fn forward_struct_or_union(offset_of_name_into_string_section: NonZeroU32, is_union: bool) -> Result<Self, BpfTypeFormatError>
	{
		Ok
		(
			Self
			{
				name_off: Self::guard_offset_of_name_into_string_section(Some(offset_of_name_into_string_section))?,
				info: Self::info(BpfTypeFormatKind::Forward, 0, is_union),
				btf_type_size_or_type: BpfTypeFormatTypeSizeOrTypeIdentifier
				{
					type_identifier: BpfTypeFormatTypeIdentifier::Void,
				},
				extra_data: (),
			}
		)
	}
	
	#[allow(dead_code)]
	#[inline(always)]
	pub(crate) fn type_definition(offset_of_name_into_string_section: NonZeroU32, type_identifier: BpfTypeFormatTypeIdentifier) -> Result<Self, BpfTypeFormatError>
	{
		Ok
		(
			Self
			{
				name_off: Self::guard_offset_of_name_into_string_section(Some(offset_of_name_into_string_section))?,
				info: Self::info(BpfTypeFormatKind::TypeDefinition, 0, false),
				btf_type_size_or_type: BpfTypeFormatTypeSizeOrTypeIdentifier
				{
					type_identifier,
				},
				extra_data: (),
			}
		)
	}
	
	#[inline(always)]
	pub(crate) const fn constant(type_identifier: BpfTypeFormatTypeIdentifier) -> Self
	{
		Self
		{
			name_off: 0,
			info: Self::info(BpfTypeFormatKind::Constant, 0, false),
			btf_type_size_or_type: BpfTypeFormatTypeSizeOrTypeIdentifier
			{
				type_identifier,
			},
			extra_data: (),
		}
	}
	
	#[allow(dead_code)]
	#[inline(always)]
	pub(crate) const fn volatile(type_identifier: BpfTypeFormatTypeIdentifier) -> Self
	{
		Self
		{
			name_off: 0,
			info: Self::info(BpfTypeFormatKind::Volatile, 0, false),
			btf_type_size_or_type: BpfTypeFormatTypeSizeOrTypeIdentifier
			{
				type_identifier,
			},
			extra_data: (),
		}
	}
	
	#[allow(dead_code)]
	#[inline(always)]
	pub(crate) const fn restrict(type_identifier: BpfTypeFormatTypeIdentifier) -> Self
	{
		Self
		{
			name_off: 0,
			info: Self::info(BpfTypeFormatKind::Restrict, 0, false),
			btf_type_size_or_type: BpfTypeFormatTypeSizeOrTypeIdentifier
			{
				type_identifier,
			},
			extra_data: (),
		}
	}
	
	#[inline(always)]
	pub(crate) const fn pointer(type_identifier: BpfTypeFormatTypeIdentifier) -> Self
	{
		Self
		{
			name_off: 0,
			info: Self::info(BpfTypeFormatKind::Pointer, 0, false),
			btf_type_size_or_type: BpfTypeFormatTypeSizeOrTypeIdentifier
			{
				type_identifier,
			},
			extra_data: (),
		}
	}
	
	/// Maximum value of `offset_of_name_into_string_section` is `2^24 - 1`.
	#[inline(always)]
	pub(crate) fn function(offset_of_name_into_string_section: NonZeroU32, function_prototype_type_identifier: BpfTypeFormatTypeIdentifier, linkage: btf_func_linkage) -> Result<Self, BpfTypeFormatError>
	{
		Ok
		(
			Self
			{
				name_off: Self::guard_offset_of_name_into_string_section(Some(offset_of_name_into_string_section))?,
				info: Self::info(BpfTypeFormatKind::Function, linkage as u16, false),
				btf_type_size_or_type: BpfTypeFormatTypeSizeOrTypeIdentifier
				{
					type_identifier: function_prototype_type_identifier,
				},
				extra_data: (),
			}
		)
	}
}

impl btf_type<btf_array>
{
	/// `index_type_identifier` should be an unsigned integer equivalent to `u8`, `u16`, `u32`, `u64` or `u128`; it isn't used for anything more than type validation.
	#[inline(always)]
	pub(crate) const fn array(element_type_identifier: BpfTypeFormatTypeIdentifier, index_type_identifier: BpfTypeFormatTypeIdentifier, number_of_elements: u32) -> Self
	{
		Self
		{
			name_off: 0,
			info: Self::info(BpfTypeFormatKind::Array, 0, false),
			btf_type_size_or_type: BpfTypeFormatTypeSizeOrTypeIdentifier
			{
				type_identifier: element_type_identifier,
			},
			extra_data: btf_array
			{
				type_: element_type_identifier,
				index_type: index_type_identifier,
				nelems: number_of_elements,
			},
		}
	}
}

impl btf_type<btf_var>
{
	#[allow(dead_code)]
	pub(crate) fn variable(offset_of_name_into_string_section: Option<NonZeroU32>, type_identifier: BpfTypeFormatTypeIdentifier, linkage: BpfTypeFormatVariableLinkage) -> Result<Self, BpfTypeFormatError>
	{
		Ok
		(
			Self
			{
				name_off: Self::guard_offset_of_name_into_string_section(offset_of_name_into_string_section)?,
				info: Self::info(BpfTypeFormatKind::Variable, 0, false),
				btf_type_size_or_type: BpfTypeFormatTypeSizeOrTypeIdentifier
				{
					type_identifier,
				},
				extra_data: btf_var
				{
					linkage,
				},
			}
		)
	}
	
}

impl btf_type<u32>
{
	/// Maximum value of `offset_of_name_into_string_section` is `2^24 - 1`.
	/// Maximum value of `bits` is `128`.
	/// `size * 8 >= (bits as u32)`.
	///
	/// `bits` and `offset` were designed to allow the encoding of bitfields in structs.
	/// In practice, `offset` is always `0`.
	///
	/// Since Rust does not support bitfields, `offset` should always be `0` and bits `8`, `16`, `32`, `64` or `128`.
	pub(crate) fn integer(offset_of_name_into_string_section: Option<NonZeroU32>, size: u32, encoding: BpfTypeFormatIntegerEncoding, offset: u8, bits: u8) -> Result<Self, BpfTypeFormatError>
	{
		use self::BpfTypeFormatError::*;
		
		if unlikely!(bits > 128)
		{
			return Err(IntegerSizeExceeds128)
		}
		
		if unlikely!(((size as u64) * 8) < (bits as u64))
		{
			return Err(IntegerBitsGreaterThanIntegerSize)
		}
		
		Ok
		(
			Self
			{
				name_off: Self::guard_offset_of_name_into_string_section(offset_of_name_into_string_section)?,
				info: Self::info(BpfTypeFormatKind::Integer, 0, false),
				btf_type_size_or_type: BpfTypeFormatTypeSizeOrTypeIdentifier
				{
					size,
				},
				extra_data: (encoding as u8 as u32) << 24 | (offset as u32) << 16 | bits as u32,
			}
		)
	}
	
	#[allow(dead_code)]
	#[inline(always)]
	const fn BTF_INT_ENCODING(VAL: u32) -> BpfTypeFormatIntegerEncoding
	{
		unsafe { transmute((VAL & 0x0F000000 >> 24) as u8) }
	}
	
	#[allow(dead_code)]
	#[inline(always)]
	const fn BTF_INT_OFFSET(VAL: u32) -> u8
	{
		(VAL & 0x00FF0000 >> 16) as u8
	}
	
	#[allow(dead_code)]
	#[inline(always)]
	const fn BTF_INT_BITS(VAL: u32) -> u8
	{
		(VAL & 0x000000FF) as u8
	}
}
