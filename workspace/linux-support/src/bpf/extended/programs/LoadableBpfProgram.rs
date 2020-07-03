// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/*

SO_BPF_EXTENSIONS
SO_ATTACH_BPF
SO_INCOMING_NAPI_ID
	- can be used by a BPF program to direct packets based on RX queue, I think.
Explore aRFS.
 */

/*
	Program types we care about:-
		* BPF_PROG_TYPE_SOCKET_FILTER
		* BPF_PROG_TYPE_SK_REUSEPORT
		* ?BPF_PROG_TYPE_XDP
		* ?BPF_PROG_TYPE_SOCK_OPS
		* ?BPF_PROG_TYPE_SK_SKB
		* ?BPF_PROG_TYPE_SK_MSG

 */

/// A loadable BPF program using the `bpf()` syscall.
///
/// TODO: Look at `RedBPF`: <https://blog.redsift.com/labs/writing-bpf-code-in-rust/>.
pub trait LoadableBpfProgram
{
	#[doc(hidden)]
	const ProgramType: bpf_prog_type;
	
	fn load(license: BpfProgramLicense, minimum_linux_kernel_version: MinimumLinuxKernelVersion, instructions: &[bpf_insn], verifier_log: Option<&mut [c_char]>, program_name: ProgramName)
	{
		let insn_cnt = instructions.len().try_into().expect("BPF program has too many instructions");
		debug_assert_ne!(insn_cnt, 0, "There must be at least one instruction");
		
		// TODO: If the size of the buffer is not large enough to store all verifier messages, -1 is returned and errno is set to ENOSPC.
		let (log_level, log_buf, log_size) = match verifier_log
		{
			None => (0, AlignedU64::Null, 0),
			Some(log_buffer) => (1, AlignedU64::from(log_buffer), log_buffer.len().try_into().expect("log_buffer is too large"))
		};
		
		let command_name: ArrayVec<[u8; BPF_OBJ_NAME_LEN]> = (program_name.0).0;
		
		let mut attr = bpf_attr
		{
			program_load: BpfCommandProgramLoad
			{
				prog_type: Self::ProgramType,
				insn_cnt,
				insns: AlignedU64::from(instructions),
				license: AlignedU64::from(license.0.as_ptr()),
				log_level,
				log_size,
				log_buf,
				kern_version: minimum_linux_kernel_version.to_u32(),
				
				prog_flags: BPF_PROG_LOAD_flags::BPF_F_STRICT_ALIGNMENT,
				prog_name: program_name.0,
				prog_ifindex: XXX,
				expected_attach_type: XXX,
				prog_btf_fd: XXX,
				func_info_rec_size: XXX,
				func_info: XXX,
				func_info_cnt: XXX,
				line_info_rec_size: XXX,
				line_info: XXX,
				line_info_cnt: XXX,
				attach_btf_id: XXX,
				attach_prog_fd: XXX,
			}
		};
		
		bpf(bpf_cmd::BPF_PROG_LOAD, &mut attributes, size)
	}
}

pub struct SocketFilterBpfProgram
{}

impl LoadableBpfProgram for SocketFilterBpfProgram
{
	const ProgramType: bpf_prog_type = bpf_prog_type::BPF_PROG_TYPE_SOCKET_FILTER;
}

pub struct ReusePortBpfProgram
{}

impl LoadableBpfProgram for SocketFilterBpfProgram
{
	const ProgramType: bpf_prog_type = bpf_prog_type::BPF_PROG_TYPE_SK_REUSEPORT;
}
