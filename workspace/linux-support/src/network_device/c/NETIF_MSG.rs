// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Extracted from ethtool source.
	#[derive(Serialize, Deserialize)]
	pub struct NETIF_MSG: u32
	{
		/// String set value is `drv`.
		const NETIF_MSG_DRV = 0x0001;
		
		/// String set value is `probe`.
		const NETIF_MSG_PROBE = 0x0002;
		
		/// String set value is `link`.
		const NETIF_MSG_LINK = 0x0004;
		
		/// String set value is `timer`.
		const NETIF_MSG_TIMER = 0x0008;
		
		/// String set value is `ifdown`.
		const NETIF_MSG_IFDOWN = 0x0010;
		
		/// String set value is `ifup`.
		const NETIF_MSG_IFUP = 0x0020;
		
		/// String set value is `rx_err`.
		const NETIF_MSG_RX_ERR = 0x0040;
		
		/// String set value is `tx_err`.
		const NETIF_MSG_TX_ERR = 0x0080;
		
		/// String set value is `tx_queued`.
		const NETIF_MSG_TX_QUEUED = 0x0100;
		
		/// String set value is `intr`.
		const NETIF_MSG_INTR = 0x0200;
		
		/// String set value is `tx_done`.
		const NETIF_MSG_TX_DONE = 0x0400;
		
		/// String set value is `rx_status`.
		const NETIF_MSG_RX_STATUS = 0x0800;
		
		/// String set value is `pktdata`.
		const NETIF_MSG_PKTDATA = 0x1000;
		
		/// String set value is `hw`.
		const NETIF_MSG_HW = 0x2000;
		
		/// String set value is `wol`.
		const NETIF_MSG_WOL = 0x4000;
	}
}
