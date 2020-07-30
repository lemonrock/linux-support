// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Resolves the value of items of type `FileDescriptor`.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct FileDescriptorsMap<FD: FileDescriptor>(HashMap<ObjectName16, Rc<FD>>);

impl<FD: FileDescriptor> FileDescriptorsMap<FD>
{
	/// New instance.
	#[inline(always)]
	pub fn with_capacity(capacity: usize) -> Self
	{
		Self(HashMap::with_capacity(capacity))
	}
	
	#[inline(always)]
	pub(crate) fn add(&mut self, program_name_or_map_name: ObjectName16, file_descriptor: FD) -> Result<Rc<FD>, FileDescriptorsMapAddError>
	{
		use std::collections::hash_map::Entry::*;
		
		match self.0.entry(program_name_or_map_name)
		{
			Vacant(vacant) => Ok(Rc::clone(vacant.insert(Rc::new(file_descriptor)))),
			Occupied(occupied) => Err(FileDescriptorsMapAddError::AlreadyAddedFileDescriptor(occupied.key().clone())),
		}
	}
	
	#[inline(always)]
	pub(crate) fn resolve<'name>(&self, program_name_or_map_name: &ObjectName16) -> Result<RawFd, FileDescriptorsMapResolveError>
	{
		match self.0.get(program_name_or_map_name)
		{
			Some(file_descriptor) => Ok(file_descriptor.as_raw_fd()),
			None => Err(FileDescriptorsMapResolveError::MissingFileDescriptor(program_name_or_map_name.clone())),
		}
	}
}
