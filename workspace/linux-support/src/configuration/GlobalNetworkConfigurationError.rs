// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global EPoll configuration error kind.
#[derive(Debug)]
pub enum GlobalNetworkConfigurationError
{
	#[allow(missing_docs)]
	CouldNotChangeGlobalMaximumSendBufferSize(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeGlobalDefaultSendBufferSize(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeGlobalTcpMinimumDefaultAndMaximumSendBufferSize(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeGlobalMaximumReceiveBufferSize(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeGlobalDefaultReceiveBufferSize(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeGlobalTcpMinimumDefaultAndMaximumReceiveBufferSize(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeGlobalMaximumControlMessageBufferSize(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeGlobalDefaultKeepAliveIntervalSeconds(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeGlobalDefaultIdlesBeforeKeepAliveSeconds(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeGlobalDefaultMaximumKeepAliveProbes(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeGlobalDefaultFinishTimeout(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeGlobalDefaultMaximumSynRetransmits(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeGlobalDefaultMaximumSynAckRetransmits(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeGlobalDefaultNotSentLowWater(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeGlobalMaximumBackLog(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeGlobalMaximumSynBackLog(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeGlobalMaximumOrphans(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeGlobalMaximumTimeWait(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeGlobalDefaultCongestionControlAlgorithm(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeGlobalDefaultQueuingDisciplineAlgorithm(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeGlobalDefaultRetries1(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeGlobalDefaultRetries2(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeGlobalDefaultRetriesOrphan(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeGlobalDefaultReorderingThreshold(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeMemoryPressure(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeGlobalDefaultReceivePacketSteeringFlowsPerCpu(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeAutoCorking(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeGlobalDefaultSocketBusyRead(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeGlobalDefaultSocketBusySelectAndPoll(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeMaximumUnixDomainSocketDatagramQueueLength(io::Error),
}

impl Display for GlobalNetworkConfigurationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for GlobalNetworkConfigurationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::GlobalNetworkConfigurationError::*;

		match self
		{
			&CouldNotChangeGlobalMaximumSendBufferSize(ref cause) => Some(cause),
			
			&CouldNotChangeGlobalDefaultSendBufferSize(ref cause) => Some(cause),
			
			&CouldNotChangeGlobalTcpMinimumDefaultAndMaximumSendBufferSize(ref cause) => Some(cause),
			
			&CouldNotChangeGlobalMaximumReceiveBufferSize(ref cause) => Some(cause),
			
			&CouldNotChangeGlobalDefaultReceiveBufferSize(ref cause) => Some(cause),
			
			&CouldNotChangeGlobalTcpMinimumDefaultAndMaximumReceiveBufferSize(ref cause) => Some(cause),
			
			&CouldNotChangeGlobalMaximumControlMessageBufferSize(ref cause) => Some(cause),
			
			&CouldNotChangeGlobalDefaultKeepAliveIntervalSeconds(ref cause) => Some(cause),
			
			&CouldNotChangeGlobalDefaultIdlesBeforeKeepAliveSeconds(ref cause) => Some(cause),
			
			&CouldNotChangeGlobalDefaultMaximumKeepAliveProbes(ref cause) => Some(cause),
			
			&CouldNotChangeGlobalDefaultFinishTimeout(ref cause) => Some(cause),
			
			&CouldNotChangeGlobalDefaultMaximumSynRetransmits(ref cause) => Some(cause),
			
			&CouldNotChangeGlobalDefaultMaximumSynAckRetransmits(ref cause) => Some(cause),
			
			&CouldNotChangeGlobalDefaultNotSentLowWater(ref cause) => Some(cause),
			
			&CouldNotChangeGlobalMaximumBackLog(ref cause) => Some(cause),
			
			&CouldNotChangeGlobalMaximumSynBackLog(ref cause) => Some(cause),
			
			&CouldNotChangeGlobalMaximumOrphans(ref cause) => Some(cause),
			
			&CouldNotChangeGlobalMaximumTimeWait(ref cause) => Some(cause),
			
			&CouldNotChangeGlobalDefaultCongestionControlAlgorithm(ref cause) => Some(cause),
			
			&CouldNotChangeGlobalDefaultQueuingDisciplineAlgorithm(ref cause) => Some(cause),
			
			&CouldNotChangeGlobalDefaultRetries1(ref cause) => Some(cause),
			
			&CouldNotChangeGlobalDefaultRetries2(ref cause) => Some(cause),
			
			&CouldNotChangeGlobalDefaultRetriesOrphan(ref cause) => Some(cause),
			
			&CouldNotChangeGlobalDefaultReorderingThreshold(ref cause) => Some(cause),
			
			&CouldNotChangeMemoryPressure(ref cause) => Some(cause),
			
			&CouldNotChangeGlobalDefaultReceivePacketSteeringFlowsPerCpu(ref cause) => Some(cause),
			
			&CouldNotChangeAutoCorking(ref cause) => Some(cause),
			
			&CouldNotChangeGlobalDefaultSocketBusyRead(ref cause) => Some(cause),
			
			&CouldNotChangeGlobalDefaultSocketBusySelectAndPoll(ref cause) => Some(cause),
			
			&CouldNotChangeMaximumUnixDomainSocketDatagramQueueLength(ref cause) => Some(cause),
		}
	}
}
