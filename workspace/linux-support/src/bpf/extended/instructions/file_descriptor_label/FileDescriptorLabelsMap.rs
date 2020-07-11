// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Resolves the value of items of type `FileDescriptor`.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct FileDescriptorLabelsMap<FD: FileDescriptor>(HashMap<FD>);

impl<FD: FileDescriptor> FileDescriptorLabelsMap<FD>
{
	#[inline(always)]
	pub(crate) fn add(&mut self, file_descriptor_label: FileDescriptorLabel, file_descriptor: FD) -> &FD
	{
		use std::collections::hash_map::Entry::*;
		match self.0.entry(file_descriptor_label.into())
		{
			Vacant(vacant) => vacant.insert(file_descriptor),
			Occupied(_) => panic!("Already added"),
		}
	}
	
	#[inline(always)]
	pub(crate) fn resolve<'name>(&self, file_descriptor_label: &FileDescriptorLabel<'name>) -> Result<RawFd, ProgramError>
	{
		self.0.resolve(file_descriptor_label.deref()).map(|file_descriptor| file_descriptor.as_raw_fd())
	}
}
