// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


use super::*;
use self::syscall::*;
use libc::pthread_sigmask;
use libc::raise;
use libc::SIG_BLOCK;
use libc::SIGABRT;
use libc::SIGALRM;
use libc::SIGBUS;
use libc::SIGCHLD;
use libc::SIGCONT;
use libc::sigfillset;
use libc::SIGFPE;
use libc::SIGHUP;
use libc::SIGILL;
use libc::SIGINT;
use libc::SIGIO;
use libc::SIGKILL;
use libc::SIGPIPE;
use libc::SIGPROF;
use libc::SIGPWR;
use libc::SIGQUIT;
use libc::SIGSEGV;
use libc::SIGSTKFLT;
use libc::SIGSYS;
use libc::SIGTERM;
use libc::SIGTRAP;
use libc::SIGTSTP;
use libc::SIGTTIN;
use libc::SIGTTOU;
use libc::SIGURG;
use libc::SIGUSR1;
use libc::SIGUSR2;
use libc::SIGVTALRM;
use libc::SIGWINCH;
use libc::SIGXCPU;
use libc::SIGXFSZ;
use std::process::abort;
use std::process::exit;
use crate::process::{ProcessIdentifier, UserIdentifier};
use crate::process::stat::ClockTicks;


/// System call and libc wrapping of system call specific details.
pub mod syscall;


include!("ArithmeticErrorCode.rs");
include!("BusCode.rs");
include!("BusFaultData.rs");
include!("ChildCode.rs");
include!("ChildData.rs");
include!("Code.rs");
include!("EmulatorTrapCode.rs");
include!("FaultData.rs");
include!("GenericSignalData.rs");
include!("IllegalInstructionCode.rs");
include!("PollCode.rs");
include!("PollData.rs");
include!("SegmentationFaultCode.rs");
include!("SignalFileDescriptor.rs");
include!("SignalHandler.rs");
include!("SpecificSignalData.rs");
include!("SystemCallCode.rs");
include!("SystemCallData.rs");
include!("TrapCode.rs");
include!("UserspaceSignalCode.rs");
