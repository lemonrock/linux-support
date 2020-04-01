// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Read, write or read and write?
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum FanotifyReadOrWrite
{
	/// Only read.
	Read = O_RDONLY as u32 | O_CLOEXEC as u32 | O_NONBLOCK as u32,

	/// Only write.
	Write = O_WRONLY as u32 | O_CLOEXEC as u32 | O_NONBLOCK as u32,

	/// Read and write.
	ReadAndWrite = O_RDWR as u32 | O_CLOEXEC as u32 | O_NONBLOCK as u32,
}
