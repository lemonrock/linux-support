// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


struct CanonicalNameChain<'message>
{
	query_name: &'message WithCompressionParsedName<'message>,
	chain: ArrayVec<[WithCompressionParsedName<'message>; Self::MaximumChainLength]>,
}

impl<'message> CanonicalNameChain<'message>
{
	// BINDS allows 16.
	// Resolving `www.microsoft.com` requires 3.
	const MaximumChainLength: usize = 6;

	#[inline(always)]
	fn validate_authority_section_name(&self, start_of_authority_name: &WithCompressionParsedName<'message>) -> bool
	{
		let chain_length = self.chain.len();
		let most_canonical_name = if unlikely!(chain_length == 0)
		{
			self.query_name
		}
		else
		{
			unsafe { self.chain.get_unchecked(chain_length - 1) }
		};

		let parent = match most_canonical_name.parent()
		{
			None => false,
			Some(parent) => parent,
		};

		parent.ends_with(start_of_authority_name)
	}
}
