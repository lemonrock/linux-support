// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// C string fragments.
///
/// Must not include a trailing NULL.
///
/// Used by `NulTerminatedCStringArray::new()`.
pub trait CStringFragments
{
	/// Specialized iteration as general iterators are too difficult to use with the various lifetimes and variable size of collections of fragments (which would require heap allocation).
	fn iterate(self, provide_fragment: &mut impl FnMut(&[u8]) -> ());
}
