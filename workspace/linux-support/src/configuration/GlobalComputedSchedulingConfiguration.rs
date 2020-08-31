// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global computed scheduling configuration for hyper-thread sensitive settings.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GlobalComputedSchedulingConfiguration
{
	/// It is recommended that this value be computed.
	///
	/// If the watchdogs are disabled (see `GlobalSchedulingConfiguration.enable_software_watchdog_lockup_detection` and `GlobalSchedulingConfiguration.enable_hardware_watchdog_lockup_detection`), then this value does not need to change.
	///
	/// Requires root.
	pub software_and_hardware_watchdog_runs_on_which_kernel_cpus: Option<HyperThreads>,

	/// It is recommended that this value be computed.
	///
	/// Requires root.
	pub work_queue_runs_on_which_kernel_cpus: Option<HyperThreads>,
	
	/// It is recommended that this value be computed.
	///
	/// Requires root.
	pub default_interrupt_request_affinity: Option<HyperThreads>,

	/// It is recommended that this value be computed.
	///
	/// It needs to take into account which network cards are being used with which NUMA nodes.
	///
	/// Requires root.
	pub interrupt_request_affinity: HashMap<InterruptRequest, HyperThreads>,

	/// Enable receive packet steering (RPS) flow limit tables.
	///
	/// Probably should not be configured.
	pub receive_packet_steering_flow_limit_tables: Option<HyperThreads>,
}

impl GlobalComputedSchedulingConfiguration
{
	/// Configures.
	#[inline(always)]
	pub fn configure(&self, sys_path: &SysPath, proc_path: &ProcPath) -> Result<(), GlobalComputedSchedulingConfigurationError>
	{
		use self::GlobalComputedSchedulingConfigurationError::*;
		
		set_value(proc_path, |proc_path, value| value.force_watchdog_to_just_these_hyper_threads(proc_path), self.software_and_hardware_watchdog_runs_on_which_kernel_cpus.as_ref(), CouldNotChangeSoftwareAndHardwareWatchdogCpus)?;
		set_value(proc_path, |_proc_path, value| value.set_work_queue_hyper_thread_affinity(sys_path), self.work_queue_runs_on_which_kernel_cpus.as_ref(), CouldNotChangeWorkQueueCpus)?;
		set_value(proc_path, |proc_path, value| InterruptRequest::set_default_smp_affinity(proc_path, value), self.default_interrupt_request_affinity.as_ref(), CouldNotChangeInterruptRequestDefaultAffinity)?;
		
		for (interrupt_request, hyper_threads) in self.interrupt_request_affinity.iter()
		{
			let interrupt_request = *interrupt_request;
			interrupt_request.set_smp_affinity(proc_path, hyper_threads).map_err(|cause| CouldNotChangeInterruptRequestAffinity(cause, interrupt_request))?;
		}
		
		set_value(proc_path, |proc_path, value| value.set_receive_packet_steering_flow_limit_tables_affinity(proc_path), self.receive_packet_steering_flow_limit_tables.as_ref(), CouldNotChangeWhichHyperThreadsHaveReceivePacketSteeringFlowLimitTablesEnabled)?;
		
		Ok(())
	}
}
