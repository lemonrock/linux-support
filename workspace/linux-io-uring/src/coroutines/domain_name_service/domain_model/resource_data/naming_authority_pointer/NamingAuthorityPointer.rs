// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Service field.
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(EnumDiscriminants)]
#[strum_discriminants(name(ServiceFieldKind))]
#[strum_discriminants(derive(PartialOrd))]
#[strum_discriminants(derive(Ord))]
#[strum_discriminants(derive(Hash))]
pub enum NamingAuthorityPointer<N: Name<TypeEquality=TE>, OOPU: OwnedOrParsedUri<TypeEquality=TE>, CS: CharacterString<TypeEquality=TE>, TE: OwnedOrParsedTypeEquality>
{
	/// A non-terminal, empty service field is one without any terminal flags *or* the `P` flag.
	NonTerminalAndEmpty
	{
		/// Most of the time, a domain name is returned that then should be queried for `NAPTR`
		domain_name_or_regular_expression: RegularExpressionResolvingToDomainNameOrQueryNaptrResourceRecord<N, CS>,
	},
	
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
		enum_services: IndexSet<EnumService>,
		
		/// Either a regular expression which resolves to an URI is provided, or a domain is provided which should be queried for `URI` resource records.
		///
		/// The later is highly unlikely.
		domain_name_or_regular_expression: RegularExpressionResolvingToUriOrQueryUriResourceRecord<N, CS>,
	},
	
	/// Unofficial (unregistered witbh IANA) use of U-NAPTR which uses U-NAPTR-incompatible regular expressions.
	///
	/// See [Business Document Metadata Service Location Version 1.0](https://docs.oasis-open.org/bdxr/BDX-Location/v1.0/BDX-Location-v1.0.html).
	BusinessDocumentMetadataServiceLocation
	{
		/// Collaboration-Protocol Profile and Agreement  CPPA
		profile: BusinessDocumentMetadataServiceLocationProfile,
		
		/// Transport protocol.
		///
		/// Can legitimately be empty as per RFC 3958 but the above specification does not seem to permit this.
		transport_protocol: BusinessDocumentMetadataServiceLocationTransportProtocol,
		
		/// Either a regular expression which resolves to an URI is provided, or a domain is provided which should be queried for `URI` resource records.
		///
		/// The later is highly unlikely.
		domain_name_or_regular_expression: RegularExpressionResolvingToUriOrQueryUriResourceRecord<N, CS>,
	},
	
	/// Legacy "Full" NAPTR (ie not S-NAPTR or U-NAPTR).
	///
	/// Seems to be valid for all flags and non-terminal, but it really isn't clear how the `U` (or `D`) flag would work.
	SessionInitiationProtocol
	{
		/// Resolution service (proxy for transport protocol).
		resolution_service: SessionInitiationProtocolResolutionService,
	
		/// What to query for next at `domain_name`.
		query_for_next: QueryResourceRecord<N>,
	},
	
	/// U-NAPTR.
	///
	/// See RFC 6011.
	///
	/// Session Initiation Protocol (SIP) User Agent Configuration.
	///
	/// Only supports the U-NAPTR flag `U` (and thus, by implication `D`).
	///
	/// The URI will be HTTPS (this is validated).
	SessionInitiationProtocolUserAgentConfiguration
	{
		/// Either a URI is provided, or a domain is provided which should be queried for `URI` resource records.
		///
		/// The later is highly unlikely.
		uri_or_query_for_uri_resource_record_next: UriOrQueryUriResourceRecord<N, OOPU>,
	},
	
	/// U-NATPR-like.
	///
	/// Defined before U-NAPTR RFC.
	///
	/// Has a restricted, but unique, REGEXP syntax, similar in spirit to that of U-NAPTR.
	///
	/// Only supports the U-NAPTR flag `U` (and thus, by implication `D`).
	///
	/// The URI will be HTTP, HTTPS or FTP (this is validated).
	NoSolicit
	{
		/// Either a URI is provided, or a domain is provided which should be queried for `URI` resource records.
		///
		/// The later is highly unlikely.
		uri_or_query_for_uri_resource_record_next: UriOrQueryUriResourceRecord<N, OOPU>,
	},
	
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
		
		/// What to query for next at `domain_name`.
		query_for_next: QueryResourceRecord<N>,
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
		
		/// What to query for next at `domain_name`.
		query_for_next: QueryResourceRecord<N>,
	},
	
	/// U-NAPTR.
	///
	/// See RFC 6503.
	///
	/// Only supports the U-NAPTR flag `U` (and thus, by implication `D`).
	///
	/// The URI must be either HTTP or HTTPS (this is validated).
	CentralizedConferencing
	{
		transport_protocol: Option<CentralizedConferencingTransportProtocol>,
		
		/// Either a URI is provided, or a domain is provided which should be queried for `URI` resource records.
		///
		/// The later is highly unlikely.
		uri_or_query_for_uri_resource_record_next: UriOrQueryUriResourceRecord<N, OOPU>,
	},
	
	/// U-NAPTR.
	///
	/// See RFC 5986.
	///
	/// Only supports the U-NAPTR flag `U` (and thus, by implication `D`).
	///
	/// The URI must be either HTTP or HTTPS (this is validated).
	LocalLocationInformationServer
	{
		transport_protocol: Option<LocalLocationInformationServerTransportProtocol>,
		
		/// Either a URI is provided, or a domain is provided which should be queried for `URI` resource records.
		///
		/// The later is highly unlikely.
		uri_or_query_for_uri_resource_record_next: UriOrQueryUriResourceRecord<N, OOPU>,
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
		
		/// What to query for next at `domain_name`.
		query_for_next: QueryResourceRecord<N>,
	},
	
	/// U-NAPTR.
	///
	/// See RFC 5222 and RFC 8917.
	///
	/// Only supports the U-NAPTR flag `U` (and thus, by implication `D`).
	///
	/// The URI must be either HTTP or HTTPS (this is validated) and should match the transport protocol (this is validated).
	LocationToServiceTranslationProtocol
	{
		/// Is this a generic LoST server, ot one that just performs validation?
		profile: LocationToServiceTranslationProtocolProfile,
		
		/// Transport protocol.
		///
		/// Can legitimately be empty.
		transport_protocol: Option<HypertextTransportProtocol>,
		
		/// Either a URI is provided, or a domain is provided which should be queried for `URI` resource records.
		///
		/// The later is highly unlikely.
		uri_or_query_for_uri_resource_record_next: UriOrQueryUriResourceRecord<N, OOPU>,
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
		
		/// What to query for next at `domain_name`.
		query_for_next: QueryResourceRecord<N>,
	},
	
	/// U-NAPTR.
	///
	/// See RFC 7286.
	///
	/// Only supports the U-NAPTR flag `U` (and thus, by implication `D`).
	///
	/// The URI must be either HTTP or HTTPS (this is validated) and should match the transport protocol (this is validated).
	ApplicationLayerTrafficOptimization
	{
		/// Transport protocol.
		///
		/// Can legitimately be empty.
		transport_protocol: Option<HypertextTransportProtocol>,
		
		/// Either a URI is provided, or a domain is provided which should be queried for `URI` resource records.
		///
		/// The later is highly unlikely.
		uri_or_query_for_uri_resource_record_next: UriOrQueryUriResourceRecord<N, OOPU>,
	},
}

