// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This structure allows a value to be sent from one thread to another, once, without the use of Arc and Once, and irrespective of whether it is Send and Sync or not.
///
/// Note that Arc requires values sent between threads to be Send and Sync; ThreadConfigurationError can not be Send and Sync.
pub(super) struct UnsafeValueExchanger<V>
{
	lock: AtomicBool,
	value: UnsafeCell<ManuallyDrop<V>>,
	#[cfg(debug_assertions)] has_been_split: Cell<bool>,
}

impl<V> UnsafeValueExchanger<V>
{
	const BarrierNotYetReached: bool = false;
	
	#[inline(always)]
	pub(super) fn new() -> Self
	{
		Self
		{
			lock: AtomicBool::new(UnsafeValueExchanger::<V>::BarrierNotYetReached),
			value: UnsafeCell::new(ManuallyDrop::new(unsafe_uninitialized())),
			#[cfg(debug_assertions)] has_been_split: Cell::new(false),
		}
	}
	
	/// Must only be called once from the thread that is going to wait.
	#[inline(always)]
	pub(super) fn split(&self) -> (UnsafeValueExchangerReleaser<V>, UnsafeValueExchangerWaiter<V>)
	{
		#[cfg(debug_assertions)]
		{
			debug_assert!(!self.has_been_split());
			self.has_been_split.set(true);
		}
		
		(
			UnsafeValueExchangerReleaser
			{
				parent: self as *const Self as usize,
				marker: PhantomData,
				waiting_thread: current(),
			},
			UnsafeValueExchangerWaiter(self),
		)
	}
	
	#[inline(always)]
	fn store_value(&self, value: V)
	{
		self.debug_assert_has_been_split();
		debug_assert!(self.lock_is_not_yet_reached());
		
		unsafe { write(self.value.get(), ManuallyDrop::new(value)) };
		self.lock.store(true, Release);
	}
	
	#[inline(always)]
	fn take_value(&self) -> V
	{
		debug_assert!(!self.lock_is_not_yet_reached());
		
		let manually_drop = unsafe { &mut * self.value.get() };
		
		unsafe { ManuallyDrop::take(manually_drop) }
	}
	
	#[inline(always)]
	fn lock_is_not_yet_reached(&self) -> bool
	{
		self.lock.load(Acquire) == UnsafeValueExchanger::<V>::BarrierNotYetReached
	}
	
	#[inline(always)]
	fn debug_assert_has_been_split(&self)
	{
		#[cfg(debug_assertions)]
		{
			debug_assert!(self.has_been_split())
		}
	}
	
	#[inline(always)]
	#[cfg(debug_assertions)]
	fn has_been_split(&self) -> bool
	{
		self.has_been_split.get()
	}
}
