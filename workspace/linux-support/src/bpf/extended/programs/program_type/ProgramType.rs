// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Program type.
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum ProgramType<'name>
{
	/// Also known as in libpbf as `scoket_filter`.
	/// Also known as the ELF section `socket`.
	///
	/// The capability `CAP_SYS_ADMIN` is required.
	SocketFilter(#[serde(default)] CommonProgramTypeDetails),
	
	/// Also known as in libpbf as `kprobe`.
	/// Also known as the ELF sections as `kprobe/` `uprobe/`, `kretprobe/` and `uretprobe/`.
	KProbe(#[serde(default)] CommonProgramTypeDetails),
	
	/// Also known as in libpbf as `sched_cls`.
	/// Also known as the ELF section `classifier`.
	SchedulerClassifier(#[serde(default)] CommonProgramTypeDetails),

	/// Also known as in libpbf as `sched_act`.
	/// Also known as the ELF section `action`.
	SchedulerAction(#[serde(default)] CommonProgramTypeDetails),
	
	/// Also known as in libpbf as `tracepoint`.
	/// Also known as the ELF section `tracepoint/` and `tp/`.
	TracePoint(#[serde(default)] CommonProgramTypeDetails),
	
	/// Also known as in libpbf as `raw_tracepoint`.
	/// Also known as the ELF section `raw_tracepoint/` and `raw_tp/`.
	RawTracePoint(#[serde(default)] CommonProgramTypeDetails),
	
	/// Also known as in libpbf as `xdp`.
	/// Also known as the ELF section `xdp`.
	Xdp(#[serde(default)] CommonProgramTypeDetails),
	
	/// Also known as in libpbf as `perf_event`.
	/// Also known as the ELF section `perf_event`.
	PerfEvent(#[serde(default)] CommonProgramTypeDetails),
	
	/// Lightweight Tunnel in.
	///
	/// Also known as in libpbf as `lwt_in`.
	/// Also known as the ELF section `lwt_in`.
	LightweightTunnelIn(#[serde(default)] CommonProgramTypeDetails),
	
	/// Lightweight Tunnel out.
	///
	/// Also known as in libpbf as `lwt_out`.
	/// Also known as the ELF section `lwt_out`.
	LightweightTunnelOut(#[serde(default)] CommonProgramTypeDetails),
	
	/// Lightweight Tunnel transmit.
	///
	/// Also known as in libpbf as `lwt_xmit`.
	/// Also known as the ELF section `lwt_xmit`.
	LightweightTunnelTransmit(#[serde(default)] CommonProgramTypeDetails),
	
	/// Lightweight Tunnel ?
	///
	/// Also known as in libpbf as `lwt_seg6local`.
	/// Also known as the ELF section `lwt_seg6local`.
	LightweightTunnelSeg6Local(#[serde(default)] CommonProgramTypeDetails),
	
	/// InfraRed Raw Mode 2.
	///
	/// Also known as in libbpf as `lirc_mode2`.
	/// Also known as the ELF section `lirc_mode2`.
	InfraRedRawMode2(#[serde(default)] CommonProgramTypeDetails),
	
	/// Also known as in libbpf as `flow_dissector`.
	/// Also known as the ELF section `flow_dissector`.
	FlowDissector(#[serde(default)] CommonProgramTypeDetails),
	
	/// Cgroup.
	Cgroup(#[serde(default)] CommonProgramTypeDetails, CgroupProgramAttachmentType),
	
	/// Also known as the ELF section `sk_skb/stream_parser`.
	/// Legacy as libbpf's `sk_skb`.
	/// Legacy with the ELF section `sk_skb`.
	SocketBufferStreamParser(#[serde(default)] CommonProgramTypeDetails),
	
	/// Also known as the ELF section `sk_skb/stream_verdict`.
	/// Legacy as libbpf's `sk_skb`.
	/// Legacy with the ELF section `sk_skb`.
	SocketBufferStreamVerdict(#[serde(default)] CommonProgramTypeDetails),
	
	/// Also known as in libbpf as `sk_msg`.
	/// Also known as the ELF section `sk_msg`.
	SocketMessage(#[serde(default)] CommonProgramTypeDetails),
	
	/// Also known as in libbpf as `struct_ops`.
	/// Also known as the ELF section `struct_ops`.
	StructOps(#[serde(default)] AttachToBpfTypeIdentifier),
	
	/// A type of tracing program.
	///
	/// Also known as the ELF section `lsm/`.
	LinuxSecurityModule(#[serde(default)] AttachToBpfTypeIdentifier),
	
	/// Also known as in libbpf as `tracing`.
	/// Also known as the ELF section `tp_btf/`.
	///
	/// A type of tracing program.
	TracingRawTracePoint(#[serde(default)] AttachProgramTypeDetails<'name>),
	
	/// Also known as in libbpf as `tracing`.
	/// Also known as the ELF section `fentry/`.
	///
	/// A type of tracing program.
	TracingFunctionEntry(#[serde(default)] AttachProgramTypeDetails<'name>),
	
	/// Also known as in libbpf as `tracing`.
	/// Also known as the ELF section `fmod_ret/`.
	///
	/// A type of tracing program.
	TracingModifyReturn(#[serde(default)] AttachProgramTypeDetails<'name>),
	
	/// Also known as in libbpf as `tracing`.
	/// Also known as the ELF section `fmod_ret/`.
	///
	/// A type of tracing program.
	TracingFunctionExit(#[serde(default)] AttachProgramTypeDetails<'name>),
	
	/// Also known as in libbpf as `ext`.
	/// Also known as the ELF section `fexit/`.
	///
	/// A type of tracing program.
	Ext(#[serde(default)] AttachProgramTypeDetails<'name>),
	
	/// Also known as in libbpf as `sk_reuseport`.
	SocketReusePort(#[serde(default)] CommonProgramTypeDetails),
	
	/// Also known as in libbpf as `raw_tracepoint_writable`.
	RawTracePointWritable(#[serde(default)] CommonProgramTypeDetails),
}

impl<'name> ProgramType<'name>
{
	pub(crate) fn to_values(&self, extended_bpf_program_file_descriptor_labels_map: &FileDescriptorLabelsMap<ExtendedBpfProgramFileDescriptor>) -> Result<(bpf_prog_type, bpf_attach_type, BtfTypeIdentifier, RawFd, u32, Option<NetworkInterfaceIndex>), ProgramError>
	{
		use self::bpf_attach_type::*;
		use self::bpf_prog_type::*;
		use self::ProgramType::*;
		
		match self
		{
			// `expected_attach_type` is ignored in `kernel/bpf/syscall.c`.
			SocketFilter(program_details) => program_details.to_values(BPF_PROG_TYPE_SOCKET_FILTER, Self::ignored()),
			
			// `expected_attach_type` is ignored in `kernel/bpf/syscall.c`.
			KProbe(program_details) => program_details.to_values(BPF_PROG_TYPE_KPROBE, Self::ignored()),
			
			// `expected_attach_type` is ignored in `kernel/bpf/syscall.c`.
			SchedulerClassifier(program_details) => program_details.to_values(BPF_PROG_TYPE_SCHED_CLS, Self::ignored()),
			
			// `expected_attach_type` is ignored in `kernel/bpf/syscall.c`.
			SchedulerAction(program_details) => program_details.to_values(BPF_PROG_TYPE_SCHED_ACT, Self::ignored()),
			
			// `expected_attach_type` is ignored in `kernel/bpf/syscall.c`.
			TracePoint(program_details) => program_details.to_values(BPF_PROG_TYPE_TRACEPOINT, Self::ignored()),
			
			// `expected_attach_type` is ignored in `kernel/bpf/syscall.c`.
			RawTracePoint(program_details) => program_details.to_values(BPF_PROG_TYPE_RAW_TRACEPOINT, Self::ignored()),
			
			// `expected_attach_type` is ignored in `kernel/bpf/syscall.c`.
			Xdp(program_details) => program_details.to_values(BPF_PROG_TYPE_XDP, Self::ignored()),
			
			// `expected_attach_type` is ignored in `kernel/bpf/syscall.c`.
			PerfEvent(program_details) => program_details.to_values(BPF_PROG_TYPE_PERF_EVENT, Self::ignored()),
			
			// `expected_attach_type` is ignored in `kernel/bpf/syscall.c`.
			LightweightTunnelIn(program_details) => program_details.to_values(BPF_PROG_TYPE_LWT_IN, Self::ignored()),
			
			// `expected_attach_type` is ignored in `kernel/bpf/syscall.c`.
			LightweightTunnelOut(program_details) => program_details.to_values(BPF_PROG_TYPE_LWT_OUT, Self::ignored()),
			
			// `expected_attach_type` is ignored in `kernel/bpf/syscall.c`.
			LightweightTunnelTransmit(program_details) => program_details.to_values(BPF_PROG_TYPE_LWT_XMIT, Self::ignored()),
			
			// `expected_attach_type` is ignored in `kernel/bpf/syscall.c`.
			LightweightTunnelSeg6Local(program_details) => program_details.to_values(BPF_PROG_TYPE_LWT_SEG6LOCAL, Self::ignored()),
			
			// `expected_attach_type` is ignored in `kernel/bpf/syscall.c`.
			InfraRedRawMode2(program_details) => program_details.to_values(BPF_PROG_TYPE_LIRC_MODE2, BPF_LIRC_MODE2),
			
			// `expected_attach_type` is ignored in `kernel/bpf/syscall.c`.
			FlowDissector(program_details) => program_details.to_values(BPF_PROG_TYPE_FLOW_DISSECTOR, BPF_FLOW_DISSECTOR),
			
			&Cgroup(ref program_details, cgroup_program_attachment_type) =>
			{
				let (program_type, expected_attach_type) = cgroup_program_attachment_type.to_bpf_prog_type_and_bpf_attach_type();
				program_details.to_values(program_type, expected_attach_type)
			}
			
			// `expected_attach_type` is ignored in `kernel/bpf/syscall.c`.
			SocketBufferStreamParser(program_details) => program_details.to_values(BPF_PROG_TYPE_SK_SKB, BPF_SK_SKB_STREAM_PARSER),
			
			// `expected_attach_type` is ignored in `kernel/bpf/syscall.c`.
			SocketBufferStreamVerdict(program_details) => program_details.to_values(BPF_PROG_TYPE_SK_SKB, BPF_SK_SKB_STREAM_VERDICT),
			
			// `expected_attach_type` is ignored in `kernel/bpf/syscall.c`.
			SocketMessage(program_details) => program_details.to_values(BPF_PROG_TYPE_SK_MSG, BPF_SK_MSG_VERDICT),
			
			// `expected_attach_type` is ignored in `kernel/bpf/syscall.c`.
			StructOps(program_details) => program_details.to_values(BPF_PROG_TYPE_STRUCT_OPS, Self::ignored()),
			
			// `expected_attach_type` is validated in `bpf_tracing_prog_attach()` in `kernel/bpf/syscall.c`.
			LinuxSecurityModule(program_details) => program_details.to_values(BPF_PROG_TYPE_LSM, BPF_LSM_MAC),
			
			TracingRawTracePoint(program_details) => program_details.to_values(BPF_PROG_TYPE_TRACING, BPF_TRACE_RAW_TP, extended_bpf_program_file_descriptor_labels_map),
			TracingFunctionEntry(program_details) => program_details.to_values(BPF_PROG_TYPE_TRACING, BPF_TRACE_FENTRY, extended_bpf_program_file_descriptor_labels_map),
			TracingModifyReturn(program_details) => program_details.to_values(BPF_PROG_TYPE_TRACING, BPF_MODIFY_RETURN, extended_bpf_program_file_descriptor_labels_map),
			TracingFunctionExit(program_details) => program_details.to_values(BPF_PROG_TYPE_TRACING, BPF_TRACE_FEXIT, extended_bpf_program_file_descriptor_labels_map),
			
			// `expected_attach_type` is explicitly tested to be zero in `bpf_prog_load_check_attach()` in `kernel/bpf/syscall.c`.
			// `expected_attach_type` is explicitly tested to be zero in `bpf_tracing_prog_attach()` in `kernel/bpf/syscall.c`.
			Ext(program_details) => program_details.to_values(BPF_PROG_TYPE_EXT, Self::ignored(), extended_bpf_program_file_descriptor_labels_map),
			
			// `expected_attach_type` is ignored in `kernel/bpf/syscall.c`.
			SocketReusePort(program_details) => program_details.to_values(BPF_PROG_TYPE_SK_REUSEPORT, Self::ignored()),
			
			// `expected_attach_type` is ignored in `kernel/bpf/syscall.c`.
			RawTracePointWritable(program_details) => program_details.to_values(BPF_PROG_TYPE_RAW_TRACEPOINT_WRITABLE, Self::ignored()),
		}
	}
	
	fn ignored() -> bpf_attach_type
	{
		unsafe { zeroed() }
	}
}
