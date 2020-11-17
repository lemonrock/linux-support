// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;


/// `CERT` record support.
pub mod certificate;


/// `CAA` record support.
pub mod certification_authority_authorization;


/// `DHCID` record support.
pub mod dhcid;


/// Digest support.
pub mod digest;


/// `TLSA` and `SMIME` record support.
pub mod dns_based_authentication_of_named_entities;


/// DNSSEC.
pub mod dnssec;


/// Host Identity Protocol (HIP) record support.
///
/// See RFC 8005.
pub mod host_identity_protocol;


/// Identifier-Locator Network Protocol (ILNP) record support.
///
/// See RFC 6742.
pub mod identifier_locator_network_protocol;


/// `IPSECKEY` (and potentially the obsolete `KEY`) record support.
pub mod ipsec;


/// `LOC` record support.
pub mod location;


/// `NAPTR` record support.
pub mod naming_authority_pointer;


/// `SSHFP` record support.
pub mod ssh_fingerprint;


/// `SOA` record support.
pub mod start_of_authority;


/// `TXT` record support.
pub mod text;


include!("HostInformation.rs");
include!("MailServerName.rs");
include!("NameServerName.rs");
include!("OpenPgpRfc4880TransferablePublicKey.rs");
include!("ResourceData.rs");
include!("ServiceLocation.rs");
