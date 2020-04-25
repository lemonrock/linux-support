// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global kernel panic configuration.
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
	pub panic_on_io_non_maskable_interupt: Option<bool>,

	/// Defaults to false.
	///
	/// Requires root.
	pub panic_on_rcu_stall: Option<bool>,

	/// Defaults to false.
	///
	/// Requires root.
	pub panic_on_warn: Option<bool>,

	/// What to print on panic.
	pub what_to_print_on_panic: Option<WhatToPrintOnAKernelPanic>
}

impl GlobalKernelPanicConfiguration
{
	/// Configures.
	#[inline(always)]
	pub fn configure(&self, proc_path: &ProcPath) -> Result<(), GlobalKernelPanicConfigurationError>
	{
		use self::GlobalKernelPanicConfigurationError::*;

		#[inline(always)]
		fn configure_value<'a>(proc_path: &ProcPath, file_name: &'static str, value: Option<impl IntoLineFeedTerminatedByteString<'a>>, error: impl FnOnce(io::Error) -> GlobalPanicConfigurationError) -> Result<(), GlobalKernelPanicConfigurationError>
		{
			if let Some(value) = value
			{
				proc_path.sys_kernel_file_path(file_name).write_value(value).map_err(error)
			}
			else
			{
				Ok(())
			}
		}

		configure_value(proc_path, "panic", self.panic_timout_in_seconds, CouldNotChangePanicTimeout)?;
		configure_value(proc_path, "panic_on_oops", self.panic_timout_in_seconds, CouldNotChangePanicOnOops)?;
		configure_value(proc_path, "panic_on_io_nmi", self.panic_on_io_non_maskable_interupt, CouldNotChangePanicOnNonMaskableInterrupt)?;
		configure_value(proc_path, "panic_on_rcu_stall", self.panic_on_rcu_stall, CouldNotChangePanicOnRcuStall)?;
		configure_value(proc_path, "panic_on_warn", self.panic_on_warn, CouldNotChangePanicOnWarn)?;
		configure_value(proc_path, "panic_print", self.what_to_print_on_panic, CouldNotChangePanicPrint)?;

		Ok(())
	}
}
