// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


macro_rules! programming_interface
{
	($enum_variant: ident, $programming_interface: ident, $programming_interface_type: ident) =>
	{
		{
			if let Some(programming_interface) = $programming_interface_type::parse($programming_interface)
			{
				Some($enum_variant(programming_interface))
			}
			else
			{
				None
			}
		}
	}
}
