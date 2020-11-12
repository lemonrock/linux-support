// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// See RFC 6116.
///
/// From <https://www.iana.org/assignments/enum-services/enum-services.xhtml#enum-services-1>
///
/// An ENUM (E.164 number to URI mapping) service.
///
/// For a real, working example use `dig -t naptr 4.4.2.2.3.3.5.6.8.1.4.4.e164.arpa` for `+44 1865 332244`:-
///
/// ```bash
/// 4.4.2.2.3.3.5.6.8.1.4.4.e164.arpa. 86400 IN NAPTR 100 20 "u" "E2U+pstn:tel" "!^(.*)$!tel:\\1!" .
/// 4.4.2.2.3.3.5.6.8.1.4.4.e164.arpa. 86400 IN NAPTR 100 10 "u" "E2U+sip" "!^\\+441865332(.*)$!sip:\\1@nominet.org.uk!" .
/// ```
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Hash)]
#[derive(EnumCount, EnumIter, IntoStaticStr)]
#[derive(EnumDiscriminants)]
#[strum_discriminants(name(EnumServiceKind))]
#[strum_discriminants(derive(PartialOrd))]
#[strum_discriminants(derive(Ord))]
#[strum_discriminants(derive(Hash))]
pub enum EnumService
{
	/// This Enumservice indicates that the resource can be identified by the associated `acct` URI RFC 7565.
	///
	/// Class: Application-Based, Ancillary
	/// Type: `acct`
	/// Subtype: (none).
	/// URI Scheme: `acct`.
	///
	/// Specified in RFC 7566.
	acct,
	
	/// This Enumservice indicates that the resource can be addressed by the associated URI in order to send an email.
	///
	/// Class: Application-Based, Common
	/// Type: `email`
	/// Subtype: `mailto`.
	/// URI Scheme: `mailto`.
	///
	/// Specified in RFC 4355 and RFC 6118.
	email(EmailEnumServiceSubType),
	
	/// This Enumservice indicates that the resource identified by the associated URI is capable of receiving a message using an email protocol.
	/// EMS content is sent over SMTP using the format specified by TS 23.140 15 Section 8.4.4 and TS 26.140 16 Section 4, as an MMS message.
	/// Within such a message, EMS content is carried as either a text or application/octet-stream MIME sub-part (see TS 26.140 16, Section 4.1).
	/// For references see RFC 4355.
	///
	/// Class: Application-Based, Common
	/// Type: `ems`
	/// Subtype: `mailto`.
	/// URI Scheme: `mailto`.
	///
	/// Class: Application-Based, Common
	/// Type: `ems`
	/// Subtype: `tel`.
	/// URI Scheme: `tel`.
	/// Notes: Note that an indication of EMS can be taken as implying that the recipient is capable of receiving SMS messages at this address as well.
	///
	/// Specified in RFC 4355 and RFC 6118.
	ems(EmsEnumServiceSubType),

	/// This Enumservice indicates that the resource identified by the associated URI is capable of being contacted to provide a communication session during which facsimile documents can be sent.
	/// A client selecting this NAPTR will have support for generating and sending facsimile documents to the recipient using the PSTN session and transfer protocols specified in 12 and 13 in RFC 4355 - in short, they will have a fax program with a local or shared PSTN access over which they can send faxes.
	/// References are contained in RFC 4355.
	///
	/// Class: Application-Based, Subset
	/// Type: `fax`
	/// Subtype: `tel`.
	/// URI Scheme: `tel`.
	///
	/// Specified in RFC 4355 and RFC 6118.
	fax(FaxEnumServiceSubType),

	/// This Enumservice indicates that the resource identified by the associated URI is a file service from which a file or file listing can be retrieved.
	///
	/// Class: Application-Based, Subset
	/// Type: `ft`
	/// Subtype: `ftp`.
	/// URI Scheme: `ftp`.
	///
	/// Specified in RFC 4002 and RFC 6118.
	ft(FileServiceServiceSubType),

