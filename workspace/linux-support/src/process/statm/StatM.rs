// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Most uses of this are better provided for by `Status`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StatM
{
	/// Total program size.
	///
	/// Equivalent to `total_program_size` converted into a number of pages in `Status`.
	pub total_program_size: NumberOfPages,

	/// Equivalent to the following in `Status`:-
	///
	/// * `resident_set_memory_size` converted into a number of pages.
	/// * the sum of `anonymous_resident_set_memory_size`, `resident_set_file_mappings_memory_size` and `resident_set_shared_memory_size` converted into a number of pages.
	pub resident_set_memory_size: NumberOfPages,

	/// Total program size.
	///
	/// Equivalent to the sum of `resident_set_file_mappings_memory_size` and `resident_set_shared_memory_size` converted into a number of pages in `Status`.
	pub resident_shared_pages: NumberOfPages,

	/// Text (program code) segment size.
	///
	/// Equivalent to `text_segment_size` converted into a number of pages in `Status`.
	pub text_segment_size: NumberOfPages,

	/// Equivalent to the sum of `private_data_segments_size` and `stack_segments_size` converted into a number of pages in `Status`.
	pub data: NumberOfPages,
}

impl StatM
{
	/// Status information from `/proc/self/statm`.
	#[inline(always)]
	pub fn self_statm(proc_path: &ProcPath) -> Result<Self, StatMParseError>
	{
		Self::process_statm(proc_path, ProcessIdentifierChoice::Current)
	}

	/// Status information from `/proc/<IDENTIFIER>/statm` where `<IDENTIFIER>` is `process_identifier`.
	#[inline(always)]
	pub fn process_statm(proc_path: &ProcPath, process_identifier: ProcessIdentifierChoice) -> Result<Self, StatMParseError>
	{
		let file_path = proc_path.process_file_path(process_identifier, "statm");
		let line = file_path.read_raw_without_line_feed()?;
		let mut columns = line.split_bytes_n(7, b' ');

		#[inline(always)]
		fn parse_field<'a>(columns: &mut impl Iterator<Item=&'a [u8]>, index: u8, name: &'static str) -> Result<NumberOfPages, StatMParseError>
		{
			use self::StatMParseError::*;

			let field = match columns.next()
			{
				None => return Err(MissingField { index: unsafe { NonZeroU8::new_unchecked(index) }, name }),
				Some(bytes) => NumberOfPages::from_bytes(bytes).map_err(|cause| ParseNumber { index: unsafe { NonZeroU8::new_unchecked(index) }, name, cause})?,
			};
			Ok(field)
		}

		#[inline(always)]
		fn parse_always_zero_field<'a>(columns: &mut impl Iterator<Item=&'a [u8]>, index: u8, name: &'static str) -> Result<(), StatMParseError>
		{
			let value = parse_field(columns, index, name)?;
			if likely!(value == 0)
			{
				Ok(())
			}
			else
			{
				Err(StatMParseError::FieldWasNotZero { index: unsafe { NonZeroU8::new_unchecked(index) }, name })
			}
		}

		Ok
		(
			Self
			{
				total_program_size: parse_field(&mut columns, 1, "size")?,
				resident_set_memory_size: parse_field(&mut columns, 2, "resident")?,
				resident_shared_pages: parse_field(&mut columns, 3, "shared")?,
				text_segment_size: parse_field(&mut columns, 4, "text")?,
				data:
				{
					parse_always_zero_field(&mut columns, 1, "lib")?;
					let data = parse_field(&mut columns, 1, "data")?;
					parse_always_zero_field(&mut columns, 1, "dt")?;
					data
				},
			}
		)
	}
}
