// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct BpfTypeFormatTypeIdentifiers
{
	last_type_identifier: BpfTypeFormatTypeIdentifier,
	known_type_identifiers: HashMap<TypeId, BpfTypeFormatTypeIdentifier>,
	
	/// This is parsed in the Linux kernel by `btf_parse_type_sec()`.
	bpf_type_format_header_then_type_identifiers_table: Vec<u8>,
	
	/// This is parsed in the Linux kernel by `btf_parse_str_sec()`.
	string_table: StringTable,
}

impl Default for BpfTypeFormatTypeIdentifiers
{
	#[inline(always)]
	fn default() -> Self
	{
		const CVoidType: &'static Type = &c_void::Type;
		
		let mut this = Self
		{
			last_type_identifier: BpfTypeFormatTypeIdentifier::Void,
			known_type_identifiers: maplit::hashmap!
			[
				CVoidType.type_id => BpfTypeFormatTypeIdentifier::Void,
			],
			bpf_type_format_header_then_type_identifiers_table: Vec::with_capacity(btf_header::Size),
			string_table: StringTable::default(),
		};
		
		this.reserve::<btf_header>(1);
		
		this.push_c_identifier_for_type(CVoidType, BpfTypeFormatKind::Unknown).unwrap();
		
		this
	}
}

impl BpfTypeFormatTypeIdentifiers
{
	#[inline(always)]
	pub(crate) fn finish(self) -> Result<Vec<u8>, ParseError>
	{
		use self::ParseError::*;
		
		let type_identifiers_section_starts_at = btf_header::Size;
		let type_identifiers_section_length = self.bpf_type_format_header_then_type_identifiers_table.len() - btf_header::Size;
		let string_section_starts_at = self.bpf_type_format_header_then_type_identifiers_table.len();
		
		let string_section = self.string_table.string_table;
		let string_section_length = string_section.len();
		
		// See check in the Linux kernel function `btf_parse_hdr()`
		if unlikely!(type_identifiers_section_length + string_section_length == 0)
		{
			return Err(NoBpfTypeFormatData)
		}
		
		let mut header_and_type_identifier_section_and_string_section = self.bpf_type_format_header_then_type_identifiers_table;
		header_and_type_identifier_section_and_string_section.extend_from_slice(&string_section[..]);
		
		let header = Self::btf_header(type_identifiers_section_starts_at, type_identifiers_section_length, string_section_starts_at,  string_section_length)?;
		unsafe { (header_and_type_identifier_section_and_string_section.as_mut_ptr() as *mut btf_header).write(header) };
		
		Ok(header_and_type_identifier_section_and_string_section)
	}
	
	#[inline(always)]
	pub(crate) fn btf_header(type_identifier_section_starts_at: usize, type_identifier_section_length: usize, string_section_starts_at: usize, string_section_length: usize) -> Result<btf_header, ParseError>
	{
		#[inline(always)]
		fn offset(starts_at: usize) -> Result<u32, TryFromIntError>
		{
			(starts_at - btf_header::Size).try_into()
		}
		
		Ok
		(
			btf_header::new
			(
				offset(type_identifier_section_starts_at)?,
				type_identifier_section_length.try_into()?,
				offset(string_section_starts_at)?,
				string_section_length.try_into()?,
			)
		)
	}
	
	
	#[inline(always)]
	pub(crate) fn create_function(&mut self, name: &Name, function_prototype: &FunctionPrototype) -> Result<BpfTypeFormatTypeIdentifier, BpfTypeFormatError>
	{
		let function_prototype_type_identifier = self.create_function_prototype(function_prototype)?;
		let header = btf_type::function(self.push_c_identifier(name, BpfTypeFormatKind::Function)?.expect("void identifier created in Self::new()"), function_prototype_type_identifier, function_prototype.linkage)?;
		self.write_btf_header_unlinked_to_type_identifier(header)
	}
	
	#[inline(always)]
	pub(crate) fn get_or_create_type_identifier(&mut self, type_: &Type) -> Result<BpfTypeFormatTypeIdentifier, BpfTypeFormatError>
	{
		let type_id = type_.type_id;
		
		if let Some(bpf_type_format_type_identifier) = self.known_type_identifiers.get(&type_id)
		{
			return Ok(*bpf_type_format_type_identifier)
		}
		
		use self::BpfTypeFormatError::*;
		use self::Data::*;
		use self::StructFields::*;
		match type_.data
		{
			Primitive(encoding) =>
			{
				let btf_type = self.primitive(type_, encoding)?;
				self.write_btf_header(type_id, btf_type)
			},
			
			ReferenceOrConstantPointer(mutable_pointer_type) =>
			{
				let btf_type = btf_type::constant(self.get_or_create_type_identifier(mutable_pointer_type)?);
				self.write_btf_header(type_id, btf_type)
			},
			
			MutableReferenceOrPointer(points_to_type) =>
			{
				let btf_type = btf_type::pointer(self.get_or_create_type_identifier(points_to_type)?);
				self.write_btf_header(type_id, btf_type)
			},
			
			Array(element_type, count) => self.write_array(type_, element_type, count),
			
			Union(fields) => self.write_struct_or_union_fields(type_, fields, true, UnionHasTooManyFields),
			
			Struct(Named(fields)) => self.write_struct_or_union_fields(type_, fields, false, StructHasTooManyNamedFields),
			
			Struct(Unnamed(fields)) => self.write_struct_or_union_fields(type_, fields, false, StructHasTooManyUnnamedFields),
			
			Struct(Unit) => self.write_struct_or_union_fields::<NamedField>(type_, &[], false, StructHasTooManyUnnamedFields),
			
			Enum(variants) => self.write_enum(type_, variants),
		}
	}
	
