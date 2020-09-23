// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Access common fields.
pub trait ExpressDataPathSocket<'a, ROTOB: 'a + ReceiveOrTransmitOrBoth, FFQ: 'a + FreeFrameQueue>
{
	/// Statistics.
	#[inline(always)]
	fn statistics(&'a self) -> xdp_statistics
	{
		self.express_data_path_socket_file_descriptor().statistics()
	}
	
	/// Options.
	#[inline(always)]
	fn options(&'a self) -> xdp_options
	{
		self.express_data_path_socket_file_descriptor().options()
	}
	
	#[doc(hidden)]
	fn user_memory(&'a self) -> &'a UserMemory<FFQ>;
	
	#[doc(hidden)]
	fn common(&'a self) -> &'a CommonExpressDataPathSocket<ROTOB>;
	
	#[doc(hidden)]
	fn express_data_path_socket_file_descriptor(&'a self) -> &'a ExpressDataPathSocketFileDescriptor;
}
