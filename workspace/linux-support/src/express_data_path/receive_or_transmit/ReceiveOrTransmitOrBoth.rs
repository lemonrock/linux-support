// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Receive, transmit or both.
pub trait ReceiveOrTransmitOrBoth<Receive, Transmit>
{
	#[allow(missing_docs)]
	const IsReceiveOrBoth: bool;
	
	#[allow(missing_docs)]
	fn receive(&self) -> &Receive;
	
	#[allow(missing_docs)]
	fn transmit(&self) -> &Transmit;
	
	#[allow(missing_docs)]
	fn use_value(&self, use_receive: impl FnOnce(&Receive), use_transmit: impl FnOnce(&Transmit));
}
