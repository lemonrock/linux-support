// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(super) const SCHED_FLAG_RESET_ON_FORK: u64 = 0x01;

pub(super) const SCHED_FLAG_RECLAIM: u64 = 0x02;

pub(super) const SCHED_FLAG_DL_OVERRUN: u64 = 0x04;

pub(super) const SCHED_FLAG_KEEP_POLICY: u64 = 0x08;

pub(super) const SCHED_FLAG_KEEP_PARAMS: u64 = 0x10;

pub(super) const SCHED_FLAG_UTIL_CLAMP_MIN: u64 = 0x20;

pub(super) const SCHED_FLAG_UTIL_CLAMP_MAX: u64 = 0x40;

pub(super) const SCHED_FLAG_KEEP_ALL: u64 = SCHED_FLAG_KEEP_POLICY | SCHED_FLAG_KEEP_PARAMS;

pub(super) const SCHED_FLAG_UTIL_CLAMP: u64 = SCHED_FLAG_UTIL_CLAMP_MIN | SCHED_FLAG_UTIL_CLAMP_MAX;

pub(super) const SCHED_FLAG_ALL: u64 = SCHED_FLAG_RESET_ON_FORK | SCHED_FLAG_RECLAIM | SCHED_FLAG_DL_OVERRUN | SCHED_FLAG_KEEP_ALL | SCHED_FLAG_UTIL_CLAMP;
