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
		
		/// Raising write protect events and giving a thread identifier.
		const AllPageFaultEventFeaturesWithoutThreadIdentifier = Feature::RaisePageFaultWriteProtectEvents as u64;
		
		/// Raising write protect events and giving a thread identifier.
		const AllPageFaultEventFeaturesWithThreadIdentifier = Self::AllPageFaultEventFeaturesWithoutThreadIdentifier.bits | (Feature::ThreadIdentifier as u64);
		
		/// These features are effectively incompatible with single-threaded use, as they raise events that do not block the raising thread.
		const FeaturesIncompatibleWithSingleThreadedUse = (Feature::RaiseForkEvents as u64) | (Feature::RaiseRemapEvents as u64) | (Feature::RaiseMAdviseDoNotNeedOrRemoveEvents as u64) | (Feature::RaiseUnmapEvents as u64) | (Feature::DoNotRaisePageFaultEventsButRaiseSIGBUSSignalInstead as u64);
	}
}

impl Features
{
	#[inline(always)]
	fn does_not_have_requested_features_incompatible_with_single_threaded_blocking_use(self) -> bool
	{
		!self.intersects(Features::FeaturesIncompatibleWithSingleThreadedUse)
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
	
	/// Do not request a feature.
	#[inline(always)]
	pub fn do_not_request_feature(&mut self, feature: Feature)
	{
		self.remove(unsafe { transmute(feature) })
	}
}
