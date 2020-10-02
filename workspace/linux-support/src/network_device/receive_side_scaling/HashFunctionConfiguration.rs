// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Settings for a RETA table.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct HashFunctionConfiguration
{
	/// Hash function in use.
	pub(crate) function: Option<HashFunctionName>,
	
	/// Hash indirection table (RETA).
	///
	/// Uses the value produced by the hash `function` with the `key` as an index into this table to find a `QueueIdentifier`.
	pub(crate) indirection_table: Option<IndirectionTable>,

	/// Seed used by the hash `function`.
	///
	/// Called a 'key' in Receive Side Scaling (RSS) literature, but this is confusing; the key is actually the set of fields in the incoming data packet that are hashed.
	pub(crate) seed: Option<HashFunctionSeed>,
}

impl HashFunctionConfiguration
{
	pub(crate) const Unsupported: Self = Self
	{
		function: None,
		indirection_table: None,
		seed: None,
	};
}
