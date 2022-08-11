// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[doc(hidden)]
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum bpf_cmd
{
	BPF_MAP_CREATE = 0,
	
	BPF_MAP_LOOKUP_ELEM = 1,
	
	BPF_MAP_UPDATE_ELEM = 2,
	
	BPF_MAP_DELETE_ELEM = 3,
	
	BPF_MAP_GET_NEXT_KEY = 4,
	
	BPF_PROG_LOAD = 5,
	
	BPF_OBJ_PIN = 6,
	
	BPF_OBJ_GET = 7,
	
	BPF_PROG_ATTACH = 8,
	
	BPF_PROG_DETACH = 9,
	
	BPF_PROG_TEST_RUN = 10,
	
	BPF_PROG_GET_NEXT_ID = 11,
	
	BPF_MAP_GET_NEXT_ID = 12,
	
	BPF_PROG_GET_FD_BY_ID = 13,
	
	BPF_MAP_GET_FD_BY_ID = 14,
	
	BPF_OBJ_GET_INFO_BY_FD = 15,
	
	BPF_PROG_QUERY = 16,
	
	BPF_RAW_TRACEPOINT_OPEN = 17,
	
	BPF_BTF_LOAD = 18,
	
	BPF_BTF_GET_FD_BY_ID = 19,
	
	BPF_TASK_FD_QUERY = 20,
	
	BPF_MAP_LOOKUP_AND_DELETE_ELEM = 21,
	
	BPF_MAP_FREEZE = 22,
	
	BPF_BTF_GET_NEXT_ID = 23,
	
	BPF_MAP_LOOKUP_BATCH = 24,
	
	BPF_MAP_LOOKUP_AND_DELETE_BATCH = 25,
	
	BPF_MAP_UPDATE_BATCH = 26,
	
	BPF_MAP_DELETE_BATCH = 27,
	
	BPF_LINK_CREATE = 28,
	
	BPF_LINK_UPDATE = 29,
}
