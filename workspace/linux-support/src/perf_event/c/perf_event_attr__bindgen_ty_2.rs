// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Wake up events or watermark.
#[repr(C)]
#[derive(Copy, Clone)]
pub union perf_event_attr__bindgen_ty_2
{
	/// Wake up events.
	pub wakeup_events: u32,
	
	/// Wake up watermark.
	pub wakeup_watermark: u32,
}

impl Default for perf_event_attr__bindgen_ty_2
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for perf_event_attr__bindgen_ty_2
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "perf_event_attr__bindgen_ty_2 {{ union }}")
	}
}

impl PartialEq for perf_event_attr__bindgen_ty_2
{
	#[inline(always)]
	fn eq(&self, rhs: &Self) -> bool
	{
		unsafe { self.wakeup_watermark == rhs.wakeup_watermark }
	}
}

impl Eq for perf_event_attr__bindgen_ty_2
{
}

impl PartialOrd for perf_event_attr__bindgen_ty_2
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
	{
		unsafe { self.wakeup_watermark.partial_cmp(&rhs.wakeup_watermark) }
	}
}

impl Ord for perf_event_attr__bindgen_ty_2
{
	#[inline(always)]
	fn cmp(&self, rhs: &Self) -> Ordering
	{
		unsafe { self.wakeup_watermark.cmp(&rhs.wakeup_watermark) }
	}
}

impl Hash for perf_event_attr__bindgen_ty_2
{
	#[inline(always)]
	fn hash<H>(&self, state: &mut H) where H: Hasher
	{
		unsafe { self.wakeup_watermark.hash(state) }
	}
}
