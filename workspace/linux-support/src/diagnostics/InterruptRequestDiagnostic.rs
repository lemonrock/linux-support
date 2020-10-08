// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InterruptRequestDiagnostic
{
	/// Actions (Interrupt Names).
	///
	/// Values observed on a Parallels VM:-
	///
	/// * (empty string).
	/// * `acpi`.
	/// * `ahci[0000:00:1f.2]`.
	/// * `ata_piix`.
	/// * `i8042`.
	/// * `rtc0`.
	/// * `timer`.
	/// * `virtio1`.
	/// * `virtio0-config`.
	/// * `virtio0-input.0`.
	/// * `virtio0-output.0`.
	pub sysfs_actions: DiagnosticUnobtainableResult<Vec<InterruptRequestActionName>>,
	
	/// Actions (Interrupt Names).
	///
	/// Values observed on a Parallels VM:-
	///
	/// * (empty string).
	/// * `acpi`.
	/// * `ahci[0000:00:1f.2]`.
	/// * `ata_piix`.
	/// * `i8042`.
	/// * `rtc0`.
	/// * `timer`.
	/// * `virtio1`.
	/// * `virtio0-config`.
	/// * `virtio0-input.0`.
	/// * `virtio0-output.0`.
	pub procfs_actions: DiagnosticUnobtainableResult<Vec<InterruptRequestActionName>>,
	
	/// Values observed on a Parallels VM:-
	///
	/// * `IO-APIC`.
	/// * `PCI-MSI`.
	/// * `XT-PIC`.
	pub chip_name: DiagnosticUnobtainableResult<Option<CString>>,
	
	/// Values observed on a Parallels VM:-
	///
	/// eg `10`; usually the same value as tje IRQ number but can be a large value, eg `512000`.
	///
	/// Note that `0xFFFF_FFFF` is invalid (this is not an error).
	pub hardware_interrupt_request_line: DiagnosticUnobtainableResult<Option<u32>>,
	
	/// Values observed on a Parallels VM:-
	///
	/// * `Some(edge)`.
	/// * `Some(fasteoi)`.
	pub name: DiagnosticUnobtainableResult<Option<CString>>,
	
	/// Type.
	pub type_: DiagnosticUnobtainableResult<InterruptRequestType>,
	
	/// Values observed on a Parallels VM:-
	///
	/// * `disabled`.
	pub wake_up: DiagnosticUnobtainableResult<InterruptRequestWakeUp>,
	
	/// Number of occurrences per-HyperThread.
	///
	/// Number of indices is the number of possible HyperThreads (`/sys/devices/system/cpu/possible`).
	pub occurrences_per_hyper_thread: DiagnosticUnobtainableResult<PerBitSetAwareData<HyperThread, u64>>,

	/// Obtaining may cause a `panic!`.
	pub affinity_hint: HyperThreads,

	/// Obtaining may cause a `panic!`.
	pub effective_affinity: HyperThreads,

	/// Obtaining may cause a `panic!`.
	pub effective_affinity_list: HyperThreads,

	/// Obtaining may cause a `panic!`.
	pub numa_node: Option<NumaNode>,

	/// Obtaining may cause a `panic!`.
	pub smp_affinity: HyperThreads,

	/// Obtaining may cause a `panic!`.
	pub smp_affinity_list: HyperThreads,

	/// Obtaining may cause a `panic!`.
	pub spurious: DiagnosticUnobtainableResult<SpuriousInterruptRequestInformation>,
}

impl InterruptRequestDiagnostic
{
	fn gather(sys_path: &SysPath, proc_path: &ProcPath, interrupt_request: InterruptRequest) -> Self
	{
		Self
		{
			sysfs_actions: interrupt_request.sysfs_actions(sys_path).map_err(DiagnosticUnobtainable::from),
			procfs_actions: interrupt_request.procfs_actions(sys_path).map_err(DiagnosticUnobtainable::from),
			chip_name: interrupt_request.chip_name(sys_path).map_err(DiagnosticUnobtainable::from),
			hardware_interrupt_request_line: interrupt_request.hardware_interrupt_request_line(sys_path).map_err(DiagnosticUnobtainable::from),
			name: interrupt_request.name(sys_path).map_err(DiagnosticUnobtainable::from),
			type_: interrupt_request.type_(sys_path).map_err(DiagnosticUnobtainable::from),
			wake_up: interrupt_request.wake_up(sys_path).map_err(DiagnosticUnobtainable::from),
			occurrences_per_hyper_thread: interrupt_request.occurrences_per_hyper_thread(sys_path).map_err(DiagnosticUnobtainable::from),
			affinity_hint: interrupt_request.affinity_hint(proc_path),
			effective_affinity: interrupt_request.effective_affinity(proc_path),
			effective_affinity_list: interrupt_request.effective_affinity_list(proc_path),
			numa_node: interrupt_request.numa_node(proc_path),
			smp_affinity: interrupt_request.smp_affinity(proc_path),
			smp_affinity_list: interrupt_request.smp_affinity_list(proc_path),
			spurious: interrupt_request.spurious(proc_path).map_err(DiagnosticUnobtainable::from),
		}
	}
}
