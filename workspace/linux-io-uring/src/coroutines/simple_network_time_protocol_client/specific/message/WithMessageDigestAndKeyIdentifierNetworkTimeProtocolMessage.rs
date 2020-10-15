// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// 'Authenticator'.
#[repr(C)]
pub(crate) struct WithDigestAndKeyIdentifierNetworkTimeProtocolMessage
{
	packet: NetworkTimeProtocolMessage,
	
	key_identifier: KeyIdentifier,
	
	message_digest: MessageDigest,
}

impl Message for WithDigestAndKeyIdentifierNetworkTimeProtocolMessage
{
	const PacketSize: usize = size_of::<Self>();
	
	#[inline(always)]
	fn serialize_request_from_client_to_server(mut self) -> ([u8; Self::PacketSize], UnsignedTimestampFormat)
	{
		let sent_transmit_timestamp = UnsignedTimestampFormat::now();
		self.packet.transmit_timestamp = sent_transmit_timestamp;
		
		(unsafe { transmute(self) }, sent_transmit_timestamp)
	}
}
