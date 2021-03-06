// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use crate::cpu::*;
use crate::file_descriptors::CreationError;
use crate::file_descriptors::process_identifier::ProcessIdentifierFileDescriptor;
use crate::file_descriptors::standard::StandardFileDescriptor;
use crate::memory::numa::*;
use crate::paths::*;
use crate::scheduling::RoundRobinInterval;
use crate::signals::Signal;
use crate::thread::ThreadIdentifier;
use crate::vectors::MaximumNumberOfBuffers;
use crate::vectors::VectoredRead;
use crate::vectors::VectoredWrite;
use crate::process_control::{process_control_wrapper2, result_must_be_zero};


pub(crate) mod c;


/// `/proc/<N>/stat`.
pub mod stat;


/// `/proc/<N>/statm`.
pub mod statm;


/// `/proc/<N>/status`.
pub mod status;


include!("CommandName.rs");
include!("daemonize.rs");
include!("get_program_invocation_short_name.rs");
include!("ProcessGroupIdentifier.rs");
include!("ProcessGroupIdentifierChoice.rs");
include!("ProcessIdentifier.rs");
include!("ProcessIdentifierChoice.rs");
include!("ProcessIdentifierVectoredRead.rs");
include!("ProcessIdentifierVectoredWrite.rs");
include!("ProcessName.rs");
include!("ProcessState.rs");
