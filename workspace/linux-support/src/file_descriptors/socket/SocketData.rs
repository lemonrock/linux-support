// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.



/// Represents socket data.
pub trait SocketData: Sized + Default + Debug
{
	/// Socket family (eg `AF_UNIX`).
	fn family(&self) -> sa_family_t;

	#[doc(hidden)]
	#[inline(always)]
	fn specialized_drop(socket_file_descriptor: &mut SocketFileDescriptor<Self>)
	{
		socket_file_descriptor.0.close()
	}
}
