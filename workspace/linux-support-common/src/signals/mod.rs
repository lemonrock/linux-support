// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use errno::errno;
use libc::*;
use std::collections::HashSet;
use std::mem::size_of;
#[allow(deprecated)] use std::mem::uninitialized;
use std::ptr::null_mut;


include!("block_all_signals_on_current_thread.rs");
include!("block_all_signals_on_current_thread_bar.rs");
include!("hash_set_to_signal_set.rs");
include!("one_millisecond_timed_wait_for_signals.rs");
include!("SignalBitSet.rs");
include!("SignalNumber.rs");
include!("TimedSignalWait.rs");