impl<'message> ParsedRecord for NamingAuthorityPointer<ParsedName<'message>, ParsedUri<'message>, ParsedCharacterString<'message>, ParsedTypeEquality>
{
	type OrderPriorityAndWeight = (Order, Priority);
	
	type OwnedRecord = NamingAuthorityPointer<DomainTarget, OwnedUri, OwnedCharacterString, OwnedTypeEquality>;
	
	#[inline(always)]
	fn into_owned_records(records: OwnerNameToParsedRecordsValue<Self>) -> <Self::OwnedRecord as OwnedRecord>::OwnedRecords
	{
		let mut parsed_records = records.records();
		parsed_records.sort_unstable_by(|left, right|
		{
			use self::Ordering::Equal;
			
			let (left_parsed_record, left_order, left_priority) = left;
			let (right_parsed_record, right_order, right_priority) = right;
			
			let order_ordering = left_order.cmp(right_order);
			
			if order_ordering != Ordering::Equal
			{
				return order_ordering
			}
			
			left_priority.cmp(right_priority)
		});
		
		let length = parsed_records.len();
		let mut owned_records: Vec<MultipleOrderedThenPrioritizedThenUnsortedRecordsItem<NamingAuthorityPointer<DomainTarget, OwnedUri, OwnedCharacterString, OwnedTypeEquality>>> = Vec::with_capacity(length);
		unsafe { owned_records.set_len(length) };
		
		// Safe because `NoData` is a special case; there is always at least one record.
		let (_, (initial_order, initial_priority)) = parsed_records.get_unchecked_safe(0);
		
		let mut index = 0;
		let mut accumulating_order_count = 0usize;
		let mut accumulating_order_count_for_index = 0;
		let mut accumulating_order_count_for_order = *initial_order;
		for (parsed_record, (order, priority)) in parsed_records.into_iter()
		{
			if order == accumulating_order_count_for_order
			{
				accumulating_order_count += 1
			}
			else
			{
				debug_assert!(order > accumulating_order_count_for_order);
				debug_assert!(accumulating_order_count_for_index < index);
				
				owned_records.get_unchecked_mut_safe(accumulating_order_count_for_index).set_order_count(accumulating_order_count);
				
				accumulating_order_count = 1;
				accumulating_order_count_for_index = index;
				accumulating_order_count_for_order = order;
			}
			
			let owned_record = parsed_record.into_owned_record();
			let item = MultipleOrderedThenPrioritizedThenUnsortedRecordsItem::new(order, priority, owned_record);
			unsafe { owned_records.as_mut_ptr().add(index).write(item) }
			
			index + 1;
		}
		
		// Safe because `NoData` is a special case; there is always at least one record and so `accumulating_order_count` will never be `0`.
		owned_records.get_unchecked_mut_safe(accumulating_order_count_for_index).set_order_count(accumulating_order_count);
		
		MultipleOrderedThenPrioritizedThenUnsortedRecords::new(owned_records)
	}
}

