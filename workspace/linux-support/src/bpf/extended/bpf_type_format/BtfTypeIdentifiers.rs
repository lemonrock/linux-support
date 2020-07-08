// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct BtfTypeIdentifiers
{
	last_type_identifier: BtfTypeIdentifier,
	known_type_identifiers: HashMap<TypeId, BtfTypeIdentifier>,
	
	/// This is parsed in the Linux kernel by `btf_parse_type_sec()`.
	btf_header_then_type_identifiers_table: Vec<u8>,
	
	/// This is parsed in the Linux kernel by `btf_parse_str_sec()`.
	string_table: StringTable,
}

impl Default for BtfTypeIdentifiers
{
	#[inline(always)]
	fn default() -> Self
	{
		const CVoidType: &'static Type = &c_void::Type;
		
		let mut this = Self
		{
			last_type_identifier: BtfTypeIdentifier::Void,
			known_type_identifiers: maplit::hashmap!
			[
				CVoidType.type_id => BtfTypeIdentifier::Void,
			],
			btf_header_then_type_identifiers_table: Vec::with_capacity(btf_header::Size()),
			string_table: StringTable::default(),
		};
		
		this.reserve(btf_header::Size as u16);
		
		this.push_c_identifier_for_type(CVoidType, BtfKind::Unknown);
		
		this
	}
}

impl BtfTypeIdentifiers
{
	#[inline(always)]
	pub(crate) fn finish(self) -> Result<Vec<u8>, ProgramError>
	{
		use self::ProgramError::*;
		
		let type_identifiers_section_starts_at = btf_header::Size;
		let type_identifiers_section_length = self.btf_header_then_type_identifiers_table.len() - btf_header::Size;
		let string_section_starts_at = self.btf_header_then_type_identifiers_table.len();
		let string_section_length = self.string_table.len();
		
		// See check in the Linux kernel function `btf_parse_hdr()`
		if unlikely!(type_identifiers_section_length + string_section_length == 0)
		{
			return Err(NoBtfData)
		}
		
		let header_and_type_identifier_section_and_string_section = self.btf_header_then_type_identifiers_table;
		header_and_type_identifier_section_and_string_section.extend_from_slice(&self.string_table.string_table[..]);
		
		let header = btf_header::new(type_identifiers_section_starts_at, type_identifiers_section_length, string_section_starts_at,  string_section_length).map_err(InvalidBtfDataSize)?;
		unsafe { (header_and_type_identifier_section_and_string_section.as_mut_ptr() as *mut btf_header).write(header) };
		
		Ok(header_and_type_identifier_section_and_string_section)
	}
	
	#[inline(always)]
	pub(crate) fn create_function(&mut self, name: &Name, function_prototype: &FunctionPrototype) -> Result<BtfTypeIdentifier, BtfTypeError>
	{
		let function_prototype_type_identifier = self.create_function_prototype(function_prototype)?;
		let header = btf_type::function(self.push_c_identifier(name, BtfKind::Function)?.expect("void identifier created in Self::new()"), function_prototype_type_identifier, function_prototype.linkage)?;
		self.write_btf_type_header_unlinked_to_type_id(header)
	}
	
	#[inline(always)]
	pub(crate) fn get_or_create_type_identifier(&mut self, type_: &Type) -> Result<BtfTypeIdentifier, BtfTypeError>
	{
		let type_id = type_.type_id;
		
		if let Some(btf_type_identifier) = self.known_type_identifiers.get(&type_id)
		{
			return Ok(*btf_type_identifier)
		}
		
		use self::BtfTypeError::*;
		use self::Data::*;
		use self::StructFields::*;
		match type_.data
		{
			Primitive(encoding) => self.write_btf_type_header(type_id, self.primitive(type_, encoding)?),
			
			ReferenceOrConstantPointer(mutable_pointer_type) => self.write_btf_type_header(type_id, btf_type::constant(self.get_or_create_type_identifier(mutable_pointer_type)?)),
			
			MutableReferenceOrPointer(points_to_type) => self.write_btf_type_header(type_id, btf_type::pointer(self.get_or_create_type_identifier(points_to_type)?)),
			
			Array(element_type, count) => self.write_array(type_, element_type, count),
			
			Union(fields) => self.write_struct_or_union_fields(type_, fields, true, UnionHasTooManyFields),
			
			Struct(Named(fields)) => self.write_struct_or_union_fields(type_, fields, false, StructHasTooManyNamedFields),
			
			Struct(Unnamed(fields)) => self.write_struct_or_union_fields(type_, fields, false, StructHasTooManyUnnamedFields),
			
			Struct(Unit) => self.write_struct_or_union_fields(type_, &[], false, StructHasTooManyUnnamedFields),
			
			Enum(variants) => self.write_enum(type_, variants),
		}
	}
	
