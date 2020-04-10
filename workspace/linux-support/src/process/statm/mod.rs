// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use crate::memory::NumberOfPages;
use crate::paths::ProcPath;
use crate::paths::PathExt;
use crate::process::ProcessIdentifierChoice;
use crate::strings::FromBytes;
use crate::strings::parse_number::ParseNumberError;
use likely::likely;
use std::error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Debug;
use std::io;
use std::num::NonZeroU8;


include!("StatM.rs");
include!("StatMParseError.rs");