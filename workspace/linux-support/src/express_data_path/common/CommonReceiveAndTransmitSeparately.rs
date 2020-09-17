// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[doc(hidden)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommonReceiveAndTransmitSeparately<RP: ReceivePoll>(CommonReceiveOnly<RP>, CommonTransmitOnly);

impl<RP: ReceivePoll> Supports for CommonReceiveAndTransmitSeparately<RP>
{
	const SupportsReceive: bool = true;
	
	const SupportsTransmit: bool = true;
}

impl<RP: ReceivePoll> ReceiveOrTransmitOrBoth for CommonReceiveAndTransmitSeparately<RP>
{
}

impl<RP: ReceivePoll> Receives<CommonReceiveOnly> for CommonReceiveAndTransmitSeparately<RP>
{
	#[inline(always)]
	fn receive(&self) -> &CommonReceiveOnly
	{
		self.0.receive()
	}
}

impl<RP: ReceivePoll> Transmits<CommonTransmitOnly> for CommonReceiveAndTransmitSeparately<RP>
{
	#[inline(always)]
	fn transmit(&self) -> &CommonTransmitOnly
	{
		self.1.transmit()
	}
}

impl<RP: ReceivePoll> CommonReceiveAndTransmitSeparately<RP>
{
	#[inline(always)]
	pub(crate) const fn new(common_receive_only: CommonReceiveOnly<RP>, common_transmit_only: CommonTransmitOnly) -> Self
	{
		Self(common_receive_only, common_transmit_only)
	}
}
