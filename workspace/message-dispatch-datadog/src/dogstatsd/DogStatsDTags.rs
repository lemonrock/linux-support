// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Tags.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DogStatsDTags(ArrayVec<[Cow<'static, DogStatsDTag>; 8]>);

impl DogStatsDTags
{
	/// Push a tag.
	#[inline(always)]
	pub fn push(&mut self, tag: impl Into<Cow<'static, DogStatsDTag>>)
	{
		self.0.push(tag.into())
	}
	
	#[inline(always)]
	fn dog_stats_d_write(&self, dog_stats_d_writer: &mut DogStatsDWriter) -> Result<(), ()>
	{
		if self.0.is_empty()
		{
			return Ok(())
		}
		
		dog_stats_d_writer.write_bytes(b"|#")?;
		
		let mut after_first = false;
		for tag in self.0.iter()
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
