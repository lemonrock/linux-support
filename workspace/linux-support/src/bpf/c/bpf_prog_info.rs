// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Program information.
#[repr(C, align(8))]
#[derive(Debug, Copy, Clone)]
pub struct bpf_prog_info
{
	pub(crate) type_: bpf_prog_type,
	
	pub(crate) id: ExtendedBpfProgramIdentifier,
	
	/// Tag.
	pub tag: [u8; BPF_TAG_SIZE],
	
	pub(crate) jited_prog_len: u32,
	
	pub(crate) xlated_prog_len: u32,
	
	pub(crate) jited_prog_insns: AlignedU64,
	
	pub(crate) xlated_prog_insns: AlignedU64,
	
	/// Value relative to system boot.
	pub load_time: u64,
	
	/// User who created this program.
	pub created_by_uid: UserIdentifier,
	
	pub(crate) nr_map_ids: u32,
	
	pub(crate) map_ids: AlignedU64,
	
	pub(crate) name: [c_char; BPF_OBJ_NAME_LEN],
	
	pub(crate) ifindex: Option<NetworkInterfaceIndex>,
	
	_bitfield_1: __BindgenBitfieldUnit<[u8; 4], u8>,
	
	pub(crate) netns_dev: u64,
	
	pub(crate) netns_ino: Inode,
	
	pub(crate) nr_jited_ksyms: u32,
	
	pub(crate) nr_jited_func_lens: u32,
	
	pub(crate) jited_ksyms: AlignedU64,
	
	pub(crate) jited_func_lens: AlignedU64,
	
	pub(crate) btf_id: BpfTypeFormatIdentifier,
	
	pub(crate) func_info_rec_size: u32,
	
	pub(crate) func_info: AlignedU64,
	
	pub(crate) nr_func_info: u32,
	
	pub(crate) nr_line_info: u32,
	
	pub(crate) line_info: AlignedU64,
	
	pub(crate) jited_line_info: AlignedU64,
	
	pub(crate) nr_jited_line_info: u32,
	
	pub(crate) line_info_rec_size: u32,
	
	pub(crate) jited_line_info_rec_size: u32,
	
	pub(crate) nr_prog_tags: u32,
	
	pub(crate) prog_tags: AlignedU64,
	
	/// Time spent running, in nanoseconds.
	pub run_time_ns: u64,
	
	/// Number of times run.
	pub run_cnt: u64,
}

impl Information for bpf_prog_info
{
	type Identifier = ExtendedBpfProgramIdentifier;
	
	#[inline(always)]
	fn identifier(&self) -> Self::Identifier
	{
		self.id
	}
}

impl bpf_prog_info
{
	/// Name (clones internally).
	#[inline(always)]
	pub fn name(&self) -> ProgramName
	{
		ProgramName::try_from(&self.name).unwrap()
	}
	
	/// Associated BTF identifier, if any.
	#[inline(always)]
	pub fn btf_identifier(&self) -> BpfTypeFormatIdentifier
	{
		self.btf_id
	}
	
	/// Network device bound to, if any.
	#[inline(always)]
	pub fn network_device_network_namespace_dev_and_network_namespace_inode(&self) -> (Option<NetworkInterfaceIndex>, u64, Inode)
	{
		(self.ifindex, self.netns_dev, self.netns_ino)
	}
	
	/// Jitted instructions.
	#[inline(always)]
	pub fn jitted_instructions(&self) -> Option<&[u8]>
	{
		self.jited_prog_insns.to_slice(self.jited_prog_len)
	}
	
	/// Translated instructions.
	#[inline(always)]
	pub fn translated_instructions(&self) -> Option<&[u8]>
	{
		self.xlated_prog_insns.to_slice(self.xlated_prog_len)
	}
	
	/// Map identifiers.
	#[inline(always)]
	pub fn map_identifiers(&self) -> Option<&[MapIdentifier]>
	{
		self.map_ids.to_slice(self.nr_map_ids)
	}
	
	/// Jitted kernel symbols.
	#[inline(always)]
	pub fn jitted_kernel_symbols(&self) -> Option<&[u64]>
	{
		self.jited_ksyms.to_slice(self.nr_jited_ksyms)
	}
	
	/// Jitted kernel symbols.
	#[inline(always)]
	pub fn jitted_function_lengths(&self) -> Option<&[u32]>
	{
		self.jited_func_lens.to_slice(self.nr_jited_func_lens)
	}
	
	/// Function information.
	#[inline(always)]
	pub fn function_information(&self) -> Option<ArrayOfElementsWhoseSizeVariesByLinuxVersion>
	{
		self.func_info.to_array_of_elements_whose_size_varies_by_linux_version(self.nr_func_info, self.func_info_rec_size)
	}
	
	/// Line information.
	#[inline(always)]
	pub fn line_information(&self) -> Option<ArrayOfElementsWhoseSizeVariesByLinuxVersion>
	{
		self.func_info.to_array_of_elements_whose_size_varies_by_linux_version(self.nr_line_info, self.line_info_rec_size)
	}
	
	/// Line information.
	#[inline(always)]
	pub fn jitted_line_information(&self) -> Option<ArrayOfElementsWhoseSizeVariesByLinuxVersion>
	{
		self.func_info.to_array_of_elements_whose_size_varies_by_linux_version(self.nr_jited_line_info, self.jited_line_info_rec_size)
	}
	
	/// Program tags.
	#[inline(always)]
	pub fn program_tags(&self) -> Option<&[[u8; BPF_TAG_SIZE]]>
	{
		self.prog_tags.to_slice(self.nr_prog_tags)
	}
	
	/// Is GPL compatible?
	#[inline(always)]
	pub fn is_gpl_compatible(&self) -> u32
	{
		unsafe { transmute(self._bitfield_1.get(0, 1) as u32) }
	}
}
