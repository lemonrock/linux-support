// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


fn subtype_spec<'a>(permutation: &Permutation<'a, NaptrSubType, EnumServiceSubTypeMember>) -> SubTypeSpec
{
	let mut subtype_spec = String::with_capacity(MaximumServiceFieldSize);
	for &(naptr_sub_type, _) in permutation
	{
		subtype_spec.push(':');
		subtype_spec.push_str(*naptr_sub_type);
	}
	subtype_spec
}
