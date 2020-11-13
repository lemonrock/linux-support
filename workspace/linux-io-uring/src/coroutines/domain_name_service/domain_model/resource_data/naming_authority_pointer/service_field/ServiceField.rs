// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Service field.
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(EnumDiscriminants)]
#[strum_discriminants(name(ServiceFieldKind))]
#[strum_discriminants(derive(PartialOrd))]
#[strum_discriminants(derive(Ord))]
#[strum_discriminants(derive(Hash))]
pub enum ServiceField
{
	/// Legacy "Full" NAPTR (ie not S-NAPTR or U-NAPTR).
	///
	/// See RFC 6116.
	///
	/// Only supports the U-NAPTR flag `U` (and thus, by implication `D`).
	/// Also supports the non-terminal (absence of flags) state, in which case a REGEXP is not permitted.
	///
	/// Also known as `E.164 to URI` or `E2U` and E.164 number to URI mapping (`ENUM`).
	Enum
	{
		/// Will never be empty.
		enum_services: &'static IndexSet<EnumService>
	},
	
	/// Legacy "Full" NAPTR (ie not S-NAPTR or U-NAPTR).
	///
	/// Seems to be valid for all flags and non-terminal, but it really isn't clear how the `U` (or `D`) flag would work.
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
	/// Has a restricted, but unique, REGEXP syntax, similar in spirit to that of U-NAPTR.
	///
	/// Only supports the U-NAPTR flag `U` (and thus, by implication `D`).
	///
	/// The URI must be HTTP, HTTPS or FTP.
	NoSolicit,
	
	/// S-NAPTR.
	///
	/// Diameter using S-NAPTR and legacy diameter using "Full" NAPTR.
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
	
