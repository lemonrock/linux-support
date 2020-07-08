// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Data associated with type information.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Data
{
	/// The associated type is a primitive type.
	Primitive(BtfTypeEncoding),
	
	/// `&T` or `*const T`.
	///
	/// The `&'static Type` will have a data of `MutableReferenceOrPointer`, eg a `*const T` is defined as a constant version of `*mut T`.
	ReferenceOrConstantPointer(&'static Type),
	
	/// `&mut T`, `*mut T`, `NonNull<T>` and `Option<NonNull<T>>`.
	MutableReferenceOrPointer(&'static Type),
	
	/// The associated type is an array.
	Array(&'static Type, usize),
	
	/// The associated type is a `struct`.
	Struct(StructFields),
	
	/// The associated type is an `enum`.
	///
	/// Has variants.
	Enum(&'static [Variant]),
	
	/// The associated type is an `union`.
	///
	/// Has named fields.
	Union(&'static [NamedField]),
}
