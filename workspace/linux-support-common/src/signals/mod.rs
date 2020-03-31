// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use crate::bit_set::*;
use errno::errno;
use libc::*;
use libc_extra::unix::string::strsignal;
use likely::unlikely;
use serde::Deserialize;
use serde::Serialize;
use strum_macros::EnumIter;
use std::borrow::Cow;
use std::ffi::CStr;
use std::mem::transmute;
#[allow(deprecated)] use std::mem::uninitialized;
use std::ptr::null_mut;


include!("BitSetSignal.rs");
include!("one_millisecond_timed_wait_for_signals.rs");
include!("Signal.rs");
include!("TimedSignalWait.rs");
