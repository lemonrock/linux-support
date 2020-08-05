// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global kernel panic configuration.
///
/// This does not configure 'panic on Out-of-Memory', which is very much a setting to use if an application takes over a machine; see `GlobalOutOfMemoryConfiguration`.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GlobalKernelPanicConfiguration
{
	/// Defaults to 120 seconds.
	///
	/// Requires root.
	pub panic_timout_in_seconds: Option<KernelPanicRebootAction>,

	/// Defaults to false.
	///
	/// Requires root.
	pub panic_on_oops: Option<bool>,

	/// Defaults to false.
	///
	/// Requires root.
	pub panic_on_rcu_stall: Option<bool>,

	/// Defaults to false.
	///
	/// Requires root.
	pub panic_on_warn: Option<bool>,

	/// Defaults to false.
	///
	/// Requires root.
	pub panic_on_io_non_maskable_interupt: Option<bool>,

	/// Defaults to true but for security should be set to false.
	///
	/// Requires root.
	pub panic_on_unknown_non_maskable_interupt: Option<bool>,

	/// Defaults to true but for security should be set to false.
	///
	/// Requires root.
	pub panic_on_unrecovered_non_maskable_interupt: Option<bool>,

	/// Defaults to false and only usually available if the kernel has been compiled for debugging.
	///
	/// Requires root.
	pub panic_on_stack_overflow: Option<bool>,

	/// ?Kernel only tasks?
	///
	/// Requires root.
	pub panic_on_hung_task: Option<bool>,
	
	/// Panic on a memory check failure.
	///
	/// Requires root.
	pub panic_on_memory_check_failure: Option<bool>,

	/// If the software watchdog lockup detector is running, panic on a detected soft lockup.
	pub panic_on_software_watchdog_lockup: Option<bool>,

	/// Tries to capture debug information on software watchdog lockup.
	///
	/// See also `GlobalSchedulingConfiguration::enable_software_watchdog_lockup_detection`.
	pub capture_debug_information_on_software_watchdog_lockup: Option<bool>,

	/// If the software watchdog lockup detector is running, panic on a detected soft lockup.
	pub panic_on_hardware_watchdog_lockup: Option<bool>,

	/// Tries to capture debug information on software watchdog lockup.
	///
	/// See also `GlobalSchedulingConfiguration::enable_software_watchdog_lockup_detection`.
	pub capture_debug_information_on_hardware_watchdog_lockup: Option<bool>,

	/// What to print on panic.
	pub what_to_print_on_panic: Option<WhatToPrintOnAKernelPanic>,

	/// Reports panic data to Hyper-V.
	///
	/// Defaults to `true` if running under Hyper-V.
	pub report_panic_data_to_hyper_v: Option<bool>,
}

impl GlobalKernelPanicConfiguration
{
	/// Configures.
	#[inline(always)]
	pub fn configure(&self, proc_path: &ProcPath) -> Result<(), GlobalKernelPanicConfigurationError>
	{
		use self::GlobalKernelPanicConfigurationError::*;

		set_proc_sys_kernel_value(proc_path, "panic", self.panic_timout_in_seconds, CouldNotChangePanicTimeout)?;
		set_proc_sys_kernel_value(proc_path, "panic_on_oops", self.panic_timout_in_seconds, CouldNotChangePanicOnOops)?;
		set_proc_sys_kernel_value(proc_path, "panic_on_rcu_stall", self.panic_on_rcu_stall, CouldNotChangePanicOnRcuStall)?;
		set_proc_sys_kernel_value(proc_path, "panic_on_warn", self.panic_on_warn, CouldNotChangePanicOnWarn)?;
		set_proc_sys_kernel_value(proc_path, "panic_on_io_nmi", self.panic_on_io_non_maskable_interupt, CouldNotChangePanicOnIoNonMaskableInterrupt)?;
		set_proc_sys_kernel_value(proc_path, "unknown_nmi_panic", self.panic_on_unknown_non_maskable_interupt, CouldNotChangePanicOnUnknownNonMaskableInterrupt)?;
		set_proc_sys_kernel_value(proc_path, "panic_on_unrecovered_nmi", self.panic_on_unrecovered_non_maskable_interupt, CouldNotChangePanicOnUnrecoverableNonMaskableInterrupt)?;
		set_proc_sys_kernel_value(proc_path, "panic_on_stackoverflow", self.panic_on_stack_overflow, CouldNotChangePanicOnStackOverflow)?;
		set_proc_sys_kernel_value(proc_path, "panic_on_hung_task", self.panic_on_stack_overflow, CouldNotChangePanicOnHungTask)?;
		set_proc_sys_vm_value(proc_path, "memory_failure_recovery", self.panic_on_memory_check_failure.map(|value| !value), CouldNotChangePanicOnMemoryFailure)?;
		set_proc_sys_kernel_value(proc_path, "softlockup_panic", self.panic_on_software_watchdog_lockup, CouldNotChangePanicOnSoftwareWatchdogLockup)?;
		set_proc_sys_kernel_value(proc_path, "softlockup_all_cpu_backtrace", self.capture_debug_information_on_software_watchdog_lockup, CouldNotChangeSoftwareWatchdogLockupDebugInformation)?;
		set_proc_sys_kernel_value(proc_path, "hardlockup_panic", self.panic_on_hardware_watchdog_lockup, CouldNotChangePanicOnHardwareWatchdogLockup)?;
		set_proc_sys_kernel_value(proc_path, "hardlockup_all_cpu_backtrace", self.capture_debug_information_on_hardware_watchdog_lockup, CouldNotChangeHardwareWatchdogLockupDebugInformation)?;
		set_proc_sys_kernel_value(proc_path, "panic_print", self.what_to_print_on_panic, CouldNotChangePanicPrint)?;
		set_proc_sys_kernel_value(proc_path, "hyperv_record_panic_msg", self.report_panic_data_to_hyper_v, CouldNotChangeReportPanicDataToHyperV)?;

		Ok(())
	}
}
