// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use super::instructions::*;
use super::instructions::offset::program_counter::*;
use super::programs::*;
use self::reflection::*;
use crate::file_descriptors::bpf::BtfFileDescriptor;


/// A type system inspired by the `type_info` crate to use for runtime-reflection of Rust types.
#[macro_use]
pub mod reflection;


include!("BtfDataArray.rs");
include!("BtfKind.rs");
include!("BtfProgramDetails.rs");
include!("BtfTypeError.rs");
include!("BtfTypeIdentifier.rs");
include!("BtfTypeIdentifiers.rs");
include!("BtfTypeInformationParser.rs");
include!("BtfTypeIntegerEncoding.rs");
include!("ParsedBtfData.rs");
include!("StringTable.rs");
