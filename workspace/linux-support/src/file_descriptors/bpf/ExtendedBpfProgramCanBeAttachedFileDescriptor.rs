// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Supports attaching and detaching programs.
pub trait ExtendedBpfProgramCanBeAttachedFileDescriptor: FileDescriptor
{
	/// Program attachment type.
	type ProgramAttachmentType: ProgramAttachmentType;
	
	/// Program query flags, if any.
	type ProgramQueryFlags: ProgramQueryFlags;
	
	/// Program attachment flags, if any.
	type ProgramAttachmentFlags: ProgramAttachmentFlags;
	
	/// Program attachment options, if any.
	type ProgramAttachmentOptions: ProgramAttachmentOptions;
	
	#[doc(hidden)]
	const InitialProgramCountGuess: usize;
	
	/// Requires the capability `CAP_NET_ADMIN`.
	///
	/// `query_flags` is only appropriate for queries on `CgroupFileDescriptor`.
	/// `attach_flags` is only appropriate for queries on `CgroupFileDescriptor`.
	///
	/// Returns an error if permission is denied.
	///
	/// Works for the following `attach_types`:-
	///
	/// * `CgroupFileDescriptor`.
	///   * `BPF_PROG_TYPE_CGROUP_SKB`.
	///     * `BPF_CGROUP_INET_INGRESS`.
	///     * `BPF_CGROUP_INET_EGRESS`.
	///   * `BPF_PROG_TYPE_CGROUP_SOCK`
	///     * `BPF_CGROUP_INET_SOCK_CREATE`.
	///     * `BPF_CGROUP_INET4_POST_BIND`.
	///     * `BPF_CGROUP_INET6_POST_BIND`.
	///   * `BPF_PROG_TYPE_CGROUP_SOCK_ADDR`.
	///     * `BPF_CGROUP_INET4_BIND`.
	///     * `BPF_CGROUP_INET6_BIND`.
	///     * `BPF_CGROUP_INET4_CONNECT`.
	///     * `BPF_CGROUP_INET6_CONNECT`.
	///     * `BPF_CGROUP_UDP4_SENDMSG`.
	///     * `BPF_CGROUP_UDP6_SENDMSG`.
	///     * `BPF_CGROUP_UDP4_RECVMSG`.
	///     * `BPF_CGROUP_UDP6_RECVMSG`.
	///   * `BPF_PROG_TYPE_SOCK_OPS`.
	///     * `BPF_CGROUP_SOCK_OPS`.
	///   * `BPF_PROG_TYPE_CGROUP_DEVICE`.
	///     * `BPF_CGROUP_DEVICE`.
	///   * `BPF_CGROUP_SYSCTL`.
	///     * `BPF_PROG_TYPE_CGROUP_SYSCTL`.
	///   * `BPF_PROG_TYPE_CGROUP_SOCKOPT`.
	///     * `BPF_CGROUP_GETSOCKOPT`.
	///     * `BPF_CGROUP_SETSOCKOPT`.
	/// * `NetworkNamespaceFileDescriptor`.
	///   * `BPF_FLOW_DISSECTOR`.
	///     * `BPF_PROG_TYPE_FLOW_DISSECTOR`.
	/// * `LinuxInfraRedRemoteControlRawMode2FileDescriptor`.
	///   * `BPF_LIRC_MODE2`.
	///     * `BPF_PROG_TYPE_LIRC_MODE2`.
	///
	/// Unsupported for the `attach_types`:-
	///   * `BPF_PROG_TYPE_SK_MSG`.
	///     * `BPF_SK_MSG_VERDICT`.
	///   * `BPF_PROG_TYPE_SK_SKB`.
	///     * `BPF_SK_SKB_STREAM_PARSER`.
	///     * `BPF_SK_SKB_STREAM_VERDICT`.
	#[inline(always)]
	fn query_attached_extended_bpf_programs(&self, program_attachment_type: Self::ProgramAttachmentType, program_query_flags: Self::ProgramQueryFlags) -> Result<(Vec<ExtendedBpfProgramIdentifier>, Self::ProgramAttachmentFlags), ()>
	{
		let mut programs: Vec<ExtendedBpfProgramIdentifier> = Vec::with_capacity(Self::InitialProgramCountGuess);
		
		let mut attr = bpf_attr::default();
		let attach_type = program_attachment_type.to_bpf_attach_type();
		attr.query = BpfCommandProgramQuery
		{
			target_fd: self.as_raw_fd(),
			attach_type,
			query_flags: program_query_flags.to_query_flags(),
			attach_flags: BPF_PROG_ATTACH_flags::empty(),
			prog_ids: AlignedU64::from(programs.as_mut_ptr()),
			prog_cnt: programs.capacity() as u32,
		};
		
		loop
		{
			match attr.syscall(bpf_cmd::BPF_PROG_QUERY).as_usize()
			{
				0 =>
				{
					unsafe { programs.set_len(attr.query.prog_cnt as usize) };
					programs.shrink_to_fit();
					let program_attachment_flags = Self::ProgramAttachmentFlags::parse(unsafe { attr.query.attach_flags });
					return Ok((programs, program_attachment_flags))
				}
				
				// Only returned as of Linux 5.6 for CgroupFileDescriptor.
				SystemCallResult::ENOSPC_usize =>
				{
					let required = unsafe { attr.query.prog_cnt as usize };
					debug_assert!(required > programs.capacity());
					programs.reserve(required - programs.capacity());
					continue
				}
				SystemCallResult::EPERM_usize => return Err(()),
				SystemCallResult::EINVAL_usize => panic!("Invalid attr or invalid attach type"),
				SystemCallResult::EFAULT_usize => panic!("Fault copying to / from userspace"),
				unexpected_error @ _ => unexpected_error!(bpf, BPF_PROG_QUERY, SystemCallResult::usize_to_system_call_error_number(unexpected_error)),
				
				unexpected @ _ => unexpected_result!(bpf, BPF_PROG_QUERY, unexpected),
			}
		}
	}
	
