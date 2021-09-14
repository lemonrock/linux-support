// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use self::number_as_decimal_string_formats::NumberAsDecimalStringFormat;
use super::*;
use crate::vec::VecExt;


/// Number as decimal string formats.
pub mod number_as_decimal_string_formats;


include!("ByteWritable.rs");
include!("NumberAsBytes.rs");
include!("NumberAsDecimalString.rs");
include!("UnsafePerformantByteWritable.rs");
