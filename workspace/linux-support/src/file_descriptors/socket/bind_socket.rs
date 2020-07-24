// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[inline(always)]
pub(crate) fn bind_socket<SA: Sized>(socket_file_descriptor: &impl FileDescriptor, socket_address: &SA) -> Result<(), SocketBindError>
{
	bind_socket_with_length(socket_file_descriptor, socket_address, size_of::<SA>())
}
