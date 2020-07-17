// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// * `BPF_PROG_TYPE_CGROUP_SKB`.
///   * `BPF_CGROUP_INET_INGRESS`.
///   * `BPF_CGROUP_INET_EGRESS`.
/// * `BPF_PROG_TYPE_CGROUP_SOCK`
///   * `BPF_CGROUP_INET_SOCK_CREATE`.
///   * `BPF_CGROUP_INET4_POST_BIND`.
///   * `BPF_CGROUP_INET6_POST_BIND`.
/// * `BPF_PROG_TYPE_CGROUP_SOCK_ADDR`.
///   * `BPF_CGROUP_INET4_BIND`.
///   * `BPF_CGROUP_INET6_BIND`.
///   * `BPF_CGROUP_INET4_CONNECT`.
///   * `BPF_CGROUP_INET6_CONNECT`.
///   * `BPF_CGROUP_UDP4_SENDMSG`.
///   * `BPF_CGROUP_UDP6_SENDMSG`.
///   * `BPF_CGROUP_UDP4_RECVMSG`.
///   * `BPF_CGROUP_UDP6_RECVMSG`.
/// * `BPF_PROG_TYPE_SOCK_OPS`.
///   * `BPF_CGROUP_SOCK_OPS`.
/// * `BPF_PROG_TYPE_CGROUP_DEVICE`.
///   * `BPF_CGROUP_DEVICE`.
/// * `BPF_PROG_TYPE_CGROUP_SYSCTL`.
///   * `BPF_CGROUP_SYSCTL`.
/// * `BPF_PROG_TYPE_CGROUP_SOCKOPT`.
///   * `BPF_CGROUP_GETSOCKOPT`.
///   * `BPF_CGROUP_SETSOCKOPT`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u32)]
pub enum CgroupProgramAttachmentType
{
	/// `BPF_CGROUP_INET_INGRESS`.
	///
	/// Also known as the ELF section `cgroup_skb/ingress`.
	/// Legacy as libbpf's `cgroup_skb`.
	/// Legacy with the ELF section `cgroup/skb`.
	///
	/// The capability `CAP_SYS_ADMIN` is required.
	SocketBufferIngress = bpf_attach_type::BPF_CGROUP_INET_INGRESS as u32,

	/// `BPF_CGROUP_INET_EGRESS`.
	///
	/// Also known as the ELF section `cgroup_skb/egress`.
	/// Legacy as libbpf's `cgroup_skb`.
	/// Legacy with the ELF section `cgroup/skb`.
	///
	/// The capability `CAP_SYS_ADMIN` is required.
	SocketBufferEgress = bpf_attach_type::BPF_CGROUP_INET_EGRESS as u32,

	/// `BPF_CGROUP_INET_SOCK_CREATE`.
	///
	/// Also known as in libpbf as `cgroup_sock`.
	/// Also known as the ELF section `cgroup/sock`.
	CreateSocket = bpf_attach_type::BPF_CGROUP_INET_SOCK_CREATE as u32,

	/// `BPF_CGROUP_INET4_POST_BIND`.
	/// 
	/// Also known as the ELF section `cgroup/post_bind4`.
	PostBindInternetProtocolVersion4 = bpf_attach_type::BPF_CGROUP_INET4_POST_BIND as u32,

	/// `BPF_CGROUP_INET6_POST_BIND`.
	/// 
	/// Also known as the ELF section `cgroup/post_bind6`.
	PostBindInternetProtocolVersion6 = bpf_attach_type::BPF_CGROUP_INET6_POST_BIND as u32,

	/// `BPF_CGROUP_INET4_BIND`.
	///
	/// Also known as the ELF section `cgroup/bind4`.
	/// ?Overlaps with libbpf's `cgroup_sock_addr`.
	BindInternetProtocolVersion4 = bpf_attach_type::BPF_CGROUP_INET4_BIND as u32,

	/// `BPF_CGROUP_INET6_BIND`
	///
	/// Also known as the ELF section `cgroup/bind6`.
	/// ?Overlaps with libbpf's `cgroup_sock_addr`.
	BindInternetProtocolVersion6 = bpf_attach_type::BPF_CGROUP_INET6_BIND as u32,

	/// `BPF_CGROUP_INET4_CONNECT`.
	///
	/// Also known as the ELF section `cgroup/connect4`.
	/// ?Overlaps with libbpf's `cgroup_sock_addr`.
	ConnectInternetProtocolVersion4 = bpf_attach_type::BPF_CGROUP_INET4_CONNECT as u32,

	/// `BPF_CGROUP_INET6_CONNECT`.
	///
	/// Also known as the ELF section `cgroup/connect6`.
	/// ?Overlaps with libbpf's `cgroup_sock_addr`.
	ConnectInternetProtocolVersion6 = bpf_attach_type::BPF_CGROUP_INET6_CONNECT as u32,

	/// `BPF_CGROUP_UDP4_SENDMSG`.
	///
	/// Also known as the ELF section `cgroup/sendmsg4`.
	/// ?Overlaps with libbpf's `cgroup_sock_addr`.
	SendMessageUdpOverInternetProtocolVersion4 = bpf_attach_type::BPF_CGROUP_UDP4_SENDMSG as u32,

	/// `BPF_CGROUP_UDP6_SENDMSG`.
	///
	/// Also known as the ELF section `cgroup/sendmsg6`.
	/// ?Overlaps with libbpf's `cgroup_sock_addr`.
	SendMessageUdpOverInternetProtocolVersion6 = bpf_attach_type::BPF_CGROUP_UDP6_SENDMSG as u32,

