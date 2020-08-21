// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use self::codes::*;
use self::c::*;
use crate::memory::VirtualAddress;
use crate::process::ProcessIdentifier;
use crate::process::status::StatusStatisticParseError;
use crate::syscall::UnconstrainedSystemCallNumber;
use crate::time::ClockTicks;
use crate::user_and_groups::UserIdentifier;
use crate::process_control::{process_control_wrapper2, result_must_be_zero, error_number_to_io_error};


/// `si_code` ranges of values.
pub mod codes;


/// System call and libc wrapping of system call specific details.
pub mod c;


include!("AuditArchitecture.rs");
include!("ChildStatus.rs");
include!("ElfMachine.rs");
include!("FaultCode.rs");
include!("one_millisecond_timed_wait_for_signals.rs");
include!("OutOfRangeSignalNumberError.rs");
include!("reset_all_signal_handlers_to_default.rs");
include!("ParsedSignal.rs");
include!("Signal.rs");
include!("SignalQueueStatus.rs");
include!("Signals.rs");
include!("TimedSignalWait.rs");
