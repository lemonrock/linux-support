// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Atomically-allocated for tags not known statically but which are often long-lived.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AdditionalDogStatsDTags<CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>>(ArrayVec<AdditionalDogStatsDTag<CoroutineHeapSize, GTACSA>, 4>);

impl<CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>> AdditionalDogStatsDTags<CoroutineHeapSize, GTACSA>
{
	/// New instance.
	#[inline(always)]
	pub fn new() -> Self
	{
		Self(ArrayVec::new())
	}
	
	/// Push a tag.
	#[inline(always)]
	pub fn push(&mut self, tag: AdditionalDogStatsDTag<CoroutineHeapSize, GTACSA>)
	{
		self.0.push(tag)
	}
	
	#[inline(always)]
	fn dog_stats_d_write(&self, dog_stats_d_writer: &mut DogStatsDWriter, static_tags: &DogStatsDTags) -> Result<(), ()>
	{
		let static_tags = &static_tags.0;
		let additional_tags = &self.0;
		
		if (static_tags.len() + additional_tags.len()) == 0
		{
			return Ok(())
		}
		
		dog_stats_d_writer.write_bytes(b"|#")?;
		
		let mut after_first = false;
		
		for tag in static_tags.iter()
		{
			if after_first
			{
				dog_stats_d_writer.write_comma()?
			}
			else
			{
				after_first = true
			}
			
			tag.dog_stats_d_write(dog_stats_d_writer)?
		}
		
		for tag in additional_tags.iter()
		{
			if after_first
			{
				dog_stats_d_writer.write_comma()?
			}
			else
			{
				after_first = true
			}
			
			tag.dog_stats_d_write(dog_stats_d_writer)?
		}
		
		Ok(())
	}
}