	/// See RFC 3762, Section 3.
	///
	/// Class: Protocol-Based
	/// Type: `h323`
	/// Subtype: (none).
	/// URI Scheme: `h323`.
	///
	/// Specified in RFC 3762 and RFC 6118.
	h323,

	/// The 'iax' Enumservice is used to map E.164 numbers to IAX URIs. Such URIs identify resources capable of being contacted to provide a communication session using the IAX protocol RFC 5456.
	/// A client selecting this NAPTR needs to be able to support communication utilizing the IAX protocol.
	///
	/// Class: Protocol-Based
	/// Type: `iax`
	/// Subtype: (none).
	/// URI Scheme: `iax`.
	///
	/// Specified in RFC 6315.
	iax,

	/// This Enumservice indicates that the resource identified can be addressed by the associated URI in order to access a user's calendar (for example free/busy status) using the CalDAV 7 protocol for Internet calendaring.
	/// References are contained in RFC 5333.
	///
	/// Class: Application-Based, Common
	/// Type: `ical-access`
	/// Subtype: `http`.
	/// URI Scheme: `http`.
	///
	/// Class: Application-Based, Common
	/// Type: `ical-access`
	/// Subtype: `https`.
	/// URI Scheme: `https`.
	///
	/// Specified in RFC 5333 and RFC 6118.
	ical_access(WebEnumServiceSubType),

	/// This Enumservice indicates that the resource identified can be addressed by the associated URI used for scheduling using Internet calendaring via Internet mail with the iMIP 6 protocol.
	/// References are contained in RFC 5333.
	///
	/// Class: Application-Based, Common
	/// Type: `ical-sched`
	/// Subtype: `mailto`.
	/// URI Scheme: `mailto`.
	///
	/// Specified in RFC 5333 and RFC 6118.
	ical_sched(EmailEnumServiceSubType),

	/// See RFC 4143, Section 1.
	///
	/// Class: Application-Based, Subset
	/// Type: `ifax`
	/// Subtype: `mailto`.
	/// URI Scheme: `mailto`.
	///
	/// The URI Scheme is `mailto` because facsimile is a profile of standard Internet mail and uses standard Internet mail addressing.
	///
	/// Specified in RFC 4143 and RFC 6118.
	ifax(EmailEnumServiceSubType),
	
	/// This Enumservice indicates that the resource identified is an `im:` URI.
	/// The `im:` URI scheme does not identify any particular protocol that will be used to handle instant messaging receipt or delivery, rather the mechanism in RFC 3861 4 is used to discover whether an IM protocol supported by the party querying ENUM is also supported by the target resource.
	/// References are contained in RFC 5028.
	///
	/// Class: Application-Based, Common
	/// Type: `im`
	/// Subtype: (none).
	/// URI Scheme: `im`.
	///
	/// Specified in RFC 5028 and RFC 6118.
	im,

