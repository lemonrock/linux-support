// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Used by `BPF_PROG_LOAD` command.
///
/// `libbpf` calls this `struct bpf_load_program_attr`.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub(crate) struct BpfCommandProgramLoad
{
	pub(crate) prog_type: bpf_prog_type,
	
	/// An array of `bpf_insn` instructions.
	pub(crate) insn_cnt: u32,
	
	/// An array of `bpf_insn` instructions.
	pub(crate) insns: AlignedU64,
	
	/// Pointer to a C string (ASCII NULL terminated string) such as `"GPL\0"`.
	pub(crate) license: AlignedU64,
	
	/// verbosity level of verifier.
	pub(crate) log_level: u32,
	
	/// size of user-supplied buffer.
	pub(crate) log_size: u32,
	
	/// pointer to user-supplied buffer.
	pub(crate) log_buf: AlignedU64,
	
	/// Unused unless `prog_type == bpf_prog_type::BPF_PROG_TYPE_KPROBE`.
	pub(crate) kern_version: u32,
	
	/// eg `BPF_F_TEST_RND_HI32`.
	pub(crate) prog_flags: BPF_PROG_LOAD_flags,
	
	pub(crate) prog_name: [c_char; BPF_OBJ_NAME_LEN],
	
	/// ifindex of netdev to prep for.
	///
	/// For some prog types expected attach type must be known at load time to verify attach type specific parts of prog (eg context accesses, allowed helpers, etc).
	pub(crate) prog_ifindex: Option<NetworkInterfaceIndex>,
	
	pub(crate) expected_attach_type: bpf_attach_type,
	
	/// File descriptor pointing to BTF type data.
	pub(crate) prog_btf_fd: RawFd,
	
	/// Either `0` or `size_of::<bpf_func_info>()`.
	///
	/// Used for BTF.
	pub(crate) func_info_rec_size: u32,
	
	/// Pointer to an array of `struct bpf_func_info` records used for BTF.
	pub(crate) func_info: AlignedU64,
	
	/// number of `struct bpf_func_info` records used for BTF.
	///
	/// If not `0`, then `func_info_rec_size` is checked by the kernel to be either `0` or `size_of::<bpf_func_info>()`.
	pub(crate) func_info_cnt: u32,
	
	/// Either `0` or `size_of::<bpf_line_info>()`.
	///
	/// Used for BTF.
	pub(crate) line_info_rec_size: u32,
	
	/// Pointer to an array of `struct bpf_line_info` records used for BTF.
	pub(crate) line_info: AlignedU64,
	
	/// number of `struct bpf_line_info` records used for BTF.
	///
	/// If not `0`, then `line_info_rec_size` is checked by the kernel to be either `0` or `size_of::<bpf_line_info>()`.
	pub(crate) line_info_cnt: u32,
	
	/// in-kernel BTF type id to attach to.
	pub(crate) attach_btf_id: BpfTypeFormatTypeIdentifier,
	
	/// A file descriptor.
	///
	/// Use `0` to attach to vmlinux.
	pub(crate) attach_prog_fd: RawFd,
}
