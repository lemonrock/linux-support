// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Query<A: Allocator>
{
	data_type: DataType,
	query_name: UncompressedName<A>,
}

impl<A: Allocator> Query<A>
{
	#[allow(deprecated)]
	#[inline(always)]
	pub fn write_query<'yielder, SD: SocketData>(&self, stream: &mut TlsClientStream<'yielder, SD>, message_identifier: MessageIdentifier)
	{
		let mut buffer: [u8; TcpMessage::MaximumQueryBufferSize] = unsafe { uninitialized() };

		let buffer_pointer = (&mut buffer[..]).start_pointer();

		let buffer_length = TcpMessage::write_query_tcp_message(buffer_pointer, message_identifier, self.data_type, &self.query_name);

		stream.write_all_data(&buffer[..])
	}

	pub(crate) fn matches<'message>(&self, received_data_type: DataType, received_query_name: WithoutCompressionParsedName<'message>) -> Result<(), DnsProtocolError>
	{
		if unlikely!(self.data_type != received_data_type)
		{
			return Err(ResponseWasForADifferentDataType)
		}

		let expected_query_name = self.query_name.name();

		if likely!(expected_query_name == received_query_name)
		{
			Ok(())
		}
		else
		{
			Err(ResponseWasForADifferentName)
		}
	}
}