	/// S-NAPTR.
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

impl ServiceField
{
	pub(crate) fn validate_mutually_exclusive_flags<'message>(&self, mutually_exclusive_flag: Option<NamingAuthorityMutuallyExclusiveFlag>, regular_expression: Either<ParsedName<'message>, ParsedCharacterString<'message>>) -> Result<Replacement<'message, ParsedName<'message>, ParsedUri<'message>, ParsedCharacterString<'message>>, IgnoreServiceFieldReason>
	{
		use self::HypertextTransportProtocol::*;
		use self::IgnoreServiceFieldReason::*;
		use self::NamingAuthorityMutuallyExclusiveFlag::*;
		use self::Replacement::*;
		use self::ServiceField::*;
		
		match (self, mutually_exclusive_flag, regular_expression)
		{
			(&Enum { .. }, None, Left(domain_name)) => Ok(DomainName(domain_name)),
			(&Enum { .. }, None, Right(_)) => Err(NonTerminalServiceFieldMustNotHaveARegularExpression(ServiceFieldKind::Enum)),
			(&Enum { .. }, Some(P), _) => Err(DoesNotSupportMutuallyExclusiveFlag(ServiceFieldKind::Enum, P)),
			(&Enum { .. }, Some(D), Right(_)) => Err(DFlagFieldMustNotHaveARegularExpression(ServiceFieldKind::Enum)),
			(&Enum { .. }, Some(S), _) => Err(DoesNotSupportMutuallyExclusiveFlag(ServiceFieldKind::Enum, S)),
			(&Enum { .. }, Some(A), _) => Err(DoesNotSupportMutuallyExclusiveFlag(ServiceFieldKind::Enum, A)),
			(&Enum { .. }, Some(U), Left(_)) => Err(RequiresAnUNaptrRegularExpression(ServiceFieldKind::Enum)),
			(&Enum { .. }, Some(U), Some(regular_expression)) => Ok(UnvalidatedRegularExpression(regular_expression)),
			(&Enum { .. }, Some(D), Left(domain_name)) => Ok(DomainName(domain_name)),
			
			(&SessionInitiationProtocol { .. }, None, Left(domain_name)) => Ok(DomainName(domain_name)),
			(&SessionInitiationProtocol { .. }, None, Right(_)) => Err(NonTerminalServiceFieldMustNotHaveARegularExpression(ServiceFieldKind::SessionInitiationProtocol)),
			(&SessionInitiationProtocol { .. }, Some(P), _) => Err(DoesNotSupportMutuallyExclusiveFlag(ServiceFieldKind::SessionInitiationProtocol, P)),
			(&SessionInitiationProtocol { .. }, Some(D), Right(_)) => Err(DFlagFieldMustNotHaveARegularExpression(ServiceFieldKind::SessionInitiationProtocol)),
			(&SessionInitiationProtocol { .. }, Some(S), Left(domain_name)) => Ok(DomainName(domain_name)),
			(&SessionInitiationProtocol { .. }, Some(S), Right(_)) => Err(TerminalServiceFieldMustNotHaveARegularExpression(ServiceFieldKind::SessionInitiationProtocol, S)),
			(&SessionInitiationProtocol { .. }, Some(A), Left(domain_name)) => Ok(DomainName(domain_name)),
			(&SessionInitiationProtocol { .. }, Some(A), Right(_)) => Err(TerminalServiceFieldMustNotHaveARegularExpression(ServiceFieldKind::SessionInitiationProtocol, S)),
			(&SessionInitiationProtocol { .. }, Some(U), _) => Err(DoesNotSupportMutuallyExclusiveFlag(ServiceFieldKind::NoSolicit, U)),
			(&SessionInitiationProtocol { .. }, Some(D), Left(domain_name)) => Ok(DomainName(domain_name)),
			
			(&SessionInitiationProtocolUserAgentConfiguration, None, Left(domain_name)) => Ok(DomainName(domain_name)),
			(&SessionInitiationProtocolUserAgentConfiguration, None, Right(_)) => Err(NonTerminalServiceFieldMustNotHaveARegularExpression(ServiceFieldKind::SessionInitiationProtocolUserAgentConfiguration)),
			(&SessionInitiationProtocolUserAgentConfiguration, Some(P), _) => Err(DoesNotSupportMutuallyExclusiveFlag(ServiceFieldKind::SessionInitiationProtocolUserAgentConfiguration, P)),
			(&SessionInitiationProtocolUserAgentConfiguration, Some(D), Right(_)) => Err(DFlagFieldMustNotHaveARegularExpression(ServiceFieldKind::SessionInitiationProtocolUserAgentConfiguration)),
			(&SessionInitiationProtocolUserAgentConfiguration, Some(S), _) => Err(DoesNotSupportMutuallyExclusiveFlag(ServiceFieldKind::SessionInitiationProtocolUserAgentConfiguration, S)),
			(&SessionInitiationProtocolUserAgentConfiguration, Some(A), _) => Err(DoesNotSupportMutuallyExclusiveFlag(ServiceFieldKind::SessionInitiationProtocolUserAgentConfiguration, A)),
			(&SessionInitiationProtocolUserAgentConfiguration, Some(U), Left(_)) => Err(RequiresAnUNaptrRegularExpression(ServiceFieldKind::SessionInitiationProtocolUserAgentConfiguration)),
			(&SessionInitiationProtocolUserAgentConfiguration, Some(U), Some(regular_expression)) => Self::is_valid_u_naptr_regular_expression_uri(ServiceFieldKind::SessionInitiationProtocolUserAgentConfiguration, regular_expression, Some(https)),
			(&SessionInitiationProtocolUserAgentConfiguration, Some(D), Left(domain_name)) => Ok(DomainName(domain_name)),
			
			(&NoSolicit, None, Left(domain_name)) => Ok(DomainName(domain_name)),
			(&NoSolicit, None, Right(_)) => Err(NonTerminalServiceFieldMustNotHaveARegularExpression(ServiceFieldKind::NoSolicit)),
			(&NoSolicit, Some(P), _) => Err(DoesNotSupportMutuallyExclusiveFlag(ServiceFieldKind::NoSolicit, P)),
			(&NoSolicit, Some(D), Right(_)) => Err(DFlagFieldMustNotHaveARegularExpression(ServiceFieldKind::NoSolicit)),
			(&NoSolicit, Some(S), _) => Err(DoesNotSupportMutuallyExclusiveFlag(ServiceFieldKind::NoSolicit, S)),
			(&NoSolicit, Some(A), _) => Err(DoesNotSupportMutuallyExclusiveFlag(ServiceFieldKind::NoSolicit, A)),
			(&NoSolicit, Some(U), Some(regular_expression)) => Self::is_valid_no_solicit_regular_expression_uri(regular_expression),
			(&NoSolicit, Some(D), Left(domain_name)) => Ok(DomainName(domain_name)),
			
			(&Diameter { .. }, None, Left(domain_name)) => Ok(DomainName(domain_name)),
			(&Diameter { .. }, None, Right(_)) => Err(NonTerminalServiceFieldMustNotHaveARegularExpression(ServiceFieldKind::Diameter)),
			(&Diameter { .. }, Some(P), _) => Err(DoesNotSupportMutuallyExclusiveFlag(ServiceFieldKind::Diameter, P)),
			(&Diameter { .. }, Some(D), Right(_)) => Err(DFlagFieldMustNotHaveARegularExpression(ServiceFieldKind::Diameter)),
			(&Diameter { .. }, Some(S), Left(domain_name)) => Ok(DomainName(domain_name)),
			(&Diameter { .. }, Some(S), Right(_)) => Err(TerminalServiceFieldMustNotHaveARegularExpression(ServiceFieldKind::Diameter, S)),
			(&Diameter { .. }, Some(A), Left(domain_name)) => Ok(DomainName(domain_name)),
			(&Diameter { .. }, Some(A), Right(_)) => Err(TerminalServiceFieldMustNotHaveARegularExpression(ServiceFieldKind::Diameter, S)),
			(&Diameter { .. }, Some(U), _) => Err(DoesNotSupportMutuallyExclusiveFlag(ServiceFieldKind::NoSolicit, U)),
			(&Diameter { .. }, Some(D), Left(domain_name)) => Ok(DomainName(domain_name)),
			
			(&Radius { .. }, None, Left(domain_name)) => Ok(DomainName(domain_name)),
			(&Radius { .. }, None, Right(_)) => Err(NonTerminalServiceFieldMustNotHaveARegularExpression(ServiceFieldKind::Radius)),
			(&Radius { .. }, Some(P), _) => Err(DoesNotSupportMutuallyExclusiveFlag(ServiceFieldKind::Radius, P)),
			(&Radius { .. }, Some(D), Right(_)) => Err(DFlagFieldMustNotHaveARegularExpression(ServiceFieldKind::Radius)),
			(&Radius { .. }, Some(S), Left(domain_name)) => Ok(DomainName(domain_name)),
			(&Radius { .. }, Some(S), Right(_)) => Err(TerminalServiceFieldMustNotHaveARegularExpression(ServiceFieldKind::Radius, S)),
			(&Radius { .. }, Some(A), Left(domain_name)) => Ok(DomainName(domain_name)),
			(&Radius { .. }, Some(A), Right(_)) => Err(TerminalServiceFieldMustNotHaveARegularExpression(ServiceFieldKind::Radius, S)),
			(&Radius { .. }, Some(U), _) => Err(DoesNotSupportMutuallyExclusiveFlag(ServiceFieldKind::NoSolicit, U)),
			(&Radius { .. }, Some(D), Left(domain_name)) => Ok(DomainName(domain_name)),
			
			(&CentralizedConferencing { .. }, None, Left(domain_name)) => Ok(DomainName(domain_name)),
			(&CentralizedConferencing { .. }, None, Right(_)) => Err(NonTerminalServiceFieldMustNotHaveARegularExpression(ServiceFieldKind::CentralizedConferencing)),
			(&CentralizedConferencing { .. }, Some(P), _) => Err(DoesNotSupportMutuallyExclusiveFlag(ServiceFieldKind::CentralizedConferencing, P)),
			(&CentralizedConferencing { .. }, Some(D), Right(_)) => Err(DFlagFieldMustNotHaveARegularExpression(ServiceFieldKind::CentralizedConferencing)),
			(&CentralizedConferencing { .. }, Some(S), _) => Err(DoesNotSupportMutuallyExclusiveFlag(ServiceFieldKind::CentralizedConferencing, S)),
			(&CentralizedConferencing { .. }, Some(A), _) => Err(DoesNotSupportMutuallyExclusiveFlag(ServiceFieldKind::CentralizedConferencing, A)),
			(&CentralizedConferencing { .. }, Some(U), Left(_)) => Err(RequiresAnUNaptrRegularExpression(ServiceFieldKind::CentralizedConferencing)),
			(&CentralizedConferencing { .. }, Some(U), Some(regular_expression)) => Self::is_valid_u_naptr_regular_expression_uri(ServiceFieldKind::CentralizedConferencing, regular_expression, None),
			(&CentralizedConferencing { .. }, Some(D), Left(domain_name)) => Ok(DomainName(domain_name)),
			
			(&LocalLocationInformationServer { .. }, None, Left(domain_name)) => Ok(DomainName(domain_name)),
			(&LocalLocationInformationServer { .. }, None, Right(_)) => Err(NonTerminalServiceFieldMustNotHaveARegularExpression(ServiceFieldKind::LocalLocationInformationServer)),
			(&LocalLocationInformationServer { .. }, Some(P), _) => Err(DoesNotSupportMutuallyExclusiveFlag(ServiceFieldKind::LocalLocationInformationServer, P)),
			(&LocalLocationInformationServer { .. }, Some(D), Right(_)) => Err(DFlagFieldMustNotHaveARegularExpression(ServiceFieldKind::LocalLocationInformationServer)),
			(&LocalLocationInformationServer { .. }, Some(S), _) => Err(DoesNotSupportMutuallyExclusiveFlag(ServiceFieldKind::LocalLocationInformationServer, S)),
			(&LocalLocationInformationServer { .. }, Some(A), _) => Err(DoesNotSupportMutuallyExclusiveFlag(ServiceFieldKind::LocalLocationInformationServer, A)),
			(&LocalLocationInformationServer { .. }, Some(U), Left(_)) => Err(RequiresAnUNaptrRegularExpression(ServiceFieldKind::LocalLocationInformationServer)),
			(&LocalLocationInformationServer { .. }, Some(U), Some(regular_expression)) => Self::is_valid_u_naptr_regular_expression_uri(ServiceFieldKind::LocalLocationInformationServer, regular_expression, None),
			(&LocalLocationInformationServer { .. }, Some(D), Left(domain_name)) => Ok(DomainName(domain_name)),
			
			(&InternetRegistryInformationService { .. }, None, Left(domain_name)) => Ok(DomainName(domain_name)),
			(&InternetRegistryInformationService { .. }, None, Right(_)) => Err(NonTerminalServiceFieldMustNotHaveARegularExpression(ServiceFieldKind::InternetRegistryInformationService)),
			(&InternetRegistryInformationService { .. }, Some(P), _) => Err(DoesNotSupportMutuallyExclusiveFlag(ServiceFieldKind::InternetRegistryInformationService, P)),
			(&InternetRegistryInformationService { .. }, Some(D), Right(_)) => Err(DFlagFieldMustNotHaveARegularExpression(ServiceFieldKind::InternetRegistryInformationService)),
			(&InternetRegistryInformationService { .. }, Some(S), Left(domain_name)) => Ok(DomainName(domain_name)),
			(&InternetRegistryInformationService { .. }, Some(S), Right(_)) => Err(TerminalServiceFieldMustNotHaveARegularExpression(ServiceFieldKind::InternetRegistryInformationService, S)),
			(&InternetRegistryInformationService { .. }, Some(A), Left(domain_name)) => Ok(DomainName(domain_name)),
			(&InternetRegistryInformationService { .. }, Some(A), Right(_)) => Err(TerminalServiceFieldMustNotHaveARegularExpression(ServiceFieldKind::InternetRegistryInformationService, S)),
			(&InternetRegistryInformationService { .. }, Some(U), _) => Err(DoesNotSupportMutuallyExclusiveFlag(ServiceFieldKind::NoSolicit, U)),
			(&InternetRegistryInformationService { .. }, Some(D), Left(domain_name)) => Ok(DomainName(domain_name)),
			
			(&LocationToServiceTranslationProtocol { .. }, None, Left(domain_name)) => Ok(DomainName(domain_name)),
			(&LocationToServiceTranslationProtocol { .. }, None, Right(_)) => Err(NonTerminalServiceFieldMustNotHaveARegularExpression(ServiceFieldKind::LocationToServiceTranslationProtocol)),
			(&LocationToServiceTranslationProtocol { .. }, Some(P), _) => Err(DoesNotSupportMutuallyExclusiveFlag(ServiceFieldKind::LocationToServiceTranslationProtocol, P)),
			(&LocationToServiceTranslationProtocol { .. }, Some(D), Right(_)) => Err(DFlagFieldMustNotHaveARegularExpression(ServiceFieldKind::LocationToServiceTranslationProtocol)),
			(&LocationToServiceTranslationProtocol { .. }, Some(S), _) => Err(DoesNotSupportMutuallyExclusiveFlag(ServiceFieldKind::LocationToServiceTranslationProtocol, S)),
			(&LocationToServiceTranslationProtocol { .. }, Some(A), _) => Err(DoesNotSupportMutuallyExclusiveFlag(ServiceFieldKind::LocationToServiceTranslationProtocol, A)),
			(&LocationToServiceTranslationProtocol { .. }, Some(U), Left(_)) => Err(RequiresAnUNaptrRegularExpression(ServiceFieldKind::LocationToServiceTranslationProtocol)),
			(&LocationToServiceTranslationProtocol { transport_protocol, .. }, Some(U), Some(regular_expression)) => Self::is_valid_u_naptr_regular_expression_uri(ServiceFieldKind::LocationToServiceTranslationProtocol, regular_expression, transport_protocol),
			(&LocationToServiceTranslationProtocol { .. }, Some(D), Left(domain_name)) => Ok(DomainName(domain_name)),
			
			(&TraversalUsingRelaysAroundNetworkAddressTranslation { .. }, None, Left(domain_name)) => Ok(DomainName(domain_name)),
			(&TraversalUsingRelaysAroundNetworkAddressTranslation { .. }, None, Right(_)) => Err(NonTerminalServiceFieldMustNotHaveARegularExpression(ServiceFieldKind::TraversalUsingRelaysAroundNetworkAddressTranslation)),
			(&TraversalUsingRelaysAroundNetworkAddressTranslation { .. }, Some(P), _) => Err(DoesNotSupportMutuallyExclusiveFlag(ServiceFieldKind::TraversalUsingRelaysAroundNetworkAddressTranslation, P)),
			(&TraversalUsingRelaysAroundNetworkAddressTranslation { .. }, Some(D), Right(_)) => Err(DFlagFieldMustNotHaveARegularExpression(ServiceFieldKind::TraversalUsingRelaysAroundNetworkAddressTranslation)),
			(&TraversalUsingRelaysAroundNetworkAddressTranslation { .. }, Some(S), Left(domain_name)) => Ok(DomainName(domain_name)),
			(&TraversalUsingRelaysAroundNetworkAddressTranslation { .. }, Some(S), Right(_)) => Err(TerminalServiceFieldMustNotHaveARegularExpression(ServiceFieldKind::TraversalUsingRelaysAroundNetworkAddressTranslation, S)),
			(&TraversalUsingRelaysAroundNetworkAddressTranslation { .. }, Some(A), Left(domain_name)) => Ok(DomainName(domain_name)),
			(&TraversalUsingRelaysAroundNetworkAddressTranslation { .. }, Some(A), Right(_)) => Err(TerminalServiceFieldMustNotHaveARegularExpression(ServiceFieldKind::TraversalUsingRelaysAroundNetworkAddressTranslation, S)),
			(&TraversalUsingRelaysAroundNetworkAddressTranslation { .. }, Some(U), _) => Err(DoesNotSupportMutuallyExclusiveFlag(ServiceFieldKind::NoSolicit, U)),
			(&TraversalUsingRelaysAroundNetworkAddressTranslation { .. }, Some(D), Left(domain_name)) => Ok(DomainName(domain_name)),
			
			(&ApplicationLayerTrafficOptimization { .. }, None, Left(domain_name)) => Ok(DomainName(domain_name)),
			(&ApplicationLayerTrafficOptimization { .. }, None, Right(_)) => Err(NonTerminalServiceFieldMustNotHaveARegularExpression(ServiceFieldKind::ApplicationLayerTrafficOptimization)),
			(&ApplicationLayerTrafficOptimization { .. }, Some(P), _) => Err(DoesNotSupportMutuallyExclusiveFlag(ServiceFieldKind::ApplicationLayerTrafficOptimization, P)),
			(&ApplicationLayerTrafficOptimization { .. }, Some(D), Right(_)) => Err(DFlagFieldMustNotHaveARegularExpression(ServiceFieldKind::ApplicationLayerTrafficOptimization)),
			(&ApplicationLayerTrafficOptimization { .. }, Some(S), _) => Err(DoesNotSupportMutuallyExclusiveFlag(ServiceFieldKind::ApplicationLayerTrafficOptimization, S)),
			(&ApplicationLayerTrafficOptimization { .. }, Some(A), _) => Err(DoesNotSupportMutuallyExclusiveFlag(ServiceFieldKind::ApplicationLayerTrafficOptimization, A)),
			(&ApplicationLayerTrafficOptimization { .. }, Some(U), Left(_)) => Err(RequiresAnUNaptrRegularExpression(ServiceFieldKind::ApplicationLayerTrafficOptimization)),
			(&ApplicationLayerTrafficOptimization { transport_protocol, .. }, Some(U), Some(regular_expression)) => Self::is_valid_u_naptr_regular_expression_uri(ServiceFieldKind::ApplicationLayerTrafficOptimization, regular_expression, transport_protocol),
			(&ApplicationLayerTrafficOptimization { .. }, Some(U)) => Ok(()),
			(&ApplicationLayerTrafficOptimization { .. }, Some(D), Left(domain_name)) => Ok(DomainName(domain_name)),
		}
	}
	
