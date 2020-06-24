// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// TCP socket settings.
#[allow(missing_docs)]
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields, default)]
pub struct TransmissionControlProtocolSocketSettings
{
	pub idles_before_keep_alive_seconds: IdlesBeforeKeepAliveSeconds,
	
	pub keep_alive_interval_seconds: KeepAliveIntervalSeconds,
	
	pub maximum_keep_alive_probes: MaximumKeepAliveProbes,
	
	pub socket_linger_seconds: SocketLingerSeconds,
	
	pub finish_timeout_seconds: FinishTimeoutSeconds,
	
	pub maximum_syn_retransmits: MaximumSynRetransmits,
	
	/// Set this to 16KB for HTTP/2 prioritization to work reliably.
	pub not_sent_low_water_in_bytes: NotSentLowWaterInBytes,
}
