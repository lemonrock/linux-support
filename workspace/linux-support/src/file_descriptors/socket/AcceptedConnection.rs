// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// An accepted connection.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AcceptedConnection<SD: SocketData>
{
	/// A streaming socket instance between two peers.
	pub streaming_socket_file_descriptor: StreamingSocketFileDescriptor<SD>,

	/// Peer (remote) address.
	pub peer: SocketDataWithLength<SD>,
}
