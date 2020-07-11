// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Arguments.
#[derive(Debug)]
pub struct ExtendedBpfProgramArguments<'map_file_descriptor_label_map, 'extended_bpf_program_file_descriptor_label_map>
{
	/// Immediates.
	pub i32_immediates_map: OffsetsMap<i32>,
	
	/// Immediates.
	pub u64_immediates_map: OffsetsMap<u64>,
	
	/// Memory offsets.
	pub memory_offsets_map: OffsetsMap<i16>,
	
	/// Maps.
	pub map_file_descriptor_labels_map: &'map_file_descriptor_label_map FileDescriptorLabelsMap<MapFileDescriptor>,
	
	/// Programs.
	pub extended_bpf_program_file_descriptor_labels_map: &'map_file_descriptor_label_map FileDescriptorLabelsMap<ExtendedBpfProgramFileDescriptor>,
}
