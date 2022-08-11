// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Mirrors `libbpf_num_possible_cpus()` but with sanity checks.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NumberOfPossibleHyperThreads(NonZeroUsize);

impl NumberOfPossibleHyperThreads
{
	/// Creates a new instance; uses a static to cache the reading of data read from disk using `sys_path`.
	#[inline(always)]
	pub fn new(sys_path: &SysPath) -> Self
	{
		static mut NumberOfPossibleHyperThreads: usize = 0;
		
		let mut number_of_possible_hyper_threads = unsafe { NumberOfPossibleHyperThreads };
		if unlikely!(number_of_possible_hyper_threads == 0)
		{
			number_of_possible_hyper_threads = HyperThreads::number_of_possible_hyper_threads_unless_there_are_missing_possible_hyper_threads(sys_path).expect("Strange Linux system with `/sys/devices/system/cpu/possible` using a CPU mask with some HyperThreads not possible");
			assert_ne!(number_of_possible_hyper_threads, 0, "Number of possible HyperThreads is zero");
			assert!(number_of_possible_hyper_threads < HyperThread::LinuxExclusiveMaximum as usize);
			unsafe { NumberOfPossibleHyperThreads = number_of_possible_hyper_threads };
		}
		
		Self(new_non_zero_usize(number_of_possible_hyper_threads))
	}
	
	#[inline(always)]
	pub(crate) fn row_size_in_bytes<V: Copy>(self) -> usize
	{
		self.length() * size_of::<PerHyperThreadValue<V>>()
	}
	
	#[inline(always)]
	pub(crate) fn length(&self) -> usize
	{
		self.0.get()
	}
	
	/// New array of per-cpu values.
	#[inline(always)]
	pub fn initialized_per_hyper_thread_values<V: Copy>(self, initializer: &impl Fn(HyperThread) -> V) -> Vec<PerHyperThreadValue<V>>
	{
		let length = self.length();
		let mut values = Vec::with_capacity(length);
		for index in 0 .. length
		{
			values.push(PerHyperThreadValue(initializer(HyperThread::from_validated_u16(index as u16))))
		}
		values
	}
	
	/// Very dangerous.
	#[inline(always)]
	pub unsafe fn uninitialized_per_hyper_thread_values<V: Copy>(self) -> Vec<PerHyperThreadValue<V>>
	{
		let length = self.length();
		let mut values = Vec::with_capacity(length);
		values.set_len(length);
		values
	}
}
