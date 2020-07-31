// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[doc(hidden)]
#[derive(Copy, Clone)]
#[repr(C)]
pub union TunableIdentifier
{
	normal: tunable_id,
	phy: phy_tunable_id,
}

impl TunableIdentifier
{
	const fn normal(normal: tunable_id) -> Self
	{
		Self
		{
			normal,
		}
	}
	
	const fn phy(phy: phy_tunable_id) -> Self
	{
		Self
		{
			phy,
		}
	}
}
