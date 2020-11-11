// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// See RFC 6408.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum DiameterApplicationIdentifier
{
	/// Unofficial, represents the `aaa` application service tag without an application identifier (`+ap`).
	Unspecified
	{
		/// Was either `AAA+D2T` or `AAA+D2S`.
		legacy: bool,
	},
	
	/// `NASREQ`.
	///
	/// Defined by RFC 6408, Section 7.1.
	///
	/// See RFC 3588.
	NASREQ = 1,
	
	/// Mobile Internet Protocol version 4.
	///
	/// Defined by RFC 6408, Section 7.1.
	///
	/// See RFC 4004.
	MobileInternetProtocolVersion4 = 2,
	
	/// Base Accounting.
	///
	/// Defined by RFC 6408, Section 7.1.
	///
	/// See RFC 3588.
	BaseAccounting = 3,
	
	/// Credit Control.
	///
	/// Defined by RFC 6408, Section 7.1.
	///
	/// See RFC 4006.
	CreditControl = 4,
	
	/// Extensible Authentication Protocol (EAP).
	///
	/// Defined by RFC 6408, Section 7.1.
	///
	/// See RFC 4072.
	ExtensibleAuthenticationProtocol = 5,
	
	/// Session Initiation Protocol (SIP).
	///
	/// Defined by RFC 6408, Section 7.1.
	///
	/// See RFC 4740.
	SessionInitiationProtocol = 6,
	
	/// Mobile Internet Protocol version 6 with Internet Key Exchange version 2 (IKEv2) and Extensible Authentication Protocol (EAP).
	///
	/// Defined by RFC 6408, Section 7.1.
	///
	/// See RFC 5778.
	MobileInternetProtocolVersion6WithInternetKeyExchangeVersion4AndExtensibleAuthenticationProtocol = 7,
	
	/// Mobile Internet Protocol version 6 Authentication Protocol.
	///
	/// Defined by RFC 6408, Section 7.1.
	///
	/// See RFC 5778.
	MobileInternetProtocolVersion6AuthenticationProtocol = 8,
	
	/// Quality-of-Service (QoS).
	///
	/// Defined by RFC 6408, Section 7.1.
	///
	/// See RFC 5866.
	QualityOfService = 9,
	
	/// 3ʳᵈ Generation Partnership Project (3GPP) STa.
	///
	/// Defined by RFC 6408, Section 7.2.
	///
	/// See [TS 29.273](http://www.3gpp.org/ftp/Specs/html-info/29273.htm).
	_3rdGenerationPartnershipProjectSTa = 16777250,
	
	/// 3ʳᵈ Generation Partnership Project (3GPP) S6a.
	///
	/// Defined by RFC 6408, Section 7.2.
	///
	/// See [TS 29.272](http://www.3gpp.org/ftp/Specs/html-info/29272.htm>).
	_3rdGenerationPartnershipProjectS6a = 16777251,
	
	/// 3ʳᵈ Generation Partnership Project (3GPP) SWm.
	///
	/// Defined by RFC 6408, Section 7.2.
	///
	/// See [TS 29.273](http://www.3gpp.org/ftp/Specs/html-info/29273.htm>).
	_3rdGenerationPartnershipProjectSWm = 16777264,
	
	/// 3ʳᵈ Generation Partnership Project (3GPP) S9.
	///
	/// Defined by RFC 6408, Section 7.2.
	///
	/// See [TS 29.215](http://www.3gpp.org/ftp/Specs/html-info/29215.htm>).
	_3rdGenerationPartnershipProjectS9 = 16777267,
	
	/// Worldwide Interoperability for Microwave Access (WiMAX) Forum Network Access Authentication and Authorization Diameter Application (WNAAADA).
	///
	/// Defined by RFC 6408, Section 7.3.
	///
	/// See [WMF-T33-001-R015v02 - WiMAX Forum® Network Architecture - Detailed Protocols and Procedures, Base Specification - Release 1.5](http://www.wimaxforum.org/resources/documents/technical/T33).
	WNAAADA = 16777281,
	
	/// Worldwide Interoperability for Microwave Access (WiMAX) Forum Network Accounting Diameter Application (WNADA).
	///
	/// Defined by RFC 6408, Section 7.3.
	///
	/// See [WMF-T33-001-R015v02 - WiMAX Forum® Network Architecture - Detailed Protocols and Procedures, Base Specification - Release 1.5](http://www.wimaxforum.org/resources/documents/technical/T33).
	WNADA = 16777282,
	
	/// Worldwide Interoperability for Microwave Access (WiMAX) Forum MIP4 Diameter Application (WM4DA).
	///
	/// Defined by RFC 6408, Section 7.3.
	///
	/// See [WMF-T33-001-R015v02 - WiMAX Forum® Network Architecture - Detailed Protocols and Procedures, Base Specification - Release 1.5](http://www.wimaxforum.org/resources/documents/technical/T33).
	WM4DA = 16777283,
	
	/// WiMAX Forum MIP6 Diameter Application (WM6DA).
	///
	/// Defined by RFC 6408, Section 7.3.
	///
	/// See [WMF-T33-001-R015v02 - WiMAX Forum® Network Architecture - Detailed Protocols and Procedures, Base Specification - Release 1.5](http://www.wimaxforum.org/resources/documents/technical/T33).
	WM6DA = 16777284,
	
	/// Worldwide Interoperability for Microwave Access (WiMAX) Forum DHCP Diameter Application (WDDA).
	///
	/// Defined by RFC 6408, Section 7.3.
	///
	/// See [WMF-T33-001-R015v02 - WiMAX Forum® Network Architecture - Detailed Protocols and Procedures, Base Specification - Release 1.5](http://www.wimaxforum.org/resources/documents/technical/T33).
	WDDA = 16777285,
	
	/// Worldwide Interoperability for Microwave Access (WiMAX) Forum Location Authentication Authorization Diameter Application (WLAADA).
	///
	/// Defined by RFC 6408, Section 7.3.
	///
	/// See [WMF-T33-110-R015v01 - WiMAX Forum® Network Architecture - Protocols and Procedures for Location Based Services - Release 1.5](http://www.wimaxforum.org/resources/documents/technical/T33).
	WLAADA = 16777286,
	
	/// Worldwide Interoperability for Microwave Access (WiMAX) Forum Policy and Charging Control R3 Policies Diameter Application (WiMAX PCC-R3-P).
	///
	/// Defined by RFC 6408, Section 7.3.
	///
	/// See [WMF-T33-109-R015v02 - WiMAX Forum® Network Architecture - Detailed Protocols and Procedures, Policy and Charging Control - Release 1.5](http://www.wimaxforum.org/resources/documents/technical/T33).
	W_PCC_R3_P = 16777287,
	
	/// Worldwide Interoperability for Microwave Access (WiMAX) Forum Policy and Charging Control R3 Policies Diameter Application (WiMAX PCC-R3-OFC).
	///
	/// Defined by RFC 6408, Section 7.3.
	///
	/// See [WMF-T33-109-R015v02 - WiMAX Forum® Network Architecture - Detailed Protocols and Procedures, Policy and Charging Control - Release 1.5](http://www.wimaxforum.org/resources/documents/technical/T33).
	W_PCC_R3_OFC = 16777288,
	
	/// Worldwide Interoperability for Microwave Access (WiMAX) Forum Policy and Charging Control R3 Policies Diameter Application (WiMAX PCC-R3-OFC-PRIME).
	///
	/// Defined by RFC 6408, Section 7.3.
	///
	/// See [WMF-T33-109-R015v02 - WiMAX Forum® Network Architecture - Detailed Protocols and Procedures, Policy and Charging Control - Release 1.5](http://www.wimaxforum.org/resources/documents/technical/T33).
	W_PCC_R3_OFC_PRIME = 16777289,
	
	/// Worldwide Interoperability for Microwave Access (WiMAX) Forum Policy and Charging Control R3 Policies Diameter Application (WiMAX PCC-R3-OC).
	///
	/// Defined by RFC 6408, Section 7.3.
	///
	/// See [WMF-T33-109-R015v02 - WiMAX Forum® Network Architecture - Detailed Protocols and Procedures, Policy and Charging Control - Release 1.5](http://www.wimaxforum.org/resources/documents/technical/T33).
	W_PCC_R3_OC = 16777290,
	
	/// Relay.
	///
	/// Defined by RFC 6408, Section 7.1.
	///
	/// See RFC 3588.
	Relay = 4294967295,
}
