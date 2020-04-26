// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global Linux kernel asynchronous IO (KAIO) configuration error kind.
#[derive(Debug)]
pub enum GlobalLinuxModuleConfigurationError
{
	#[allow(missing_docs)]
	CouldNotChangeModprobeExecutablePath(io::Error),

	/// Occurs if one or more Linux kernel modules is listed as both to unload and load.
	ReloadingLinuxKernelModulesIsUnsupported,

	#[allow(missing_docs)]
	LinuxKernelModulesListParse(LinuxKernelModulesListParseError),

	#[allow(missing_docs)]
	CouldNotUnloadLinuxKernelModuleBecauseModuleUnloadingIsDisabled,

	#[allow(missing_docs)]
	CouldNotUnloadLinuxKernelModule(io::Error),

	#[allow(missing_docs)]
	CouldNotLoadLinuxKernelModuleBecauseModuleLoadingIsDisabled,

	#[allow(missing_docs)]
	CouldNotLoadLinuxKernelModuleUsingModprobe(ModProbeError),

	#[allow(missing_docs)]
	CouldNotDisableModuleLoadingAndUnloadingUntilNextReboot(io::Error),
}

impl Display for GlobalLinuxModuleConfigurationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for GlobalLinuxModuleConfigurationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::GlobalLinuxModuleConfigurationError::*;

		match self
		{
			&CouldNotChangeModprobeExecutablePath(ref cause) => Some(cause),

			&ReloadingLinuxKernelModulesIsUnsupported => None,

			&LinuxKernelModulesListParse(ref cause) => Some(cause),

			&CouldNotUnloadLinuxKernelModuleBecauseModuleUnloadingIsDisabled => None,

			&CouldNotUnloadLinuxKernelModule(ref cause) => Some(cause),

			&CouldNotLoadLinuxKernelModuleUsingModprobe(ref cause) => Some(cause),

			&CouldNotLoadLinuxKernelModuleBecauseModuleLoadingIsDisabled => None,

			&CouldNotDisableModuleLoadingAndUnloadingUntilNextReboot(ref cause) => Some(cause),
		}
	}
}

impl From<LinuxKernelModulesListParseError> for GlobalLinuxModuleConfigurationError
{
	#[inline(always)]
	fn from(cause: LinuxKernelModulesListParseError) -> Self
	{
		GlobalLinuxModuleConfigurationError::LinuxKernelModulesListParse(cause)
	}
}

impl From<ModProbeError> for GlobalLinuxModuleConfigurationError
{
	#[inline(always)]
	fn from(cause: ModProbeError) -> Self
	{
		GlobalLinuxModuleConfigurationError::CouldNotLoadLinuxKernelModuleUsingModprobe(cause)
	}
}
