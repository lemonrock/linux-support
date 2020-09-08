// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExtendedBerkeleyPacketFilterProgramDiagnostic
{
	pub id: ExtendedBpfProgramIdentifier,
	
	pub name: ProgramName,
	
	pub btf_identifier: BpfTypeFormatIdentifier,
	
	pub network_device_network_namespace_dev_and_network_namespace_inode: (Option<NetworkInterfaceIndex>, u64, Inode),
	
	pub jitted_instructions: Option<ByteBuf>,
	
	pub translated_instructions: Option<ByteBuf>,
	
	pub map_identifiers: Option<Box<[MapIdentifier]>>,
	
	pub jitted_kernel_symbols: Option<Box<[u64]>>,
	
	pub jitted_function_lengths: Option<Box<[u32]>>,
	
	// Can not be easily represented.
	//
	// pub function_information: Option<ArrayOfElementsWhoseSizeVariesByLinuxVersion>,
	//
	// pub line_information: Option<ArrayOfElementsWhoseSizeVariesByLinuxVersion>,
	//
	// pub jitted_line_information: Option<ArrayOfElementsWhoseSizeVariesByLinuxVersion>,
	
	pub program_tags: Option<Box<[[u8; BPF_TAG_SIZE]]>>,
	
	pub is_gpl_compatible: bool,
	
	pub type_: bpf_prog_type,
	
	pub tag: [u8; BPF_TAG_SIZE],
	
	/// Value relative to system boot.
	pub load_time: u64,
	
	/// User who created this program.
	pub created_by_uid: UserIdentifier,
	
	/// Time spent running, in nanoseconds.
	pub run_time_ns: u64,
	
	/// Number of times run.
	pub run_cnt: u64,
}

impl ExtendedBerkeleyPacketFilterProgramDiagnostic
{
	fn gather(file_descriptor: &ExtendedBpfProgramFileDescriptor) -> Result<Self, Errno>
	{
		file_descriptor.get_information().map(|information| Self
		{
			id: information.identifier(),
			
			name: information.name(),
			
			btf_identifier: information.btf_identifier(),
			
			network_device_network_namespace_dev_and_network_namespace_inode: information.network_device_network_namespace_dev_and_network_namespace_inode(),
			
			jitted_instructions: information.jitted_instructions().map(|slice| slice.to_vec().into()()),
			
			translated_instructions: information.translated_instructions().map(|slice| slice.to_vec().into()()),
			
			map_identifiers: information.map_identifiers().map(|slice| slice.to_vec().into_boxed_slice()),
			
			jitted_kernel_symbols: information.jitted_kernel_symbols().map(|slice| slice.to_vec().into_boxed_slice()),
			
			jitted_function_lengths: information.jitted_function_lengths().map(|slice| slice.to_vec().into_boxed_slice()),
			// Can not be easily represented.
			//
			// function_information: information.function_information(),
			// line_information: information.line_information(),
			// jitted_line_information: information.jitted_line_information(),
			program_tags: information.program_tags().map(|slice| slice.to_vec().into_boxed_slice()),
			
			is_gpl_compatible: information.is_gpl_compatible(),
			
			type_: information.type_,
			
			tag: information.tag,
			
			load_time: information.load_time,
			
			created_by_uid: information.created_by_uid,
			
			run_time_ns: information.run_time_ns,
			
			run_cnt: information.run_cnt,
		})
	}
}
