// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Used as a value in a BPF array map.
pub trait UsedAsValueInArrayMapDescriptor: FileDescriptor
{
	#[doc(hidden)]
	fn transmute_to_file_descriptor_copies(values: Vec<RawFd>) -> Vec<FileDescriptorCopy<Self>>;
	
	#[doc(hidden)]
	fn transmute_from_file_descriptor_copies(values: &[FileDescriptorCopy<Self>]) -> &[RawFd];
	
	#[doc(hidden)]
	fn transmute_to_file_descriptor_copy(value: RawFd) -> FileDescriptorCopy<Self>;
}
