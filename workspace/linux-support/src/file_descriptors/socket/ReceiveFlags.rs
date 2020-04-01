// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


bitflags!
{
	/// Flags which can be used to control receiving data on sockets.
	pub struct ReceiveFlags: i32
	{
		/// This flag requests receipt of out-of-band data that would not be received in the normal data stream.
		///
		/// Some protocols place expedited data at the head of the normal data queue, and thus this flag cannot be used with such protocols.
		///
		/// For TCP, the use of out-of-band data is deprecated.
		const OutOfBand = MSG_OOB;

		/// This flag causes the receive operation to return data from the beginning of the receive queue without removing that data from the queue.
		///
		/// A subsequent receive call will return the same data.
		const Peek = MSG_PEEK;

		/// Returns the real length of the packet of datagram, even when it was longer than the passed buffer.
		///
		/// Supported for:-
		///
		/// * Raw (`AF_PACKET`) sockets;
		/// * UDP sockets since Linux 2.6.8;
		/// * Netlink sockets since Linux 2.6.22;
		/// * Unix domain sockets with datagrams since Linux 3.4;
		/// * For TCP, this flag causes the received bytes of data to be discarded, rather than passed back in a caller-supplied buffer. Also works in conjunction with `MSG_OOB` and `MSG_PEEK`.
		///
		/// Unsupported for:-
		///
		/// * Unix domain sockets with streams.
		const Truncated = MSG_TRUNC;

		/// Enables nonblocking operation even if the file descriptor has not had non-blocking explicitly enabled; if the operation would block, the call fails with the error `EAGAIN`.
		///
		/// Not needed as all file descriptors of interest are opened as non-blocking.
		const NonBlocking = MSG_DONTWAIT;

		/// This flag requests that the operation block until the full request is satisfied.
		///
		/// However, the call may still return less data than requested if a signal is caught, an error or disconnect occurs, or the next data to be received is of a different type than that returned.
		///
		/// Not useful as all file descriptors of interest are opened as non-blocking.
		const WaitUntilFullRequestSatisfied = MSG_WAITALL;

		/// This flag specifies that queued extended errors should be received from the socket error queue instead of data.
		const QueuedErrors = MSG_ERRQUEUE;

		/// Used only by `recvmmsg()`
		///
		/// Block until one or more datagrams are available.
		const WaitForAtLeastOneDatagram = MSG_WAITFORONE;

		/// Set the close-on-exec flag for the file descriptor received via a UNIX domain file descriptor using the `SCM_RIGHTS` operation (described in `man 7 unix`)
		///
		/// Since Linux 2.6.23.
		const ControlPosixMessageCloseOnExec = MSG_CMSG_CLOEXEC;
	}
}
