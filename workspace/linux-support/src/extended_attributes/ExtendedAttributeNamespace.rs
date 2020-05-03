// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Extended attribute namespace.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ExtendedAttributeNamespace<'a>(pub &'a [u8]);

impl ExtendedAttributeNamespace<'static>
{
	/// Includes `security.capability` which is used to store file capabilities.
	pub const Security: Self = Self(b"security");

	#[allow(missing_docs)]
	pub const System: Self = Self(b"system");

	#[allow(missing_docs)]
	pub const Trusted: Self = Self(b"trusted");

	#[allow(missing_docs)]
	pub const User: Self = Self(b"user");
}