	/// This Enumservice indicates that the resource identified by the associated URI is capable of receiving a message using an email protocol.
	/// MMS messages are sent over SMTP using the format specified by TS 23.140 15 Section 8.4.4 and TS 26.140 16 Section 4. Within and between MMS Environments (MMSE, network infrastructures that support the MultiMedia Service), other pieces of state data (for example, charging-significant information) are exchanged between MMS Relay Servers.
	/// Thus, although these servers use SMTP as the bearer for their application exchanges, they map their internal state to specialized header fields carried in the SMTP message exchanges.
	/// The header fields used in such MMSE are described in detail in 17.
	/// For references see RFC 4355.
	///
	/// Class: Application-Based, Common
	/// Type: `mms`
	/// Subtype: `mailto`.
	/// URI Scheme: `mailto`.
	///
	/// The MMS Architecture describes an interface between the MMSE and legacy messaging systems (labelled as MM3) which accepts standard SMTP messages.
	/// Thus although the MMS Relay Server that supports this interface appears as a standard SMTP server from the perspective of an Internet-based mail server, it acts as a gateway and translator, adding the internal state data that is used within and between the MMS Environments.
	/// This mechanism is described in 17, which also includes references to the specifications agreed by those bodies responsible for the design of the MMS.
	/// References are contained in RFC 4355.
	///
	/// This Enumservice indicates that the resource identified by the associated URI is capable of receiving a message using the Multimedia Messaging Service (MMS) 15. For references see RFC 4355.
	///
	/// Class: Application-Based, Common
	/// Type: `mms`
	/// Subtype: `tel`.
	/// URI Scheme: `tel`.
	///
	/// Note that MMS can be used as an alternative to deliver an SMS RP-DATA RPDU if, for example, the SMS bearer is not supported.
	/// If an entry includes this Enumservice, then in effect this can be taken as implying that the recipient is capable of receiving EMS or SMS messages at this address.
	/// Such choices on the end system de do have two small caveats; whilst in practice all terminals supporting MMS today support SMS as well, it might not necessarily be the case in the future, and there may be tariff differences in using the MMS rather than using the SMS or EMS.
	///
	/// Specified in RFC 4355 and RFC 6118.
	mms(EmsEnumServiceSubType),

	/// See RFC 3953, Section 4.
	///
	/// Class: Application-Based, Common
	/// Type: `pres`
	/// Subtype: (none).
	/// URI Scheme: `pres`.
	///
	/// Specified in RFC 3953 and RFC 6118.
	pres,

	/// These Enumservices indicate that the resource identified can be addressed by the associated URI in order to initiate a telecommunication session, which may include two-way voice or other communications, to the PSTN.
	///
	/// Class: Application-Based, Common
	/// Type: `pstn`
	/// Subtype: `sip`.
	/// URI Scheme: `sip`.
	///
	/// Class: Application-Based, Ancillary
	/// Type: `pstn`
	/// Subtype: `tel`.
	/// URI Scheme: `tel`.
	///
	/// A Number Portability Dip Indicator (npdi) should be used in practice (see RFC 4769, Section 4).
	///
	/// Specified in RFC 4769 and RFC 6118.
	pstn(PstnEnumServiceSubType),

	/// See RFC 3764, Section 4.
	///
	/// Class: Protocol-Based
	/// Type: `sip`
	/// Subtype: (none).
	/// URI Schemes: `sip` and `sips`.
	///
	/// Specified in RFC 3764 and RFC 6118.
	sip,

	/// This Enumservice indicates that the resource identified by the associated URI is capable of receiving a message using an email protocol.
	/// SMS content is sent over SMTP using the format specified by TS 23.140 15 Section 8.4.4 and TS 26.140 16 Section 4, as an MMS message.
	/// Within such a message, SMS content is carried as either a text or application/octet-stream MIME sub-part (see TS 26.140 16, Section 4.1)
	/// For references see RFC 4355.
	///
	/// Class: Application-Based, Common
	/// Type: `sms`
	/// Subtype: `mailto`.
	/// URI Scheme: `mailto`.
	///
	/// This Enumservice indicates that the resource identified by the associated URI is capable of receiving a message using the Short Message Service (SMS) 14 in RFC 4355.
	///
	/// Class: Application-Based, Common
	/// Type: `sms`
	/// Subtype: `tel`.
	/// URI Scheme: `tel`.
	///
	/// Specified in RFC 4355 and RFC 6118.
	sms(EmsEnumServiceSubType),