	/// `BPF_CGROUP_UDP4_RECVMSG`.
	///
	/// Also known as the ELF section `cgroup/recvmsg4`.
	/// ?Overlaps with libbpf's `cgroup_sock_addr`.
	ReceiveMessageUdpOverInternetProtocolVersion4 = bpf_attach_type::BPF_CGROUP_UDP4_RECVMSG as u32,

	/// `BPF_CGROUP_UDP6_RECVMSG`.
	///
	/// Also known as the ELF section `cgroup/recvmsg6`.
	/// ?Overlaps with libbpf's `cgroup_sock_addr`.
	ReceiveMessageUdpOverInternetProtocolVersion6 = bpf_attach_type::BPF_CGROUP_UDP6_RECVMSG as u32,

	/// `BPF_CGROUP_SOCK_OPS`.
	///
	/// Also known as in libbpf as `sock_ops`.
	/// Also known as the ELF section `sockops`.
	SocketOps = bpf_attach_type::BPF_CGROUP_SOCK_OPS as u32,

	/// `BPF_CGROUP_DEVICE`.
	///
	/// Also known as in libbpf as `cgroup_device`.
	/// Also known as the ELF section `cgroup/dev`.
	Device = bpf_attach_type::BPF_CGROUP_DEVICE as u32,
	
	/// `BPF_CGROUP_SETSOCKOPT`.
	///
	/// ?Overlaps with libbpf's `cgroup_sockopt`.
	/// Also known as the ELF section `cgroup/getsockopt`.
	SetSocketOptions = bpf_attach_type::BPF_CGROUP_SETSOCKOPT as u32,

	/// `BPF_CGROUP_GETSOCKOPT`.
	///
	/// ?Overlaps with libbpf's `cgroup_sockopt`.
	/// Also known as the ELF section `cgroup/setsockopt`.
	GetSocketOptions = bpf_attach_type::BPF_CGROUP_GETSOCKOPT as u32,
	
	/// Also known as in libbpf as `cgroup_sysctl`.
	/// Also known as the ELF section `cgroup/sysctl`.
	SysControl = bpf_attach_type::BPF_CGROUP_SYSCTL as u32,
}

impl ProgramAttachmentType for CgroupProgramAttachmentType
{
	#[inline(always)]
	fn to_bpf_attach_type(self) -> bpf_attach_type
	{
		unsafe { transmute(self as u32) }
	}
}

impl CgroupProgramAttachmentType
{
	#[inline(always)]
	pub(crate) fn to_bpf_prog_type_and_bpf_attach_type(self) -> (bpf_prog_type, bpf_attach_type)
	{
		use self::bpf_attach_type::*;
		use self::bpf_prog_type::*;
		use self::CgroupProgramAttachmentType::*;
		
		match self
		{
			SocketBufferIngress => (BPF_PROG_TYPE_CGROUP_SKB, BPF_CGROUP_INET_INGRESS),
			SocketBufferEgress => (BPF_PROG_TYPE_CGROUP_SKB, BPF_CGROUP_INET_EGRESS),
			CreateSocket => (BPF_PROG_TYPE_CGROUP_SOCK, BPF_CGROUP_INET_SOCK_CREATE),
			PostBindInternetProtocolVersion4 => (BPF_PROG_TYPE_CGROUP_SOCK, BPF_CGROUP_INET4_POST_BIND),
			PostBindInternetProtocolVersion6 => (BPF_PROG_TYPE_CGROUP_SOCK, BPF_CGROUP_INET6_POST_BIND),
			Device => (BPF_PROG_TYPE_CGROUP_DEVICE, BPF_CGROUP_DEVICE),
			SocketOps => (BPF_PROG_TYPE_SOCK_OPS, BPF_CGROUP_SOCK_OPS),
			BindInternetProtocolVersion4 => (BPF_PROG_TYPE_CGROUP_SOCK_ADDR, BPF_CGROUP_INET4_BIND),
			BindInternetProtocolVersion6 => (BPF_PROG_TYPE_CGROUP_SOCK_ADDR, BPF_CGROUP_INET6_BIND),
			ConnectInternetProtocolVersion4 => (BPF_PROG_TYPE_CGROUP_SOCK_ADDR, BPF_CGROUP_INET4_CONNECT),
			ConnectInternetProtocolVersion6 => (BPF_PROG_TYPE_CGROUP_SOCK_ADDR, BPF_CGROUP_INET6_CONNECT),
			SendMessageUdpOverInternetProtocolVersion4 => (BPF_PROG_TYPE_CGROUP_SOCK_ADDR, BPF_CGROUP_UDP4_SENDMSG),
			SendMessageUdpOverInternetProtocolVersion6 => (BPF_PROG_TYPE_CGROUP_SOCK_ADDR, BPF_CGROUP_UDP6_SENDMSG),
			ReceiveMessageUdpOverInternetProtocolVersion4 => (BPF_PROG_TYPE_CGROUP_SOCK_ADDR, BPF_CGROUP_UDP4_RECVMSG),
			ReceiveMessageUdpOverInternetProtocolVersion6 => (BPF_PROG_TYPE_CGROUP_SOCK_ADDR, BPF_CGROUP_UDP6_RECVMSG),
			SetSocketOptions => (BPF_PROG_TYPE_CGROUP_SOCKOPT, BPF_CGROUP_SETSOCKOPT),
			GetSocketOptions => (BPF_PROG_TYPE_CGROUP_SOCKOPT, BPF_CGROUP_GETSOCKOPT),
			SysControl => (BPF_PROG_TYPE_CGROUP_SYSCTL, BPF_CGROUP_SYSCTL),
		}
	}
}
