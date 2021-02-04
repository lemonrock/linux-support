// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Feature flags.
	#[derive(Deserialize, Serialize)]
	#[serde(deny_unknown_fields)]
	pub struct Features: u64
	{
		/// Temporary-file backed memory events.
		const SupportHugetlbfsAndSharedMemoryEvents = (Feature::SupportEventsOnHugetlbfs as u64) | (Feature::SupportEventsOnSharedMemory as u64);
		
		/// Typically all possible features for this version of Linux.
		const ApplicationProgrammerInterface = UFFD_API_FEATURES;
	}
}

impl Features
{
	/// Request all features except raising fork events.
	#[inline(always)]
	pub fn request_all_page_fault_event_features() -> Self
	{
		use self::Feature::*;
		
		let mut features = Self::empty();
		features.request_feature(RaisePageFaultWriteProtectEvents);
		features.request_feature(ThreadIdentifier);
		features
	}
	
	/// Request all features except raising fork events.
	#[inline(always)]
	pub fn request_all_features_except_raising_fork_events_and_sigbus_signalling() -> Self
	{
		use self::Feature::*;
		
		let mut all = Self::ApplicationProgrammerInterface;
		all.do_not_request_feature(RaiseForkEvents);
		all.do_not_request_feature(DoNotRaisePageFaultEventsButRaiseSIGBUSSignalInstead);
		all
	}
	
	/// Supports a feature?
	#[inline(always)]
	pub fn supports(self, feature: Feature) -> bool
	{
		self.contains(unsafe { transmute(feature) })
	}
	
	/// Request a feature.
	#[inline(always)]
	pub fn request_feature(&mut self, feature: Feature)
	{
		self.insert(unsafe { transmute(feature) })
	}
	
	/// Do not equest a feature.
	#[inline(always)]
	pub fn do_not_request_feature(&mut self, feature: Feature)
	{
		self.remove(unsafe { transmute(feature) })
	}
}
