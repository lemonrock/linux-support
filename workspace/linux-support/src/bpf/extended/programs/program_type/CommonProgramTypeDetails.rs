// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Program type details.
#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct CommonProgramTypeDetails
{
	/// Minimum Linux kernel version.
	pub minimum_linux_kernel_version: MinimumLinuxKernelVersion,
	
	/// ifindex of netdev to prep for.
	///
	/// For some prog types expected attach type must be known at load time to verify attach type specific parts of prog (eg context accesses, allowed helpers, etc).
	pub ifindex: Option<NetworkInterfaceIndex>,
}

impl CommonProgramTypeDetails
{
	#[inline(always)]
	pub(crate) fn to_values(&self, program_type: bpf_prog_type, expected_attached_type: bpf_attach_type) -> Result<(bpf_prog_type, bpf_attach_type, BtfTypeIdentifier, RawFd, u32, Option<NetworkInterfaceIndex>), ProgramError>
	{
		Ok((program_type, expected_attached_type, BtfTypeIdentifier::Void, 0, self.minimum_linux_kernel_version.to_u32(), self.ifindex))
	}
}
