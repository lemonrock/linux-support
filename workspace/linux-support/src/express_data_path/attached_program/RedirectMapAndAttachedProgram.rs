// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
pub struct RedirectMapAndAttachedProgram
{
	/// Created and manipulated by functions such as `xsk_set_bpf_maps()`, `xsk_lookup_bpf_maps()`, `xsk_delete_bpf_maps()` and `xsk_create_bpf_maps()` in Linux source `tools/lib/bpf/xsk.c`.
	///
	/// Supplied with size information (`Channels`) from the function `xsk_get_max_queues()` in Linux source `tools/lib/bpf/xsk.c`.
	redirect_map: ManuallyDrop<ExpressDataPathRedirectSocketArrayMap>,
	
	attached_express_data_path_extended_bpf_program_file_descriptor: ManuallyDrop<ExtendedBpfProgramFileDescriptor>,
	
	attached_program_name: ProgramName,
	
	network_interface_index_if_removing_xdp_program_on_drop: Option<NetworkInterfaceIndex>,
}

impl Drop for RedirectMapAndAttachedProgram
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if let Some(network_interface_index) = self.network_interface_index_if_removing_xdp_program_on_drop
		{
			if let Ok(mut netlink_socket_file_descriptor) = NetlinkSocketFileDescriptor::open()
			{
				let _ignored = RouteNetlinkProtocol::xdp_fd_remove(&mut netlink_socket_file_descriptor, network_interface_index, &self.attached_express_data_path_extended_bpf_program_file_descriptor);
			}
		}
		
		unsafe
		{
			ManuallyDrop::drop(&mut self.attached_express_data_path_extended_bpf_program_file_descriptor);
			ManuallyDrop::drop(&mut self.redirect_map);
		}
	}
}

impl RedirectMapAndAttachedProgram
{
	/// Based on the function `xsk_set_bpf_maps()` in Linux source `tools/lib/bpf/xsk.c`.
	#[inline(always)]
	pub(super) fn insert_into_redirect_map_if_receive(&self, receive_queue_identifier: QueueIdentifier, express_data_path_socket_file_descriptor: &ExpressDataPathSocketFileDescriptor) -> Result<(), ExpressDataPathSocketCreationError>
	{
		Ok(self.redirect_map.insert(receive_queue_identifier, express_data_path_socket_file_descriptor)?)
	}
	
	/// Based on `libbpf`'s `xsk_delete_bpf_maps()`.
	#[inline(always)]
	pub(super) fn remove_receive_map_receive_queue_identifier(&self, receive_queue_identifier: QueueIdentifier) -> Result<bool, SystemCallErrorNumber>
	{
		self.redirect_map.delete(receive_queue_identifier)
	}
	
	/// This test relies on the program's `ProgramName`, which, although unique, is a value that another process on Linux could have set for its XDP eBPF.
	#[inline(always)]
	pub(super) fn is_our_owned_program_and_thus_can_not_be_shared(&self) -> bool
	{
		self.attached_program_name == Self::our_owned_program_name()
	}
	
	/// Based on the function `xsk_setup_xdp_prog()` in Linux source `tools/lib/bpf/xsk.c`.
	pub(super) fn new_suitable_for_owned_or_reuse_already_attached(network_interface_index: NetworkInterfaceIndex, settings: RedirectMapAndAttachedProgramSettings) -> Result<Self, AttachProgramError>
	{
		let RedirectMapAndAttachedProgramSettings { forcibly_overwrite_already_attached, device_offload, redirect_map_numa_node } = settings;
		
		let get_link_message_data = Self::get_link_message_data(network_interface_index)?;
		
		if forcibly_overwrite_already_attached
		{
			Self::load_and_attach_owned_memory_program(network_interface_index, device_offload, redirect_map_numa_node)
		}
		else
		{
			match get_link_message_data.attached_express_data_path_program_identifiers
			{
				Some(program_identifiers) => Self::already_attached(network_interface_index, device_offload, program_identifiers),
				
				None => Self::load_and_attach_owned_memory_program(network_interface_index, device_offload, redirect_map_numa_node),
			}
		}
	}
	
