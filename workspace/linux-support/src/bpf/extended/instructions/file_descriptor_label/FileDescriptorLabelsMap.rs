// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Resolves the value of items of type `FileDescriptor`.
#[derive(Debug)]
pub struct FileDescriptorLabelsMap<'file_descriptor, FD: FileDescriptor>(UsageHashMap<&'file_descriptor FD>);

impl<'file_descriptor, FD: FileDescriptor> Default for FileDescriptorLabelsMap<'file_descriptor, FD>
{
	#[inline(always)]
	fn default() -> Self
	{
		use self::ProgramError::*;
		
		Self(UsageHashMap::new(CouldNotResolveFileDescriptorLabel, NotAllFileDescriptorLabelsHaveBeenResolved))
	}
}

impl<'file_descriptor, FD: FileDescriptor> FileDescriptorLabelsMap<'file_descriptor, FD>
{
	#[inline(always)]
	pub(crate) fn resolve<'name>(&self, file_descriptor_label: &FileDescriptorLabel<'name>) -> Result<RawFd, ProgramError>
	{
		self.0.resolve(file_descriptor_label.deref()).map(|file_descriptor| file_descriptor.as_raw_fd())
	}
	
	#[inline(always)]
	pub(crate) fn guard_all_values_have_been_resolved_at_least_once(self) -> Result<(), ProgramError>
	{
		self.0.guard_all_values_have_been_resolved_at_least_once()
	}
}
