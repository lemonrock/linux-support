// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a BTF file descriptor.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BtfFileDescriptor(RawFd);

impl Drop for BtfFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.as_raw_fd().close()
	}
}

impl AsRawFd for BtfFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl IntoRawFd for BtfFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.as_raw_fd()
	}
}

impl FromRawFd for BtfFileDescriptor
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(fd)
	}
}

impl FileDescriptor for BtfFileDescriptor
{
}

impl BpfFileDescriptor for BtfFileDescriptor
{
	type Information = bpf_btf_info;
}

impl BtfFileDescriptor
{
	pub(crate) fn load_btf_data(header_and_type_identifier_section_and_string_section: &[u8], verifier_log: Option<&mut VerifierLog>) -> Result<Self, ProgramError>
	{
		use self::ProgramError::*;
		
		const BTF_MAX_SIZE: usize = 16 * 1024 * 1024;
		
		let length = header_and_type_identifier_section_and_string_section.len();
		if unlikely!(length > BTF_MAX_SIZE)
		{
			return Err(MaximumBtfDataSizeExceeded)
		}
		
		let (btf_log_level, btf_log_buf, btf_log_size) = VerifierLog::to_values_for_syscall(verifier_log);
		
		let mut attributes = bpf_attr
		{
			btf_load: BpfCommandBtfLoad
			{
				btf: Default::default(),
				btf_log_buf,
				btf_size: header_and_type_identifier_section_and_string_section.len() as u32,
				btf_log_size,
				btf_log_level,
			}
		};
		
		let result = attributes.syscall(bpf_cmd::BPF_BTF_LOAD);
		if likely!(result >= 0)
		{
			Ok(Self(result))
		}
		else if likely!(result == -1)
		{
			Err(CouldNotLoadBtfData(errno()))
		}
		else
		{
			unreachable!("result `{}` from bpf(BPF_BTF_LOAD) was unexpected", result)
		}
	}
}
