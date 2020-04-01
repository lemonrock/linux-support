// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


bitflags!
{
	/// Flags which can be used to control sending data on sockets.
	pub struct SendFlags: i32
	{
		/// This flag requests receipt of out-of-band data that would not be received in the normal data stream.
		///
		/// Some protocols place expedited data at the head of the normal data queue, and thus this flag cannot be used with such protocols.
		///
		/// For TCP, the use of out-of-band data is deprecated.
		const OutOfBand = MSG_OOB;

		/// Don't use a gateway to send out the packet, send to hosts only on directly connected networks.
		///
		/// This is usually used only by diagnostic or routing programs.
		///
		/// This is defined only for protocol families that route; packet sockets do not.
		///
		/// Only seems to be used for CANBUS and some IPv4 legacy features.
		const DoNotRoute = MSG_DONTROUTE;

		/// Only probe path, for example, for a MTU.
		///
		/// Data is not actually sent.
		const Probe = MSG_PROXY;

		/// Enables nonblocking operation even if the file descriptor has not had non-blocking explicitly enabled; if the operation would block, the call fails with the error `EAGAIN`.
		///
		/// Not needed as all file descriptors of interest are opened as non-blocking.
		const NonBlocking = MSG_DONTWAIT;

		/// Terminates a record (when this notion is supported, as for sockets of type `SOCK_SEQPACKET` (eg SCTP)).
		const EndOfRecord = MSG_EOR;

		/// Unused in Linux.
		const SYN = MSG_SYN;

		/// Used ?internally? to initiate SCTP graceful shutdown.
		///
		/// Also known as `MSG_EOF`.
		const FIN = MSG_FIN;

		/// Unused in Linux.
		const RST = MSG_RST;

		/// ARP functionality ("Confirm path validity").
		///
		/// Tell the link layer that forward progress happened: you got a successful reply from the other side.
		/// If the link layer doesn't get this it will regularly reprobe the neighbor (eg via an unicast ARP).
		/// Valid only on `SOCK_DGRAM` and `SOCK_RAW` and currently implemented only for IPv4 and IPv6.
		const ConfirmPathValidity = MSG_CONFIRM;

		/// Don't generate a `SIGPIPE` signal if the peer on a stream-oriented socket has closed the connection.
		///
		/// The `EPIPE` error is still returned.
		///
		/// This provides similar behavior to using `sigaction()` to ignore `SIGPIPE`, but, whereas `MSG_NOSIGNAL` is a per-call feature, ignoring `SIGPIPE` sets a process attribute that affects all threads in the process.
		const NoSigPipeSignal = MSG_NOSIGNAL;

		/// The caller has more data to send.
		///
		/// This flag is used with TCP sockets to obtain the same effect as the `TCP_CORK` socket option, with the difference that this flag can be set on a per-call basis.
		///
		/// This flag is also supported for UDP sockets and informs the kernel to package all of the data sent in calls with this flag set into a single datagram which is transmitted only when a call is performed that does not specify this flag.
		/// (See also the `UDP_CORK` socket option).
		///
		/// Not supported for Unix domain sockets.
		const MoreToSend = MSG_MORE;

		/// Used only by `sendmmsg()`
		///
		/// More datagrams coming.
		///
		/// ?Not supported for Unix domain sockets?
		const BatchDatagrams = MSG_BATCH;

		/// This flag enables copy avoidance for sending data over TCP sockets.
		///
		/// The feature is currently implemented for TCP sockets.
		///
		/// For more information see <https://www.kernel.org/doc/html/latest/networking/msg_zerocopy.html>.
		const ZeroCopy = MSG_ZEROCOPY;

		/// Send data in TCP SYN packet.
		///
		/// The feature is currently implemented for TCP sockets.
		const TcpFastOpen = MSG_FASTOPEN;
	}
}
