// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Process capabilities configuration error kind.
#[derive(Debug)]
pub enum ThreadConfigurationError
{
	#[allow(missing_docs)]
	DisableTransparentHugePages(Errno),
	
	#[allow(missing_docs)]
	StoreBypassSpeculationMitigationControl(Errno),
	
	#[allow(missing_docs)]
	IndirectBranchSpeculationMitigationControl(Errno),
	
	#[allow(missing_docs)]
	PerThreadMemoryAllocatorInstantiation(MemoryMapError),
	
	#[allow(missing_docs)]
	CouldNotStartThreadLocalLogging(NewSocketClientError),

	#[allow(missing_docs)]
	CouldNotSetThreadAffinity(String),

	#[allow(missing_docs)]
	CouldNotSetNice,

	#[allow(missing_docs)]
	CouldNotSetIoPriority(bool),

	#[allow(missing_docs)]
	CouldNotSetSchedulerPolicyAndFlags(&'static str),
	
	#[allow(missing_docs)]
	ThreadFunctionInitializationFailed(anyhow::Error),
	
	#[allow(missing_docs)]
	Capabilities(ThreadCapabilitiesConfigurationError),
}

impl Display for ThreadConfigurationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for ThreadConfigurationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::ThreadConfigurationError::*;

		match self
		{
			&DisableTransparentHugePages(..) => None,
			
			&StoreBypassSpeculationMitigationControl(..) => None,
			
			&IndirectBranchSpeculationMitigationControl(..) => None,
			
			&PerThreadMemoryAllocatorInstantiation(ref cause) => Some(cause),
			
			&CouldNotStartThreadLocalLogging(ref cause) => Some(cause),

			&CouldNotSetThreadAffinity(..) => None,

			&CouldNotSetNice => None,

			&CouldNotSetIoPriority(..) => None,

			&CouldNotSetSchedulerPolicyAndFlags(..) => None,
			
			&ThreadFunctionInitializationFailed(ref cause) => Some(cause.as_ref()),
			
			&Capabilities(ref cause) => Some(cause),
		}
	}
}

impl From<ThreadCapabilitiesConfigurationError> for ThreadConfigurationError
{
	#[inline(always)]
	fn from(cause: ThreadCapabilitiesConfigurationError) -> Self
	{
		ThreadConfigurationError::Capabilities(cause)
	}
}