	/// See RFC 4095, Section 2 The No-Solicit NAPTR Application, page 4.
	/// If `valid_protocol` is `None`, then both `HTTP` and `HTTPS` are considered valid.
	fn is_valid_no_solicit_regular_expression_uri<'message>(regular_expression: ParsedCharacterString<'message>) -> Result<Replacement<'message, ParsedName<'message>, ParsedUri<'message>, ParsedCharacterString<'message>>, IgnoreServiceFieldReason>
	{
		use self::IgnoreServiceFieldReason::*;
		
		// At least 3 bytes long for 3 delimiter characters.
		const DelimiterCharacterLength: usize = 1;
		const MinimumRegularExpressionLength: usize = DelimiterCharacterLength + DelimiterCharacterLength + DelimiterCharacterLength;
		
		let regular_expression_length = regular_expression.len();
		
		if unlikely!(regular_expression_length < MinimumRegularExpressionLength)
		{
			return Err(ExpectedANoSolicitRegularExpression)
		}
		
		let first_delimiter_character = regular_expression.get_unchecked_value_safe(0);
		let second_delimiter_character = regular_expression.get_unchecked_value_safe(1);
		let last_delimiter_character = regular_expression.get_unchecked_value_safe(regular_expression_length - 1);
		
		if unlikely!(first_delimiter_character != second_delimiter_character || second_delimiter_character != last_delimiter_character)
		{
			return Err(ExpectedANoSolicitRegularExpressionToHaveTheSameDelimiterCharacter { first_delimiter_character, second_delimiter_character, last_delimiter_character })
		}
		
		match first_delimiter_character
		{
			b'/' | b'!' => (),
			b'1' ..= b'9' | b'i' => return Err(NoSolicitRegularExpressionHasAnInvalidDelimiterCharacter(first_delimiter_character)),
			_ => (),
		};
		
		let target_uri = URI::try_from(&regular_expression[(DelimiterCharacterLength + DelimiterCharacterLength) .. (regular_expression_length - DelimiterCharacterLength)]).map_err(|error| InvalidTargetUri(error, ServiceFieldKind::NoSolicit))?;
		
		use self::Scheme::*;
		match target_uri.scheme()
		{
			&HTTP => (),
			&HTTPS => (),
			&FTP => (),
			other @ _ => return Err(NoSolicitRegularExpressionUriIsNotHttpOrHttpsOrFtp(other.into_owned())),
		};
		
		Ok(Replacement::UniformResourceIdentifier(ParsedUri::from(target_uri)))
	}
	
