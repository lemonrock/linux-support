// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// `isolcpus` flags.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IsolatedCpuFlags
{
	/// Domain.
	Domain,

	/// `nohz`.
	NoHz,
}

impl Default for IsolatedCpuFlags
{
	#[inline(always)]
	fn default() -> Self
	{
		IsolatedCpuFlags::Domain
	}
}

impl IsolatedCpuFlags
{
	#[inline(always)]
	fn parse(flag: &[u8]) -> Result<Self, String>
	{
		use self::IsolatedCpuFlags::*;

		match flag
		{
			b"domain" => Ok(Domain),
			b"nohz" => Ok(NoHz),
			_ => Err(format!("Unknown flag '{:?}'", flag)),
		}
	}

	#[inline(always)]
	fn split_flags_and_cpu_list(value: &[u8]) -> (Option<&[u8]>, &[u8])
	{
		#[inline(always)]
		fn index_of_split_between_flags_and_cpu_list(value: &[u8]) -> Option<usize>
		{
			#[inline(always)]
			fn is_ascii_alpha(character: u8) -> bool
			{
				character.is_ascii_uppercase() || character.is_ascii_lowercase()
			}

			let mut index = 0;
			let mut previous_previous_character = b'\0';
			let mut previous_character = b'\0';
			for character in value.iter()
			{
				let character = *character;
				if character.is_ascii_digit() && previous_character == b',' && is_ascii_alpha(previous_previous_character)
				{
					return Some(index)
				}
				previous_previous_character = previous_character;
				previous_character = character;
				index += 1;
			}
			None
		}

		match index_of_split_between_flags_and_cpu_list(value)
		{
			None =>
			{
				let flags = None;
				let list = value;

				(flags, list)
			}

			Some(index_after_comma) =>
			{
				let split = value.split_at(index_after_comma);
				let flags = Some(&split.0[ .. (split.0.len() - 1)]);
				let list = split.1;

				(flags, list)
			}
		}
	}
}
