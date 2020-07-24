// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Receive and transmit memory ring queues.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OwnedReceiveTransmitMemoryRingQueues
{
	user_memory: ManuallyDrop<UserMemory>,
	
	/// receive is `xsk_ring_cons`.
	/// transmit is `xsk_ring_prod`.
	receive_and_transmit: ReceiveOrTransmitOrBoth<XskRingQueue>,
	
	xdp_extended_bpf_program: ManuallyDrop<ExtendedBpfProgramFileDescriptor>,
}

impl Drop for OwnedReceiveTransmitMemoryRingQueues
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe
		{
			ManuallyDrop::drop(&mut self.user_memory);
			ManuallyDrop::drop(&mut self.xdp_extended_bpf_program);
		}
	}
}

impl ReceiveTransmitMemoryRingQueues for OwnedReceiveTransmitMemoryRingQueues
{
	#[inline(always)]
	fn user_memory_and_receive_transmit(&self) -> (&UserMemory, &ReceiveOrTransmitOrBoth<XskRingQueue>)
	{
		(&self.user_memory, &self.receive_and_transmit)
	}
}

impl OwnedReceiveTransmitMemoryRingQueues
{
	/*

static int xsk_setup_xdp_prog(struct xsk_socket *xsk)
{
	__u32 prog_id = 0;
	int err;

	err = bpf_get_link_xdp_id(xsk->ifindex, &prog_id,
				  xsk->config.xdp_flags);
	if (err)
		return err;

	if (!prog_id) {
		err = xsk_create_bpf_maps(xsk);
		if (err)
			return err;

		err = xsk_load_xdp_prog(xsk);
		if (err) {
			xsk_delete_bpf_maps(xsk);
			return err;
		}
	} else {
		xsk->prog_fd = bpf_prog_get_fd_by_id(prog_id);
		if (xsk->prog_fd < 0)
			return -errno;
		err = xsk_lookup_bpf_maps(xsk);
		if (err) {
			close(xsk->prog_fd);
			return err;
		}
	}

	if (xsk->rx)
		err = xsk_set_bpf_maps(xsk);
	if (err) {
		xsk_delete_bpf_maps(xsk);
		close(xsk->prog_fd);
		return err;
	}

	return 0;
}
	 */

// static int xsk_load_xdp_prog(struct xsk_socket *xsk)
// {
// 	static const int log_buf_size = 16 * 1024;
// 	char log_buf[log_buf_size];
// 	int err, prog_fd;
//
// 	/* This is the C-program:
// 	 * SEC("xdp_sock") int xdp_sock_prog(struct xdp_md *ctx)
// 	 * {
// 	 *     int ret, index = ctx->rx_queue_index;
// 	 *
// 	 *     // A set entry here means that the correspnding queue_id
// 	 *     // has an active AF_XDP socket bound to it.
// 	 *     ret = bpf_redirect_map(&xsks_map, index, XDP_PASS);
// 	 *     if (ret > 0)
// 	 *         return ret;
// 	 *
// 	 *     // Fallback for pre-5.3 kernels, not supporting default
// 	 *     // action in the flags parameter.
// 	 *     if (bpf_map_lookup_elem(&xsks_map, &index))
// 	 *         return bpf_redirect_map(&xsks_map, index, 0);
// 	 *     return XDP_PASS;
// 	 * }
// 	 */
// 	struct bpf_insn prog[] = {
// 		/* r2 = *(u32 *)(r1 + 16) */
// 		BPF_LDX_MEM(BPF_W, BPF_REG_2, BPF_REG_1, 16),
// 		/* *(u32 *)(r10 - 4) = r2 */
// 		BPF_STX_MEM(BPF_W, BPF_REG_10, BPF_REG_2, -4),
// 		/* r1 = xskmap[] */
// 		BPF_LD_MAP_FD(BPF_REG_1, xsk->xsks_map_fd),
// 		/* r3 = XDP_PASS */
// 		BPF_MOV64_IMM(BPF_REG_3, 2),
// 		/* call bpf_redirect_map */
// 		BPF_EMIT_CALL(BPF_FUNC_redirect_map),
// 		/* if w0 != 0 goto pc+13 */
// 		BPF_JMP32_IMM(BPF_JSGT, BPF_REG_0, 0, 13),
// 		/* r2 = r10 */
// 		BPF_MOV64_REG(BPF_REG_2, BPF_REG_10),
// 		/* r2 += -4 */
// 		BPF_ALU64_IMM(BPF_ADD, BPF_REG_2, -4),
// 		/* r1 = xskmap[] */
// 		BPF_LD_MAP_FD(BPF_REG_1, xsk->xsks_map_fd),
// 		/* call bpf_map_lookup_elem */
// 		BPF_EMIT_CALL(BPF_FUNC_map_lookup_elem),
// 		/* r1 = r0 */
// 		BPF_MOV64_REG(BPF_REG_1, BPF_REG_0),
// 		/* r0 = XDP_PASS */
// 		BPF_MOV64_IMM(BPF_REG_0, 2),
// 		/* if r1 == 0 goto pc+5 */
// 		BPF_JMP_IMM(BPF_JEQ, BPF_REG_1, 0, 5),
// 		/* r2 = *(u32 *)(r10 - 4) */
// 		BPF_LDX_MEM(BPF_W, BPF_REG_2, BPF_REG_10, -4),
// 		/* r1 = xskmap[] */
// 		BPF_LD_MAP_FD(BPF_REG_1, xsk->xsks_map_fd),
// 		/* r3 = 0 */
// 		BPF_MOV64_IMM(BPF_REG_3, 0),
// 		/* call bpf_redirect_map */
// 		BPF_EMIT_CALL(BPF_FUNC_redirect_map),
// 		/* The jumps are to this instruction */
// 		BPF_EXIT_INSN(),
// 	};
// 	size_t insns_cnt = sizeof(prog) / sizeof(struct bpf_insn);
//
// 	prog_fd = bpf_load_program(BPF_PROG_TYPE_XDP, prog, insns_cnt,
// 				   "LGPL-2.1 or BSD-2-Clause", 0, log_buf,
// 				   log_buf_size);
// 	if (prog_fd < 0) {
// 		pr_warn("BPF log buffer:\n%s", log_buf);
// 		return prog_fd;
// 	}
//
// 	err = bpf_set_link_xdp_fd(xsk->ifindex, prog_fd, xsk->config.xdp_flags);
// 	if (err) {
// 		close(prog_fd);
// 		return err;
// 	}
//
// 	xsk->prog_fd = prog_fd;
// 	return 0;
// }
	

	
	#[inline(always)]
	fn new(user_memory: UserMemory, xdp_extended_bpf_program: ExtendedBpfProgramFileDescriptor, network_interface_index: NetworkInterfaceIndex, queue_identifier: QueueIdentifier, ring_queue_depths: ReceiveOrTransmitOrBoth<RingQueueDepth, RingQueueDepth>) -> Result<Self, SocketCreationOrBindError>
	{
		
		let user_memory_socket_file_descriptor = &user_memory.user_memory_socket_file_descriptor;
		let receive_and_transmit = Self::construct(user_memory_socket_file_descriptor, network_interface_index, queue_identifier, ring_queue_depths, XdpSocketAddressFlags::empty(), 0)?;
		Ok
		(
			Self
			{
				user_memory: ManuallyDrop::new(user_memory),
				receive_and_transmit,
				xdp_extended_bpf_program: ManuallyDrop::new(xdp_extended_bpf_program),
			}
		)
	}
	