	/// Requires the capability `CAP_NET_ADMIN`.
	///
	/// Supported attach types by program type (found in the Linux kernel functions `bpf_prog_attach()` and `attach_type_to_prog_type()`).
	///
	/// * `CgroupFileDescriptor`.
	///   * `BPF_PROG_TYPE_CGROUP_SKB`.
	///     * `BPF_CGROUP_INET_INGRESS`.
	///     * `BPF_CGROUP_INET_EGRESS`.
	///   * `BPF_PROG_TYPE_CGROUP_SOCK`
	///     * `BPF_CGROUP_INET_SOCK_CREATE`.
	///     * `BPF_CGROUP_INET4_POST_BIND`.
	///     * `BPF_CGROUP_INET6_POST_BIND`.
	///   * `BPF_PROG_TYPE_CGROUP_SOCK_ADDR`.
	///     * `BPF_CGROUP_INET4_BIND`.
	///     * `BPF_CGROUP_INET6_BIND`.
	///     * `BPF_CGROUP_INET4_CONNECT`.
	///     * `BPF_CGROUP_INET6_CONNECT`.
	///     * `BPF_CGROUP_UDP4_SENDMSG`.
	///     * `BPF_CGROUP_UDP6_SENDMSG`.
	///     * `BPF_CGROUP_UDP4_RECVMSG`.
	///     * `BPF_CGROUP_UDP6_RECVMSG`.
	///   * `BPF_PROG_TYPE_SOCK_OPS`.
	///     * `BPF_CGROUP_SOCK_OPS`.
	///   * `BPF_PROG_TYPE_CGROUP_DEVICE`.
	///     * `BPF_CGROUP_DEVICE`.
	///   * `BPF_CGROUP_SYSCTL`.
	///     * `BPF_PROG_TYPE_CGROUP_SYSCTL`.
	///   * `BPF_PROG_TYPE_CGROUP_SOCKOPT`.
	///     * `BPF_CGROUP_GETSOCKOPT`.
	///     * `BPF_CGROUP_SETSOCKOPT`.
	/// * `NetworkNamespaceFileDescriptor`.
	///   * `BPF_FLOW_DISSECTOR`.
	///     * `BPF_PROG_TYPE_FLOW_DISSECTOR`.
	/// * `LinuxInfraRedRemoteControlMode2FileDescriptor`.
	///   * `BPF_LIRC_MODE2`.
	///     * `BPF_PROG_TYPE_LIRC_MODE2`.
	///
	/// * `sock_map_get_from_fd` (A `MapFileDescriptor` for a map of type `BPF_MAP_TYPE_SOCKMAP` or `BPF_MAP_TYPE_SOCKHASH`).
	///   * `BPF_PROG_TYPE_SK_MSG`.
	///     * `BPF_SK_MSG_VERDICT`.
	///   * `BPF_PROG_TYPE_SK_SKB`.
	///     * `BPF_SK_SKB_STREAM_PARSER`.
	///     * `BPF_SK_SKB_STREAM_VERDICT`.
	///
	/// The program type referred to in `attach_program` must be compatible with the `program_attachment_type`.
	///
	/// Does not support programs loaded onto to devices (`NetworkInterfaceIndex`).
	///
	/// Fails if `self` has too many programs attached (the norm is 64 for Linux Infra Red Control, PerfEvent and CGroup).
	#[inline(always)]
	fn attach(&self, program_attachment_type: Self::ProgramAttachmentType, attach_program: &ExtendedBpfProgramFileDescriptor, program_attachment_options: Self::ProgramAttachmentOptions) -> Result<(), ()>
	{
		let (attach_flags, replace_bpf_fd) = program_attachment_options.to_attach_flags();
		
		let mut attr = bpf_attr::default();
		attr.program_attach_or_detach = BpfCommandProgramAttachOrDetach
		{
			target_fd: self.as_raw_fd(),
			attach_bpf_fd: attach_program.as_raw_fd(),
			attach_type: program_attachment_type.to_bpf_attach_type(),
			attach_flags,
			replace_bpf_fd,
		};
		
		match attr.syscall(bpf_cmd::BPF_PROG_ATTACH).as_usize()
		{
			0 => Ok(()),
			
			SystemCallResult::EINVAL_usize => panic!("Invalid attr or invalid attachment type to attach"),
			SystemCallResult::EPERM_usize => panic!("Permission denied"),
			SystemCallResult::EEXIST_usize => panic!("BPF_PROG_TYPE_FLOW_DISSECTOR only allows one program to exist at a time"),
			SystemCallResult::E2BIG_usize => Err(()),
			SystemCallResult::ENODEV_usize => panic!("BPF_PROG_TYPE_LIRC_MODE2 only supports raw mode 2"),
			SystemCallResult::EOPNOTSUPP_usize => panic!("BPF_PROG_TYPE_SK_* failure"),
			unexpected_error @ SystemCallResult::InclusiveErrorRangeStartsFrom_usize ..= SystemCallResult::InclusiveErrorRangeEndsAt_usize => unexpected_error!(bpf, BPF_PROG_ATTACH, SystemCallResult::usize_to_system_call_error_number(unexpected_error)),
			
			unexpected @ _ => unexpected_result!(bpf, BPF_PROG_ATTACH, unexpected),
		}
	}
	
