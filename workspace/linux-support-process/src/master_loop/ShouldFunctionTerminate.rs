/// Should the function running on the current logical core terminate?
#[derive(Debug)]
pub struct ShouldFunctionTerminate(AtomicBool);

unsafe impl Send for ShouldFunctionTerminate
{
}

unsafe impl Sync for ShouldFunctionTerminate
{
}

impl ShouldFunctionTerminate
{
	const Sleepiness: Duration = Duration::from_millis(10);

	/// New instance.
	#[inline(always)]
	pub fn new() -> Arc<Self>
	{
		Arc::new(Self(AtomicBool::new(false)))
	}

	/// Should we terminate?
	#[inline(always)]
	pub fn should_terminate(&self) -> bool
	{
		self.0.load(Relaxed)
	}

	/// Should we continue?
	#[inline(always)]
	pub fn should_continue(&self) -> bool
	{
		!self.should_terminate()
	}

	/// Sleep and check for terminate.
	#[inline(always)]
	pub fn sleep_and_check_should_terminate(&self) -> bool
	{
		::std::thread::sleep(Self::Sleepiness);
		self.should_terminate()
	}

	/// A thread-like function panicked; terminate.
	#[inline(always)]
	pub fn we_panicked(&self, payload: &(dyn Any + 'static + Send))
	{
		caught_unwind_and_log_it_to_syslog(payload);

		self.0.store(true, SeqCst)
	}

	/// The master loop was signalled (caught a signal) that was interpreted as meaning 'exit'.
	#[inline(always)]
	pub fn exit_signalled(&self, signal_number: Option<SignalNumber>)
	{
		log_exit_signalled_to_syslog(signal_number);

		self.0.store(true, SeqCst)
	}
}
