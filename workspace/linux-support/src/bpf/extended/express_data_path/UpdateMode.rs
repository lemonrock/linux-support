// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// How to update an attaching program.
///
/// The default is `CreateOrUpdate`.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum UpdateMode<'a>
{
	/// Do anything.
	///
	/// This is the default.
	CreateOrUpdate,
	
	/// Create a new program, fail if an existing one is already attached.
	Create,
	
	/// Update only if the previous program matches.
	Update(&'a ExtendedBpfProgramFileDescriptor),
}

impl<'a> Default for UpdateMode<'a>
{
	#[inline(always)]
	fn default() -> Self
	{
		UpdateMode::CreateOrUpdate
	}
}