	#[inline(always)]
	fn create_function_prototype(&mut self, function_prototype: &FunctionPrototype) -> Result<BtfTypeIdentifier, BtfTypeError>
	{
		let &FunctionPrototype { parameters, return_type , .. } = function_prototype;
		
		let (parameters, vlen) = Self::guard_number_of_fields(parameters, BtfTypeError::FunctionHasTooManyParameters)?;
		
		let return_type_identifier = self.get_or_create_type_identifier(return_type)?;
		let function_prototype_type_identifier = self.write_btf_type_header_unlinked_to_type_id(btf_type::function_prototype(vlen, return_type_identifier))?;
		
		let start_overwriting_at = self.reserve(vlen);
		for index in 0 .. vlen
		{
			let &(parameter_name, parameter_type) = unsafe { parameters.get_unchecked(index as usize) };
			
			let param = btf_param
			{
				name_off: self.push_c_identifier(parameter_name, BtfKind::FunctionPrototype)?.expect("void identifier created in Self::new()"),
				type_: self.get_or_create_type_identifier(parameter_type)?,
			};
			self.overwrite(param, start_overwriting_at, index)
		}
		
		Ok(function_prototype_type_identifier)
	}
	
	#[inline(always)]
	fn primitive(&mut self, type_: &Type, encoding: BtfTypeIntegerEncoding) -> Result<btf_type<u32>, BtfTypeError>
	{
		const NoOffset: u8 = 0;
		
		let size = type_.size;
		if unlikely!(size > 31)
		{
			return Err(BtfTypeError::IntegerSizeCanNotExceed31Bytes)
		}
		let bits = (size * 8) as u8;
		
		btf_type::<u32>::integer(self.push_c_identifier_for_type(type_, BtfKind::Integer)?, size, encoding, NoOffset, bits)
	}

	#[inline(always)]
	fn write_array(&mut self, type_: &Type, element_type: &Type, count: usize) -> Result<BtfTypeIdentifier, BtfTypeError>
	{
		if unlikely!(count > (u32::MAX as usize))
		{
			return Err(BtfTypeError::MoreThanU32MaxArrayElements)
		}
		
		let element_type_identifier = self.get_or_create_type_identifier(element_type)?;
		let index_type_identifier = self.get_or_create_type_identifier(&usize::Type)?;
		let header = btf_type::array(element_type_identifier, index_type_identifier, count as u32);
		self.write_btf_type_header(type_.type_id, header)
	}

	#[inline(always)]
	fn write_struct_or_union_fields<F: Field>(&mut self, type_: &Type, fields: &[F], is_union: bool, error: BtfTypeError) -> Result<BtfTypeIdentifier, BtfTypeError>
	{
		let (fields, vlen) = Self::guard_number_of_fields(fields, error)?;
		
		if vlen == 0 && type_.size != 0
		{
			return Err(BtfTypeError::SizeMustBeZeroIfThereAreNoFields)
		}
		
		let (offset_of_name_into_string_section, _forward_declared_type_identifier) = self.forward_declare_struct_or_union(type_, is_union)?;
		
		let header = btf_type::struct_or_union(offset_of_name_into_string_section, vlen, type_.size, is_union)?;
		let struct_or_union_type_identifier = self.write_btf_type_header(type_.type_id, header)?;
		
		let start_overwriting_at = self.reserve(vlen);
		for index in 0 .. vlen
		{
			let field = unsafe { fields.get_unchecked(index as usize) };
			let member = field.to_btf_member(self, index)?;
			self.overwrite(member, start_overwriting_at, index)
		}
		
		Ok(struct_or_union_type_identifier)
	}
	
	#[inline(always)]
	fn forward_declare_struct_or_union(&mut self, type_: &Type, is_union: bool) -> Result<(Option<NonZeroU32>, BtfTypeIdentifier), BtfTypeError>
	{
		let offset_of_name_into_string_section = self.push_c_identifier_for_type(type_, BtfKind::Forward)?;
		let forward_declared_type_identifier = self.write_btf_type_header_unlinked_to_type_id(btf_type::forward_struct_or_union(offset_of_name_into_string_section, is_union)?)?;
		
		Ok((offset_of_name_into_string_section, forward_declared_type_identifier))
	}
	
