// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use self::c::*;
use crate::paths::*;
use crate::strings::Radix;
use crate::strings::parse_number::*;
use bitflags::bitflags;
use libc::ino_t;
use likely::*;
use std::convert::TryInto;
use std::fmt::Debug;
use std::mem::size_of;
#[allow(deprecated)] use std::mem::uninitialized;
use std::num::NonZeroU32;
use std::ops::Deref;
use std::ops::RangeInclusive;
use std::slice::from_raw_parts;


/// C data structures.
pub mod c;


include!("FileAttributesSubset.rs");
include!("FileExtentFlags.rs");
include!("FileExtents.rs");
include!("Inode.rs");
include!("InodeFlags.rs");
include!("InodeGenerationNumber.rs");
include!("NumberOfInodes.rs");
include!("RetrieveFileExtentsFlags.rs");
