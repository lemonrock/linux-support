// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum bpf_task_fd_type
{
	BPF_FD_TYPE_RAW_TRACEPOINT = 0,
	BPF_FD_TYPE_TRACEPOINT = 1,
	BPF_FD_TYPE_KPROBE = 2,
	BPF_FD_TYPE_KRETPROBE = 3,
	BPF_FD_TYPE_UPROBE = 4,
	BPF_FD_TYPE_URETPROBE = 5,
}
