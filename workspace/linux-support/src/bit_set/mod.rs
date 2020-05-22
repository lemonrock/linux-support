// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use crate::strings::*;
use crate::strings::parse_number::ParseNumber;
use crate::strings::parse_number::ParseNumberError;
use crate::strings::to_number::NumberAsBytes;
use likely::*;
use serde::Deserialize;
use serde::Serialize;
use std::alloc::AllocRef;
use std::alloc::Global;
use std::alloc::Layout;
#[cfg(all(target_arch = "x86_64", target_feature = "popcnt"))] use std::arch::x86_64::_mm_popcnt_u64;
use std::borrow::Cow;
use std::cmp::min;
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::error;
use std::ffi::OsStr;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;
use std::io;
use std::io::ErrorKind;
use std::marker::PhantomData;
use std::mem::size_of;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::Index;
use std::ops::IndexMut;
use std::os::unix::ffi::OsStrExt;
use crate::strings::into_line_feed_terminated_byte_string::*;


include!("bit_set.rs");include!("bit_set_aware.rs");
include!("BitSet.rs");
include!("BitSetAware.rs");
include!("BitSetAwareTryFromU16Error.rs");
include!("BitSetIncludingEmptyIterator.rs");
include!("BitSetIterator.rs");
include!("BitsInAByte.rs");
include!("IntoBitMask.rs");
include!("IntoList.rs");
include!("ListParseError.rs");
include!("PerBitSetAwareData.rs");
