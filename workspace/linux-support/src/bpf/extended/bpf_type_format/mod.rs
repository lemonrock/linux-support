// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use super::instructions::*;
use super::instructions::offset::program_counter::*;
use super::programs::*;
use self::reflection::*;
use crate::file_descriptors::bpf::BpfTypeFormatFileDescriptor;


/// A type system inspired by the `type_info` crate to use for runtime-reflection of Rust types.
#[macro_use]
pub mod reflection;


include!("BpfTypeFormatDataArray.rs");
include!("BpfTypeFormatKeyValueTypeIdentifiers.rs");
include!("BpfTypeFormatKind.rs");
include!("BpfTypeFormatProgramDetails.rs");
include!("BpfTypeFormatError.rs");
include!("BpfTypeFormatTypeIdentifier.rs");
include!("BpfTypeFormatTypeIdentifiers.rs");
include!("BpfTypeFormatInformationParser.rs");
include!("BpfTypeFormatIntegerEncoding.rs");
include!("NonVoidBpfTypeFormatTypeIdentifier.rs");
include!("ParsedBpfTypeFormatData.rs");
include!("ParsedBpfTypeFormatMapData.rs");
include!("StringTable.rs");
include!("VMLinuxValueBpfTypeFormatTypeIdentifier.rs");
