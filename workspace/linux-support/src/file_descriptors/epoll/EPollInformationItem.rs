// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// EPoll information item from `/proc`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EPollInformationItem
{
	/// File descriptor.
	pub target_file_descriptor: RawFd,

	/// Combination of event flags registered for this item.
	pub event_flags: u32,

	/// Token registered (`epoll_data_t`).
	pub token: u64,

	/// ? used for checkpoint-restore (CRIU).
	pub position: i64,

	/// ? used for checkpoint-restore (CRIU).
	pub inode: isize,

	/// ? used for checkpoint-restore (CRIU).
	pub sdevice: u32,
}
