// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[inline(always)]
pub(crate) fn incompatible_settings(linux_kernel_command_line_parameters: &LinuxKernelCommandLineParameters) -> Result<(), &'static str>
{
	if linux_kernel_command_line_parameters.norandmaps()
	{
		return Err("Kernel has `norandmaps` enabled; this isn't secure")
	}

	if linux_kernel_command_line_parameters.nokaslr()
	{
		return Err("Kernel has `nokaslr` enabled; this isn't secure")
	}

	if linux_kernel_command_line_parameters.movable_node()
	{
		return Err("Kernel has `movable_node` enabled; this isn't compatible with this application")
	}

	if linux_kernel_command_line_parameters.nosmp()
	{
		return Err("Kernel has `nosmp` enabled; this disables support for parallel CPUs")
	}

	if linux_kernel_command_line_parameters.maxcpus() == Some(0)
	{
		return Err("Kernel has `maxcpus=0`; this disables support for parallel CPUs")
	}

	#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
	{
		match linux_kernel_command_line_parameters.acpi()
		{
			Some(b"off") => return Err("Kernel has `acpi=off`"),

			_ => (),
		}
	}

	#[cfg(target_arch = "x86_64")]
	{
		if linux_kernel_command_line_parameters.noapic()
		{
			return Err("Kernel has `noapic` enabled")
		}

		if linux_kernel_command_line_parameters.disableapic()
		{
			return Err("Kernel has `disableapic` enabled")
		}

		if linux_kernel_command_line_parameters.nolapic()
		{
			return Err("Kernel has `nolapic` enabled")
		}

		if linux_kernel_command_line_parameters.noapictimer()
		{
			return Err("Kernel has `noapictimer` enabled")
		}

		if linux_kernel_command_line_parameters.nospectre_v2()
		{
			return Err("Kernel has `nospectre_v2` enabled; this is wrong")
		}

		if let Some(mitigation) = linux_kernel_command_line_parameters.spectre_v2()
		{
			match mitigation
			{
				b"on" | b"auto" | b"retpoline" | b"retpoline,amd" => (),

				b"retpoline,google" => return Err("Kernel has `spectre_v2=retpoline,google`; this is probably not the best choice"),

				b"off" => return Err("Kernel spectre_v2 mitigation has been disabled; this is wrong"),

				_ => return return Err("Kernel spectre_v2 mitigation not recognised"),
			}
		}

		if let Some(pci_parameters) = linux_kernel_command_line_parameters.pci()
		{
			if pci_parameters.contains(&b"off"[..])
			{
				return Err("Kernel has PCI disabled")
			}

			if pci_parameters.contains(&b"noacpi"[..])
			{
				return Err("Kernel has PCI noacpi")
			}
		}

		match linux_kernel_command_line_parameters.numa()
		{
			None => (),

			Some((b"off", None)) => return Err("Kernel should not have NUMA disabled; we need it to work correctly"),

			Some((b"noacpi", None)) => return Err("Kernel should not have NUMA 'acpi' disabled; we need it to work correctly"),

			Some((b"fake", Some(_))) => return Err("Kernel should not have fake NUMA nodes; they do not have correctly assigned CPUs"),

			unrecognised @ _ => return Err("Unrecognised Kernel NUMA options"),
		}

		if linux_kernel_command_line_parameters.nohugeiomap()
		{
			return Err("Kernel has `nohugeiomap` enabled; this disables huge pages for IO")
		}

		if linux_kernel_command_line_parameters.notsc()
		{
			return Err("Kernel has `notsc` enabled; this disables support for the Time Stamp Counter, TSC")
		}

		if linux_kernel_command_line_parameters.nohpet()
		{
			return Err("Kernel has `nohpet` enabled; this disables support for the High Precision Event Timer, HPET")
		}

		if let Some(hpet_mmap_enabled) = linux_kernel_command_line_parameters.hpet_mmap()
		{
			if !hpet_mmap_enabled
			{
				return Err("Kernel has `hpet_mmap=0`, ie hpet is disabled; this disables support for the High Precision Event Timer, HPET")
			}
		}

		if linux_kernel_command_line_parameters.nopat()
		{
			return Err("Kernel has `nopat` enabled; this isn't useful")
		}

		if let Some(noexec_enabled) = linux_kernel_command_line_parameters.noexec()
		{
			if !noexec_enabled
			{
				return Err("Kernel has `noexec=off`, ie non-executable mappings are disabled")
			}
		}

		if let Some(vdso_enabled) = linux_kernel_command_line_parameters.vdso()
		{
			if !vdso_enabled
			{
				return Err("Kernel has `vdso=0`, ie vdso is disabled; this negatively impacts performance")
			}
		}

		if let Some(vdso32_enabled) = linux_kernel_command_line_parameters.vdso32()
		{
			if !vdso32_enabled
			{
				return Err("Kernel has `vdso32=0`, ie vdso is disabled; this negatively impacts performance")
			}
		}

		if linux_kernel_command_line_parameters.noinvpcid()
		{
			return Err("Kernel has `noinvpcid` enabled; this isn't useful")
		}
	}

	#[cfg(target_arch = "x86_64")]
	{
		if linux_kernel_command_line_parameters.nopti()
		{
			return Err("Kernel has `nopti` enabled; this is wrong")
		}

		if let Some(mitigation) = linux_kernel_command_line_parameters.pti()
		{
			match mitigation
			{
				b"on" | b"auto" => (),

				b"off" => return Err("Kernel Control Page Table Isolation (pti) mitigation has been disabled"),

				_ => return Err("Kernel Control Page Table Isolation (pti) mitigation not recognised"),
			}
		}

		match linux_kernel_command_line_parameters.vsyscall()
		{
			None => return Err("Kernel vsyscall mitigation should be disabled with `vsycall=none`"),

			Some(b"none") => (),

			Some(b"emulate") => return Err("Kernel vsyscall should be disabled with `vsycall=none` not `vsyscall=emulate`"),

			Some(b"native") => return Err("Kernel vsyscall mitigration has been disabled; this is wrong"),

			unknown @ _ => return Err("Kernel vsyscall mitigation not recognised"),
		}

		if linux_kernel_command_line_parameters.nopcid()
		{
			return Err("Kernel has `nopcid` enabled; this isn't useful")
		}

		match linux_kernel_command_line_parameters.numa_balancing()
		{
			None | Some(true) => return Err("Kernel has NUMA balancing enabled"),
			_ => (),
		}

		if linux_kernel_command_line_parameters.nox2apic()
		{
			return Err("Kernel has `nox2apic` enabled")
		}

		if let Some(noexec32_enabled) = linux_kernel_command_line_parameters.noexec32()
		{
			if !noexec32_enabled
			{
				return Err("Kernel has `noexec32=off`, ie non-executable mappings are disabled")
			}
		}
	}

	Ok(())
}
