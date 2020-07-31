// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// MDIO support.
	pub(crate) struct ETH_MDIO_SUPPORTS: u8
	{
		/// Device supports clause 22 register access to PHY or peripherals using the interface defined in `linux/mii.h`.
		///
		/// This should not be set if there are known to be no such peripherals present or if the driver only emulates clause 22 registers for compatibility.
		ETH_MDIO_SUPPORTS_C22 = 1;
	
		/// Device supports clause 45 register access to PHY or peripherals using the interface defined in `linux/mii.h` and `linux/mdio.h`.
		///
		/// This should not be set if there are known to be no such peripherals present.
		ETH_MDIO_SUPPORTS_C45 = 2;
	}
}

