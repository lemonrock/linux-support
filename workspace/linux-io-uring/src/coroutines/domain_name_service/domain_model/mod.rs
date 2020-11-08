// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use self::caching::*;
use self::character_strings::*;
use self::errors::*;
use self::extended_dns::*;
use self::message::*;
use self::message::header::*;
use self::message::query::*;
use self::message::resource_record::*;
use self::name::*;
use self::owned_or_parsed::*;
use self::query_processors::*;
use self::records::*;
use self::resource_data::*;
use self::resource_data::certificate::*;
use self::resource_data::certification_authority_authorization::*;
use self::resource_data::dhcid::*;
use self::resource_data::digest::*;
use self::resource_data::dns_based_authentication_of_named_entities::*;
use self::resource_data::dnssec::*;
use self::resource_data::host_identity_protocol::*;
use self::resource_data::identifier_locator_network_protocol::*;
use self::resource_data::ipsec::*;
use self::resource_data::location::*;
use self::resource_data::naming_authority_pointer::*;
use self::resource_data::naming_authority_pointer::service_field::*;
use self::resource_data::naming_authority_pointer::service_field::enum_services::*;
use self::resource_data::naming_authority_pointer::service_field::legacy_resolution_services::*;
use self::resource_data::naming_authority_pointer::service_field::s_naptr::*;
use self::resource_data::ssh_fingerprint::*;
use self::resource_data::start_of_authority::*;
use self::response_parsing::*;
use self::response_parsing::resource_record_visitors::*;
use self::support::*;


pub(crate) mod caching;


/// TXT-like character strings.
pub mod character_strings;


/// Errors.
pub mod errors;


pub(crate) mod extended_dns;


/// Message.
pub mod message;


/// Name.
pub mod name;


/// Arbitrary sized data that can exist in records.
pub mod owned_or_parsed;


pub(crate) mod query_processors;


/// Various structures for organising collections of records.
pub mod records;


/// Resource data.
pub mod resource_data;


/// Response parsing.
pub mod response_parsing;


pub(crate) mod support;


include!("Cache.rs");
include!("MessageIdentifier.rs");
include!("Query.rs");
include!("RecentMessageIdentifiers.rs");
include!("SerialNumber.rs");
include!("TimeInSeconds.rs");