	#[inline(always)]
	fn write_enum(&mut self, type_: &Type, variants: &[Variant]) -> Result<BtfTypeIdentifier, BtfTypeError>
	{
		use self::BtfTypeError::*;
		
		let (variants, vlen) = Self::guard_number_of_fields(variants, EnumHasTooManyVariants)?;
		let offset_of_name_into_string_section = self.push_c_identifier_for_type(type_, BtfKind::Enumeration)?;
		let enum_type_identifier = self.write_btf_type_header(type_.type_id, btf_type::r#enum(offset_of_name_into_string_section, vlen)?)?;
		
		for variant in variants
		{
			use self::EnumVariantFields::*;
			match variant.fields
			{
				UnitValued(val) =>
				{
					let name_off = self.push_c_identifier_for_type(type_, BtfKind::Enumeration)?.expect("void identifier created in Self::new()");
					self.write
					(
						btf_enum
						{
							name_off,
							val
						}
					)
				}
				
				Named(fields) => return Err(EnumVariantWithNamedFieldsIsUnsupported),
				
				Unnamed(fields) => return Err(EnumVariantWithUnnamedFieldsIsUnsupported),
				
				UnitValuelessOrDoesNotFitInI32 => return Err(EnumUnitVariantIsNotRepresentableInAnI32),
			}
		}
		
		Ok(enum_type_identifier)
	}
	
	#[inline(always)]
	fn guard_number_of_fields<T>(fields: &[T], error: BtfTypeError) -> Result<(&[T], u16), BtfTypeError>
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
	fn write_btf_type_header<T: Sized>(&mut self, type_id: TypeId, header: btf_type<T>) -> Result<BtfTypeIdentifier, BtfTypeError>
	{
		let type_identifier = self.write_btf_type_header_unlinked_to_type_id(header)?;
		let previous = self.known_type_identifiers.insert(type_id, type_identifier);
		debug_assert!(previous.is_none());
		Ok(type_identifier)
	}
	
	#[inline(always)]
	fn write_btf_type_header_unlinked_to_type_id<T: Sized>(&mut self, header: btf_type<T>) -> Result<BtfTypeIdentifier, BtfTypeError>
	{
		self.write::<btf_type<T>>(header);
		self.next_type_identifier()
	}
	
	#[inline(always)]
	fn reserve<T: Sized>(&mut self, vlen: u16) -> usize
	{
		let needed_capacity = size_of::<T>() * (vlen as usize);
		self.btf_header_then_type_identifiers_table.reserve(needed_capacity);
		let start_overwriting_at = self.btf_header_then_type_identifiers_table.len();
		unsafe { self.btf_header_then_type_identifiers_table.set_len(start_overwriting_at + needed_capacity) }
		start_overwriting_at
	}
	
	#[inline(always)]
	fn overwrite<T: Sized>(&mut self, value: T, start_overwriting_at: usize, index: u16)
	{
		unsafe { (self.btf_header_then_type_identifiers_table.as_mut_ptr() as *mut T).add(start_overwriting_at + ((index as usize) * size_of::<T>())).write(value) }
	}
	
	#[inline(always)]
	fn write<T: Sized>(&mut self, value: T)
	{
		let needed_capacity = size_of::<T>();
		self.btf_header_then_type_identifiers_table.reserve(needed_capacity);
		let length = self.btf_header_then_type_identifiers_table.len();
		unsafe
		{
			(self.btf_header_then_type_identifiers_table.as_mut_ptr().add(length) as *mut T).write(value);
			self.btf_header_then_type_identifiers_table.set_len(length + needed_capacity);
		}
	}
	
	#[inline(always)]
	fn next_type_identifier(&mut self) -> Result<BtfTypeIdentifier, BtfTypeError>
	{
		self.last_type_identifier.next()
	}
	
	#[inline(always)]
	pub(crate) fn push_c_identifier_for_type(&mut self, type_: &Type, kind: BtfKind) -> Result<Option<NonZeroU32>, BtfTypeError>
	{
		self.push_c_identifier(type_.ident, kind)
	}
	
	#[inline(always)]
	pub(crate) fn push_c_identifier(&mut self, ident: &str, kind: BtfKind) -> Result<Option<NonZeroU32>, BtfTypeError>
	{
		self.string_table.push_c_identifier(ident, kind)
	}
	
	#[inline(always)]
	pub(crate) fn push_any(&mut self, any: &str) -> Result<Option<NonZeroU32>, BtfTypeError>
	{
		self.string_table.push_any(any)
	}
}
