// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[cfg(not(target_arch = "x86_64"))]
#[inline(always)]
pub(crate) fn validate_huge_page_sizes(_linux_kernel_command_line_parameters: &LinuxKernelCommandLine, _cpu_supports_1gb_pages: bool) -> Result<(), &'static str>
{
	Ok(())
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub(crate) fn validate_huge_page_sizes(linux_kernel_command_line_parameters: &LinuxKernelCommandLine, cpu_supports_1gb_pages: bool) -> Result<(), &'static str>
{
	if cpu_supports_1gb_pages
	{
		if !linux_kernel_command_line_parameters.gbpages()
		{
			return Err("Kernel should have `gbpages`")
		}

		if linux_kernel_command_line_parameters.nogbpages()
		{
			return Err("Kernel should not have `nogbpages`")
		}

		match linux_kernel_command_line_parameters.default_hugepagesz()
		{
			Some(b"1G") => (),
			_ => return Err("Kernel should have `default_hugepagesz=1G` for this CPU"),
		}

		let huge_page_sizes = linux_kernel_command_line_parameters.hugepagesz().ok_or("Kernel should have `hugepagesz` for this CPU")?;

		let hugepages = linux_kernel_command_line_parameters.hugepages().ok_or("Kernel should have `hugepages` for this CPU")?;

		if huge_page_sizes.len() != hugepages.len()
		{
			return Err("Kernel should have equal numbers of definitions of `hugepagesz` and `hugepages`")
		}

		for huge_page_size in huge_page_sizes.iter()
		{
			if !(huge_page_size == b"1G" || huge_page_size == b"2M")
			{
				return Err("Invalid Kernel 'hugepagesz='")
			}
		}
	}
	else
	{
		if linux_kernel_command_line_parameters.gbpages()
		{
			return Err("Kernel should not have `gbpages`")
		}

		match linux_kernel_command_line_parameters.default_hugepagesz()
		{
			None | Some(b"2M") => (),

			_ => return Err("Kernel should have `default_hugepagesz=2M` for this CPU"),
		}

		let huge_page_sizes_option = linux_kernel_command_line_parameters.hugepagesz();
		let hugepages_option = linux_kernel_command_line_parameters.hugepages();

		if huge_page_sizes_option.is_none() && hugepages_option.is_some() || huge_page_sizes_option.is_none() && hugepages_option.is_some()
		{
			return Err("Define either both of `hugepagesz` or `hugepage` or neither")
		}

		if huge_page_sizes_option.is_some() && hugepages_option.is_some()
		{
			let unwrapped = huge_page_sizes_option.unwrap();
			if unwrapped.len() != hugepages_option.unwrap().len()
			{
				return Err("Kernel should have equal numbers of definitions of `hugepagesz` and `hugepages`")
			}
		}
	}
	Ok(())
}
