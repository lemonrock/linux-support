// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use super::extended::bpf_type_format::*;
use super::extended::bpf_type_format::reflection::*;
use super::extended::identifiers::*;
use super::extended::instructions::*;
use super::extended::instructions::LoadSize::*;
use super::extended::maps::domain::*;
use super::extended::programs::*;
use super::extended::maps::domain::access_permissions::KernelOnlyAccessPermissions;
use crate::express_data_path::attached_program::AttachMode;
use crate::file_descriptors::bpf::BpfFileDescriptor;
use crate::file_descriptors::bpf::MapFileDescriptor;
use crate::inode::Inode;
use crate::network_device::NetworkInterfaceIndex;
use crate::syscall::SystemCallNumber;
use crate::user_and_groups::UserIdentifier;


include!("AlignedU64.rs");
include!("ArrayOfElementsWhoseSizeVariesByLinuxVersion.rs");
include!("BPF_.rs");
include!("bpf_attach_type.rs");
include!("bpf_attr.rs");
include!("bpf_btf_info.rs");
include!("bpf_cgroup_storage_key.rs");
include!("bpf_cmd.rs");
include!("bpf_func_id.rs");
include!("bpf_func_info.rs");
include!("bpf_insn.rs");
include!("BPF_JUMP.rs");
include!("bpf_line_info.rs");
include!("bpf_lpm_trie_key.rs");
include!("bpf_map_info.rs");
//include!("BPF_LL_OFF.rs");
//include!("BPF_MAJOR_VERSION.rs");
include!("BPF_MAXINSNS.rs");
include!("BPF_MAP_CREATE_flags.rs");
include!("BPF_MAP_UPDATE_ELEM_flags.rs");
include!("bpf_map_type.rs");
include!("BPF_MEMWORDS.rs");
//include!("BPF_MINOR_VERSION.rs");
//include!("BPF_NET_OFF.rs");
include!("BPF_OBJ_NAME_LEN.rs");
include!("BPF_PROG_ATTACH_flags.rs");
include!("BPF_PROG_LOAD_flags.rs");
include!("BPF_PROG_QUERY_flags.rs");
include!("bpf_prog_info.rs");
include!("bpf_prog_type.rs");
include!("bpf_spin_lock.rs");
include!("bpf_stack_build_id.rs");
include!("bpf_stack_build_id_status.rs");
include!("BPF_STMT.rs");
include!("BPF_TAG_SIZE.rs");
include!("bpf_task_fd_type.rs");
include!("BpfCommandBpfTypeFormatLoad.rs");
include!("BpfCommandGetIdentifier.rs");
include!("BpfCommandGetIdentifierValueOfIdentifier.rs");
include!("BpfCommandLinkCreate.rs");
include!("BpfCommandLinkUpdate.rs");
include!("BpfCommandMapBatch.rs");
include!("BpfCommandMapChange.rs");
include!("BpfCommandMapChangeValueOrNextKey.rs");
include!("BpfCommandMapCreate.rs");
include!("BpfCommandObject.rs");
include!("BpfCommandObjectGetInformationByFileDescriptor.rs");
include!("BpfCommandProgramAttachOrDetach.rs");
include!("BpfCommandProgramLoad.rs");
include!("BpfCommandProgramQuery.rs");
include!("BpfCommandProgramTestRun.rs");
include!("BpfCommandRawTracePointOpen.rs");
include!("BpfCommandTaskFileDescriptorQuery.rs");
include!("btf_array.rs");
include!("btf_enum.rs");
include!("btf_func_linkage.rs");
include!("btf_header.rs");
include!("BTF_INT_.rs");
include!("BTF_MAX_.rs");
include!("btf_member.rs");
include!("btf_param.rs");
include!("btf_type.rs");
include!("btf_var.rs");
include!("BpfTypeFormatTypeSizeOrTypeIdentifier.rs");
include!("BpfTypeFormatVariableLinkage.rs");
include!("DestinationAndSourceRegisters.rs");
include!("elem_flags.rs");
include!("ExtendedBPF_.rs");
include!("GetExistingMapError.rs");
include!("InstructionPointer.rs");
include!("MAX_BPF_STACK.rs");
include!("OffsetOrInstructionPointer.rs");
//include!("SKF_AD_.rs");
//include!("SKF_LL_OFF.rs");
//include!("SKF_NET_OFF.rs");
include!("sock_filter.rs");
include!("sock_fprog.rs");
include!("ValidateAttachModeError.rs");