	/// Based on the function `xsk_setup_xdp_prog()` in Linux source `tools/lib/bpf/xsk.c`.
	///
	/// `Ok(ExpressDataPathRedirectSocketArrayMap, _)` is used only by `ReceiveTransmitRingQueues`.
	fn already_attached(network_interface_index: NetworkInterfaceIndex, device_offload: bool, program_identifiers: MultipleProgramIdentifiers) -> Result<Self, AttachProgramError>
	{
		use self::AttachProgramError::*;
		
		let (program_identifier, attach_mode) = program_identifiers.choose_most_performant();
		
		let attached_express_data_path_extended_bpf_program_file_descriptor = ExtendedBpfProgramFileDescriptor::from_identifier_with_access_defaults(program_identifier).map_err(CouldNotGetExistingProgramFileDescriptor)?.ok_or(NoExistingExpressDataPathProgramForAttachedExtendedBpfProgramFileDescriptor)?;
		
		let program_information = attached_express_data_path_extended_bpf_program_file_descriptor.get_information().map_err(CouldNotGetExistingProgramInformation)?;
		program_information.validate_has_program_type(bpf_prog_type::BPF_PROG_TYPE_XDP, ExistingAttachedProgramHasWrongProgramTypeForExpressDataPath)?;
		program_information.validate_attach_mode_and_device_offload_matches_program_information(attach_mode, device_offload, network_interface_index)?;
		
		let redirect_map = ExpressDataPathRedirectSocketArrayMap::rehydrate(&program_information, Self::redirect_map_name())?;
		
		Ok
		(
			Self
			{
				redirect_map: ManuallyDrop::new(redirect_map),
				attached_express_data_path_extended_bpf_program_file_descriptor: ManuallyDrop::new(attached_express_data_path_extended_bpf_program_file_descriptor),
				attached_program_name: program_information.name(),
				network_interface_index_if_removing_xdp_program_on_drop: None,
			}
		)
	}
	
	fn number_of_channels(network_interface_index: NetworkInterfaceIndex) -> Result<Channels, AttachProgramError>
	{
		let network_interface_name = NetworkInterfaceName::try_from(network_interface_index)?;
		
		use self::AttachProgramError::*;
		let (current, _maxima) = NetworkDeviceInputOutputControl::new(Cow::Owned(network_interface_name))?.number_of_channels().map_err(CouldNotGetNumberOfChannels)?.ok_or(NoSuchNetworkInterfaceIndex(network_interface_index))?.unwrap_or((Channels::Unsupported, Channels::Unsupported));
		Ok(current)
	}
	
	/// Based on the function `xsk_setup_xdp_prog()` in Linux source `tools/lib/bpf/xsk.c`.
	fn load_and_attach_owned_memory_program(network_interface_index: NetworkInterfaceIndex, device_offload: bool, redirect_map_numa_node: Option<NumaNode>) -> Result<Self, AttachProgramError>
	{
		let mut map_file_descriptors = FileDescriptorsMap::with_capacity(1);
		
		let channels = Self::number_of_channels(network_interface_index)?;
		
		// Equivalent to the function `xsk_create_bpf_maps()` in the Linux source `tools/lib/bpf/xsk.c`.
		let redirect_map = ExpressDataPathRedirectSocketArrayMap::new_express_data_path_redirect_socket_array_map_from_channels(Self::redirect_map_name(), channels, &mut map_file_descriptors, ExpressDataPathAccessPermissions::default(), redirect_map_numa_node)?;
		
		let offload_to = if device_offload
		{
			Some(network_interface_index)
		}
		else
		{
			None
		};
		let program_template = Self::owned_memory_program(offload_to);
		
		let mut extended_bpf_program_file_descriptors = FileDescriptorsMap::with_capacity(1);
		
		let rc_express_data_path_extended_bpf_program_file_descriptor = program_template.convenient_load(&map_file_descriptors, &mut extended_bpf_program_file_descriptors)?;
		
		let attached_express_data_path_extended_bpf_program_file_descriptor = Self::convert_rc_to_single(rc_express_data_path_extended_bpf_program_file_descriptor, map_file_descriptors, extended_bpf_program_file_descriptors);
		
		Self::attach_program(network_interface_index, &attached_express_data_path_extended_bpf_program_file_descriptor, device_offload)?;
		
		Ok
		(
			Self
			{
				redirect_map: ManuallyDrop::new(redirect_map),
				attached_express_data_path_extended_bpf_program_file_descriptor: ManuallyDrop::new(attached_express_data_path_extended_bpf_program_file_descriptor),
				attached_program_name: Self::our_owned_program_name(),
				network_interface_index_if_removing_xdp_program_on_drop: Some(network_interface_index),
			}
		)
	}
	
