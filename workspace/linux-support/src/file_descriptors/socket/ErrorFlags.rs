// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


bitflags!
{
	/// Flags associated with errors when sending or receiving on sockets.
	pub struct ErrorFlags: i32
	{
		/// Is returned to indicate that expedited or out-of-band data were received.
		const OutOfBand = MSG_OOB;

		/// Indicates that some control data were discarded due to lack of space in the buffer for ancillary data.
		const ControlDataTruncated = MSG_CTRUNC;

		/// Returns the real length of the packet of datagram, even when it was longer than the passed buffer.
		const Truncated = MSG_TRUNC;

		/// Indicates end-of-record; the data returned completed a record (generally used with sockets of type `SOCK_SEQPACKET` (eg SCTP)).
		///
		/// Used in the `msg_flags` field in the `msghdr` on return of `recvmsg()`.
		const EndOfRecord = MSG_EOR;

		/// Indicates that no data was received but an extended error from the socket error queue..
		const QueuedErrors = MSG_ERRQUEUE;
	}
}
