// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Process configuration error kind.
#[derive(Debug)]
pub enum ProcessConfigurationError
{
	#[allow(missing_docs)]
	CouldNotChangeInitialCapabilities(ThreadCapabilitiesConfigurationError),

	#[allow(missing_docs)]
	CouldNotObtainPersonality,

	#[allow(missing_docs)]
	CurrentPersonalityIsNotLinux(PersonalityFlags),

	#[allow(missing_docs)]
	CompiledCpuFeatureChecksFailed(FailedChecks<CompiledCpuFeatureCheck>),

	#[allow(missing_docs)]
	MandatoryCpuFeatureChecksFailed(FailedChecks<MandatoryCpuFeatureCheck>),

	#[allow(missing_docs)]
	OptionalCpuFeatureChecksFailed(FailedChecks<OptionalCpuFeatureCheck>),

	#[allow(missing_docs)]
	LinuxKernelVersionIsTooOld,

	#[allow(missing_docs)]
	CouldNotParseLinuxKernelVersion(io::Error),

	#[allow(missing_docs)]
	CouldNotChangeGlobalConfiguration(GlobalConfigurationError),

	#[allow(missing_docs)]
	CouldNotSetProcessName(io::Error),

	#[allow(missing_docs)]
	CouldNotSetLocale(LocaleName),

	#[allow(missing_docs)]
	ProcessNiceConfiguration(ProcessNiceConfigurationError),

	#[allow(missing_docs)]
	ProcessIoPriorityConfiguration(ProcessIoPriorityConfigurationError),

	#[allow(missing_docs)]
	IoFlusher(io::Error),

	#[allow(missing_docs)]
	CouldNotDisableDumpable(io::Error),

	#[allow(missing_docs)]
	CouldNotCloseAllOpenFileDescriptorsApartFromStandard(io::Error),

	#[allow(missing_docs)]
	RunningSetUid,

	#[allow(missing_docs)]
	RunningSetGid,

	#[allow(missing_docs)]
	CouldNotChangeResourceLimit(ResourceLimitError),

	#[allow(missing_docs)]
	CouldNotChangeProcessAffinity(String),

	#[allow(missing_docs)]
	JoinPaths(JoinPathsError),

	#[allow(missing_docs)]
	CouldNotChangeWorkingDirectory(io::Error),

	#[allow(missing_docs)]
	UtcFilePathDoesNotExistOrIsNotReadable(io::Error),

	#[allow(missing_docs)]
	CouldNotPreventTheGrantingOfNoNewPrivileges(io::Error),
}

impl Display for ProcessConfigurationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for ProcessConfigurationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::ProcessConfigurationError::*;

		match self
		{
			&CouldNotChangeInitialCapabilities(ref cause) => Some(cause),

			&CouldNotObtainPersonality => None,

			&CompiledCpuFeatureChecksFailed(ref cause) => Some(cause),

			&MandatoryCpuFeatureChecksFailed(ref cause) => Some(cause),

			&OptionalCpuFeatureChecksFailed(ref cause) => Some(cause),

			&CurrentPersonalityIsNotLinux(..) => None,

			&CouldNotParseLinuxKernelVersion(ref cause) => Some(cause),

			&LinuxKernelVersionIsTooOld => None,

			&CouldNotChangeGlobalConfiguration(ref cause) => Some(cause),

			&CouldNotSetProcessName(ref cause) => Some(cause),

			&CouldNotSetLocale(..) => None,

			&ProcessNiceConfiguration(ref cause) => Some(cause),

			&ProcessIoPriorityConfiguration(ref cause) => Some(cause),

			&IoFlusher(ref cause) => Some(cause),

			&CouldNotDisableDumpable(ref cause) => Some(cause),

			&CouldNotCloseAllOpenFileDescriptorsApartFromStandard(ref cause) => Some(cause),

			&RunningSetUid => None,

			&RunningSetGid => None,

			&CouldNotChangeResourceLimit(ref cause) => Some(cause),

			&CouldNotChangeProcessAffinity(..) => None,

			&JoinPaths(ref cause) => Some(cause),

			&CouldNotChangeWorkingDirectory(ref cause) => Some(cause),

			&CouldNotPreventTheGrantingOfNoNewPrivileges(ref cause) => Some(cause),

			&UtcFilePathDoesNotExistOrIsNotReadable(ref cause) => Some(cause),
		}
	}
}

impl From<FailedChecks<MandatoryCpuFeatureCheck>> for ProcessConfigurationError
{
	#[inline(always)]
	fn from(error: FailedChecks<MandatoryCpuFeatureCheck>) -> Self
	{
		ProcessConfigurationError::MandatoryCpuFeatureChecksFailed(error)
	}
}

impl From<FailedChecks<OptionalCpuFeatureCheck>> for ProcessConfigurationError
{
	#[inline(always)]
	fn from(error: FailedChecks<OptionalCpuFeatureCheck>) -> Self
	{
		ProcessConfigurationError::OptionalCpuFeatureChecksFailed(error)
	}
}

impl From<GlobalConfigurationError> for ProcessConfigurationError
{
	#[inline(always)]
	fn from(error: GlobalConfigurationError) -> Self
	{
		ProcessConfigurationError::CouldNotChangeGlobalConfiguration(error)
	}
}

impl From<ProcessNiceConfigurationError> for ProcessConfigurationError
{
	#[inline(always)]
	fn from(error: ProcessNiceConfigurationError) -> Self
	{
		ProcessConfigurationError::ProcessNiceConfiguration(error)
	}
}

impl From<ProcessIoPriorityConfigurationError> for ProcessConfigurationError
{
	#[inline(always)]
	fn from(error: ProcessIoPriorityConfigurationError) -> Self
	{
		ProcessConfigurationError::ProcessIoPriorityConfiguration(error)
	}
}

impl From<JoinPathsError> for ProcessConfigurationError
{
	#[inline(always)]
	fn from(cause: JoinPathsError) -> Self
	{
		ProcessConfigurationError::JoinPaths(cause)
	}
}