	/// This Enumservice indicates that the resource identified by the associated URI scheme is capable of being a source of information.
	/// Note that the kind of information retrieved can be manifold.
	/// Usually, contacting a resource by an 'http:' 11 URI provides a document.
	/// This document can contain references that will trigger the download of many different kinds of information, such as text, audio, video, executable code, or even video message files.
	/// Thus, one cannot be more specific about the kind of information expected when contacting the resource.
	/// References are contained in RFC 5278.
	///
	/// Class: Application-Based, Common
	/// Type: `unifmsg`
	/// Subtype: `http`.
	/// URI Scheme: `http`.
	///
	/// This Enumservice indicates that the resource identified by the associated URI scheme is capable of being a source of information, which can be contacted using TLS or the Secure Socket Layer protocol.
	/// Note that the kind of information retrieved can be manifold.
	/// Usually, contacting a resource by an 'https:' 12 URI provides a document.
	/// This document can contain references that will trigger the download of many different kinds of information, such as text, audio, video, executable code, or even video message files.
	/// Thus, one cannot be more specific about the kind of information expected when contacting the resource.
	/// References are contained in RFC 5278.
	///
	/// Class: Application-Based, Common
	/// Type: `unifmsg`
	/// Subtype: `https`.
	/// URI Scheme: `https`.
	///
	/// This Enumservice indicates that the resource identified can be addressed by the associated URI scheme in order to initiate a unified communication session to a unified messaging system.
	///
	/// Class: Application-Based, Common
	/// Type: `unifmsg`
	/// Subtype: `sip`.
	/// URI Scheme: `sip`.
	///
	/// This Enumservice indicates that the resource identified can be addressed by the associated URI scheme in order to initiate a unified communication session to a unified messaging system.
	///
	/// Class: Application-Based, Common
	/// Type: `unifmsg`
	/// Subtype: `sips`.
	/// URI Scheme: `sips`.
	///
	/// Implementers should review a non-exclusive list of examples in Section 7 of RFC 5278.
	///
	/// Specified in RFC 5278 and RFC 6118.
	unifmsg(UnifiedMessagingEnumServiceSubType),

	/// This Enumservice indicates that the resource identified is a plain vCard, according to RFC 2426, which may be accessed using HTTP / HTTPS.
	/// Clients fetching the vCard from the resource indicated should expect access to be restricted.
	/// Additionally, the comprehension of the data provided may vary depending on the client's identity.
	/// References are contained in RFC 4969.
	///
	/// Class: Data Type-Based
	/// Type: `vcard`
	/// Subtype: (none).
	/// URI Scheme: `http, https`.
	///
	/// Specified in RFC 4969 and RFC 6118.
	vcard(WebEnumServiceSubType),

	/// This Enumservice indicates that the resource identified by the associated URI scheme is capable of being a source of information.
	/// Note that the kind of information retrieved can be manifold.
	/// Usually, contacting a resource by an 'http:' 11 URI provides a document.
	/// This document can contain references that will trigger the download of many different kinds of information, such as text, audio, video, executable code, or even video message files.
	/// Thus, one cannot be more specific about the kind of information expected when contacting the resource.
	/// References are contained in RFC 5278.
	///
	/// Class: Application-Based, Common
	/// Type: `videomsg`
	/// Subtype: `http`.
	/// URI Scheme: `http`.
	///
	/// This Enumservice indicates that the resource identified by the associated URI scheme is capable of being a source of information, which can be contacted using TLS or the Secure Socket Layer protocol.
	/// Note that the kind of information retrieved can be manifold.
	/// Usually, contacting a resource by an 'https:' 12 URI provides a document.
	/// This document can contain references that will trigger the download of many different kinds of information, such as text, audio, video, executable code, or even video message files.
	/// Thus, one cannot be more specific about the kind of information expected when contacting the resource.
	/// References are contained in RFC 5278.
	///
	/// Class: Application-Based, Common
	/// Type: `videomsg`
	/// Subtype: `https`.
	/// URI Scheme: `https`.
	///
	/// This Enumservice indicates that the resource identified can be addressed by the associated URI scheme in order to initiate a video communication session to a video messaging system.
	///
	/// Class: Application-Based, Common
	/// Type: `videomsg`
	/// Subtype: `sip`.
	/// URI Scheme: `sip`.
	///
	/// This Enumservice indicates that the resource identified can be addressed by the associated URI scheme in order to initiate a video communication session to a video messaging system.
	///
	/// Class: Application-Based, Common
	/// Type: `videomsg`
	/// Subtype: `sips`.
	/// URI Scheme: `sips`.
	///
	/// Implementers should review a non-exclusive list of examples in Section 7 of RFC 5278.
	///
	/// Specified in RFC 5278 and RFC 6118.
	videomsg(UnifiedMessagingEnumServiceSubType),