	/// Treats `self` as master; returns a slave.
	///
	/// When all slaves have been dropped the master is dropped.
	/// This ensures the correct ordering of close for socket file descriptors.
	///
	/// The `xdp_extended_bpf_program` in use with `self` must be suitable for use with shared user memory.
	pub fn share(&self, network_interface_index: NetworkInterfaceIndex, queue_identifier: QueueIdentifier, ring_queue_depths: ReceiveOrTransmitOrBoth<RingQueueDepth, RingQueueDepth>) -> Result<SharedReceiveTransmitMemoryRingQueues, SocketCreationOrBindError>
	{
		let xsk_socket_file_descriptor = ExpressDataPathSocketFileDescriptor::new()?;
		let receive_and_transmit = Self::construct(&xsk_socket_file_descriptor, network_interface_index, queue_identifier, ring_queue_depths, XdpSocketAddressFlags::SharedUserMemory, self.user_memory.user_memory_socket_file_descriptor.as_raw_fd())?;
		Ok
		(
			SharedReceiveTransmitMemoryRingQueues
			{
				user_memory: &self.user_memory,
				receive_and_transmit,
				xsk_socket_file_descriptor,
			}
		)
	}
	
	/// Based on `libbpf`'s `xsk_socket__create()`.
	fn construct(xsk_socket_file_descriptor: &ExpressDataPathSocketFileDescriptor, network_interface_index: NetworkInterfaceIndex, queue_identifier: QueueIdentifier, ring_queue_depths: ReceiveOrTransmitOrBoth<RingQueueDepth, RingQueueDepth>, sxdp_flags: XdpSocketAddressFlags, sxdp_shared_umem_fd: RawFd) -> Result<ReceiveOrTransmitOrBoth<XskRingQueue>, SocketBindError>
	{
		ring_queue_depths.use_value
		(
			|receive_ring_queue_depth| xsk_socket_file_descriptor.set_xdp_socket_option_receive_ring(receive_ring_queue_depth),
			|transmit_ring_queue_depth| xsk_socket_file_descriptor.set_xdp_socket_option_transmit_ring(transmit_ring_queue_depth),
		);
		
		// NOTE: Valid memory map offsets are not available until the socket options above have been set.
		let memory_map_offsets = xsk_socket_file_descriptor.get_memory_map_offsets();
		
		let receive_and_transmit = ring_queue_depths.map
		(
			|receive_ring_queue_depth| XskRingQueue::from_receive_memory_map_offsets(&xsk_socket_file_descriptor, &memory_map_offsets, receive_ring_queue_depth, defaults),
			|transmit_ring_queue_depth| XskRingQueue::from_transmit_memory_map_offsets(&xsk_socket_file_descriptor, &memory_map_offsets, transmit_ring_queue_depth, defaults),
		);
		
		let socket_address = sockaddr_xdp
		{
			sxdp_family: AF_XDP as u16,
			sxdp_flags,
			sxdp_ifindex: network_interface_index,
			sxdp_queue_id: queue_identifier,
			sxdp_shared_umem_fd,
		};
		
		const len: u32 = size_of::<sockaddr_xdp>() as u32;
		bind_socket(xsk_socket_file_descriptor, &socket_address)?;
		
		// load a program if not using shared user memory.
		
		let owned = xsk_socket_file_descriptor == user_memory_socket_file_descriptor;
		if owned
		{
			xsk_setup_xdp_prog(xsk);
		}
		
		Ok(receive_and_transmit)
	}
}
