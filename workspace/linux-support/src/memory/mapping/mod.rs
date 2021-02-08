// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use self::c::*;
use crate::file_descriptors::CreationError;
use crate::file_descriptors::MemoryMappableFileDescriptor;
use super::numa::SetMemoryPolicy;
use super::numa::SetMemoryPolicyStrictness;
use super::*;
use super::huge_pages::*;


mod c;


include!("AddressHint.rs");
include!("ExtendedProtection.rs");
include!("LockAllMemory.rs");
include!("MappedMemory.rs");
include!("MappedMemoryConfiguration.rs");
include!("MappedMemorySettings.rs");
include!("MappedMemorySubrange.rs");
include!("MemoryAdvice.rs");
include!("MemoryLockSettings.rs");
include!("MemoryMapError.rs");
include!("Protection.rs");
include!("RemapMemoryHint.rs");
include!("Sharing.rs");
include!("SynchronizeFlags.rs");
