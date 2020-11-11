// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Service field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceField
{
	/// Legacy "Full" NAPTR (ie not S-NAPTR or U-NAPTR).
	///
	/// See RFC 6116.
	///
	/// Only supports the U-NAPTR flag `U` (and thus, by implication `D`).
	/// Also supports the non-terminal (absence of flags) state, in which case a REGEXP is not permitted.
	Enum
	{
		/// Will never be empty.
		enum_services: &'static IndexSet<EnumService>
	},
	
	/// Legacy "Full" NAPTR (ie not S-NAPTR or U-NAPTR).
	///
	/// Seems to be valid for `A` and `S` flags, but not `U`.
	SessionInitiationProtocol
	{
		/// Resolution service (proxy for transport protocol).
		resolution_service: SessionInitiationProtocolResolutionService,
	},
	
	/// U-NAPTR.
	///
	/// See RFC 6011.
	///
	/// Session Initiation Protocol (SIP) User Agent Configuration.
	///
	/// Only supports the U-NAPTR flag `U` (and thus, by implication `D`).
	///
	/// The URI must be HTTPS.
	/// There is never a transport protocol.
	SessionInitiationProtocolUserAgentConfiguration,
	
	/// U-NATPR-like.
	///
	/// Defined before U-NAPTR RFC.
	///
	/// Has a restricted, but unique, REGEXP syntax, similar in spirit to that of S-NAPTR.
	///
	/// Only supports the U-NAPTR flag `U` (and thus, by implication `D`).
	///
	/// The URI must be HTTP, HTTPS or FTP.
	NoSolicit,
	
	/// S-NAPTR.
	///
	/// Diameter using S-NAPTR.
	///
	/// See RFC 6408.
	///
	/// This is a subset of <https://www.iana.org/assignments/s-naptr-parameters/s-naptr-parameters.xhtml#s-naptr-parameters-1>.
	///
	/// Seems to be valid for `A` and `S` flags, but not `U`.
	Diameter
	{
		/// An IANA-registered application identifier, or `Unspecified` if not known.
		application_identifier: DiameterApplicationIdentifier,
	
		/// Transport protocols.
		///
		/// Can legitimately be empty, unless `application_identifier` is `DiameterApplicationIdentifier::Unspecified { legacy: true }`, in which case either one of `DiameterTransportProtocol::TCP` or `DiameterTransportProtocol::SCTP` will be present.
		transport_protocols: &'static HashSet<DiameterTransportProtocol>,
	},
	
	/// S-NAPTR.
	///
	/// RADIUS using S-NAPTR.
	///
	/// See RFC 7585.
	///
	/// This is a subset of <https://www.iana.org/assignments/s-naptr-parameters/s-naptr-parameters.xhtml#s-naptr-parameters-1>.
	///
	/// Seems to be valid for `A` and `S` flags, but not `U`.
	Radius
	{
		/// Technically, not an application identifier but for all intents and purposes this is.
		traffic_identifier: RadiusTrafficIdentifier,
		
		/// Transport protocols.
		///
		/// Can legitimately be empty.
		transport_protocols: &'static HashSet<RadiusTransportProtocol>,
	},
	
	/// U-NAPTR.
	///
	/// See RFC 6503.
	///
	/// Only supports the U-NAPTR flag `U` (and thus, by implication `D`).
	///
	/// The URI must be either HTTP or HTTPS.
	CentralizedConferencing
	{
		transport_protocol: Option<CentralizedConferencingTransportProtocol>,
	},
	
	/// U-NAPTR.
	///
	/// See RFC 5986.
	///
	/// Only supports the U-NAPTR flag `U` (and thus, by implication `D`).
	///
	/// The URI must be either HTTP or HTTPS.
	LocalLocationInformationServer
	{
		transport_protocol: Option<LocalLocationInformationServerTransportProtocol>,
	},
	
	/// S-NAPTR.
	///
	/// Defined in various RFCS, including 3982, 4414, 4698 and 5144.
	InternetRegistryInformationService
	{
		registry_type: InternetRegistryInformationServiceRegistryType,
		
		/// Transport protocols.
		///
		/// Can legitimately be empty (default protocol is now XPC).
		transport_protocols: &'static HashSet<InternetRegistryInformationServiceTransportProtocol>,
	},
	
	/// U-NAPTR.
	///
	/// See RFC 5222 and RFC 8917.
	///
	/// Only supports the U-NAPTR flag `U` (and thus, by implication `D`).
	///
	/// The URI must be either HTTP or HTTPS and presumably should match the transport protocol.
	LocationToServiceTranslationProtocol
	{
		/// Is this a generic LoST server, ot one that just performs validation?
		profile: LocationToServiceTranslationProtocolProfile,
		
		/// Transport protocols.
		///
		/// Can legitimately be empty.
		transport_protocol: Option<HypertextTransportProtocol>,
	},
	
	/// U-NAPTR.
	///
	/// See RFC 5928.
	///
	/// Traversal using Relays around NAT (TURN).
	TraversalUsingRelaysAroundNetworkAddressTranslation
	{
		/// Transport protocols.
		///
		/// Can legitimately be empty.
		transport_protocols: &'static HashSet<TraversalUsingRelaysAroundNetworkAddressTranslationTransportProtocol>,
	},
	
	/// U-NAPTR.
	///
	/// See RFC 7286.
	///
	/// Only supports the U-NAPTR flag `U` (and thus, by implication `D`).
	///
	/// The URI must be either HTTP or HTTPS and presumably should match the transport protocol.
	ApplicationLayerTrafficOptimization
	{
		/// Transport protocols.
		///
		/// Can legitimately be empty.
		transport_protocol: Option<HypertextTransportProtocol>,
	},
}