	#[inline(always)]
	fn create_function_prototype(&mut self, function_prototype: &FunctionPrototype) -> Result<BpfTypeFormatTypeIdentifier, BpfTypeFormatError>
	{
		let &FunctionPrototype { parameters, return_type , .. } = function_prototype;
		
		let (parameters, vlen) = Self::guard_number_of_fields(parameters, BpfTypeFormatError::FunctionHasTooManyParameters)?;
		
		let return_type_identifier = self.get_or_create_type_identifier(return_type)?;
		let function_prototype_type_identifier = self.write_btf_header_unlinked_to_type_identifier(btf_type::function_prototype(vlen, return_type_identifier))?;
		
		let start_overwriting_at = self.reserve::<btf_param>(vlen);
		for index in 0 .. vlen
		{
			let &(parameter_name, parameter_type) = unsafe { parameters.get_unchecked(index as usize) };
			
			let param = btf_param
			{
				name_off: unsafe { transmute(self.push_c_identifier(parameter_name, BpfTypeFormatKind::FunctionPrototype)?.expect("void identifier created in Self::new()")) },
				type_: self.get_or_create_type_identifier(parameter_type)?,
			};
			self.overwrite(param, start_overwriting_at, index)
		}
		
		Ok(function_prototype_type_identifier)
	}
	
	#[inline(always)]
	fn primitive(&mut self, type_: &Type, encoding: BpfTypeFormatIntegerEncoding) -> Result<btf_type<u32>, BpfTypeFormatError>
	{
		const NoOffset: u8 = 0;
		
		let size = type_.size;
		if unlikely!(size > 31)
		{
			return Err(BpfTypeFormatError::IntegerSizeCanNotExceed31Bytes)
		}
		let bits = (size * 8) as u8;
		
		btf_type::<u32>::integer(self.push_c_identifier_for_type(type_, BpfTypeFormatKind::Integer)?, size, encoding, NoOffset, bits)
	}

	#[inline(always)]
	fn write_array(&mut self, type_: &Type, element_type: &Type, count: usize) -> Result<BpfTypeFormatTypeIdentifier, BpfTypeFormatError>
	{
		if unlikely!(count > (u32::MAX as usize))
		{
			return Err(BpfTypeFormatError::MoreThanU32MaxArrayElements)
		}
		
		let element_type_identifier = self.get_or_create_type_identifier(element_type)?;
		let index_type_identifier = self.get_or_create_type_identifier(&usize::Type)?;
		let header = btf_type::array(element_type_identifier, index_type_identifier, count as u32);
		self.write_btf_header(type_.type_id, header)
	}

	#[inline(always)]
	fn write_struct_or_union_fields<F: Field>(&mut self, type_: &Type, fields: &[F], is_union: bool, error: BpfTypeFormatError) -> Result<BpfTypeFormatTypeIdentifier, BpfTypeFormatError>
	{
		let (fields, vlen) = Self::guard_number_of_fields(fields, error)?;
		
		if vlen == 0 && type_.size != 0
		{
			return Err(BpfTypeFormatError::SizeMustBeZeroIfThereAreNoFields)
		}
		
		let (offset_of_name_into_string_section, _forward_declared_type_identifier) = self.forward_declare_struct_or_union(type_, is_union)?;
		
		let header = btf_type::struct_or_union(offset_of_name_into_string_section, vlen, type_.size, is_union)?;
		let struct_or_union_type_identifier = self.write_btf_header(type_.type_id, header)?;
		
		let start_overwriting_at = self.reserve::<btf_member>(vlen);
		for index in 0 .. vlen
		{
			let field = unsafe { fields.get_unchecked(index as usize) };
			let member = field.to_btf_member(self, index)?;
			self.overwrite(member, start_overwriting_at, index)
		}
		
		Ok(struct_or_union_type_identifier)
	}
	
	#[inline(always)]
	fn forward_declare_struct_or_union(&mut self, type_: &Type, is_union: bool) -> Result<(Option<NonZeroU32>, BpfTypeFormatTypeIdentifier), BpfTypeFormatError>
	{
		let offset_of_name_into_string_section = self.push_c_identifier_for_type(type_, BpfTypeFormatKind::Forward)?;
		let forward_declared_type_identifier = self.write_btf_header_unlinked_to_type_identifier(btf_type::forward_struct_or_union(offset_of_name_into_string_section.unwrap(), is_union)?)?;
		
		Ok((offset_of_name_into_string_section, forward_declared_type_identifier))
	}
	
