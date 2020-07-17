// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents an extended BPF program file descriptor.
///
/// Created by `ExtendedBpfProgram::parse_and_load()`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExtendedBpfProgramFileDescriptor(RawFd);

impl Drop for ExtendedBpfProgramFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.as_raw_fd().close()
	}
}

impl AsRawFd for ExtendedBpfProgramFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl IntoRawFd for ExtendedBpfProgramFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.as_raw_fd()
	}
}

impl FromRawFd for ExtendedBpfProgramFileDescriptor
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(fd)
	}
}

impl FileDescriptor for ExtendedBpfProgramFileDescriptor
{
}

impl BpfFileDescriptor for ExtendedBpfProgramFileDescriptor
{
	type Information = bpf_prog_info;
}

impl UsedAsValueInArrayMapDescriptor for ExtendedBpfProgramFileDescriptor
{
	#[inline(always)]
	fn transmute_to_file_descriptor_copies(values: Vec<RawFd>) -> Vec<FileDescriptorCopy<Self>>
	{
		unsafe { transmute(values) }
	}
	
	#[inline(always)]
	fn transmute_from_file_descriptor_copies(values: &[FileDescriptorCopy<Self>]) -> &[RawFd]
	{
		unsafe { transmute(values) }
	}
	
	#[inline(always)]
	fn transmute_to_file_descriptor_copy(value: RawFd) -> FileDescriptorCopy<Self>
	{
		unsafe { transmute(value) }
	}
}

impl ExtendedBpfProgramFileDescriptor
{
	/// Test run.
	#[allow(deprecated)]
	pub fn test_run<C: Sized>(&self, context: &C, test_data: &[u8], expected_output_data_length: usize, repetitions: NonZeroU32) -> Result<TestRunResults<C>, Errno>
	{
		let mut attr = bpf_attr::default();
		let mut context_out: C = unsafe { uninitialized() };
		let mut data_out = Vec::with_capacity(expected_output_data_length);
		attr.test = BpfCommandProgramTestRun
		{
			prog_fd: self.as_raw_fd(),
			retval: 0,
			data_size_in: test_data.len() as u32,
			data_size_out: data_out.capacity() as u32,
			data_in: AlignedU64::from(test_data.as_ptr()),
			data_out: AlignedU64::from(data_out.as_mut_ptr()),
			repeat: repetitions,
			duration: 0,
			ctx_size_in: size_of::<C>() as u32,
			ctx_size_out: size_of::<C>() as u32,
			ctx_in: AlignedU64::from(context),
			ctx_out: AlignedU64::from(&mut context_out),
		};
		
		let result = attr.syscall(bpf_cmd::BPF_PROG_TEST_RUN);
		if likely!(result == 0)
		{
			let test = unsafe { attr.test };
			
			let ctx_size_out = test.ctx_size_out;
			debug_assert!(ctx_size_out <= size_of::<C>() as u32);
			
			let data_size_out = test.data_size_out;
			debug_assert!(data_size_out <= data_out.len() as u32);
			unsafe { data_out.set_len(data_size_out as usize) };
			data_out.shrink_to_fit();
			
			Ok
			(
				TestRunResults
				{
					context: context_out,
					data: data_out,
					result_code: test.retval,
					duration: test.duration,
				}
			)
		}
		else if likely!(result == -1)
		{
			Err(errno())
		}
		else
		{
			unreachable!("Unexpected result `{}` from bpf(BPF_PROG_TEST_RUN)", result)
		}
	}
	
