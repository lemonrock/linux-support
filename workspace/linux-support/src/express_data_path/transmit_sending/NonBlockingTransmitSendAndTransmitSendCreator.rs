// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A simple, non-blocking transmit send.
#[derive(Debug)]
pub struct NonBlockingTransmitSendAndTransmitSendCreator(RawFd);

impl TransmitSendCreator for NonBlockingTransmitSendAndTransmitSendCreator
{
	type TS = Self;
	
	#[inline(always)]
	fn create(self, express_data_path_socket_file_descriptor: &ExpressDataPathSocketFileDescriptor) -> Self::TS
	{
		Self(express_data_path_socket_file_descriptor.as_raw_fd())
	}
}

impl TransmitSend for NonBlockingTransmitSendAndTransmitSendCreator
{
	#[inline(always)]
	fn send(&mut self)
	{
		let result = unsafe { sendto(self.0, null(), 0, MSG_DONTWAIT, null(), 0) };
		if likely!(result >= 0)
		{
			return
		}
		else if likely!(result == -1)
		{
			match SystemCallErrorNumber::from_errno()
			{
				ENOBUFS | EAGAIN | EBUSY | ENETDOWN => return,
				
				unexpected @ _ => panic!("Unexpected error `{}` from `sendto()`", unexpected)
			}
		}
		else
		{
			unreachable_code(format_args!("Unexpected result `{}` from `sendto()`", result))
		};
	}
}
