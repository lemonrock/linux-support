// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a BPF Type Format (BTF) file descriptor.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BpfTypeFormatFileDescriptor(RawFd);

impl Drop for BpfTypeFormatFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.as_raw_fd().close()
	}
}

impl AsRawFd for BpfTypeFormatFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl IntoRawFd for BpfTypeFormatFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.as_raw_fd()
	}
}

impl FromRawFd for BpfTypeFormatFileDescriptor
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(fd)
	}
}

impl FileDescriptor for BpfTypeFormatFileDescriptor
{
}

impl BpfFileDescriptor for BpfTypeFormatFileDescriptor
{
	type Identifier = BpfTypeFormatIdentifier;
	
	type Information = bpf_btf_info;
	
	type Access = ();
	
	const GetFileDescriptor: bpf_cmd = bpf_cmd::BPF_BTF_GET_FD_BY_ID;
	
	const DefaultAccess: Self::Access = ();
	
	#[inline(always)]
	fn access_permissions_to_open_flags(_access: Self::Access) -> u32
	{
		0
	}
}

impl BpfTypeFormatFileDescriptor
{
	pub(crate) fn load_data(header_and_type_identifier_section_and_string_section: &[u8], mut verifier_log: Option<VerifierLog>) -> Result<(Self, Option<VerifierLog>), ParseError>
	{
		use self::ParseError::*;
		
		const BTF_MAX_SIZE: usize = 16 * 1024 * 1024;
		
		let length = header_and_type_identifier_section_and_string_section.len();
		if unlikely!(length > BTF_MAX_SIZE)
		{
			return Err(MaximumBpfTypeFormatDataSizeExceeded)
		}
		
		let (btf_log_level, btf_log_buf, btf_log_size) = VerifierLog::to_values_for_syscall(verifier_log.as_mut());
		
		let mut attributes = bpf_attr
		{
			bpf_type_format_load: BpfCommandBpfTypeFormatLoad
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
			Ok((Self(result), verifier_log))
		}
		else if likely!(result == -1)
		{
			Err(CouldNotLoadBpfTypeFormatData(errno(), verifier_log.map(|verifier_log| verifier_log.into())))
		}
		else
		{
			unreachable_code(format_args!("result `{}` from bpf(BPF_BTF_LOAD) was unexpected", result))
		}
	}
}
