// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Write synchronization.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum WriteSynchronization
{
	/// Do no synchronization.
	DoNone,

	/// Equivalent to calling `Synchronize::synchronize_data_only()` after every `write()`.
	DataOnly,

	/// Equivalent to calling `Synchronize::synchronize_data_and_metadata()` after every `write()`.
	MetaDataAndData,
}
