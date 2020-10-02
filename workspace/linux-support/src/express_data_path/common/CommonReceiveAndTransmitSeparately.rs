// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[doc(hidden)]
#[derive(Debug)]
pub struct CommonReceiveAndTransmitSeparately<RP: ReceivePoll, TS: TransmitSend>(CommonReceiveOnly<RP>, CommonTransmitOnly<TS>);

impl<RP: ReceivePoll, TS: TransmitSend> Supports for CommonReceiveAndTransmitSeparately<RP, TS>
{
	const SupportsReceive: bool = true;
	
	const SupportsTransmit: bool = true;
}

impl<RP: ReceivePoll, TS: TransmitSend> ReceiveOrTransmitOrBoth for CommonReceiveAndTransmitSeparately<RP, TS>
{
	type RP = RP;
	
	type TS = TS;
}

impl<RP: ReceivePoll, TS: TransmitSend> Receives<CommonReceiveOnly<RP>> for CommonReceiveAndTransmitSeparately<RP, TS>
{
	#[inline(always)]
	fn receive(&self) -> &CommonReceiveOnly<RP>
	{
		self.0.receive()
	}
}

impl<RP: ReceivePoll, TS: TransmitSend> Transmits<CommonTransmitOnly<TS>> for CommonReceiveAndTransmitSeparately<RP, TS>
{
	#[inline(always)]
	fn transmit(&self) -> &CommonTransmitOnly<TS>
	{
		self.1.transmit()
	}
}

impl<RP: ReceivePoll, TS: TransmitSend> CommonReceiveAndTransmitSeparately<RP, TS>
{
	#[inline(always)]
	pub(crate) const fn new(common_receive_only: CommonReceiveOnly<RP>, common_transmit_only: CommonTransmitOnly<TS>) -> Self
	{
		Self(common_receive_only, common_transmit_only)
	}
}
