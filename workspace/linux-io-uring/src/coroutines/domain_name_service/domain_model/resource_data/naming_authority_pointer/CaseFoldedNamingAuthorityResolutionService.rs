// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Validated, case-folded string that:-
///
/// * Can not be empty;
/// * Has a first ASCII byte of `a` to `z` inclusive.
/// * If 2 or more bytes long, has second and subsequent ASCII bytes of `a` to `z` inclusive or `0` to `9` inclusive.
/// * Is a maximum of 32 bytes.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct CaseFoldedNamingAuthorityResolutionService(Box<[u8]>);

impl Deref for CaseFoldedNamingAuthorityResolutionService
{
	type Target = [u8];
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.0.deref()
	}
}
