// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


macro_rules! validate_opcode
{
	($message_header: ident) =>
	{
		match $message_header.raw_message_opcode()
		{
			MessageOpcode::Query => (),
			MessageOpcode::InverseQuery => return Err(InvalidResponseOpcode(MessageOpcode::InverseQuery)),
			MessageOpcode::Status => return Err(InvalidResponseOpcode(MessageOpcode::Status)),
			opcode @ 3 => return Err(UnassignedResponseOpcode(opcode)),
			MessageOpcode::Notify => return Err(InvalidResponseOpcode(MessageOpcode::Notify)),
			MessageOpcode::Update => return Err(InvalidResponseOpcode(MessageOpcode::Update)),
			MessageOpcode::DnsStatefulOperations => return Err(InvalidResponseOpcode(MessageOpcode::DnsStatefulOperations)),
			opcode @ 7 ..= 15 => return Err(UnassignedResponseOpcode(opcode)),
			_ => unreachable!(),
		}
	}
}
