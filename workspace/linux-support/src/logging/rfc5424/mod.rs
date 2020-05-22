// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use super::system_information;
use crate::process::*;
use crate::strings::into_line_feed_terminated_byte_string::*;
use chrono::DateTime;
use chrono::SecondsFormat;
use chrono::Utc;
use lazy_static::lazy_static;
use likely::*;
use maplit::hashset;
use memchr::memchr3;
use std::borrow::Cow;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::mem::transmute;
use std::net::IpAddr;
use std::num::NonZeroU64;
use std::ops::Deref;


include!("ApplicationName.rs");
include!("HostName.rs");
include!("MessageIdentifier.rs");
include!("ParameterName.rs");
include!("PrintableAsciiCharacter.rs");
include!("PrintableAsciiCharacterPushError.rs");
include!("ProcessIdentifierOrName.rs");
include!("Rfc5424MessageTemplate.rs");
include!("StructuredDataName.rs");
include!("StructuredDataIdentifier.rs");
include!("UnescapedParameterValue.rs");
include!("Version.rs");
