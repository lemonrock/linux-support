/// A CPU set wrapper.
pub struct CpuSet(cpu_set_t);

impl Default for CpuSet
{
	#[inline(always)]
	fn default() -> Self
	{
		Self(unsafe { zeroed() })
	}
}

impl<'a> From<&'a BTreeSet<HyperThread>> for CpuSet
{
	#[inline(always)]
	fn from(hyper_threads: &BTreeSet<HyperThread>) -> Self
	{
		let mut cpu_set = Self::default();
		for hyper_thread in hyper_threads.iter()
		{
			cpu_set.set_hyper_thread(*hyper_thread);
		}
		cpu_set
	}
}

impl CpuSet
{
	const SizeOfCpuSetT: usize = size_of::<cpu_set_t>();

	/// Set process affinity for current process.
	#[inline(always)]
	pub fn set_current_process_affinity(&self) -> io::Result<()>
	{
		self.set_process_affinity(0)
	}

	/// Set process affinity.
	#[inline(always)]
	pub fn set_process_affinity(&self, process_identifier: pid_t) -> io::Result<()>
	{
		let result = unsafe { sched_setaffinity(process_identifier, Self::SizeOfCpuSetT, &self.0) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else
		{
			Err(io::Error::from_raw_os_error(result))
		}
	}

	/// Set thread affinity.
	#[inline(always)]
	pub fn set_thread_affinity(&self, thread_identifier: pthread_t) -> io::Result<()>
	{
		let result = unsafe { pthread_setaffinity_np(thread_identifier, Self::SizeOfCpuSetT, &self.0) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else
		{
			Err(io::Error::from_raw_os_error(result))
		}
	}

	/// Set thread affinity for current thread.
	#[inline(always)]
	pub fn set_current_thread_affinity(&self) -> io::Result<()>
	{
		self.set_thread_affinity(unsafe { pthread_self() })
	}

	/// Set a hyper thread in the CPU set.
	#[inline(always)]
	pub fn set_hyper_thread(&mut self, hyper_thread: HyperThread)
	{
		unsafe { CPU_SET(hyper_thread.0 as usize, &mut self.0) }
	}
}
