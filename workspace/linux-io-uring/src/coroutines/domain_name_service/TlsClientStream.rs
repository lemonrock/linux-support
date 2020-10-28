// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// TLS client stream.
pub struct TlsClientStream<'yielder, SD: SocketData>
{
	marker: PhantomData<&'yielder SD>,
}

impl<'yielder, SD: SocketData> TlsClientStream<'yielder, SD>
{
	pub(crate) fn write_all_data(&mut self, _buffer: &[u8])
	{
	}
	
	pub(crate) fn read_all_data(&mut self, _buffer: &mut [u8])
	{
	}
}
