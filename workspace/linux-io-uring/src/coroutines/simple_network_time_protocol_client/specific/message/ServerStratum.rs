// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum ServerStratum
{
	Primary(ReferenceIdentifier),

	/// `0` to `15` inclusive.
	Secondary(u8),
}

impl ServerStratum
{
	fn parse_server_reply(stratum: Stratum, server_reply: &NetworkTimeProtocolMessage, sent_poll: UnsignedExponentOfTwo8) -> Result<Self, NetworkTimeProtocolMessageServerReplyParseError>
	{
		use self::NetworkTimeProtocolMessageServerReplyParseError::*;
		use self::ServerStratum::*;
		use self::Stratum::*;
		
		match stratum
		{
			UnspecifiedOrInvalid => Err(KissOfDeath(unsafe { server_reply.reference_identifier.kiss_of_death_code })),
			
			PrimaryServer =>
			{
				let reference_identifier = unsafe { server_reply.reference_identifier.reference_identifier };
				
				let root_delay_received = server_reply.root_delay;
				if unlikely!(!root_delay_received.is_zero())
				{
					return Err(PrimaryServersMustHaveARootDelayOfZero { root_delay_received })
				}
				
				let root_dispersion_received = server_reply.root_dispersion;
				if unlikely!(!root_dispersion_received.is_zero())
				{
					return Err(PrimaryServersMustHaveARootDispersionOfZero { root_dispersion_received })
				}
				
				let poll_received = server_reply.poll;
				if unlikely!(poll_received != sent_poll)
				{
					return Err(PrimaryServersMustHavePollEqualToThatSent { poll_received, poll_sent })
				}
				
				Ok(Primary(reference_identifier))
			}
			
			secondary_server @ _ =>
			{
				let reference_identifier_unspecified = (unsafe { &server_reply.reference_identifier.unspecified });
				if unlikely!(sent_from.is_reference_identifier_valid(reference_identifier_unspecified))
				{
					return Err(InvalidSecondaryServerReferenceIdentifier { reference_identifier_unspecified: *reference_identifier_unspecified })
				}
				
				// "A truly paranoid client can check that the Root Delay and Root Dispersion fields are each greater than or equal to 0 and lesst han infinity, where infinity is currently a cozy number like one second.
				// This check avoids using a server whose synchronization source has expired for a very long time".
				let root_delay_received: Unsigned1616FixedPoint = server_reply.root_delay.into();
				const RootDelayInfinity: Unsigned1616FixedPoint = Unsigned1616FixedPoint::One;
				if root_delay_received > RootDelayInfinity
				{
					return Err(SecondaryServersMustHaveARootDelayOfLessThanOne { root_delay_received })
				}
				let root_dispersion_received: Signed1616FixedPoint = server_reply.root_dispersion_received.into();
				const RootDispersionInfinity: Signed1616FixedPoint = Signed1616FixedPoint::One;
				if root_dispersion_received < Signed1616FixedPoint::Zero || root_dispersion_received > RootDispersionInfinity
				{
					return Err(SecondaryServersMustHaveARootDispersionOfBetweenZeroAndLessThanOne { root_dispersion_received })
				}
				
				let secondary_server_index = (secondary_server as u8 - 2);
				Ok(Secondary(secondary_server_index))
			}
		}
	}
}
