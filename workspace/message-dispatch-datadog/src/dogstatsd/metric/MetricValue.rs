// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A DogStatsD metric value.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum MetricValue
{
	/// Count change.
	COUNT
	{
		/// Value.
		value: i64,
		
		/// Sample rate.
		sample_rate: Option<SampleRate>,
	},
	
	/// Gauge.
	///
	/// There is a defect in gauge's definition in the StatsD protocol which makes it impossible to distinguish negative absolute values with relative changes.
	GAUGE
	{
		/// Value.
		value: Either<i64, f64>,
	},
	
	/// DataDog-specific.
	HISTOGRAM
	{
		/// Value.
		value: Either<i64, f64>,
		
		/// Sample rate.
		sample_rate: Option<SampleRate>
	},
	
	/// DataDog-specific.
	DISTRIBUTION
	{
		/// Value.
		value: Either<i64, f64>,
	},
	
	/// In DataDog, treated as a sub-type of `HISTOGRAM`.
	TIMER
	{
		/// Value.
		value: Duration,
		
		/// Sample rate.
		sample_rate: Option<SampleRate>
	},
	
	/// In DataDog, treated the same as `GAUGE`.
	SET
	{
		/// Value.
		value: Either<i64, f64>,
	},
}

impl MetricValue
{
	/// Increment.
	#[inline(always)]
	pub fn increment(sample_rate: Option<SampleRate>) -> Self
	{
		MetricValue::COUNT
		{
			value: 1,
			sample_rate,
		}
	}
	
	/// Decrement.
	#[inline(always)]
	pub fn decrement(sample_rate: Option<SampleRate>) -> Self
	{
		MetricValue::COUNT
		{
			value: -1,
			sample_rate,
		}
	}
	
	#[inline(always)]
	fn dog_stats_d_write(&self, dog_stats_d_writer: &mut DogStatsDWriter) -> Result<(), ()>
	{
		#[inline(always)]
		fn either_value_to_string(value: Either<i64, f64>) -> String
		{
			match value
			{
				Left(signed_integer) => signed_integer.to_string(),
				Right(floating_point) => floating_point.to_string(),
			}
		}
		
		#[inline(always)]
		fn write_basic_metric(dog_stats_d_writer: &mut DogStatsDWriter, value: String, denominator: &[u8]) -> Result<(), ()>
		{
			dog_stats_d_writer.write_string(value)?;
			dog_stats_d_writer.write_bytes(denominator)
		}
		
		#[inline(always)]
		fn write_sampled_metric(dog_stats_d_writer: &mut DogStatsDWriter, value: String, denominator: &[u8], sample_rate: Option<SampleRate>) -> Result<(), ()>
		{
			write_basic_metric(dog_stats_d_writer, value, denominator)?;
			
			match sample_rate
			{
				None => Ok(()),
				
				Some(SampleRate(sample_rate)) => dog_stats_d_writer.write_string(format!("|@{}", sample_rate)),
			}
		}
		
		use self::MetricValue::*;
		match self
		{
			&COUNT { value, sample_rate } => write_sampled_metric(dog_stats_d_writer, value.to_string(), b"|c", sample_rate),
			
			&GAUGE { value } => write_basic_metric(dog_stats_d_writer, either_value_to_string(value), b"|g"),
			
			&HISTOGRAM { value, sample_rate } => write_sampled_metric(dog_stats_d_writer, either_value_to_string(value), b"|h", sample_rate),
			
			&DISTRIBUTION { value } => write_basic_metric(dog_stats_d_writer, either_value_to_string(value), b"|d"),
			
			&TIMER { value, sample_rate } => write_sampled_metric(dog_stats_d_writer, DogStatsDWriter::duration_to_string(value), b"|ms", sample_rate),
			
			&SET { value } => write_basic_metric(dog_stats_d_writer, either_value_to_string(value), b"|s"),
		}
	}
}
