// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Parse number error.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ParseNumberError
{
	/// Too short to start with a prefix such as `0x` or `0`.
	TooShortToStartWithPrefix
	{
		/// The prefix, eg `0x`.
		prefix: &'static [u8],

		/// Minimum size without prefix.
		minimum_size_without_prefix: usize,
	},

	/// Does not start with a prefix such as `0x` or `0`.
	DoesNotStartWithPrefix
	{
		/// The prefix, eg `0x`.
		prefix: &'static [u8],
	},

	/// A fixed width hexadecimal number has the wrong number of bytes.
	HexadecimalFixedWidthNumberHasWrongNumberOfBytes
	{
		/// Fixed width.
		fixed_width: usize,
	},

	/// A fixed width octal number has the wrong number of bytes.
	OctalFixedWidthNumberHasWrongNumberOfBytes
	{
		/// Fixed width.
		fixed_width: usize,
	},

	/// A number must be at least one byte long.
	TooShort,

	/// A number must be at least two bytes long if it has a minus sign.
	TooShortWithMinusSign,

	/// Overflow when scaling.
	ScalingOverflow,

	/// Overflow when adding.
	AddOverflow,

	/// An invalid byte.
	InvalidByte
	{
		/// Value of invalid byte.
		byte: u8,
	},

	/// Non-zero numbers are not allowed to be zero.
	WasZero,

	/// eg 0xFFFF or 0xFFFFFFFF; for example, PCI vendor and device numbers are not allowed to be 0xFFFF.
	WasMaximum,

	/// Too small.
	TooSmall,

	/// Too large.
	TooLarge,
}

impl Display for ParseNumberError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<ParseNumberError as Debug>::fmt(self, f)
	}
}

impl error::Error for ParseNumberError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		None
	}
}

impl ParseNumberError
{
	#[inline(always)]
	fn validate_prefix<'a>(bytes: &'a [u8], prefix: &'static [u8]) -> Result<&'a [u8], Self>
	{
		Self::validate_prefix_minimum_width(bytes, prefix, 1)
	}

	#[inline(always)]
	fn validate_prefix_minimum_width<'a>(bytes: &'a[u8], prefix: &'static [u8], minimum_size_without_prefix: usize) -> Result<&'a [u8], Self>
	{
		use self::ParseNumberError::*;

		if unlikely!(bytes.len() < (prefix.len() + minimum_size_without_prefix))
		{
			return Err(TooShortToStartWithPrefix { prefix, minimum_size_without_prefix })
		}

		if &bytes[0 .. prefix.len()] != prefix
		{
			return Err(DoesNotStartWithPrefix { prefix })
		}

		Ok(&bytes[prefix.len() .. ])
	}
}
