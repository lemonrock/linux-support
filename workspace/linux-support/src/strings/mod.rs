// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use crate::memory::PageSize;
use crate::strings::parse_number::ParseNumber;
use crate::strings::parse_number::ParseNumberError;
use self::to_number::NumberAsBytes;


include!("c_string_pointer_to_path_buf.rs");
include!("ConstCStr.rs");
include!("CStringExt.rs");
include!("CStringFragments.rs");
include!("FromBytes.rs");
include!("LinuxStringEscapeSequence.rs");
include!("NonNumericDigitCase.rs");
include!("NulTerminatedCStringArray.rs");
include!("OsStrExtMore.rs");
include!("parse_ascii_nul_string_values.rs");
include!("path_to_cstring.rs");
include!("Radix.rs");
include!("replace.rs");
include!("to_c_string_robustly.rs");
include!("without_suffix.rs");


/// Into a line-feed terminated byte-string.
pub mod into_line_feed_terminated_byte_string;


/// Conversions to numbers.
pub mod to_number;


/// Conversions from numbers.
pub mod parse_number;
