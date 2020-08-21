// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Sample period or frequency?
#[repr(C)]
#[derive(Copy, Clone)]
pub union perf_event_attr__bindgen_ty_1
{
	/// Sample period.
	pub sample_period: u64,
	
	/// Sample frequency.
	pub sample_freq: u64,
}

impl Default for perf_event_attr__bindgen_ty_1
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for perf_event_attr__bindgen_ty_1
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "perf_event_attr__bindgen_ty_1 {{ union }}")
	}
}

impl PartialEq for perf_event_attr__bindgen_ty_1
{
	#[inline(always)]
	fn eq(&self, rhs: &Self) -> bool
	{
		unsafe { self.sample_freq == rhs.sample_freq }
	}
}

impl Eq for perf_event_attr__bindgen_ty_1
{
}

impl PartialOrd for perf_event_attr__bindgen_ty_1
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
	{
		unsafe { self.sample_freq.partial_cmp(&rhs.sample_freq) }
	}
}

impl Ord for perf_event_attr__bindgen_ty_1
{
	#[inline(always)]
	fn cmp(&self, rhs: &Self) -> Ordering
	{
		unsafe { self.sample_freq.cmp(&rhs.sample_freq) }
	}
}

impl Hash for perf_event_attr__bindgen_ty_1
{
	#[inline(always)]
	fn hash<H>(&self, state: &mut H) where H: Hasher
	{
		unsafe { self.sample_freq.hash(state) }
	}
}
