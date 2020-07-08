// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Pointer types.
#[macro_export]
macro_rules! pointer_type
{
	($type_: ty) =>
	{
		impl HasReflectionInformation for *const $type_
		{
			const Type: Type = Type::const_pointer::<$type_>(stringify!("*const ", $type_));
		}
		
		impl HasReflectionInformation for &'static $type_
		{
			const Type: Type = Type::static_reference::<$type_>(stringify!("&'static ", $type_));
		}
		
		impl HasReflectionInformation for *mut $type_
		{
			const Type: Type = Type::mutable_pointer::<$type_>(stringify!("*mut ", $type_));
		}
		
		impl HasReflectionInformation for &'static mut $type_
		{
			const Type: Type = Type::static_mutable_reference::<$type_>(stringify!("&'static mut ", $type_));
		}
		
		impl HasReflectionInformation for NonNull<$type_>
		{
			const Type: Type = Type::non_null::<$type_>(stringify!("NonNull<", $type_, ">"));
		}
		
		impl HasReflectionInformation for Option<NonNull<$type_>>
		{
			const Type: Type = Type::option_non_null::<$type_>(stringify!("Option<NonNull<", $type_, ">>"));
		}
	}
}
