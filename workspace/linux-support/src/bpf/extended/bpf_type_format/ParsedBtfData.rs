// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
pub struct ParsedBtfData
{
	btf_file_descriptor: BtfFileDescriptor,
	function_information: Box<[bpf_func_info]>,
	line_information: Box<[bpf_line_info]>,
}

impl ParsedBtfData
{
	#[inline(always)]
	pub fn optionally_to_bpf_load_data(parsed_btf_data: Option<&Self>) -> Result<(u32, BtfDataArray, BtfDataArray), ProgramLoadError>
	{
		const NoBtfFileDescriptor: u32 = 0;
		const NoBtfArray: BtfDataArray = (0, AlignedU64::Null, 0);
		match parsed_btf_data
		{
			None => Ok
			(
				(
					NoBtfFileDescriptor,
					NoBtfArray,
					NoBtfArray,
				)
			),
			
			Some(this) => this.to_bpf_load_data(),
		}
	}
	
	#[inline(always)]
	pub fn to_bpf_load_data(&self) -> Result<(u32, BtfDataArray, BtfDataArray), ProgramLoadError>
	{
		use self::ProgramLoadError::*;
		
		Ok
		(
			(
				self.btf_file_descriptor.as_raw_fd() as u32,
				Self::to_array(&self.function_information[..], FunctionInformationArrayIsLargerThanU32Max)?,
				Self::to_array(&self.line_information[..], LineInformationArrayIsLargerThanU32Max)?,
			)
		)
	}
	
	#[inline(always)]
	fn to_array<T>(information: &[T], error: impl FnOnce(TryFromIntError) -> ProgramLoadError) -> Result<BtfDataArray, ProgramLoadError>
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
