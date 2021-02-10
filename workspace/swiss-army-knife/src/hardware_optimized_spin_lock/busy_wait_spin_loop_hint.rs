// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// On `x86_64`, issues a `REP NOP` which cause the CPU to potentially yield to another `HyperThread`.
///
/// Internally inlines calls to Rust's `std::hint::spin_loop()` but deals with the fact that this is unstable, but its former usage via `std::sync::atomic_spin_loop_hint()` is deprecated!
#[inline(always)]
#[allow(deprecated)]
pub fn busy_wait_spin_loop_hint()
{
	std::sync::atomic::spin_loop_hint()
}
