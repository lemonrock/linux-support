// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union bpf_attr
{
	pub(crate) map_create: BpfCommandMapCreate,
	pub(crate) map_change: BpfCommandMapChange,
	pub(crate) batch: BpfCommandMapBatch,
	pub(crate) program_load: BpfCommandProgramLoad,
	pub(crate) object: BpfCommandObject,
	pub(crate) program_attach_or_detach: BpfCommandProgramAttachOrDetach,
	pub(crate) test: BpfCommandProgramTestRun,
	pub(crate) get_identifier: BpfCommandGetIdentifier,
	pub(crate) info: BpfCommandObjectGetInformationByFileDescriptor,
	pub(crate) query: BpfCommandProgramQuery,
	pub(crate) raw_tracepoint: BpfCommandRawTracePointOpen,
	pub(crate) btf_load: BpfCommandBtfLoad,
	pub(crate) task_fd_query: BpfCommandTaskFileDescriptorQuery,
	pub(crate) link_create: BpfCommandLinkCreate,
	pub(crate) link_update: BpfCommandLinkUpdate,
	_bindgen_union_align: [u64; 15],
}

impl Default for bpf_attr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for bpf_attr
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "bpf_attr {{ union }}")
	}
}

impl bpf_attr
{
	#[inline(always)]
	pub(crate) fn syscall(&mut self, command: bpf_cmd) -> i32
	{
		const Size: u32 = size_of::<bpf_attr>() as u32;
		
		bpf(command, self, Size)
	}
}
