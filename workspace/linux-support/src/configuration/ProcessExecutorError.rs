// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Process executor error kind.
#[derive(Debug)]
pub enum ProcessExecutorError
{
	#[allow(missing_docs)]
	CouldNotLoadSeccompFilters(io::Error),

	#[allow(missing_docs)]
	CouldNotSynchronizeSeccompFiltersOnThread(ThreadIdentifier),

	#[allow(missing_docs)]
	UserAndGroupChoice(UserAndGroupChoiceError),

	#[allow(missing_docs)]
	CouldNotDisableDumpable(SystemCallErrorNumber),

	#[allow(missing_docs)]
	CouldNotSetParentDeathSignal(SystemCallErrorNumber),

	#[allow(missing_docs)]
	CouldNotSetChildSubreaper(SystemCallErrorNumber),

	#[allow(missing_docs)]
	CouldNotConfigureChildThreads(SpawnedThreadError),

	#[allow(missing_docs)]
	CouldNotConfigureMainThread(MainThreadConfigurationError),

	#[allow(missing_docs)]
	TerminatedDueToPanicOrIrrecoverableError,
}

impl Display for ProcessExecutorError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for ProcessExecutorError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::ProcessExecutorError::*;

		match self
		{
			&CouldNotLoadSeccompFilters(ref cause) => Some(cause),

			&CouldNotSynchronizeSeccompFiltersOnThread(..) => None,

			&UserAndGroupChoice(ref cause) => Some(cause),

			&CouldNotDisableDumpable(..) => None,

			&CouldNotSetParentDeathSignal(..) => None,

			&CouldNotSetChildSubreaper(..) => None,

			&CouldNotConfigureChildThreads(ref cause) => Some(cause),

			&CouldNotConfigureMainThread(ref cause) => Some(cause),

			&TerminatedDueToPanicOrIrrecoverableError => None,
		}
	}
}

impl From<UserAndGroupChoiceError> for ProcessExecutorError
{
	#[inline(always)]
	fn from(cause: UserAndGroupChoiceError) -> Self
	{
		ProcessExecutorError::UserAndGroupChoice(cause)
	}
}

impl From<SpawnedThreadError> for ProcessExecutorError
{
	#[inline(always)]
	fn from(cause: SpawnedThreadError) -> Self
	{
		ProcessExecutorError::CouldNotConfigureChildThreads(cause)
	}
}

impl From<MainThreadConfigurationError> for ProcessExecutorError
{
	#[inline(always)]
	fn from(cause: MainThreadConfigurationError) -> Self
	{
		ProcessExecutorError::CouldNotConfigureMainThread(cause)
	}
}