	/*
	
	
int bpf_prog_test_run_xattr(struct bpf_prog_test_run_attr *test_attr)
{
	union bpf_attr attr;
	int ret;

	if (!test_attr->data_out && test_attr->data_size_out > 0)
		return -EINVAL;

	memset(&attr, 0, sizeof(attr));
	attr.test.prog_fd = test_attr->prog_fd;
	attr.test.data_in = ptr_to_u64(test_attr->data_in);
	attr.test.data_out = ptr_to_u64(test_attr->data_out);
	attr.test.data_size_in = test_attr->data_size_in;
	attr.test.data_size_out = test_attr->data_size_out;
	attr.test.ctx_in = ptr_to_u64(test_attr->ctx_in);
	attr.test.ctx_out = ptr_to_u64(test_attr->ctx_out);
	attr.test.ctx_size_in = test_attr->ctx_size_in;
	attr.test.ctx_size_out = test_attr->ctx_size_out;
	attr.test.repeat = test_attr->repeat;

	ret = sys_bpf(BPF_PROG_TEST_RUN, &attr, sizeof(attr));
	test_attr->data_size_out = attr.test.data_size_out;
	test_attr->ctx_size_out = attr.test.ctx_size_out;
	test_attr->retval = attr.test.retval;
	test_attr->duration = attr.test.duration;
	return ret;
}

	 */
	
	
	// TODO: eg attach_kprobe for kprobe and uprobe but not kretprobe and uretprobe load program, perf_event_open_tracepoint, attach
	// TODO: eg attach_tp for TRACEPOINT (tracepoint_category and tracepoint_name) load program, perf_event_open_probe, attach
		// bpf_program__attach_tracepoint(prog, tp_cat, tp_name)
	
	/*
	struct bpf_link *bpf_program__attach_kprobe(struct bpf_program *prog,
					    bool retprobe,
					    const char *func_name)
{
	char errmsg[STRERR_BUFSIZE];
	struct bpf_link *link;
	int pfd, err;

	pfd = perf_event_open_probe(false /* uprobe */, retprobe, func_name,
				    0 /* offset */, -1 /* pid */);
	if (pfd < 0) {
		pr_warn("program '%s': failed to create %s '%s' perf event: %s\n",
			bpf_program__title(prog, false),
			retprobe ? "kretprobe" : "kprobe", func_name,
			libbpf_strerror_r(pfd, errmsg, sizeof(errmsg)));
		return ERR_PTR(pfd);
	}
	link = bpf_program__attach_perf_event(prog, pfd);
	if (IS_ERR(link)) {
		close(pfd);
		err = PTR_ERR(link);
		pr_warn("program '%s': failed to attach to %s '%s': %s\n",
			bpf_program__title(prog, false),
			retprobe ? "kretprobe" : "kprobe", func_name,
			libbpf_strerror_r(err, errmsg, sizeof(errmsg)));
		return link;
	}
	return link;
}
	
	
	struct bpf_link *bpf_program__attach_tracepoint(struct bpf_program *prog,
						const char *tp_category,
						const char *tp_name)
{
	char errmsg[STRERR_BUFSIZE];
	struct bpf_link *link;
	int pfd, err;

	pfd = perf_event_open_tracepoint(tp_category, tp_name);
	if (pfd < 0) {
		pr_warn("program '%s': failed to create tracepoint '%s/%s' perf event: %s\n",
			bpf_program__title(prog, false),
			tp_category, tp_name,
			libbpf_strerror_r(pfd, errmsg, sizeof(errmsg)));
		return ERR_PTR(pfd);
	}
	link = bpf_program__attach_perf_event(prog, pfd);
	if (IS_ERR(link)) {
		close(pfd);
		err = PTR_ERR(link);
		pr_warn("program '%s': failed to attach to tracepoint '%s/%s': %s\n",
			bpf_program__title(prog, false),
			tp_category, tp_name,
			libbpf_strerror_r(err, errmsg, sizeof(errmsg)));
		return link;
	}
	return link;
}
	
	 */
	//
	// TODO: eg attach_raw_tp for RAW_TRACEPOINT (tracepoint_name) => Load program then call attach_raw_trace_point
	// TODO: eg attach_trace for TRACING and EXT and LSM => Load program then call attach_raw_trace_point
	
	// TODO: Only for a sub-type of programs!!!
	
	
}