	/// The kind of communication indicated by this Enumservice is Interactive Voice.
	/// From a protocol perspective, this communication is expected to involve bidirectional media streams carrying audio data.
	/// A client may imply that the person controlling population of a NAPTR holding this Enumservice indicates their capability to engage in an interactive voice session when contacted using the URI generated by this NAPTR.
	///
	/// Class: Application-Based, Common
	/// Type: `voice`
	/// Subtype: `tel`.
	/// URI Scheme: `tel`.
	///
	/// This Enumservice indicates that the person responsible for the NAPTR is accessible via the PSTN (Public Switched Telephone Network) or PLMN (Public Land Mobile Network) using the value of the generated URI.
	/// The kind of subsystem required to initiate a Voice Enumservice with this Subtype is a Dialler.
	/// This is a subsystem that either provides a local connection to the PSTN or PLMN, or that provides an indirect connection to those networks.
	/// The subsystem will use the telephone number held in the generated URI to place a voice call.
	/// The voice call is placed to a network that uses E.164 numbers to route calls to an appropriate destination.
	/// Note that the PSTN/PLMN connection may be indirect.
	/// The end user receiving this NAPTR may have a relationship with a Communications Service Provider that accepts call initiation requests from that subsystem using an IP-based protocol such as SIP or H.323, and places the call to the PSTN using a remote gateway service.
	/// In this case the Provider may either accept requests using tel: URIs or has a defined mechanism to convert tel: URI values into a protocol-native form.
	/// The tel: URI value SHOULD be fully qualified (using the global phone number form of RFC 3966 10).
	/// A local phone number as defined in that document SHOULD NOT be used unless the controller of the zone in which the NAPTR appears is sure that it can be distinguished unambiguously by all clients that can access the resource record and that a call from their network access points can be routed to that destination. References are contained in RFC 4415.
	///
	/// Specified in RFC 4415 and RFC 6118.
	voice(VoiceEnumServiceSubType),

	/// This Enumservice indicates that the resource identified by the associated URI scheme is capable of being a source of information.
	/// Note that the kind of information retrieved can be manifold.
	/// Usually, contacting a resource by an 'http:' 11 URI provides a document.
	/// This document can contain references that will trigger the download of many different kinds of information, such as text, audio, video, executable code, or even voice message files.
	/// Thus, one cannot be more specific about the kind of information expected when contacting the resource. References are contained in RFC 5278.
	///
	/// Class: Application-Based, Common
	/// Type: `voicemsg`
	/// Subtype: `http`.
	/// URI Scheme: `http`.
	///
	/// Class: Application-Based, Common
	/// Type: `voicemsg`
	/// Subtype: `https`.
	/// URI Scheme: `https`.
	///
	/// Class: Application-Based, Common
	/// Type: `voicemsg`
	/// Subtype: `sip`.
	/// URI Scheme: `sip`.
	///
	/// Class: Application-Based, Common
	/// Type: `voicemsg`
	/// Subtype: `sips`.
	/// URI Scheme: `sips`.
	///
	/// Class: Application-Based, Common
	/// Type: `voicemsg`
	/// Subtype: `tel`.
	/// URI Scheme: `tel`.
	///
	/// Implementers should review a non-exclusive list of examples in Section 7 of RFC 5278.
	///
	/// Specified in RFC 5278 and RFC 6118.
	voicemsg(VoiceMessageEnumServiceSubType),
	
