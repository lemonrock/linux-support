// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Parsed BTF data.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParsedBpfTypeFormatData
{
	bpf_type_format_file_descriptor: BpfTypeFormatFileDescriptor,
	function_information: Box<[bpf_func_info]>,
	line_information: Box<[bpf_line_info]>,
}

impl ParsedBpfTypeFormatData
{
	pub(crate) const NoBpfTypeFormatFileDescriptor: RawFd = 0;
	
	#[inline(always)]
	pub(crate) fn optionally_to_bpf_load_data(parsed_bpf_type_format_data: Option<&Self>) -> Result<(RawFd, BpfTypeFormatDataArray, BpfTypeFormatDataArray), ProgramLoadError>
	{
		const NoData: BpfTypeFormatDataArray = (0, AlignedU64::Null, 0);
		match parsed_bpf_type_format_data
		{
			None => Ok
			(
				(
					Self::NoBpfTypeFormatFileDescriptor,
					NoData,
					NoData,
				)
			),
			
			Some(this) => this.to_bpf_load_data(),
		}
	}
	
	#[inline(always)]
	pub(crate) fn to_raw_file_descriptor(&self) -> RawFd
	{
		self.bpf_type_format_file_descriptor.as_raw_fd()
	}
	
	#[inline(always)]
	pub(crate) fn to_bpf_load_data(&self) -> Result<(RawFd, BpfTypeFormatDataArray, BpfTypeFormatDataArray), ProgramLoadError>
	{
		use self::ProgramLoadError::*;
		
		Ok
		(
			(
				self.to_raw_file_descriptor(),
				Self::to_array(&self.function_information[..], BpfTypeFormatFunctionInformationArrayIsLargerThanU32Max)?,
				Self::to_array(&self.line_information[..], BpfTypeFormatLineInformationArrayIsLargerThanU32Max)?,
			)
		)
	}
	
	#[inline(always)]
	fn to_array<T>(information: &[T], error: impl FnOnce(TryFromIntError) -> ProgramLoadError) -> Result<BpfTypeFormatDataArray, ProgramLoadError>
	{
		let length = information.len();
		debug_assert_ne!(length, 0, "information length can not be zero; there must always be at least one function information and one line information");
		Ok
		(
			(
				size_of::<T>().try_into().unwrap(),
			 	AlignedU64::from(information),
				length.try_into().map_err(error)?
			)
		)
	}
}
