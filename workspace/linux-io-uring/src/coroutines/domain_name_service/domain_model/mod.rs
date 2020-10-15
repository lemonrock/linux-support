// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use self::DnsProtocolError::*;
use self::message::*;
use self::name::*;
use self::support::*;


pub(crate) mod character_strings;


pub(crate) mod extended_dns;


pub(crate) mod message;


pub(crate) mod name;


pub(crate) mod resource_data;


pub(crate) mod response_parsing;


pub(crate) mod support;


include!("DnsProtocolError.rs");
include!("MessageIdentifier.rs");
include!("Query.rs");
include!("SerialNumber.rs");
include!("TimeInSeconds.rs");
include!("TimeToLiveInSeconds.rs");