impl OwnedRecord for NamingAuthorityPointer<DomainTarget, OwnedUri, OwnedCharacterString, OwnedTypeEquality>
{
	type OwnedRecords = MultipleOrderedThenPrioritizedThenUnsortedRecords<Self>;
	
	#[inline(always)]
	fn retrieve(query_types_cache: &mut QueryTypesCache) -> &mut Option<QueryTypeCache<Self::OwnedRecords>>
	{
		&mut query_types_cache.NAPTR
	}
}

impl<'message> NamingAuthorityPointer<ParsedName<'message>, ParsedUri<'message>, ParsedCharacterString<'message>, ParsedTypeEquality>
{
	#[inline(always)]
	fn into_owned_record(self) -> NamingAuthorityPointer<FullyQualifiedDomainName, OwnedUri, OwnedCharacterString, OwnedTypeEquality>
	{
		use self::NamingAuthorityPointer::*;
		
		match self
		{
			NonTerminalAndEmpty { domain_name_or_regular_expression } => NonTerminalAndEmpty { domain_name_or_regular_expression: domain_name_or_regular_expression.into() },
			
			Enum { enum_services, domain_name_or_regular_expression } => Enum { enum_services, domain_name_or_regular_expression: domain_name_or_regular_expression.into(), },
			
			BusinessDocumentMetadataServiceLocation { profile, transport_protocol, domain_name_or_regular_expression } => BusinessDocumentMetadataServiceLocation { profile, transport_protocol, domain_name_or_regular_expression: domain_name_or_regular_expression.into(), },
			
			SessionInitiationProtocol { resolution_service, query_for_next } => SessionInitiationProtocol { resolution_service, query_for_next: query_for_next.into(), },
		
			SessionInitiationProtocolUserAgentConfiguration { uri_or_query_for_uri_resource_record_next } => SessionInitiationProtocolUserAgentConfiguration { uri_or_query_for_uri_resource_record_next: uri_or_query_for_uri_resource_record_next.into(), },
			
			NoSolicit { uri_or_query_for_uri_resource_record_next } => NoSolicit { uri_or_query_for_uri_resource_record_next: uri_or_query_for_uri_resource_record_next.into(), },
			
			Diameter { application_identifier, transport_protocols, query_for_next } => Diameter { application_identifier, transport_protocols, query_for_next: query_for_next.into(), },
			
			Radius { traffic_identifier, transport_protocols, query_for_next } => Radius { traffic_identifier, transport_protocols, query_for_next: query_for_next.into(), },
			
			CentralizedConferencing { transport_protocol, uri_or_query_for_uri_resource_record_next } => CentralizedConferencing { transport_protocol, uri_or_query_for_uri_resource_record_next: uri_or_query_for_uri_resource_record_next.into(), },
			
			LocalLocationInformationServer { transport_protocol, uri_or_query_for_uri_resource_record_next } => LocalLocationInformationServer { transport_protocol, uri_or_query_for_uri_resource_record_next: uri_or_query_for_uri_resource_record_next.into(), },
			
			InternetRegistryInformationService { registry_type, transport_protocols, query_for_next } => InternetRegistryInformationService { registry_type, transport_protocols, query_for_next: query_for_next.into(), },
			
			LocationToServiceTranslationProtocol { profile, transport_protocol, uri_or_query_for_uri_resource_record_next } => LocationToServiceTranslationProtocol { profile, transport_protocol, uri_or_query_for_uri_resource_record_next: uri_or_query_for_uri_resource_record_next.into(), },
			
			TraversalUsingRelaysAroundNetworkAddressTranslation { transport_protocols, query_for_next } => TraversalUsingRelaysAroundNetworkAddressTranslation { transport_protocols, query_for_next: query_for_next.into(), },
			
			ApplicationLayerTrafficOptimization { transport_protocol, uri_or_query_for_uri_resource_record_next } => ApplicationLayerTrafficOptimization { transport_protocol, uri_or_query_for_uri_resource_record_next: uri_or_query_for_uri_resource_record_next.into(), },
		}
	}
}
