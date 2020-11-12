// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


fn protocol_permutation_to_delimited_string<'a, Element: AsRef<str>, V>(delimiter: char, protocol_permutation: &Permutation<'a, Element, V>) -> String
{
	let mut string = String::with_capacity(MaximumServiceFieldSize);
	for &(element, _value) in protocol_permutation
	{
		string.push_str(&format!("{}{}", delimiter, element.as_ref()))
	}
	string
}
