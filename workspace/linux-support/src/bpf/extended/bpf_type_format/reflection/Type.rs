// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Type information for a type that implements `TypeInfo`.
///
/// Does not work well for types with generic parameters.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Type
{
	/// The globally unique identifier for this type, `T: HasReflectionInformation`.
	///
	/// `Type::of::<T>()`.
	pub type_id: TypeId,
	
	/// The size of this type, `T: HasReflectionInformation`.
	///
	/// `size_of::<T>() as u32`.
	pub size: u32,
	
	/// The identifier of this type, `T: HasReflectionInformation`.
	///
	/// Should be the fully-qualified path, eg `std::string::String`.
	pub ident: &'static str,
	
	/// Additional data about this type, `T: HasReflectionInformation`.
	pub data: Data,
}

impl Type
{
	/// Primitive.
	#[inline(always)]
	pub const fn primitive<Primitive: 'static + Sized>(ident: &'static str, encoding: BtfTypeIntegerEncoding) -> Type
	{
		Type
		{
			type_id: TypeId::of::<Primitive>(),
			size: size_of::<Primitive>() as u32,
			ident,
			data: Data::Primitive(encoding),
		}
	}
	
	/// `*const T`.
	#[inline(always)]
	pub const fn const_pointer<T: 'static + Sized>(ident: &'static str) -> Type
	where *mut T: HasReflectionInformation
	{
		Type
		{
			type_id: TypeId::of::<*const T>(),
			size: size_of::<*const T>() as u32,
			ident,
			data: Data::ReferenceOrConstantPointer(&<*mut T>::Type),
		}
	}
	
	/// `&'static T`.
	#[inline(always)]
	pub const fn static_reference<T: 'static + Sized>(ident: &'static str) -> Type
	where &'static mut T: 'static + HasReflectionInformation
	{
		Type
		{
			type_id: TypeId::of::<&'static T>(),
			size: size_of::<&'static T>() as u32,
			ident,
			data: Data::ReferenceOrConstantPointer(&<&mut T>::Type),
		}
	}
	
	/// `*mut T`.
	#[inline(always)]
	pub const fn mutable_pointer<T: 'static + HasReflectionInformation>(ident: &'static str) -> Type
	{
		Type
		{
			type_id: TypeId::of::<*mut T>(),
			size: size_of::<*mut T>() as u32,
			ident,
			data: Data::MutableReferenceOrPointer(&T::Type),
		}
	}
	
	/// `&'static mut T`.
	#[inline(always)]
	pub const fn static_mutable_reference<T: 'static + HasReflectionInformation>(ident: &'static str) -> Type
	{
		Type
		{
			type_id: TypeId::of::<&'static mut T>(),
			size: size_of::<&'static mut T>() as u32,
			ident,
			data: Data::MutableReferenceOrPointer(&T::Type),
		}
	}
	
	/// `NonNull<T>`.
	#[inline(always)]
	pub const fn non_null<T: 'static + HasReflectionInformation>(ident: &'static str) -> Type
	{
		Type
		{
			type_id: TypeId::of::<NonNull<T>>(),
			size: size_of::<NonNull<T>>() as u32,
			ident,
			data: Data::MutableReferenceOrPointer(&T::Type),
		}
	}
	
	/// `Option<NonNull<T>>`.
	#[inline(always)]
	pub const fn option_non_null<T: 'static + HasReflectionInformation>(ident: &'static str) -> Type
	{
		Type
		{
			type_id: TypeId::of::<Option<NonNull<T>>>(),
			size: size_of::<Option<NonNull<T>>>() as u32,
			ident,
			data: Data::MutableReferenceOrPointer(&T::Type),
		}
	}
}
