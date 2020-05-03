// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A simple terminate that does no logging of errors.
#[derive(Debug)]
pub struct SimpleTerminate(AtomicU64);

impl Terminate for SimpleTerminate
{
	/// Begin termination.
	#[inline(always)]
	fn begin_termination(&self)
	{
		self.0.compare_and_swap(Self::Continue, Self::NormalTerminate, AcqRel);
	}

	#[inline(always)]
	fn begin_termination_due_to_panic(&self, _panic_info: &PanicInfo)
	{
		self.0.compare_and_swap(Self::Continue, Self::PanicTerminate, AcqRel);
	}

	#[inline(always)]
	fn begin_termination_due_to_irrecoverable_error(&self, _irrecoverable_error: &(dyn Any + Send))
	{
		self.0.compare_and_swap(Self::Continue, Self::PanicTerminate, AcqRel);
	}

	#[inline(always)]
	fn should_finish(&self) -> bool
	{
		self.0.load(Acquire) != Self::Continue
	}

	#[inline(always)]
	fn terminated_due_to_panic_or_irrecoverable_error(&self) -> bool
	{
		let value = self.0.load(Acquire);
		debug_assert_ne!(value, Self::Continue, "should not be checking this unless `should_finish()` was `true`");
		value == Self::PanicTerminate
	}
}

impl SimpleTerminate
{
	const Continue: u64 = 0x00;

	const NormalTerminate: u64 = 0x01;

	const PanicTerminate: u64 = 0x02;

	/// New instance.
	#[inline(always)]
	pub fn new() -> Arc<Self>
	{
		Arc::new(Self(AtomicU64::new(Self::Continue)))
	}
}
