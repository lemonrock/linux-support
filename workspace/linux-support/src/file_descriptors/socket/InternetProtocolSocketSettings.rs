// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Internet protocol socket settings.
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InternetProtocolSocketSettings
{
	#[serde(default = "InternetProtocolSocketSettings::send_buffer_size_default")] pub send_buffer_size: SendBufferSizeSocketOption,
	
	#[serde(default = "InternetProtocolSocketSettings::receive_buffer_size_default")] pub receive_buffer_size: ReceiveBufferSizeSocketOption,
	
	#[serde(default)] pub queuing_discipline_send_priority: QueuingDisciplineSendPriority,
	
	/// A value of `Some(n)` requires the capability `CAP_NET_ADMIN` if `n` is greater than the system default (which is usually `0`, ie `off`).
	///
	/// Only exists if the Linux kernel has been configured with `CONFIG_NET_RX_BUSY_POLL`.
	#[serde(default)] pub busy_poll_microseconds: Option<BusyPollMicroseconds>,
}

impl Default for InternetProtocolSocketSettings
{
	fn default() -> Self
	{
		Self
		{
			send_buffer_size: Self::send_buffer_size_default(),
			receive_buffer_size: Self::receive_buffer_size_default(),
			queuing_discipline_send_priority: QueuingDisciplineSendPriority::default(),
			busy_poll_microseconds: None,
		}
	}
}

impl InternetProtocolSocketSettings
{
	#[inline(always)]
	const fn send_buffer_size_default() -> SendBufferSizeSocketOption
	{
		SendBufferSizeSocketOption
		{
			size: SendBufferSizeInBytes::UsualGlobalDefaultForTcp,
			force: false,
		}
	}
	
	#[inline(always)]
	const fn receive_buffer_size_default() -> ReceiveBufferSizeSocketOption
	{
		ReceiveBufferSizeSocketOption
		{
			size: ReceiveBufferSizeInBytes::UsualGlobalDefaultForTcp,
			force: false,
		}
	}
}
