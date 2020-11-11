// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


fn application_protocol_permutation_to_colon_delimited_string(application_protocol_permutation: &Permutation<&'static str>) -> String
{
	let mut string = String::with_capacity(256);
	for element in application_protocol_permutation
	{
		string.push_str(&format!(":{}", *element))
	}
	string
}
