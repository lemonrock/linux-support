// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Resolves the value of items of type `MapFileDescriptor`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapFileDescriptorLabelsMap<'map_file_descriptor>(UsageHashMap<&'map_file_descriptor MapFileDescriptor>);

impl<'map_file_descriptor> Default for MapFileDescriptorLabelsMap<'map_file_descriptor>
{
	#[inline(always)]
	fn default() -> Self
	{
		use self::ProgramError::*;
		
		Self(UsageHashMap::new(CouldNotResolveMapFileDescriptorLabel, NotAllMapFileDescriptorLabelsHaveBeenResolved))
	}
}

impl<'map_file_descriptor> MapFileDescriptorLabelsMap<'map_file_descriptor>
{
	#[inline(always)]
	pub(crate) fn resolve<'de>(&self, map_file_descriptor_label: &MapFileDescriptorLabel) -> Result<RawFd, ProgramError>
	{
		self.0.resolve(map_file_descriptor_label.deref()).map(|map_file_descriptor| map_file_descriptor.as_raw_fd())
	}
	
	#[inline(always)]
	pub(crate) fn guard_all_values_have_been_resolved_at_least_once(self) -> Result<(), ProgramError>
	{
		self.0.guard_all_values_have_been_resolved_at_least_once()
	}
}
