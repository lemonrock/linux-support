// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A set of fields associated with a type or `enum` variant.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EnumVariantFields
{
	/// A set of named fields.
	Named(&'static [NamedField]),
	
	/// A set of index-addressed fields.
	Unnamed(&'static [UnnamedField]),
	
	/// A unit enum variant without a value or does not fit into an `i32`.
	UnitValuelessOrDoesNotFitInI32,
	
	/// A unit enum variant with a value that fits into an `i32`; used for C bindings.
	UnitValued(i32),
}
