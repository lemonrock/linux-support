// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents an extended BPF program file descriptor.
///
/// Created by `ExtendedBpfProgram::parse_and_load()`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExtendedBpfProgramFileDescriptor(RawFd);

impl Drop for ExtendedBpfProgramFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.as_raw_fd().close()
	}
}

impl AsRawFd for ExtendedBpfProgramFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl IntoRawFd for ExtendedBpfProgramFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.as_raw_fd()
	}
}

impl FromRawFd for ExtendedBpfProgramFileDescriptor
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(fd)
	}
}

impl FileDescriptor for ExtendedBpfProgramFileDescriptor
{
}

impl BpfFileDescriptor for ExtendedBpfProgramFileDescriptor
{
	type Identifier = ExtendedBpfProgramIdentifier;
	
	type Information = bpf_prog_info;
	
	type Access = ();
	
	const GetFileDescriptor: bpf_cmd = bpf_cmd::BPF_PROG_GET_FD_BY_ID;
	
	const DefaultAccess: Self::Access = ();
	
	#[inline(always)]
	fn access_permissions_to_open_flags(_access: Self::Access) -> u32
	{
		0
	}
}

impl UsedAsValueInArrayMapDescriptor for ExtendedBpfProgramFileDescriptor
{
	#[inline(always)]
	fn transmute_from_file_descriptor_copies(values: &[Self]) -> &[RawFd]
	{
		unsafe { transmute(values) }
	}
}

impl ProvidesIdentifierWhenUsedAsValueInArrayMapDescriptor for ExtendedBpfProgramFileDescriptor
{
	type Identifier = ExtendedBpfProgramIdentifier;
}

impl ExtendedBpfProgramFileDescriptor
{
	/// Test run.
	pub fn test_run<C: Sized>(&self, context: &C, test_data: &[u8], expected_output_data_length: usize, repetitions: NonZeroU32) -> Result<TestRunResults<C>, SystemCallErrorNumber>
	{
		let mut attr = bpf_attr::default();
		let mut context_out: C = unsafe_uninitialized();
		let mut data_out = Vec::with_capacity(expected_output_data_length);
		attr.test = BpfCommandProgramTestRun
		{
			prog_fd: self.as_raw_fd(),
			retval: 0,
			data_size_in: test_data.len() as u32,
			data_size_out: data_out.capacity() as u32,
			data_in: AlignedU64::from(test_data.as_ptr()),
			data_out: AlignedU64::from(data_out.as_mut_ptr()),
			repeat: repetitions,
			duration: 0,
			ctx_size_in: size_of::<C>() as u32,
			ctx_size_out: size_of::<C>() as u32,
			ctx_in: AlignedU64::from(context),
			ctx_out: AlignedU64::from(&mut context_out),
		};
		
		match attr.syscall(bpf_cmd::BPF_PROG_TEST_RUN).as_usize()
		{
			0 =>
			{
				let test = unsafe { attr.test };
				
				let ctx_size_out = test.ctx_size_out;
				debug_assert!(ctx_size_out <= size_of::<C>() as u32);
				
				let data_size_out = test.data_size_out;
				debug_assert!(data_size_out <= data_out.len() as u32);
				unsafe { data_out.set_len(data_size_out as usize) };
				data_out.shrink_to_fit();
				
				Ok
				(
					TestRunResults
					{
						context: context_out,
						data: data_out,
						result_code: test.retval,
						duration: test.duration,
					}
				)
			}
			
			error @ SystemCallResult::InclusiveErrorRangeStartsFrom_usize ..= SystemCallResult::InclusiveErrorRangeEndsAt_usize => Err(SystemCallResult::usize_to_system_call_error_number(error)),
			
			unexpected @ _ => unexpected_result!(bpf, BPF_PROG_TEST_RUN, unexpected),
		}
	}
}