	/// See RFC 4848, Section 2.2 Permitted Regular Expressions.
	/// If `valid_protocol` is `None`, then both `HTTP` and `HTTPS` are considered valid.
	fn is_valid_u_naptr_regular_expression_uri<'message>(service_field_kind: ServiceFieldKind, regular_expression: ParsedCharacterString<'message>, valid_protocol: Option<HypertextTransportProtocol>) -> Result<Replacement<'message, ParsedName<'message>, ParsedUri<'message>, ParsedCharacterString<'message>>, IgnoreServiceFieldReason>
	{
		use self::IgnoreServiceFieldReason::*;
		
		// `u-naptr-regexp = "!.*!"<URI>"!"`.
		const Prefix: &'static [u8] = b"!.*!";
		const Suffix: &'static [u8] = b"!";
		const PrefixLength: usize = Prefix.len();
		const SuffixLength: usize = Suffix.len();
		
		const MinimumRegularExpressionLength: usize = PrefixLength + SuffixLength;
		
		let regular_expression_length = regular_expression.len();
		
		if unlikely!(regular_expression_length < MinimumRegularExpressionLength)
		{
			return Err(ExpectedAnUNaptrRegularExpression(service_field_kind))
		}
		
		if unlikely!(&regular_expression[0 .. PrefixLength] != Prefix)
		{
			return Err(UNaptrRegularExpressionDoesNotStartWithCorrectPrefix(service_field_kind))
		}
		
		let suffix_starts_at_index = (regular_expression_length - SuffixLength);
		if unlikely!(&regular_expression[suffix_starts_at_index ..] != Suffix)
		{
			return Err(UNaptrRegularExpressionDoesNotEndWithCorrectSuffix(service_field_kind))
		}
		
		let target_uri = URI::try_from(&regular_expression[PrefixLength .. suffix_starts_at_index]).map_err(|error| InvalidTargetUri(error, service_field_kind))?;
		
		use self::Scheme::*;
		use self::HypertextTransportProtocol::*;
		match (valid_protocol, target_uri.scheme())
		{
			(None, &HTTP) => (),
			(None, &HTTPS) => (),
			(None, other @ _) => return Err(UNaptrRegularExpressionUriIsNotHttpOrHttps(service_field_kind, other.into_owned())),
			
			(Some(http), &HTTP) => (),
			(Some(http), other @ _) => return Err(UNaptrRegularExpressionUriIsNotHttp(service_field_kind, other.into_owned())),
			
			(Some(https), &HTTPS) => (),
			(Some(https), other @ _) => return Err(UNaptrRegularExpressionUriIsNotHttps(service_field_kind, other.into_owned())),
		};
		
		Ok(Replacement::UniformResourceIdentifier(ParsedUri::from(target_uri)))
	}
}