	#[inline(always)]
	fn convert_rc_to_single(rc_express_data_path_extended_bpf_program_file_descriptor: Rc<ExtendedBpfProgramFileDescriptor>, map_file_descriptors: FileDescriptorsMap<MapFileDescriptor>, extended_bpf_program_file_descriptors: FileDescriptorsMap<ExtendedBpfProgramFileDescriptor>) -> ExtendedBpfProgramFileDescriptor
	{
		drop(map_file_descriptors);
		drop(extended_bpf_program_file_descriptors);
		
		// We should now have only one instance of an `Rc<ExtendedBpfProgramFileDescriptor>` after the drops above, but it is difficult reason certainly this is the case.
		// Hence this belt-and-braces approach which uses a near-equivalent of `dup()` - which will infinitely loop if the Linux kernel is out of memory.
		match Rc::try_unwrap(rc_express_data_path_extended_bpf_program_file_descriptor)
		{
			Ok(express_data_path_extended_bpf_program_file_descriptor) => express_data_path_extended_bpf_program_file_descriptor,
			
			Err(rc_express_data_path_extended_bpf_program_file_descriptor) => loop
			{
				match rc_express_data_path_extended_bpf_program_file_descriptor.duplicate_with_close_on_exec_non_blocking()
				{
					Ok(Some(express_data_path_extended_bpf_program_file_descriptor)) => break express_data_path_extended_bpf_program_file_descriptor,
					
					Ok(None) | Err(CreationError::KernelWouldBeOutOfMemory) => continue,
					
					Err(error) => panic!("{}", error),
				}
			}
		}
	}
	
	#[inline(always)]
	fn attach_program(network_interface_index: NetworkInterfaceIndex, express_data_path_extended_bpf_program_file_descriptor: &ExtendedBpfProgramFileDescriptor, device_offload: bool) -> Result<(), AttachProgramError>
	{
		let mut netlink_socket_file_descriptor = NetlinkSocketFileDescriptor::open()?;
		use self::AttachMode::*;
		let attach_mode = if device_offload
		{
			Offloaded
		}
		else
		{
			GenericOrNative
		};
		RouteNetlinkProtocol::xdp_fd_replace(&mut netlink_socket_file_descriptor, network_interface_index, express_data_path_extended_bpf_program_file_descriptor, attach_mode, UpdateMode::CreateOrUpdate).map_err(AttachProgramError::CouldNotAttachXdpProgram)
	}
	
