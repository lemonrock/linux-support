// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NameAsLabelsIncludingRoot(Box<[Box<[u8]>]>);

impl NameAsLabelsIncludingRoot
{
	#[inline(always)]
	pub(crate) fn new_from_parsed_data<'message>(name: WithCompressionParsedName<'message>) -> Self
	{
		let mut labels = Vec::with_capacity(8);
		for label in name
		{
			labels.push(label.to_vec().into_boxed_slice())
		}
		Self(labels.into_boxed_slice())
	}
}
