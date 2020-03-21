#[derive(Debug)]
#[doc(hidden)]
pub struct HiddenChildProcessArguments<CPA>
{
	send_pipe_file_descriptor: SendPipeFileDescriptor,
	receive_pipe_file_descriptor: ReceivePipeFileDescriptor,
	new_root: PathBuf,
	child_process_argument: CPA,
}

impl<CPA> HiddenChildProcessArguments<CPA>
{
	#[inline(always)]
	pub(crate) fn unbox(self: Box<Self>) -> Self
	{
		*self
	}
}