	#[inline(always)]
	fn write_enum(&mut self, type_: &Type, variants: &[Variant]) -> Result<BpfTypeFormatTypeIdentifier, BpfTypeFormatError>
	{
		use self::BpfTypeFormatError::*;
		
		let (variants, vlen) = Self::guard_number_of_fields(variants, EnumHasTooManyVariants)?;
		let offset_of_name_into_string_section = self.push_c_identifier_for_type(type_, BpfTypeFormatKind::Enumeration)?;
		let enum_type_identifier = self.write_btf_header(type_.type_id, btf_type::r#enum(offset_of_name_into_string_section, vlen)?)?;
		
		for variant in variants
		{
			use self::EnumVariantFields::*;
			match variant.fields
			{
				UnitValued(val) =>
				{
					let name_off = unsafe { transmute(self.push_c_identifier_for_type(type_, BpfTypeFormatKind::Enumeration)?.expect("void identifier created in Self::new()")) };
					self.write
					(
						btf_enum
						{
							name_off,
							val
						}
					)
				}
				
				Named(_fields) => return Err(EnumVariantWithNamedFieldsIsUnsupported),
				
				Unnamed(_fields) => return Err(EnumVariantWithUnnamedFieldsIsUnsupported),
				
				UnitValuelessOrDoesNotFitInI32 => return Err(EnumUnitVariantIsNotRepresentableInAnI32),
			}
		}
		
		Ok(enum_type_identifier)
	}
	
	#[inline(always)]
	fn guard_number_of_fields<T>(fields: &[T], error: BpfTypeFormatError) -> Result<(&[T], u16), BpfTypeFormatError>
	{
		if unlikely!(fields.len() >= BTF_MAX_VLEN)
		{
			Err(error)
		}
		else
		{
			Ok((fields, fields.len() as u16))
		}
	}
	
	#[inline(always)]
	fn write_btf_header<T: Sized>(&mut self, type_id: TypeId, header: btf_type<T>) -> Result<BpfTypeFormatTypeIdentifier, BpfTypeFormatError>
	{
		let type_identifier = self.write_btf_header_unlinked_to_type_identifier(header)?;
		let previous = self.known_type_identifiers.insert(type_id, type_identifier);
		debug_assert!(previous.is_none());
		Ok(type_identifier)
	}
	
	#[inline(always)]
	fn write_btf_header_unlinked_to_type_identifier<T: Sized>(&mut self, header: btf_type<T>) -> Result<BpfTypeFormatTypeIdentifier, BpfTypeFormatError>
	{
		self.write::<btf_type<T>>(header);
		self.next_type_identifier()
	}
	
	#[inline(always)]
	fn reserve<T: Sized>(&mut self, vlen: u16) -> usize
	{
		let needed_capacity = size_of::<T>() * (vlen as usize);
		self.bpf_type_format_header_then_type_identifiers_table.reserve(needed_capacity);
		let start_overwriting_at = self.bpf_type_format_header_then_type_identifiers_table.len();
		unsafe { self.bpf_type_format_header_then_type_identifiers_table.set_len(start_overwriting_at + needed_capacity) }
		start_overwriting_at
	}
	
	#[inline(always)]
	fn overwrite<T: Sized>(&mut self, value: T, start_overwriting_at: usize, index: u16)
	{
		unsafe { (self.bpf_type_format_header_then_type_identifiers_table.as_mut_ptr() as *mut T).add(start_overwriting_at + ((index as usize) * size_of::<T>())).write(value) }
	}
	
	#[inline(always)]
	fn write<T: Sized>(&mut self, value: T)
	{
		let needed_capacity = size_of::<T>();
		self.bpf_type_format_header_then_type_identifiers_table.reserve(needed_capacity);
		let length = self.bpf_type_format_header_then_type_identifiers_table.len();
		unsafe
		{
			(self.bpf_type_format_header_then_type_identifiers_table.as_mut_ptr().add(length) as *mut T).write(value);
			self.bpf_type_format_header_then_type_identifiers_table.set_len(length + needed_capacity);
		}
	}
	
	#[inline(always)]
	fn next_type_identifier(&mut self) -> Result<BpfTypeFormatTypeIdentifier, BpfTypeFormatError>
	{
		self.last_type_identifier.next()
	}
	
	#[inline(always)]
	pub(crate) fn push_c_identifier_for_type(&mut self, type_: &Type, kind: BpfTypeFormatKind) -> Result<Option<NonZeroU32>, BpfTypeFormatError>
	{
		self.push_c_identifier(type_.ident, kind)
	}
	
	#[inline(always)]
	pub(crate) fn push_c_identifier(&mut self, ident: &str, kind: BpfTypeFormatKind) -> Result<Option<NonZeroU32>, BpfTypeFormatError>
	{
		self.string_table.push_c_identifier(ident, kind)
	}
	
	#[inline(always)]
	pub(crate) fn push_any(&mut self, any: &str) -> Result<Option<NonZeroU32>, BpfTypeFormatError>
	{
		self.string_table.push_any(any)
	}
}
