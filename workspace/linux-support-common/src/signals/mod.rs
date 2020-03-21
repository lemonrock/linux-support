use errno::errno;
use libc::*;
use std::collections::HashSet;
#[allow(deprecated)] use std::mem::uninitialized;
use std::ptr::null_mut;


include!("block_all_signals_on_current_thread.rs");
include!("block_all_signals_on_current_thread_bar.rs");
include!("hash_set_to_signal_set.rs");
include!("one_millisecond_timed_wait_for_signals.rs");
include!("SignalNumber.rs");
include!("TimedSignalWait.rs");
