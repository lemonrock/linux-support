// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Group identifiers (GIDs).
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProcessGroupIdentifiers
{
	/// Real group identifier (GID).
	pub real: gid_t,

	/// Effective group identifier (GID).
	pub effective: gid_t,

	/// Saved set group identifier (GID).
	pub saved_set: gid_t,

	/// File system group identifier (GID).
	pub file_system: gid_t,
}