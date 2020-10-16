// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


macro_rules! validate_message_response_code
{
	($message_header: ident, $is_authoritative_answer: ident, $is_authenticated_data: ident) =>
	{
		{
			use self::Outcome::*;

			match $message_header.raw_message_response_code()
			{
				MessageResponseCode::NoError => Normal,

				MessageResponseCode::FormatError => return Err(MessageResponseCodeWasFormatError),

				MessageResponseCode::ServerFailure => if unlikely!(!$is_authenticated_data)
				{
					return Ok(DnsSecDataFailedAuthentication)
				}
				else
				{
					return Err(MessageResponseCodeWasServerFailure)
				},

				MessageResponseCode::NonExistentDomain => if unlikely!($is_authoritative_answer)
				{
					AuthoritativeServerReportsNoDomainButThisIsNotValidated
				}
				else
				{
					return Err(MessageResponseCodeWasNonExistentDomainForANonAuthoritativeServer)
				},

				MessageResponseCode::NotImplemented => return Err(MessageResponseCodeWasNotImplemented),

				MessageResponseCode::Refused => return Err(MessageResponseCodeWasRefused),

				// RFC 6672, Section 2.2 Final Paragraph allows this code to occur if DNAME substitution would produce a FQDN longer than 255 bytes.
				MessageResponseCode::NameExistsWhenItShouldNot => return Err(MessageResponseCodeShouldNotBeDynamicDnsAssociated(MessageResponseCode::NameExistsWhenItShouldNot)),

				MessageResponseCode::ResourceRecordSetExistsWhenItShouldNot => return Err(MessageResponseCodeShouldNotBeDynamicDnsAssociated(MessageResponseCode::ResourceRecordSetExistsWhenItShouldNot)),

				MessageResponseCode::ResourceRecordSetThatShouldExistDoesNot => return Err(MessageResponseCodeShouldNotBeDynamicDnsAssociated(MessageResponseCode::ResourceRecordSetThatShouldExistDoesNot)),

				MessageResponseCode::ServerNotAuthoritativeForZoneOrNotAuthorized => return Err(MessageResponseCodeShouldNotBeDynamicDnsAssociated(MessageResponseCode::ServerNotAuthoritativeForZoneOrNotAuthorized)),

				MessageResponseCode::NameNotContainedInZone => return Err(MessageResponseCodeShouldNotBeDynamicDnsAssociated(MessageResponseCode::NameNotContainedInZone)),

				MessageResponseCode::DnsStatefulOperationsTypeNotImplemented => return Err(MessageResponseCodeShouldNotBeDnsStatefulOperationsTypeNotImplemented),

				response_code @ 12 ..= 15 => return Err(MessageResponseCodeUnassigned(response_code)),

				_ => unreachable!(),
			}
		}
	}
}
