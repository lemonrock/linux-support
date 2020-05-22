// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use crate::paths::*;
use crate::process::ProcessIdentifierChoice;
use crate::process::status::StatusStatisticParseError;
use crate::strings::FromBytes;
use crate::strings::into_line_feed_terminated_byte_string::UnpaddedDecimalInteger;
use crate::strings::parse_number::*;
use errno::errno;
use likely::*;
use libc::*;
use memchr::memchr_iter;
use memchr::Memchr;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeSet;
use std::collections::HashSet;
use std::error;
use std::ffi::*;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;
use std::io;
#[allow(deprecated)] use std::mem::uninitialized;
use std::ops::Deref;
use std::os::unix::ffi::OsStringExt;
use std::path::PathBuf;
use std::ptr::null;


include!("assert_effective_user_id_is_root.rs");
include!("EtcGroup.rs");
include!("EtcGroupIterator.rs");
include!("EtcGroupParseError.rs");
include!("EtcGroupRecord.rs");
include!("EtcPasswd.rs");
include!("EtcPasswdIterator.rs");
include!("EtcPasswdParseError.rs");
include!("EtcPasswdRecord.rs");
include!("GroupIdentifier.rs");
include!("GroupIdentifiers.rs");
include!("GroupName.rs");
include!("Groups.rs");
include!("SupplementaryGroupChoice.rs");
include!("SupplementaryGroupSetting.rs");
include!("UserAndGroupChoice.rs");
include!("UserAndGroupChoiceError.rs");
include!("UserAndGroupSettings.rs");
include!("UserName.rs");
include!("UserOrGroupIdentifier.rs");
include!("UserIdentifier.rs");
include!("UserIdentifiers.rs");
