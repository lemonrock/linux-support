// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Channels is for `network_interface_name` implying that it existed at some point before calling this method.
pub fn reuse_or_load_owned_memory_program(network_interface_name: NetworkInterfaceName, device_offload: bool, redirect_map_settings: (Channels, Option<NumaNode>)) -> Result<(ExpressDataPathRedirectSocketArrayMap, Rc<ExtendedBpfProgramFileDescriptor>), LoadOwnedMemoryProgramError>
{
	use self::LoadOwnedMemoryProgramError::*;
	
	let redirect_map_name: MapName = "xsks_map".try_into().unwrap();
	
	let mut netlink_socket_file_descriptor = NetlinkSocketFileDescriptor::open()?;
	let result = RouteNetlinkProtocol::get_link(&mut netlink_socket_file_descriptor, &|get_link_message_data| get_link_message_data.is_for(&network_interface_name));
	let option = result.map_err(GetLinksUsingNetlink)?;
	let get_link_message_data = option.ok_or_else(|| NoSuchNetworkInterfaceName(network_interface_name.clone()))?;
	
	let network_interface_index = get_link_message_data.network_interface_index;
	match get_link_message_data.attached_express_data_path_program_identifiers
	{
		Some(program_identifiers) => already_attached(redirect_map_name, network_interface_index, device_offload, program_identifiers),
		
		None => load_owned_memory_program(network_interface_index, redirect_map_name, network_interface_name, device_offload, redirect_map_settings)
	}
}

fn already_attached(redirect_map_name: MapName, network_interface_index: NetworkInterfaceIndex, device_offload: bool, program_identifiers: MultipleProgramIdentifiers) -> Result<(ExpressDataPathRedirectSocketArrayMap, Rc<ExtendedBpfProgramFileDescriptor>), LoadOwnedMemoryProgramError>
{
	use self::LoadOwnedMemoryProgramError::*;
	
	let (program_identifier, attach_mode) = program_identifiers.choose_most_performant();
	
	let x: ExtendedBpfProgramFileDescriptor = ExtendedBpfProgramFileDescriptor::from_identifier(program_identifier, ()).map_err(CouldNotGetExistingProgramFileDescriptor)?.ok_or(NoExistingExpressDataPathProgramForAttachedExtendedBpfProgramFileDescriptor)?;
	
	let program_information = x.get_information().map_err(CouldNotGetExistingProgramInformation)?;
	program_information.validate_has_program_type(bpf_prog_type::BPF_PROG_TYPE_XDP, ExistingAttachedProgramHasWrongProgramTypeForExpressDataPath)?;
	program_information.validate_attach_mode_and_device_offload_matches_program_information(attach_mode, device_offload, network_interface_index)?;
	
	let redirect_map = ExpressDataPathRedirectSocketArrayMap::rehydrate(&program_information, &redirect_map_name)?;
	
	Ok((redirect_map, Rc::new(x)))
}

fn load_owned_memory_program(network_interface_index: NetworkInterfaceIndex, redirect_map_name: MapName, network_interface_name: NetworkInterfaceName, device_offload: bool, (channels, numa_node): (Channels, Option<NumaNode>)) -> Result<(ExpressDataPathRedirectSocketArrayMap, Rc<ExtendedBpfProgramFileDescriptor>), LoadOwnedMemoryProgramError>
{
	let mut map_file_descriptors = FileDescriptorsMap::with_capacity(1);
	
	let redirect_map = ExpressDataPathRedirectSocketArrayMap::new_express_data_path_redirect_socket_array_map_from_channels(&redirect_map_name, channels, &mut map_file_descriptors, ExpressDataPathAccessPermissions::default(), numa_node)?;
	
	let offload_to = if device_offload
	{
		Some(network_interface_index)
	}
	else
	{
		None
	};
	let program_template = owned_memory_program(&redirect_map_name, offload_to);
	
	let mut extended_bpf_program_file_descriptors = FileDescriptorsMap::with_capacity(1);
	let x: Rc<ExtendedBpfProgramFileDescriptor> = program_template.convenient_load(&map_file_descriptors, &mut extended_bpf_program_file_descriptors)?;
	
	// TODO: attach!
	// bpf_set_link_xdp_fd
	xxxx;
	
	// TODO: How do we use this program?
	
	Ok((redirect_map, x))
}

/// Specify `Some` for `offload_to` if using a network card that supports offloading of eBPF (currently, only Netronome NFP drivers support this).
fn owned_memory_program(xskmap: &MapName, offload_to: Option<NetworkInterfaceIndex>) -> ExtendedBpfProgramTemplate<'static>
{
	use self::AluOperation::*;
	use self::bpf_func_id::*;
	use self::JumpOperation::*;
	use self::Register::*;
	use self::xdp_action::*;
	
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
		program_name: ProgramName::from_bytes(b"OwnedXDP").unwrap(),
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
