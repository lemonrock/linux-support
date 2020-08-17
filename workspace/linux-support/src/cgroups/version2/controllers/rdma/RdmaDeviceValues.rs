// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A RDMA (or InfiniBand, IB) device's values.
///
/// Depending on context, these are either the current values or the configured maximima.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub struct RdmaDeviceValues
{
	/// ?
	pub handles: MaximumNumber<i32>,
	
	/// ?
	pub objects: MaximumNumber<i32>,
}

impl FromBytes for RdmaDeviceValues
{
	type Error = RdmaParseError;
	
	fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
	{
		use self::RdmaParseError::*;
		
		#[inline(always)]
		fn parse_maximum_number<'a>(name: &'static [u8], mut key_value: impl Iterator<Item=&'a [u8]>) -> Result<Option<MaximumNumber<i32>>, RdmaParseError>
		{
			let value = key_value.next().ok_or(MissingStatisticValue { name })?;
			let maximum_number = MaximumNumber::from_bytes(value).map_err(|cause| InvalidStatisticValue { name, value: value.to_vec(), cause })?;
			Ok(Some(maximum_number))
		}
		
		#[inline(always)]
		fn unwrap_value(name: &'static [u8], value: Option<MaximumNumber<i32>>) -> Result<MaximumNumber<i32>, RdmaParseError>
		{
			value.ok_or(MissingStatistic { name })
		}
		
		let mut duplicate_key_detection = HashSet::with_capacity(2);
		let mut hca_handle = None;
		let mut hca_object = None;
		
		let mut fields = bytes.split_bytes(b' ');
		for key_value in fields
		{
			let mut key_value = key_value.split_bytes_n(2, b'=');
			let key = key_value.next().expect("Split always should produce at least one item");
			
			if unlikely!(duplicate_key_detection.insert(key.to_vec()))
			{
				return Err(DuplicateStatisticName { name: key.to_vec() })
			}
			
			match key
			{
				b"hca_handle" =>
				{
					hca_handle = parse_maximum_number(b"hca_handle", key_value)?;
				}
				
				b"hca_object" =>
				{
					hca_object = parse_maximum_number(b"hca_object", key_value)?;
				}
			
				_ => continue,
			}
		}
		
		Ok
		(
			Self
			{
				handles: unwrap_value(b"hca_handles", hca_handle)?,
				objects: unwrap_value(b"hca_objects", hca_object)?,
			}
		)
	}
}
