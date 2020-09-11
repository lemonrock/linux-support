// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HyperThreadsDiagnostics
{
	pub is_hyper_threading_active: DiagnosticUnobtainableResult<Option<bool>>,
	
	pub hyper_threading_control: DiagnosticUnobtainableResult<Option<HyperThreadingStatus>>,
	
	pub kernel_exclusive_maximum: DiagnosticUnobtainableResult<u16>,
	
	pub sysconf_exclusive_maximum: u16,
	
	pub has_a_folder_path: DiagnosticUnobtainableResult<HyperThreads>,
	
	pub is_in_proc_self_status: DiagnosticUnobtainableResult<HyperThreads>,

	pub possible: DiagnosticUnobtainableResult<HyperThreads>,

	pub online: DiagnosticUnobtainableResult<HyperThreads>,

	pub offline: DiagnosticUnobtainableResult<HyperThreads>,

	pub valid: DiagnosticUnobtainableResult<HyperThreads>,
	
	pub present: DiagnosticUnobtainableResult<HashMap<HyperThread, HyperThreadDiagnostic>>,
	
	pub work_queue_affinity: DiagnosticUnobtainableResult<HyperThreads>,
	
	pub work_queue_writeback_affinity: DiagnosticUnobtainableResult<HyperThreads>,
	
	pub watchdog_affinity: DiagnosticUnobtainableResult<HyperThreads>,
	
	pub receive_packet_steering_flow_limit_tables_affinity: DiagnosticUnobtainableResult<HyperThreads>,
}

impl HyperThreadsDiagnostics
{
	fn gather(sys_path: &SysPath, proc_path: &ProcPath) -> Self
	{
		#[inline(always)]
		fn wrap_panic<R>(sys_path: &SysPath, callback: impl FnOnce(&SysPath) -> R) -> DiagnosticUnobtainableResult<R>
		{
			catch_unwind(AssertUnwindSafe(|| callback(sys_path))).map_err(|_| DiagnosticUnobtainable(format!("Panicked")))
		}
		
		let (work_queue_affinity, work_queue_writeback_affinity) = HyperThreads::work_queue_hyper_thread_affinity(sys_path);
		
		Self
		{
			is_hyper_threading_active: wrap_panic(sys_path, HyperThread::is_hyper_threading_active),
			
			hyper_threading_control: wrap_panic(sys_path, HyperThread::hyper_threading_control),
			
			kernel_exclusive_maximum: wrap_panic(sys_path, HyperThread::kernel_exclusive_maximum),
			
			sysconf_exclusive_maximum: HyperThread::sysconf_exclusive_maximum(),
			
			has_a_folder_path: wrap_panic(sys_path, HyperThreads::has_a_folder_path),
			
			is_in_proc_self_status: wrap_panic(sys_path, |_| HyperThreads::is_in_proc_self_status(proc_path)),
			
			possible: wrap_panic(sys_path, HyperThreads::possible),
			
			online: wrap_panic(sys_path, HyperThreads::online),
			
			offline: wrap_panic(sys_path, HyperThreads::offline),
			
			valid: wrap_panic(sys_path, |sys_path| HyperThreads::valid(sys_path, proc_path)),
			
			present: wrap_panic(sys_path, HyperThreads::present).map(|hyper_threads|
			{
				let mut hyper_thread_diagnotiscs = HashMap::with_capacity(hyper_threads.len());
				for hyper_thread in hyper_threads.iterate()
				{
					hyper_thread_diagnotiscs.insert(hyper_thread, HyperThreadDiagnostic::gather(sys_path, hyper_thread));
				}
				hyper_thread_diagnotiscs
			}),
			
			work_queue_affinity: work_queue_affinity.map_err(DiagnosticUnobtainable::from),
			
			work_queue_writeback_affinity: work_queue_writeback_affinity.map_err(DiagnosticUnobtainable::from),
			
			watchdog_affinity: HyperThreads::watchdog(proc_path).map_err(DiagnosticUnobtainable::from),
			
			receive_packet_steering_flow_limit_tables_affinity: HyperThreads::receive_packet_steering_flow_limit_tables_affinity(proc_path).map_err(DiagnosticUnobtainable::from),
		}
	}
}
