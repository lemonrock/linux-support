// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Asserts that the effective user id (`uid`) is root.
///
/// Takes a necessity to explain why the user must be root.
#[inline(always)]
pub fn assert_effective_user_id_is_root(necessity: &str)
{
	assert_eq!(UserIdentifier::current_effective(), UserIdentifier::root, "Effective User Id (euid) is not root (0). Necessity: {}", necessity);
}