	/// Specify `Some` for `offload_to` if using a network card that supports offloading of eBPF (currently, only Netronome NFP drivers support this).
	///
	/// Based on the function `xsk_load_xdp_prog()` in Linux source `tools/lib/bpf/xsk.c`.
	fn owned_memory_program(offload_to: Option<NetworkInterfaceIndex>) -> ExtendedBpfProgramTemplate<'static>
	{
		use self::AluOperation::*;
		use self::bpf_func_id::*;
		use self::JumpOperation::*;
		use self::Register::*;
		use self::xdp_action::*;
		
		let xskmap = Self::redirect_map_name();
		const exit: &'static str = "exit";
		let rx_queue_index: VariableSlotU64 = VariableSlotU64::try_from(1).unwrap();
		
		#[inline(always)]
		fn redirect_map<'name, MN: TryInto<MapName>, VS: TryInto<VariableSlotU64>>(map_name: MN, rx_queue_index: Option<VS>, flags: xdp_action) -> ProgramLineWrapper<'name>
		where MN::Error: Debug, VS::Error: Debug
		{
			let mut program_lines = Vec::with_capacity(4);
			
			// `r1 map: *const bpf_map = look_up_file_descriptor_for(map_name)`.
			program_lines.push(load_map_file_descriptor(r1, map_name));
			
			if let Some(rx_queue_index) = rx_queue_index
			{
				// `r2 rx_queue_index = stack rx_queue_index`.
				program_lines.push(load_from_stack_variable_32(r2, rx_queue_index))
			}
			
			// `r3 flags = flags`.
			program_lines.push(move_64(r3, flags));
			
			// `r0 exit_code: xdp_action = redirect_map(map: *const bpf_map, rx_queue_index: u32, flags: u64)`.
			program_lines.push(function_call(BPF_FUNC_redirect_map));
			
			ProgramLineWrapper::ProgramLines(program_lines)
		}
		
		ExtendedBpfProgramTemplate
		{
			program_type: ProgramType::ExpressDataPath
			(
				CommonProgramTypeDetails
				{
					minimum_linux_kernel_version: MinimumLinuxKernelVersion::Any,
					ifindex: offload_to,
				}
			),
			program_name: Self::our_owned_program_name(),
			license: BpfProgramLicense::DualBSDAndGPL,
			bpf_type_format_program_details: None,
			
			/*
				fn main(ctx: &mut xdp_md)
				{
					let rx_queue_index: u32 = ctx.rx_queue_index;
					
					let ret: xdp_action = bpf_redirect_map(&xskmap, rx_queue_index, XDP_PASS);
					if ret != xdp_action:XDP_ABORTED
					{
						return ret
					}
					
					// Fallback for pre-5.3 kernels, not supporting default action in the flags parameter.
					if bpf_map_lookup_elem(&xskmap, index, 0)
					{
						return bpf_redirect_map(&xsks_map, index, 0)
					}
					XDP_PASS
				}
			 */
			program_lines: program_lines!
			[
				// `r2 rx_queue_index = r1 ctx.rx_queue_index`.
				load_from_memory_32(r2, r1, xdp_md::rx_queue_index),
				
				// `stack rx_queue_index = r2 rx_queue_index`.
				store_to_stack_variable_32(r2, rx_queue_index),
				
				redirect_map::<_, VariableSlotU64>(xskmap, None, XDP_PASS),
				
				// `if r0 result != xdp_action:XDP_ABORTED goto exit`.
				conditional_jump_32(NotEqual, r0, xdp_action::XDP_ABORTED, exit),
				
				block(vec!
				[
					// `r1 map: *const bpf_map = xskmap`.
					load_map_file_descriptor(r1, xskmap),
					
					// `r2 key: *const u32 = &stack rx_queue_index`.
					move_stack_pointer(r2),
					alu_64(Add, r2, rx_queue_index),
					
					// `r0 result: i32 = bpf_map_lookup_elem(map: *const bpf_map, key: *const u32)`.
					function_call(BPF_FUNC_map_lookup_elem),
				]),
				
				// `r1 result = r0 result`.
				move_64(r1, r0),
				
				// `r0 exit_code: xdp_action = XDP_PASS`.
				move_64(r0, XDP_PASS),
				
				// `if r1 result == 0 goto exit`.
				conditional_jump_32(Equal, r1, 0, exit),
				
				redirect_map(xskmap, Some(rx_queue_index), XDP_ABORTED),
				
				// `exit(r0 exit_code)`.
				label(exit), program_exit(),
			]
		}
	}
	
	/// Based on the function `xsk_setup_xdp_prog()` in Linux source `tools/lib/bpf/netlink.c`.
	fn get_link_message_data(network_interface_index: NetworkInterfaceIndex) -> Result<GetLinkMessageData, AttachProgramError>
	{
		use self::AttachProgramError::*;
		
		let mut netlink_socket_file_descriptor = NetlinkSocketFileDescriptor::open()?;
		let result = RouteNetlinkProtocol::get_link(&mut netlink_socket_file_descriptor, |get_link_message_data| get_link_message_data.is_for_index(network_interface_index));
		let option = result.map_err(GetLinksUsingNetlink)?;
		match option
		{
			None => Err(NoSuchNetworkInterfaceIndex(network_interface_index)),
			Some(get_link_message_data) => Ok(get_link_message_data),
		}
	}
	
	#[inline(always)]
	fn redirect_map_name() -> &'static MapName
	{
		lazy_static!
		{
    		static ref redirect_map_name: MapName = "xsks_map".try_into().unwrap();
    	}

		&redirect_map_name
	}
	
	#[inline(always)]
	fn our_owned_program_name() -> ProgramName
	{
		ProgramName::from_bytes(b"OwnedXDP").unwrap()
	}
}
