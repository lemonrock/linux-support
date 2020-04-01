// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Validates the Linux Kernel command line for essential settings we need (or don't).
#[derive(Debug)]
pub struct LinuxKernelCommandLineValidator(LinuxKernelCommandLineParameters);

impl Deref for LinuxKernelCommandLineValidator
{
	type Target = LinuxKernelCommandLineParameters;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl LinuxKernelCommandLineValidator
{
	/// New.
	#[inline(always)]
	pub fn new(proc_path: &ProcPath) -> Self
	{
		Self(LinuxKernelCommandLineParameters::parse(proc_path).expect("Could not parse linux command line parameters"))
	}

	/// Validate and find isolated hyper threads, ie those hyper threads not in general use for the Kernel or other applications.
	#[inline(always)]
	pub fn validate_and_find_isolated_hyper_threads<E: error::Error, F: FnOnce(&LinuxKernelCommandLineParameters) -> Result<(), E>>(&self, isolated_cpus_required: bool, warnings_to_suppress: &WarningsToSuppress, cpu_features: &CpuFeatures, additional_validations: F) -> Result<BitSet<HyperThread>, LinuxKernelCommandLineValidationError<E>>
	{
		use self::LinuxKernelCommandLineValidationError::*;

		let isolated_hyper_threads = self.validate_cpus(isolated_cpus_required).map_err(|explanation| CpusInvalid(explanation))?;
		self.validate_huge_page_sizes(cpu_features.has_1gb_huge_pages).map_err(|explanation| HugePageSizesInvalid(explanation))?;
		self.incompatible_settings(cpu_features.has_1gb_huge_pages, warnings_to_suppress).map_err(|explanation| IncompatibleValidations(explanation))?;
		self.warnings(warnings_to_suppress);
		additional_validations(&self).map_err(|error| AdditionalLinuxKernelCommandLineValidationFailed(error))?;

		Ok(isolated_hyper_threads)
	}

	#[inline(always)]
	fn validate_cpus(&self, isolated_cpus_required: bool) -> Result<BitSet<HyperThread>, String>
	{
		if let Some((isolated_cpu_flags, isolated_cpus)) = self.isolcpus()
		{
			if !isolated_cpu_flags.contains(&IsolatedCpuFlags::Domain)
			{
				fail!("Kernel parameter `isolcpus` does not contain (or imply) the domain flag")
			}

			let rcu_nocbs = self.rcu_nocbs().ok_or("Kernel parameter `rcu_nocbs` should be specified because isolcpus was specified".to_string())?;

			let nohz_full = self.nohz_full().ok_or("Kernel parameter `nohz_full` should be specified because isolcpus was specified".to_string())?;

			// let irqaffinity = self.irqaffinity().ok_or("Kernel parameter `irqaffinity` should be specified because isolcpus was specified".to_string())?;

			if isolated_cpus != rcu_nocbs
			{
				fail!("Kernel parameters `isolcpus` and `rcu_nocbs` should match")
			}

			if isolated_cpus != nohz_full
			{
				fail!("Kernel parameters `isolcpus` and `nohz_full` should match")
			}

			Ok(isolated_cpus)
		}
		else
		{
			if isolated_cpus_required
			{
				fail!("Kernel parameter `isolcpus` should be specified")
			}
			else
			{
				Ok(BitSet::empty())
			}
		}
	}

	#[inline(always)]
	fn validate_huge_page_sizes(&self, cpu_supports_1gb_pages: bool) -> Result<(), String>
	{
		if cpu_supports_1gb_pages
		{
			match self.default_hugepagesz()
			{
				Some(b"1G") => (),
				_ => fail!("Kernel should have `default_hugepagesz=1G` for this CPU"),
			}

			let huge_page_sizes = self.hugepagesz().ok_or("Kernel should have `hugepagesz` for this CPU".to_string())?;

			let hugepages = self.hugepages().ok_or("Kernel should have `hugepages` for this CPU".to_string())?;

			if huge_page_sizes.len() != hugepages.len()
			{
				fail!("Kernel should have equal numbers of definitions of `hugepagesz` and `hugepages`")
			}

			for huge_page_size in huge_page_sizes.iter()
			{
				if !(huge_page_size == b"1G" || huge_page_size == b"2M")
				{
					return Err(format!("Invalid Kernel 'hugepagesz={:?}'", huge_page_size))
				}
			}
		}
		else
		{
			match self.default_hugepagesz()
			{
				None | Some(b"2M") => (),

				_ => fail!("Kernel should have `default_hugepagesz=2M` for this CPU"),
			}

			let huge_page_sizes_option = self.hugepagesz();
			let hugepages_option = self.hugepages();

			if huge_page_sizes_option.is_none() && hugepages_option.is_some() || huge_page_sizes_option.is_none() && hugepages_option.is_some()
			{
				fail!("Define either both of `hugepagesz` or `hugepage` or neither")
			}

			if huge_page_sizes_option.is_some() && hugepages_option.is_some()
			{
				let unwrapped = huge_page_sizes_option.unwrap();
				if unwrapped.len() != hugepages_option.unwrap().len()
				{
					fail!("Kernel should have equal numbers of definitions of `hugepagesz` and `hugepages`")
				}
			}
		}

		Ok(())
	}

	#[inline(always)]
	fn incompatible_settings(&self, cpu_supports_1gb_pages: bool, warnings_to_suppress: &WarningsToSuppress) -> Result<(), String>
	{
		if self.norandmaps()
		{
			fail!("Kernel has `norandmaps` enabled; this isn't secure")
		}

		if self.nokaslr()
		{
			fail!("Kernel has `nokaslr` enabled; this isn't secure")
		}

		if self.movable_node()
		{
			fail!("Kernel has `movable_node` enabled; this isn't compatible with this application")
		}

		if self.nosmp()
		{
			fail!("Kernel has `nosmp` enabled; this disables support for parallel CPUs")
		}

		if self.maxcpus() == Some(0)
		{
			fail!("Kernel has `maxcpus=0`; this disables support for parallel CPUs")
		}

		#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
		{
			match self.acpi()
			{
				Some(b"off") => fail!("Kernel has `acpi=off`"),

				_ => (),
			}
		}

		#[cfg(target_arch = "x86_64")]
		{
			if self.noapic()
			{
				fail!("Kernel has `noapic` enabled")
			}

			if self.disableapic()
			{
				fail!("Kernel has `disableapic` enabled")
			}

			if self.nolapic()
			{
				fail!("Kernel has `nolapic` enabled")
			}

			if self.noapictimer()
			{
				fail!("Kernel has `noapictimer` enabled")
			}

			if self.nospectre_v2()
			{
				fail!("Kernel has `nospectre_v2` enabled; this is wrong")
			}

			if let Some(mitigation) = self.spectre_v2()
			{
				match mitigation
				{
					b"on" | b"auto" | b"retpoline" | b"retpoline,amd" => (),

					b"retpoline,google" => warnings_to_suppress.kernel_warn_without_check("spectre_v2_google", "Kernel has `spectre_v2=retpoline,google`; this is probably not the best choice"),

					b"off" => fail!("Kernel spectre_v2 mitigation has been disabled; this is wrong and also useless, as DPDK-based applications make very few syscalls"),

					_ => return Err(format!("Kernel spectre_v2 mitigation '{:?}' not recognised; double-check this is intended", mitigation)),
				}
			}

			if let Some(pci_parameters) = self.pci()
			{
				if pci_parameters.contains(&b"off"[..])
				{
					fail!("Kernel has PCI disabled")
				}

				if pci_parameters.contains(&b"noacpi"[..])
				{
					fail!("Kernel has PCI noacpi")
				}
			}

			match self.numa()
			{
				None => (),

				Some((b"off", None)) => fail!("Kernel should not have NUMA disabled; we need it to work correctly"),

				Some((b"noacpi", None)) => fail!("Kernel should not have NUMA 'acpi' disabled; we need it to work correctly"),

				Some((b"fake", Some(_))) => fail!("Kernel should not have fake NUMA nodes; they do not have correctly assigned CPUs"),

				unrecognised @ _ => return Err(format!("Unrecognised Kernel NUMA options '{:?}", unrecognised)),
			}

			if self.nogbpages()
			{
				fail!("Kernel should not have `nogbpages`; on older systems, simply do not specify this flag")
			}

			if self.nohugeiomap()
			{
				fail!("Kernel has `nohugeiomap` enabled; this disables huge pages for IO")
			}

			if self.notsc()
			{
				fail!("Kernel has `notsc` enabled; this disables support for the Time Stamp Counter, TSC")
			}

			if self.nohpet()
			{
				fail!("Kernel has `nohpet` enabled; this disables support for the High Precision Event Timer, HPET")
			}

			if let Some(hpet_mmap_enabled) = self.hpet_mmap()
			{
				if !hpet_mmap_enabled
				{
					fail!("Kernel has `hpet_mmap=0`, ie hpet is disabled; this disables support for the High Precision Event Timer, HPET")
				}
			}

			if self.nopat()
			{
				fail!("Kernel has `nopat` enabled; this isn't useful")
			}

			if let Some(noexec_enabled) = self.noexec()
			{
				if !noexec_enabled
				{
					fail!("Kernel has `noexec=off`, ie non-executable mappings are disabled")
				}
			}

			if let Some(vdso_enabled) = self.vdso()
			{
				if !vdso_enabled
				{
					fail!("Kernel has `vdso=0`, ie vdso is disabled; this negatively impacts performance")
				}
			}

			if let Some(vdso32_enabled) = self.vdso32()
			{
				if !vdso32_enabled
				{
					fail!("Kernel has `vdso32=0`, ie vdso is disabled; this negatively impacts performance")
				}
			}

			if self.noinvpcid()
			{
				fail!("Kernel has `noinvpcid` enabled; this isn't useful")
			}
		}

		#[cfg(target_arch = "x86_64")]
		{
			if self.nopti()
			{
				fail!("Kernel has `nopti` enabled; this is wrong and also useless, as DPDK-based applications make very few syscalls")
			}

			if let Some(mitigation) = self.pti()
			{
				match mitigation
				{
					b"on" | b"auto" => (),

					b"off" => fail!("Kernel Control Page Table Isolation (pti) mitigation has been disabled; this is wrong and also useless, as DPDK-based applications make very few syscalls"),

					_ => return Err(format!("Kernel Control Page Table Isolation (pti) mitigation '{:?}' not recognised; double-check this is intended", mitigation)),
				}
			}

			match self.vsyscall()
			{
				None => fail!("Kernel vsyscall mitigation should be disabled with `vsycall=none`"),

				Some(b"none") => (),

				Some(b"emulate") => fail!("Kernel vsyscall should be disabled with `vsycall=none` not `vsyscall=emulate`"),

				Some(b"native") => fail!("vKernel syscall mitigration has been disabled; this is wrong and also useless, as DPDK-based applications do not use vsyscall"),

				unknown @ _ => return Err(format!("Kernel vsyscall mitigation '{:?}' not recognised; double-check this is intended", unknown.unwrap())),
			}

			if self.nopcid()
			{
				fail!("Kernel has `nopcid` enabled; this isn't useful")
			}

			match self.numa_balancing()
			{
				None | Some(true) => fail!("Kernel has NUMA balancing enabled"),
				_ => (),
			}

			if self.nox2apic()
			{
				fail!("Kernel has `nox2apic` enabled")
			}

			if let Some(noexec32_enabled) = self.noexec32()
			{
				if !noexec32_enabled
				{
					fail!("Kernel has `noexec32=off`, ie non-executable mappings are disabled")
				}
			}

			if cpu_supports_1gb_pages
			{
				if !self.gbpages()
				{
					fail!("Kernel should have `gbpages`")
				}

				if self.nogbpages()
				{
					fail!("Kernel should not have `nogbpages`")
				}
			}
		}

		Ok(())
	}

	#[inline(always)]
	fn warnings(&self, warnings_to_suppress: &WarningsToSuppress)
	{
		match self.hashdist()
		{
			None => warnings_to_suppress.kernel_warn_without_check("hashdist", "Kernel should be booted with `hashdist=0` to disable NUMA hash distribution"),

			Some(false) => warnings_to_suppress.kernel_warn_without_check("hashdist", "Kernel should be booted with `hashdist=0` to disable NUMA hash distribution; it was booted with `hashdist=1`, which is the default and so pointless"),

			_ => (),
		}

		warnings_to_suppress.kernel_warn("noaliencache", "Kernel has `noaliencache` enabled; this is likely to hurt performance", || !self.noaliencache());

		warnings_to_suppress.kernel_warn("numa_zonelist_order", "Kernel has `numa_zonelist_order` enabled; this is likely to hurt performance", || self.numa_zonelist_order().is_none());

		match self.skew_tick()
		{
			None | Some(false) => warnings_to_suppress.kernel_warn_without_check("skew_tick", "Kernel should have `skew_tick=1` for maximum performance at the cost of power consumption"),
			Some(true) => (),
		}

		#[cfg(target_arch = "x86_64")]
		{
			match self.idle()
			{
				None => warnings_to_suppress.kernel_warn_without_check("idle_poll", "Kernel should be booted with `idle=poll` for maximum performance at the cost of power consumption"),

				Some(value) => match value
				{
					b"idle" => (),

					b"halt" | b"nomwait" => warnings_to_suppress.kernel_warn_without_check("idle_poll", &format!("Kernel should be booted with `idle=poll` rather than `idle={:?}` for maximum performance at the cost of power consumption, although there may be impacts on hyper threading", value)),

					_ => (),
				},
			}

			warnings_to_suppress.kernel_warn("noxsaveopt", "Kernel has `noxsaveopt` enabled; this is likely to hurt performance", || !self.noxsaveopt());
		}
	}
}
