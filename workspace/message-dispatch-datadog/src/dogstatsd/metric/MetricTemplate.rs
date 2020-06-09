// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A DogStatsD metric template, intending for use with `lazy_static!` initialization.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MetricTemplate
{
	/// Name.
	pub name: Name,
	
	/// Tags.
	pub tags: DogStatsDTags,
}

impl MetricTemplate
{
	/// Creates a metric.
	#[inline(always)]
	pub fn with(&self, metric_value: MetricValue) -> Metric
	{
		Metric
		{
			template: self,
			metric_value,
		}
	}
}