	/// Requires the capability `CAP_NET_ADMIN`.
	///
	/// Supported attach types by program type (found in the Linux kernel functions `bpf_prog_attach()` and `attach_type_to_prog_type()`).
	///
	/// * `CgroupFileDescriptor`.
	///   * `BPF_PROG_TYPE_CGROUP_SKB`.
	///     * `BPF_CGROUP_INET_INGRESS`.
	///     * `BPF_CGROUP_INET_EGRESS`.
	///   * `BPF_PROG_TYPE_CGROUP_SOCK`
	///     * `BPF_CGROUP_INET_SOCK_CREATE`.
	///     * `BPF_CGROUP_INET4_POST_BIND`.
	///     * `BPF_CGROUP_INET6_POST_BIND`.
	///   * `BPF_PROG_TYPE_CGROUP_SOCK_ADDR`.
	///     * `BPF_CGROUP_INET4_BIND`.
	///     * `BPF_CGROUP_INET6_BIND`.
	///     * `BPF_CGROUP_INET4_CONNECT`.
	///     * `BPF_CGROUP_INET6_CONNECT`.
	///     * `BPF_CGROUP_UDP4_SENDMSG`.
	///     * `BPF_CGROUP_UDP6_SENDMSG`.
	///     * `BPF_CGROUP_UDP4_RECVMSG`.
	///     * `BPF_CGROUP_UDP6_RECVMSG`.
	///   * `BPF_PROG_TYPE_SOCK_OPS`.
	///     * `BPF_CGROUP_SOCK_OPS`.
	///   * `BPF_PROG_TYPE_CGROUP_DEVICE`.
	///     * `BPF_CGROUP_DEVICE`.
	///   * `BPF_CGROUP_SYSCTL`.
	///     * `BPF_PROG_TYPE_CGROUP_SYSCTL`.
	///   * `BPF_PROG_TYPE_CGROUP_SOCKOPT`.
	///     * `BPF_CGROUP_GETSOCKOPT`.
	///     * `BPF_CGROUP_SETSOCKOPT`.
	/// * `NetworkNamespaceFileDescriptor`.
	///   * `BPF_FLOW_DISSECTOR`.
	///     * `BPF_PROG_TYPE_FLOW_DISSECTOR`.
	/// * `LinuxInfraRedRemoteControlMode2FileDescriptor`.
	///   * `BPF_LIRC_MODE2`.
	///     * `BPF_PROG_TYPE_LIRC_MODE2`.
	///
	/// * `sock_map_get_from_fd` (A `MapFileDescriptor` for a map of type `type, BPF_MAP_TYPE_SOCKMAP` or `BPF_MAP_TYPE_SOCKHASH`).
	///   * `BPF_PROG_TYPE_SK_MSG`.
	///     * `BPF_SK_MSG_VERDICT`.
	///   * `BPF_PROG_TYPE_SK_SKB`.
	///     * `BPF_SK_SKB_STREAM_PARSER`.
	///     * `BPF_SK_SKB_STREAM_VERDICT`.
	#[inline(always)]
	fn detach(&self, program_attachment_type: Self::ProgramAttachmentType, attach_program: Option<&ExtendedBpfProgramFileDescriptor>)
	{
		let mut attr = bpf_attr::default();
		attr.program_attach_or_detach = BpfCommandProgramAttachOrDetach
		{
			target_fd: self.as_raw_fd(),
			attach_bpf_fd: match attach_program
			{
				None => 0,
				Some(file_descriptor) => file_descriptor.as_raw_fd(),
			},
			attach_type: program_attachment_type.to_bpf_attach_type(),
			attach_flags: BPF_PROG_ATTACH_flags::empty(),
			replace_bpf_fd: 0
		};
		
		match attr.syscall(bpf_cmd::BPF_PROG_DETACH).as_usize()
		{
			0 => (),
			
			SystemCallResult::EINVAL_usize => panic!("Invalid attr or invalid attachment type to detach"),
			SystemCallResult::EPERM_usize => panic!("Permission denied"),
			unexpected_error @ SystemCallResult::InclusiveErrorRangeStartsFrom_usize ..= SystemCallResult::InclusiveErrorRangeEndsAt_usize => unexpected_error!(bpf, BPF_PROG_DETACH, SystemCallResult::usize_to_system_call_error_number(unexpected_error)),
			
			unexpected @ _ => unexpected_result!(bpf, BPF_PROG_DETACH, unexpected),
		}
	}
}
