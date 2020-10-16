// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use super::query::QueryType;
use super::super::resource_data::*;
use super::super::resource_data::certificate::*;
use super::super::resource_data::certification_authority_authorization::*;
use super::super::resource_data::dhcid::*;
use super::super::resource_data::dns_based_authentication_of_named_entities::*;
use super::super::resource_data::dnssec::*;
use super::super::resource_data::host_identity_protocol::*;
use super::super::resource_data::identifier_locator_network_protocol::*;
use super::super::resource_data::ipsec::*;
use super::super::resource_data::location::*;
use super::super::resource_data::naming_authority_pointer::*;
use super::super::resource_data::ssh_fingerprint::*;
use super::super::resource_data::start_of_authority::*;


include!("guard_hash_digest_if_final_field.rs");
include!("ipsec_like_public_key.rs");
include!("guard_delegation_signer.rs");
include!("guard_dns_key.rs");


include!("ResourceRecord.rs");
include!("ResourceRecordClass.rs");
include!("ResourceRecordFooter.rs");
