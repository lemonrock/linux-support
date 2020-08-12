// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


fn parse_key_value_statistics(file_path: &Path, callback: &mut FnMut(&[u8], usize) -> bool) -> Result<(), StatisticsParseError>
{
	use self::StatisticsParseError::*;
	
	let reader = file_path.read_raw()?;
	
	let mut duplicate_statistic_detection = HashSet::with_capacity(6);
	for line in reader.split_bytes(b'\n')
	{
		let mut name_and_value = line.split_bytes_n(2, b' ');
		let name = name_and_value.next().expect("Split always should produce at least one item");
		
		if unlikely!(!duplicate_statistic_detection.insert(name))
		{
			return Err(DuplicateStatisticName { name: name.to_vec() })
		}
		
		#[inline(always)]
		fn parse_value<'a>(name: &'static [u8], mut name_and_value: impl Iterator<Item=&'a [u8]>) -> Result<usize, StatisticsParseError>
		{
			let bytes_value = name_and_value.next().ok_or(MissingStatisticValue { name })?;
			usize::parse_decimal_number(bytes_value).map_err(|cause| InvalidStatisticValue { name, value: bytes_value.to_vec(), cause })
		}

		let statistic_rejected = callback(name, parse_value(name, name_and_value)?);
		if statistic_rejected
		{
			return Err(InvalidStatisticName { name: name.to_vec() })
		}
	}
	
	Ok(())
}
