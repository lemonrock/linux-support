// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Resolves the value of items of type `MapFileDescriptor`.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct MapFileDescriptorLabelsMap<'a>(HashMap<String, &'a MapFileDescriptor>);

impl<'a> MapFileDescriptorLabelsMap<'a>
{
	#[inline(always)]
	pub(crate) fn resolve<'de>(&self, map_file_descriptor_label: &MapFileDescriptorLabel) -> Result<RawFd, InstructionError>
	{
		match self.0.get(map_file_descriptor_label.deref())
		{
			None => Err(InstructionError::CouldNotResolveMapFileDescriptorLabel),
			Some(map_file_descriptor) => Ok(map_file_descriptor.as_raw_fd())
		}
	}
}
