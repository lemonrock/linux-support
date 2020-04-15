// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Read access time updating.
///
/// Default is to update.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum ReadAccessTimeUpdating
{
	/// Default.
	Update = 0,

	/// Reads do not update file access time.
	///
	/// This can be used only if one of the following conditions is true:-
	///
	/// *  The effective UID of the process matches the owner UID of the file.
	/// *  The calling process has the `CAP_FOWNER` capability in its user namespace and the owner UID of the file has a mapping in the namespace.
	///
	/// Not supported for NFS.
	///
	/// Since Linux 2.6.8.
	DoNotUpdate = O_NOATIME,
}

impl Default for ReadAccessTimeUpdating
{
	#[inline(always)]
	fn default() -> Self
	{
		ReadAccessTimeUpdating::Update
	}
}