	/// See RFC 4238, Section 3.2 - 3.3.
	///
	/// Class: Data Type-Based
	/// Type: `vpim`
	/// Subtype: `ldap`.
	/// URI Scheme: `ldap`.
	///
	/// See RFC 4238, Section 4.2 - 4.4.
	///
	/// Class: Data Type-Based
	/// Type: `vpim`
	/// Subtype: `mailto`.
	/// URI Scheme: `mailto`.
	///
	/// Error Conditions:
	/// * E.164 number not in the numbering plan E.164 number in the numbering plan, but no URIs exist for that number.
	/// * E2U+vpim:mailto Service unavailable of email addresses where only the telephone number was previously known.
	///
	/// Specified in RFC 4238 and RFC 6118.
	vpim,

	/// This Enumservice indicates that the resource identified by the associated URI is capable of being a source of information.
	/// It has to be noted that the kind of information retrieved can be manifold.
	/// Usually, contacting a resource by an http: URI provides a document.
	/// This document can contain references that will trigger download of many different kinds of information, like audio or video or executable code.
	/// Thus, one can not be more specific about the kind of information that can be expected when contacting the resource.
	///
	/// Class: Application-Based, Common
	/// Type: `web`
	/// Subtype: `http`.
	/// URI Scheme: `http`.
	///
	/// This Enumservice indicates that the resource identified by the associated URI is capable of being a source of information, which can be contacted by using TLS or Secure Socket Layer protocol.
	/// It has to be noted that the kind of information retrieved can be manifold.
	/// Usually, contacting a resource by an https: URI provides a document.
	/// This document can contain all different kind of information, like audio or video or executable code.
	/// Thus, one can not be more specific what information to expect when contacting the resource.
	///
	/// Class: Application-Based, Common
	/// Type: `web`
	/// Subtype: `https`.
	/// URI Scheme: `https`.
	///
	/// Specified in RFC 4002 and RFC 6118.
	web(WebEnumServiceSubType),

	/// This Enumservice indicates that the resource identified is an XMPP entity.
	///
	/// Class: Protocol-Based
	/// Type: `xmpp`
	/// Subtype: (none).
	/// URI Scheme: `xmpp`.
	///
	/// Specified in RFC 4979 and RFC 6118.
	xmpp,
}

impl ToEnumUriScheme for EnumService
{
	#[inline(always)]
	fn to_uri_scheme(self) -> EnumUriScheme
	{
		use self::EnumService::*;
		
		match self
		{
			acct => EnumUriScheme::acct,
			
			email(sub_type) => sub_type.to_uri_scheme(),
			
			ems(sub_type) => sub_type.to_uri_scheme(),
			
			fax(sub_type) => sub_type.to_uri_scheme(),
			
			ft(sub_type) => sub_type.to_uri_scheme(),
			
			h323 => EnumUriScheme::h323,
			
			iax => EnumUriScheme::iax,
			
			ical_access(sub_type) => sub_type.to_uri_scheme(),
			
			ical_sched(sub_type) => sub_type.to_uri_scheme(),
			
			ifax(sub_type) => sub_type.to_uri_scheme(),
			
			im => EnumUriScheme::im,
			
			mms(sub_type) => sub_type.to_uri_scheme(),
			
			pres => EnumUriScheme::pres,
			
			pstn(sub_type) => sub_type.to_uri_scheme(),
			
			sip => EnumUriScheme::sip,
			
			sms(sub_type) => sub_type.to_uri_scheme(),
			
			unifmsg(sub_type) => sub_type.to_uri_scheme(),
			
			vcard(sub_type) => sub_type.to_uri_scheme(),
			
			videomsg(sub_type) => sub_type.to_uri_scheme(),
			
			voice(sub_type) => sub_type.to_uri_scheme(),
			
			voicemsg(sub_type) => sub_type.to_uri_scheme(),
			
			web(sub_type) => sub_type.to_uri_scheme(),
			
			xmpp => EnumUriScheme::xmpp,
		}
	}
}
