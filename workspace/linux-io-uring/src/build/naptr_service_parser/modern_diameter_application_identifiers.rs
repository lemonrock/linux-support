// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Subset of <https://www.iana.org/assignments/s-naptr-parameters/s-naptr-parameters.xhtml#s-naptr-parameters-1> with the `aaa+ap` prefix removed.
fn modern_diameter_application_identifiers() -> HashSet<u32>
{
	hashset!
	{
		1,
		2,
		3,
		4,
		5,
		6,
		7,
		8,
		9,
		9,
		16777250,
		16777251,
		16777264,
		16777267,
		16777281,
		16777282,
		16777283,
		16777284,
		16777285,
		16777286,
		16777287,
		16777288,
		16777289,
		16777290,
		4294967295,
	}
}
