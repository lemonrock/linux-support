// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Configured hash settings.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct ConfiguredHashSettings
{
	/// Hash function in use.
	pub(crate) function: Option<ETH_RSS_HASH>,
	
	/// Hash indirection table (RETA).
	///
	/// Uses the value produced by the hash `function` with the `key` as an index into this table to find a `QueueIdentifier`.
	pub(crate) indirection_table: Option<IndirectionTable>,

	/// Key used by the hash `function`.
	pub(crate) key: Option<HashFunctionKey>,
}

impl ConfiguredHashSettings
{
	pub(crate) const Unsupported: Self = Self
	{
		function: None,
		indirection_table: None,
		key: None,
	};
}
