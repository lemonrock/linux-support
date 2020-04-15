// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) struct open_how
{
	/// `O_*` flags.
	pub(super) flags: u64,

	/// Mode for either `O_CREAT` or `O_TMPFILE`, otherwise zero.
	pub(super) mode: u64,

	/// `RESOLVE_*` flags.
	pub(super) resolve: u64,
}
