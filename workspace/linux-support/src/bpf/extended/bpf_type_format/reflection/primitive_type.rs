// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Primitive type.
#[macro_export]
macro_rules! primitive_type
{
	($type_: ty, $encoding: ident) =>
	{
		impl HasReflectionInformation for $type_
		{
			const Type: Type = Type::primitive::<$type_>(stringify!($type_), BtfTypeEncoding::$encoding);
		}
		
		pointer_type!($type_)
	}
}

primitive_type!(u8, Unsigned);
primitive_type!(u16, Unsigned);
primitive_type!(u32, Unsigned);
primitive_type!(u64, Unsigned);
primitive_type!(u128, Unsigned);
primitive_type!(usize, Unsigned);
primitive_type!(i8, Signed);
primitive_type!(i16, Signed);
primitive_type!(i32, Signed);
primitive_type!(i64, Signed);
primitive_type!(i128, Signed);
primitive_type!(isize, Signed);
primitive_type!(f32, Signed);
primitive_type!(f64, Signed);
primitive_type!(bool, Boolean);
primitive_type!(char, Char);
