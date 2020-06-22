// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Tags.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DogStatsDTags(ArrayVec<[&'static DogStatsDTag; 16]>);

impl DogStatsDTags
{
	/// New instance.
	#[inline(always)]
	pub fn new() -> Self
	{
		Self(ArrayVec::new())
	}
	
	/// Push a tag.
	#[inline(always)]
	pub fn push(&mut self, tag: &'static DogStatsDTag)
	{
		self.0.push(tag)
	}
	
	/// Common tags.
	#[inline(always)]
	pub fn common_dog_stats_d_tags() -> DogStatsDTags
	{
		dog_stats_d_tags!
		[
			DogStatsDTag::environment(),
			DogStatsDTag::process_name(),
			DogStatsDTag::process_identifier(),
			DogStatsDTag::cargo_name(),
			DogStatsDTag::cargo_version(),
			DogStatsDTag::numa_node(),
			DogStatsDTag::thread_name(),
			DogStatsDTag::thread_identifier(),
			DogStatsDTag::hyper_thread()
		]
	}
}
