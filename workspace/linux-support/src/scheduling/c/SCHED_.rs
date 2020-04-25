// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This constant name is used in the Linux kernel.
///
/// It is the same as `SCHED_OTHER`.
pub(super) const SCHED_NORMAL: u32 = 0;

/// This constant name is used in libc.
///
/// It is the same as `SCHED_NORMAL`.
#[allow(dead_code)]
pub(super) const SCHED_OTHER: u32 = SCHED_NORMAL;

pub(super) const SCHED_FIFO: u32 = 1;

pub(super) const SCHED_RR: u32 = 2;

pub(super) const SCHED_BATCH: u32 = 3;

/// Not implemented by Linux; value reserved but not defined.
#[allow(dead_code)]
pub(super) const SCHED_ISO: u32 = 4;

pub(super) const SCHED_IDLE: u32 = 5;

pub(super) const SCHED_DEADLINE: u32 = 6;
