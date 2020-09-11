// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Errors when parsing a statistic.
#[derive(Debug)]
pub enum RdmaParseError
{
	/// Input error.
	Input(io::Error),

	/// Duplicate.
	DuplicateDevice
	{
		#[allow(missing_docs)]
		device_name: RdmaDeviceName,
	},

	/// Missing key-value fields.
	MissingKeyValueFields
	{
		#[allow(missing_docs)]
		device_name: RdmaDeviceName,
	},

	/// Duplicate statistic name.
	DuplicateStatisticName
	{
		/// Name.
		name: Vec<u8>,
	},

	/// Missing statistic value.
	MissingStatisticValue
	{
		/// Name.
		name: &'static [u8],
	},

	/// Invalid statistic value.
	InvalidStatisticValue
	{
		/// Name.
		name: &'static [u8],

		/// Value.
		value: Vec<u8>,

		/// Cause.
		cause: ParseNumberError,
	},
	
	/// Missing statistic value.
	MissingStatistic
	{
		#[allow(missing_docs)]
		name: &'static [u8]
	},
}

impl Display for RdmaParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl Error for RdmaParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn Error + 'static)>
	{
		use self::RdmaParseError::*;

		match self
		{
			&Input(ref source) => Some(source),

			&DuplicateDevice { .. } => None,
			
			&MissingKeyValueFields { .. } => None,

			&DuplicateStatisticName { .. } => None,

			&MissingStatisticValue { .. } => None,

			&InvalidStatisticValue { ref cause, .. } => Some(cause),

			&MissingStatistic { .. } => None,
		}
	}
}

impl From<io::Error> for RdmaParseError
{
	#[inline(always)]
	fn from(value: io::Error) -> Self
	{
		RdmaParseError::Input(value)
	}
}
