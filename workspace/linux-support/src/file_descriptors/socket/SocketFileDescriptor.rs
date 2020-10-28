// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Represents a socket instance.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SocketFileDescriptor<SD: SocketData>(RawFd, PhantomData<SD>);

impl<SD: SocketData> Drop for SocketFileDescriptor<SD>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		SD::specialized_drop(self)
	}
}

impl<SD: SocketData> AsRawFd for SocketFileDescriptor<SD>
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl<SD: SocketData> IntoRawFd for SocketFileDescriptor<SD>
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.0
	}
}

impl<SD: SocketData> FromRawFd for SocketFileDescriptor<SD>
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(fd, PhantomData)
	}
}

impl<SD: SocketData> FileDescriptor for SocketFileDescriptor<SD>
{
}

impl<SD: SocketData> SocketFileDescriptor<SD>
{
	/// This is mostly useful for `StreamingSocketFileDescriptors` after `connect()` or created by `accept()`, to identify an appropriate CPU to most efficiently handle the network traffic.
	#[inline(always)]
	pub fn hyper_thread(&self) -> Option<HyperThread>
	{
		let result: i32 = self.get_socket_option(SOL_SOCKET, SO_INCOMING_CPU);
		if likely!(result >= 0 && result <= (u16::MAX as i32))
		{
			Some(HyperThread::try_from(result as u32 as u16).unwrap())
		}
		else if likely!(result == -1)
		{
			None
		}
		else
		{
			unreachable_code(format_args!("Invalid result {} for get SO_INCOMING_CPU socket option", result))
		}
	}
	
	/// ***SLOW***.
	///
	/// Use a system call.
	#[allow(deprecated)]
	#[inline(always)]
	pub fn information(&self) -> tcp_info
	{
		self.get_socket_option(SOL_TCP, TCP_INFO)
	}

	/// Obtain our local address and its length; the length is essential when interpreting Unix Domain Sockets.
	#[inline(always)]
	pub fn local_address(&self) -> Result<(SD, usize), ()>
	{
		let mut socket_address = unsafe { zeroed() };
		let mut socket_address_length = size_of::<SD>() as u32;
		let result = unsafe { getsockname(self.0, &mut socket_address as *mut _ as *mut _, &mut socket_address_length) };

		if likely!(result == 0)
		{
			Ok((socket_address, socket_address_length as usize))
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				ENOBUFS => Err(()),

				EBADF => panic!("The argument `sockfd` is not a valid file descriptor"),
				EFAULT => panic!("The `addr` argument points to memory not in a valid part of the process address space"),
				EINVAL => panic!("`addrlen` is invalid"),
				ENOTSOCK => panic!("The file descriptor `sockfd` does not refer to a socket"),

				_ => unreachable_code(format_args!("")),
			}
		}
		else
		{
			unreachable_code(format_args!(""));
		}
	}
	
	#[inline(always)]
	fn set_incoming_cpu(&self, hyper_thread: HyperThread)
	{
		let hyper_thread: i32 = hyper_thread.into();
		self.set_socket_option(SOL_SOCKET, SO_INCOMING_CPU, &hyper_thread);
	}
	
	#[inline(always)]
	fn listen(self, back_log: BackLog, hyper_thread: HyperThread) -> Result<StreamingServerListenerSocketFileDescriptor<SD>, SocketListenError>
	{
		let result = unsafe { listen(self.0, back_log.0 as i32) };
		if likely!(result == 0)
		{
			self.set_incoming_cpu(hyper_thread);
			Ok(StreamingServerListenerSocketFileDescriptor(self))
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EADDRINUSE => Err(SocketListenError::AddressInUse),
				EBADF => panic!("`sockfd` is not a valid descriptor"),
				ENOTSOCK => panic!("`sockfd` is not a socket file descriptor"),
				EOPNOTSUPP => panic!("The socket is not of a type that supports the `listen()` operation"),

				_ => unreachable_code(format_args!("")),
			}
		}
		else
		{
			unreachable_code(format_args!(""))
		}
	}
	
	#[inline(always)]
	fn connect(&self, socket_data: &SD, length: usize, tcp_writes_before_reading: bool) -> Result<(), SocketConnectError>
	{
		if tcp_writes_before_reading
		{
			self.set_socket_option_true(SOL_TCP, TCP_DEFER_ACCEPT);
			self.set_socket_option_true(SOL_TCP, TCP_FASTOPEN_CONNECT);
		}
		
		use self::SocketConnectError::*;

		let result = unsafe { connect(self.0, &socket_data as *const _ as *const sockaddr_storage, length as socklen_t) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			Err
			(
				match errno().0
				{
					EACCES | EPERM => PermissionDenied,
					EADDRINUSE => AddressInUse,
					EAGAIN => NoMoreFreeLocalPorts,
					ECONNREFUSED => ConnectionRefused,
					EINPROGRESS => InProgress,
					EINTR => Interrupted,
					ETIMEDOUT => TimedOut,
					ENETUNREACH => NetworkUnreachable,
					EISCONN => panic!("The socket is already connected."),
					EALREADY => panic!("The socket is nonblocking and a previous connection attempt has not yet been completed."),
					EBADF => panic!("`sockfd` is not a valid descriptor"),
					EINVAL => panic!("already bound, or the `addrlen` is wrong, or the socket was not in the `AF_UNIX` family"),
					ENOTSOCK => panic!("`sockfd` is not a socket file descriptor"),
					EFAULT => panic!("`addr` points outside the user's accessible address space"),
					EAFNOSUPPORT => panic!("Invalid `sa_family_t` value"),

					_ => unreachable_code(format_args!("")),
				}
			)
		}
		else
		{
			unreachable_code(format_args!(""))
		}
	}

	#[inline(always)]
	fn get_socket_option<T>(&self, level: c_int, optname: c_int) -> T
	{
		#[allow(deprecated)]
		let mut value: T = unsafe { uninitialized() };
		let mut value_length = size_of::<T>() as u32;
		let result = unsafe { getsockopt(self.0, level, optname, &mut value as *mut _ as *mut _, &mut value_length) };

		if likely!(result == 0)
		{
			return value
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EBADF => panic!("The argument `sockfd` is not a valid descriptor"),
				EFAULT => panic!("The address pointed to by `optval` is not in a valid part of the process address space"),
				EINVAL => panic!("`optlen` is invalid, or there is an invalid value in `optval`"),
				ENOPROTOOPT => panic!("The option is unknown at the level indicated"),
				ENOTSOCK => panic!("The argument `sockfd` is a file, not a socket"),
				EOPNOTSUPP => panic!("Unsupported sockopt"),

				_ => unreachable_code(format_args!("")),
			}
		}
		else
		{
			unreachable_code(format_args!(""));
		}
	}

	#[inline(always)]
	fn set_socket_option<T>(&self, level: c_int, optname: c_int, value: &T)
	{
		let result = unsafe { setsockopt(self.0, level, optname, value as *const _ as *const _, size_of::<T>() as socklen_t) };

		if likely!(result == 0)
		{
			return
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EBADF => panic!("The argument `sockfd` is not a valid descriptor"),
				EFAULT => panic!("The address pointed to by `optval` is not in a valid part of the process address space"),
				EINVAL => panic!("`optlen` is invalid, or there is an invalid value in `optval`"),
				ENOPROTOOPT => panic!("The option is unknown at the level indicated"),
				ENOTSOCK => panic!("The argument `sockfd` is a file, not a socket"),

				_ => unreachable_code(format_args!("")),
			}
		}
		else
		{
			unreachable_code(format_args!(""));
		}
	}

	#[inline(always)]
	fn set_socket_option_true(&self, level: c_int, optname: c_int)
	{
		static is_true: c_int = 1;
		self.set_socket_option(level, optname, &is_true);
	}

	#[allow(dead_code)]
	#[inline(always)]
	fn set_socket_option_false(&self, level: c_int, optname: c_int)
	{
		static is_false: c_int = 0;
		self.set_socket_option(level, optname, &is_false);
	}

	#[inline(always)]
	fn set_receive_low_water_mark_in_bytes(&self, receive_low_water_mark_in_bytes: ReceiveLowWaterMarkInBytes)
	{
		let value = receive_low_water_mark_in_bytes.0.get() as i32;
		self.set_socket_option(SOL_SOCKET, SO_RCVLOWAT, &value);
	}

	#[inline(always)]
	fn set_send_buffer_size_unix_domain_socket(&self, send_buffer_size_socket_option: SendBufferSizeSocketOption)
	{
		let send_buffer_adjusted = send_buffer_size_socket_option.adjusted_unix_domain_socket_buffer_size();
		let socket_option_name = send_buffer_size_socket_option.socket_option_name();
		self.set_socket_option(SOL_SOCKET, socket_option_name, &send_buffer_adjusted);
	}

	#[inline(always)]
	fn set_send_buffer_size(&self, send_buffer_size_socket_option: &SendBufferSizeSocketOption)
	{
		let send_buffer_halved = send_buffer_size_socket_option.adjusted_tcp_buffer_size();
		let socket_option_name = send_buffer_size_socket_option.socket_option_name();
		self.set_socket_option(SOL_SOCKET, socket_option_name, &send_buffer_halved);
	}

	#[inline(always)]
	fn set_receive_buffer_size(&self, receive_buffer_size_socket_option: &ReceiveBufferSizeSocketOption)
	{
		let receive_buffer_halved = receive_buffer_size_socket_option.adjusted_tcp_buffer_size();
		let socket_option_name = receive_buffer_size_socket_option.socket_option_name();
		self.set_socket_option(SOL_SOCKET, socket_option_name, &receive_buffer_halved);
	}

	#[inline(always)]
	fn set_queuing_discipline_send_priority(&self, queuing_discipline_send_priority: QueuingDisciplineSendPriority)
	{
		let value = queuing_discipline_send_priority as i32;
		self.set_socket_option(SOL_SOCKET, SO_PRIORITY, &value);
	}

	#[inline(always)]
	fn set_busy_poll_microseconds(&self, busy_poll_microseconds: Option<BusyPollMicroseconds>)
	{
		if let Some(busy_poll_microseconds) = busy_poll_microseconds
		{
			let value = busy_poll_microseconds.to_socket_option_value();
			self.set_socket_option(SOL_SOCKET, SO_BUSY_POLL, &value);
		}
	}

	#[inline(always)]
	fn set_broadcast(&self)
	{
		self.set_socket_option_true(SOL_SOCKET, SO_BROADCAST)
	}

	#[inline(always)]
	fn set_out_of_band_inline(&self)
	{
		self.set_socket_option_true(SOL_SOCKET, SO_OOBINLINE)
	}

	/// This is a send-side option.
	#[inline(always)]
	fn disable_nagle_algorithm(&self)
	{
		self.set_socket_option_true(SOL_TCP, TCP_NODELAY)
	}

	#[inline(always)]
	fn set_tcp_max_SYN_transmits(&self, maximum_syn_retransmits: MaximumSynRetransmits)
	{
		let maximum_syn_retransmits = maximum_syn_retransmits.0.get() as i32;
		self.set_socket_option(SOL_TCP, TCP_SYNCNT, &maximum_syn_retransmits);
	}

	#[inline(always)]
	fn set_tcp_linger(&self, socket_linger_seconds: SocketLingerSeconds)
	{
		#[repr(C)]
		struct linger
		{
			l_onoff: c_int,
			l_linger: c_int,
		}
		
		use self::SocketLingerSeconds::*;
		let linger = match socket_linger_seconds
		{
			Off => linger
			{
				l_onoff: 0,
				l_linger: 0,
			},
			
			On(seconds) => linger
			{
				l_onoff: 1,
				l_linger: seconds as i32
			}
		};
		
		self.set_socket_option(SOL_SOCKET, SO_LINGER, &linger);
	}

	#[inline(always)]
	fn set_internet_protocol_socket_options(&self, internet_protocol_socket_settings: &InternetProtocolSocketSettings)
	{
		self.set_send_buffer_size(&internet_protocol_socket_settings.send_buffer_size);
		self.set_receive_buffer_size(&internet_protocol_socket_settings.receive_buffer_size);
		self.set_queuing_discipline_send_priority(internet_protocol_socket_settings.queuing_discipline_send_priority);
		self.set_busy_poll_microseconds(internet_protocol_socket_settings.busy_poll_microseconds);
	}

	#[inline(always)]
	fn set_tcp_socket_options(&self, transmission_control_protocol_socket_settings: &TransmissionControlProtocolSocketSettings)
	{
		self.set_socket_option_true(SOL_SOCKET, SO_KEEPALIVE);

		self.set_out_of_band_inline();

		self.disable_nagle_algorithm();

		let idles_before_keep_alive_seconds = transmission_control_protocol_socket_settings.idles_before_keep_alive_seconds.0 as i32;
		self.set_socket_option(SOL_TCP, TCP_KEEPIDLE, &idles_before_keep_alive_seconds);

		let keep_alive_interval_seconds = transmission_control_protocol_socket_settings.keep_alive_interval_seconds.0 as i32;
		self.set_socket_option(SOL_TCP, TCP_KEEPINTVL, &keep_alive_interval_seconds);

		let maximum_keep_alive_probes = transmission_control_protocol_socket_settings.maximum_keep_alive_probes.0 as i32;
		self.set_socket_option(SOL_TCP, TCP_KEEPCNT, &maximum_keep_alive_probes);

		self.set_tcp_linger(transmission_control_protocol_socket_settings.socket_linger_seconds);

		let finish_timeout_seconds = transmission_control_protocol_socket_settings.finish_timeout_seconds.0 as i32;
		self.set_socket_option(SOL_TCP, TCP_LINGER2, &finish_timeout_seconds);

		self.set_tcp_max_SYN_transmits(transmission_control_protocol_socket_settings.maximum_syn_retransmits);
		
		let not_sent_low_water_in_bytes: i32 = transmission_control_protocol_socket_settings.not_sent_low_water_in_bytes.into();
		self.set_socket_option(SOL_TCP, TCP_NOTSENT_LOWAT, &not_sent_low_water_in_bytes);
		
		self.set_receive_low_water_mark_in_bytes(transmission_control_protocol_socket_settings.receive_low_water_mark_in_bytes);
	}

	#[inline(always)]
	fn set_udp_socket_options(&self)
	{
		self.set_broadcast();
	}

	#[inline(always)]
	fn set_internet_protocol_server_listener_socket_options(&self)
	{
		self.set_socket_option_true(SOL_SOCKET, SO_REUSEADDR);
		self.set_socket_option_true(SOL_SOCKET, SO_REUSEPORT);
	}

	#[inline(always)]
	fn set_tcp_server_listener_socket_options(&self)
	{
		self.set_socket_option_true(SOL_TCP, TCP_DEFER_ACCEPT);
		self.set_socket_option_true(SOL_TCP, TCP_FASTOPEN);
	}

	#[inline(always)]
	fn new_transmission_control_protocol_over_internet_protocol_version_4(internet_protocol_socket_settings: &InternetProtocolSocketSettings, transmission_control_protocol_socket_settings: &TransmissionControlProtocolSocketSettings, blocking: &Blocking) -> Result<Self, CreationError>
	{
		Self::new(AF_INET, SOCK_STREAM, IPPROTO_TCP, blocking).map(|this|
		{
			this.set_internet_protocol_socket_options(internet_protocol_socket_settings);
			this.set_tcp_socket_options(transmission_control_protocol_socket_settings);
			this
		})
	}

	#[inline(always)]
	fn new_transmission_control_protocol_over_internet_protocol_version_6(internet_protocol_socket_settings: &InternetProtocolSocketSettings, transmission_control_protocol_socket_settings: &TransmissionControlProtocolSocketSettings, blocking: &Blocking) -> Result<Self, CreationError>
	{
		Self::new(AF_INET6, SOCK_STREAM, IPPROTO_TCP, blocking).map(|this|
		{
			this.set_internet_protocol_socket_options(internet_protocol_socket_settings);
			this.set_tcp_socket_options(transmission_control_protocol_socket_settings);
			this
		})
	}

	#[inline(always)]
	fn new_user_datagram_protocol_over_internet_protocol_version_4(internet_protocol_socket_settings: &InternetProtocolSocketSettings, blocking: &Blocking) -> Result<Self, CreationError>
	{
		Self::new(AF_INET, SOCK_DGRAM, IPPROTO_UDP, blocking).map(|this|
		{
			this.set_internet_protocol_socket_options(internet_protocol_socket_settings);
			this.set_udp_socket_options();
			this
		})
	}

	#[inline(always)]
	fn new_user_datagram_protocol_over_internet_protocol_version_6(internet_protocol_socket_settings: &InternetProtocolSocketSettings, blocking: &Blocking) -> Result<Self, CreationError>
	{
		Self::new(AF_INET6, SOCK_DGRAM, IPPROTO_UDP, blocking).map(|this|
		{
			this.set_internet_protocol_socket_options(internet_protocol_socket_settings);
			this.set_udp_socket_options();
			this
		})
	}

	#[inline(always)]
	fn new_streaming_unix_domain_socket(send_buffer_size: SendBufferSizeSocketOption, receive_low_water_mark_in_bytes: ReceiveLowWaterMarkInBytes, blocking: &Blocking) -> Result<Self, CreationError>
	{
		Self::new(AF_UNIX, SOCK_STREAM, 0, blocking).map(|this|
		{
			this.set_send_buffer_size_unix_domain_socket(send_buffer_size);
			this.set_receive_low_water_mark_in_bytes(receive_low_water_mark_in_bytes);
			this
		})
	}

	#[inline(always)]
	fn new_datagram_unix_domain_socket(send_buffer_size: SendBufferSizeSocketOption, blocking: &Blocking) -> Result<Self, CreationError>
	{
		Self::new(AF_UNIX, SOCK_DGRAM, 0, blocking).map(|this|
		{
			this.set_send_buffer_size_unix_domain_socket(send_buffer_size);
			this
		})
	}

	#[inline(always)]
	fn type_and_flags(type_: c_int, blocking: &Blocking) -> c_int
	{
		let flags = type_ | SOCK_CLOEXEC;
		if blocking.is_non_blocking()
		{
			flags | SOCK_NONBLOCK
		}
		else
		{
			flags
		}
	}

	#[inline(always)]
	fn socketpair(type_: c_int, lefthand_send_buffer_size_in_bytes: SendBufferSizeSocketOption, righthand_send_buffer_size_in_bytes: SendBufferSizeSocketOption, blocking: &Blocking) -> Result<(Self, Self), CreationError>
	{
		const domain: c_int = AF_UNIX;
		const ethernet_protocol: c_int = 0;

		#[allow(deprecated)]
		let mut sv = unsafe { uninitialized() };
		let result = unsafe { socketpair(domain, Self::type_and_flags(type_, blocking), ethernet_protocol, &mut sv) };

		if likely!(result == 0)
		{
			let lefthand = SocketFileDescriptor(unsafe { *sv.get_unchecked(0) }, PhantomData);
			lefthand.set_send_buffer_size_unix_domain_socket(lefthand_send_buffer_size_in_bytes);

			let righthand = SocketFileDescriptor(unsafe { *sv.get_unchecked(1) }, PhantomData);
			righthand.set_send_buffer_size_unix_domain_socket(righthand_send_buffer_size_in_bytes);

			Ok((lefthand, righthand))
		}
		else if likely!(result == -1)
		{
			use self::CreationError::*;

			Err
			(
				match errno().0
				{
					EMFILE => PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded,
					ENFILE => SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded,
					EAFNOSUPPORT => panic!("The specified address family is not supported on this machine"),
					EFAULT => panic!("The address `sv` does not specify a valid part of the process address space"),
					EOPNOTSUPP => panic!("The specified `protocol` does not support creation of socket pairs"),
					EPROTONOSUPPORT => panic!("TThe specified `protocol` is not supported on this machine"),

					_ => unreachable_code(format_args!("")),
				}
			)
		}
		else
		{
			unreachable_code(format_args!(""));
		}
	}
	
	#[inline(always)]
	fn new(domain: c_int, type_: c_int, ethernet_protocol: c_int, blocking: &Blocking) -> Result<Self, CreationError>
	{
		let file_descriptor = new_socket(domain, type_, ethernet_protocol, blocking.is_non_blocking())?;
		let socket_file_descriptor = SocketFileDescriptor(file_descriptor, PhantomData);
		blocking.set_time_outs(&socket_file_descriptor);
		Ok(socket_file_descriptor)
	}
}

impl SocketFileDescriptor<sockaddr_in>
{
	/// Creates a new instance of a Transmission Control Protocol (TCP) socket over Internet Protocol (IP) version 4 server listener.
	#[inline(always)]
	pub fn new_transmission_control_protocol_over_internet_protocol_version_4_server_listener(socket_address: &sockaddr_in, internet_protocol_socket_settings: &InternetProtocolSocketSettings, transmission_control_protocol_socket_settings: &TransmissionControlProtocolSocketSettings, back_log: BackLog, blocking: &Blocking, hyper_thread: HyperThread) -> Result<StreamingServerListenerSocketFileDescriptor<sockaddr_in>, NewSocketServerListenerError>
	{
		let this = SocketFileDescriptor::<sockaddr_in>::new_transmission_control_protocol_over_internet_protocol_version_4(internet_protocol_socket_settings, transmission_control_protocol_socket_settings, blocking)?;
		this.set_internet_protocol_server_listener_socket_options();
		this.set_tcp_server_listener_socket_options();
		this.bind_internet_protocol_version_4_socket(socket_address)?;
		Ok(this.listen(back_log, hyper_thread)?)
	}

	/// Creates a new instance of a Transmission Control Protocol (TCP) socket over Internet Protocol (IP) version 4 client.
	#[inline(always)]
	pub fn new_transmission_control_protocol_over_internet_protocol_version_4_client(socket_address: &sockaddr_in, internet_protocol_socket_settings: &InternetProtocolSocketSettings, transmission_control_protocol_socket_settings: &TransmissionControlProtocolSocketSettings, writes_before_reading: bool, blocking: &Blocking) -> Result<StreamingSocketFileDescriptor<sockaddr_in>, NewSocketClientError>
	{
		let this = SocketFileDescriptor::<sockaddr_in>::new_transmission_control_protocol_over_internet_protocol_version_4(internet_protocol_socket_settings, transmission_control_protocol_socket_settings, blocking)?;
		this.connect_internet_protocol_version_4_socket(socket_address, writes_before_reading)?;
		Ok(StreamingSocketFileDescriptor(this))
	}

	/// Creates a new instance of a User Datagram Protocol (UDP) socket over Internet Protocol (IP) version 4 server listener.
	#[inline(always)]
	pub fn new_user_datagram_protocol_over_internet_protocol_version_4_server_listener(socket_address: &sockaddr_in, internet_protocol_socket_settings: &InternetProtocolSocketSettings, blocking: &Blocking, hyper_thread: HyperThread) -> Result<DatagramServerListenerSocketFileDescriptor<sockaddr_in>, NewSocketServerListenerError>
	{
		let this = SocketFileDescriptor::<sockaddr_in>::new_user_datagram_protocol_over_internet_protocol_version_4(internet_protocol_socket_settings, blocking)?;
		this.set_incoming_cpu(hyper_thread);
		this.set_internet_protocol_server_listener_socket_options();
		this.bind_internet_protocol_version_4_socket(socket_address)?;
		Ok(DatagramServerListenerSocketFileDescriptor(this))
	}

	/// Creates a new instance of a User Datagram Protocol (UDP) socket over Internet Protocol (IP) version 4 client.
	#[inline(always)]
	pub fn new_user_datagram_protocol_over_internet_protocol_version_4_client(socket_address: &sockaddr_in, internet_protocol_socket_settings: &InternetProtocolSocketSettings, blocking: &Blocking) -> Result<DatagramClientSocketFileDescriptor<sockaddr_in>, NewSocketClientError>
	{
		let this = SocketFileDescriptor::<sockaddr_in>::new_user_datagram_protocol_over_internet_protocol_version_4(internet_protocol_socket_settings, blocking)?;
		this.connect_internet_protocol_version_4_socket(socket_address, false)?;
		Ok(DatagramClientSocketFileDescriptor(this))
	}

	/// Creates a new instance of a User Datagram Protocol (UDP) socket over Internet Protocol (IP) version 4 combined client and listener.
	#[inline(always)]
	pub fn new_user_datagram_protocol_over_internet_protocol_version_4_client_listener(socket_address: &sockaddr_in, internet_protocol_socket_settings: &InternetProtocolSocketSettings, blocking: &Blocking, hyper_thread: HyperThread) -> Result<DatagramClientListenerSocketFileDescriptor<sockaddr_in>, NewSocketClientListenerError>
	{
		let this = SocketFileDescriptor::<sockaddr_in>::new_user_datagram_protocol_over_internet_protocol_version_4(internet_protocol_socket_settings, blocking)?;
		this.set_incoming_cpu(hyper_thread);
		this.set_internet_protocol_server_listener_socket_options();
		this.bind_internet_protocol_version_4_socket(socket_address)?;
		this.connect_internet_protocol_version_4_socket(socket_address, false)?;
		Ok(DatagramClientListenerSocketFileDescriptor(this))
	}
	
	#[inline(always)]
	fn connect_internet_protocol_version_4_socket(&self, socket_address: &sockaddr_in, tcp_writes_before_reading: bool) -> Result<(), SocketConnectError>
	{
		self.connect(socket_address, size_of::<sockaddr_in>(), tcp_writes_before_reading)
	}

	#[inline(always)]
	fn bind_internet_protocol_version_4_socket(&self, socket_address: &sockaddr_in) -> Result<(), SocketBindError>
	{
		bind_socket(self, socket_address)
	}
}

impl SocketFileDescriptor<sockaddr_in6>
{
	/// Creates a new instance of a Transmission Control Protocol (TCP) socket over Internet Protocol (IP) version 6 server listener.
	#[inline(always)]
	pub fn new_transmission_control_protocol_over_internet_protocol_version_6_server_listener(socket_address: &sockaddr_in6, internet_protocol_socket_settings: &InternetProtocolSocketSettings, transmission_control_protocol_socket_settings: &TransmissionControlProtocolSocketSettings, back_log: BackLog, blocking: &Blocking, hyper_thread: HyperThread) -> Result<StreamingServerListenerSocketFileDescriptor<sockaddr_in6>, NewSocketServerListenerError>
	{
		let this = SocketFileDescriptor::<sockaddr_in6>::new_transmission_control_protocol_over_internet_protocol_version_6(internet_protocol_socket_settings, transmission_control_protocol_socket_settings, blocking)?;
		this.set_internet_protocol_server_listener_socket_options();
		this.set_tcp_server_listener_socket_options();
		this.bind_internet_protocol_version_6_socket(socket_address)?;
		Ok(this.listen(back_log, hyper_thread)?)
	}

	/// Creates a new instance of a Transmission Control Protocol (TCP) socket over Internet Protocol (IP) version 6 client.
	#[inline(always)]
	pub fn new_transmission_control_protocol_over_internet_protocol_version_6_client(socket_address: &sockaddr_in6, internet_protocol_socket_settings: &InternetProtocolSocketSettings, transmission_control_protocol_socket_settings: &TransmissionControlProtocolSocketSettings, writes_before_reading: bool, blocking: &Blocking) -> Result<StreamingSocketFileDescriptor<sockaddr_in6>, NewSocketClientError>
	{
		let this = SocketFileDescriptor::<sockaddr_in6>::new_transmission_control_protocol_over_internet_protocol_version_6(internet_protocol_socket_settings, transmission_control_protocol_socket_settings, blocking)?;
		this.connect_internet_protocol_version_6_socket(socket_address, writes_before_reading)?;
		Ok(StreamingSocketFileDescriptor(this))
	}

	/// Creates a new instance of a User Datagram Protocol (UDP) socket over Internet Protocol (IP) version 6 server listener.
	#[inline(always)]
	pub fn new_user_datagram_protocol_over_internet_protocol_version_6_server_listener(socket_address: &sockaddr_in6, internet_protocol_socket_settings: &InternetProtocolSocketSettings, blocking: &Blocking, hyper_thread: HyperThread) -> Result<DatagramServerListenerSocketFileDescriptor<sockaddr_in6>, NewSocketServerListenerError>
	{
		let this = SocketFileDescriptor::<sockaddr_in6>::new_user_datagram_protocol_over_internet_protocol_version_6(internet_protocol_socket_settings, blocking)?;
		this.set_incoming_cpu(hyper_thread);
		this.set_internet_protocol_server_listener_socket_options();
		this.bind_internet_protocol_version_6_socket(socket_address)?;
		Ok(DatagramServerListenerSocketFileDescriptor(this))
	}

	/// Creates a new instance of a User Datagram Protocol (UDP) socket over Internet Protocol (IP) version 6 client.
	#[inline(always)]
	pub fn new_user_datagram_protocol_over_internet_protocol_version_6_client(socket_address: &sockaddr_in6, internet_protocol_socket_settings: &InternetProtocolSocketSettings, blocking: &Blocking) -> Result<DatagramClientSocketFileDescriptor<sockaddr_in6>, NewSocketClientError>
	{
		let this = SocketFileDescriptor::<sockaddr_in6>::new_user_datagram_protocol_over_internet_protocol_version_6(internet_protocol_socket_settings, blocking)?;
		this.connect_internet_protocol_version_6_socket(socket_address, false)?;
		Ok(DatagramClientSocketFileDescriptor(this))
	}
	
	/// Creates a new instance of a User Datagram Protocol (UDP) socket over Internet Protocol (IP) version 4 combined client and listener.
	#[inline(always)]
	pub fn new_user_datagram_protocol_over_internet_protocol_version_6_client_listener(socket_address: &sockaddr_in6, internet_protocol_socket_settings: &InternetProtocolSocketSettings, blocking: &Blocking, hyper_thread: HyperThread) -> Result<DatagramClientListenerSocketFileDescriptor<sockaddr_in6>, NewSocketClientListenerError>
	{
		let this = SocketFileDescriptor::<sockaddr_in6>::new_user_datagram_protocol_over_internet_protocol_version_6(internet_protocol_socket_settings, blocking)?;
		this.set_incoming_cpu(hyper_thread);
		this.set_internet_protocol_server_listener_socket_options();
		this.bind_internet_protocol_version_6_socket(socket_address)?;
		this.connect_internet_protocol_version_6_socket(socket_address, false)?;
		Ok(DatagramClientListenerSocketFileDescriptor(this))
	}
	
	#[inline(always)]
	fn connect_internet_protocol_version_6_socket(&self, socket_address: &sockaddr_in6, tcp_writes_before_reading: bool) -> Result<(), SocketConnectError>
	{
		self.connect(socket_address, size_of::<sockaddr_in6>(), tcp_writes_before_reading)
	}

	#[inline(always)]
	fn bind_internet_protocol_version_6_socket(&self, socket_address: &sockaddr_in6) -> Result<(), SocketBindError>
	{
		bind_socket(self, socket_address)
	}
}

impl SocketFileDescriptor<sockaddr_un>
{
	/// Tries to obtain remote peer credentials.
	///
	/// * Not available for Datagram sockets (unless created using `socketpair()`).
	/// * Not available for Sequenced Packet sockets.
	///
	/// The returned credentials are those that were in effect at the time of the call to `connect()` or `socketpair()`.
	#[inline(always)]
	pub fn remote_peer_credentials(&self) -> Credentials
	{
		unsafe { transmute::<ucred, Credentials>(self.get_socket_option(SOL_SOCKET, SO_PEERCRED)) }
	}

	/// Receive file descriptors.
	pub fn receive_file_descriptors(&self, maximum_file_descriptors_to_receive: usize) -> Result<Vec<RawFd>, ReceiveFileDescriptorsError>
	{
		let space_for_file_descriptors = size_of::<RawFd>() * maximum_file_descriptors_to_receive;

		let mut ancillary_data_buffer: Vec<u8> = Vec::with_capacity(cmsghdr::CMSG_SPACE(space_for_file_descriptors));

		const NothingLength: usize = 1;

		let mut nothing = b'A';
		let mut nothing_ptr = iovec
		{
			iov_base: &mut nothing as *mut _ as *mut _,
			iov_len: NothingLength,
		};

		let mut message = msghdr::new(null_mut(), 0, &mut nothing_ptr, NothingLength as i32, ancillary_data_buffer.as_mut_ptr() as *mut _, ancillary_data_buffer.len() as u32, 0);

		const InvalidFileDescriptorSentinel: RawFd = -1;

		// Insert a magic value of `-1` to detect where sent file descriptors stop as no length is specified in the data set by `recvmsg()`.
		let file_descriptor_end_pointer =
		{
			let mut borrow_checker = message.first_header_mut();
			let first_header = borrow_checker.as_mut().unwrap();
			first_header.initialize_known_fields(SOL_SOCKET, SCM_RIGHTS, size_of::<RawFd>() * maximum_file_descriptors_to_receive);
			let mut file_descriptor_current_pointer = first_header.CMSG_DATA_mut() as *mut RawFd;
			let file_descriptor_end_pointer = unsafe { file_descriptor_current_pointer.add(maximum_file_descriptors_to_receive) };
			while file_descriptor_current_pointer != file_descriptor_end_pointer
			{
				unsafe
				{
					*file_descriptor_current_pointer = InvalidFileDescriptorSentinel;
					file_descriptor_current_pointer = file_descriptor_current_pointer.add(1)
				}
			}
			file_descriptor_end_pointer
		};

		let result = unsafe { recvmsg(self.0, &mut message, ReceiveFlags::ControlPosixMessageCloseOnExec.bits) };

		use self::ReceiveFileDescriptorsError::*;

		if unlikely!(result == -1)
		{
			use self::StructReadError::*;

			let read_error = match errno().0
			{
				EAGAIN => WouldBlock,
				ECANCELED => Cancelled,
				EINTR => Interrupted,
				EIO => Cancelled,
				EBADF => panic!("`fd` is not a valid file descriptor or is not open for reading"),
				EFAULT => panic!("`buf` is outside your accessible address space"),
				EINVAL => panic!("`fd` is attached to an object which is unsuitable for reading OR was created via a call to `timerfd_create()` and the wrong size buffer was given to `read()`"),
				EISDIR => panic!("`fd` refers to a directory"),

				_ => panic!("Unexpected error `{}`", errno()),
			};

			return Err(Read(read_error))
		}
		else if unlikely!(result < -1)
		{
			unreachable_code(format_args!(""));
		}

		match message.first_header()
		{
			None => Ok(vec![]),

			Some(first_header) =>
			{
				if unlikely!(first_header.next(&message).is_some())
				{
					return Err(MoreThanOneHeader)
				}

				match first_header.cmsg_level
				{
					SOL_SOCKET => match first_header.cmsg_type
					{
						SCM_RIGHTS =>
						{
							let mut file_descriptors = Vec::with_capacity(maximum_file_descriptors_to_receive);
							let mut file_descriptor_current_pointer = first_header.CMSG_DATA() as *mut RawFd;
							while file_descriptor_current_pointer != file_descriptor_end_pointer
							{
								unsafe
								{
									let file_descriptor = *file_descriptor_current_pointer;
									if file_descriptor == InvalidFileDescriptorSentinel
									{
										break
									}
									file_descriptors.push(file_descriptor);
									file_descriptor_current_pointer = file_descriptor_current_pointer.add(1)
								}
							}

							file_descriptors.shrink_to_fit();
							Ok(file_descriptors)
						}

						_ => Err(WasNotScmRights),
					}

					_ => Err(WasNotSocketLevelPosixMessage)
				}
			}
		}
	}

	/// Tries to send file descriptors to a remote peer over an Unix Domain Socket.
	///
	/// `file_descriptors`: File Descriptors to send.
	#[inline(always)]
	pub fn send_file_descriptors(&self, file_descriptors: &[RawFd]) -> io::Result<()>
	{
		self.send_ancillary_data(SOL_SOCKET, SCM_RIGHTS, file_descriptors)
	}

	/// Tries to send credentials to a remote peer over an Unix Domain Socket.
	///
	/// `process_identifier`: Process identifier (also known as `pid`). Unless the process has capability `CAP_SYS_ADMIN`, this must be its own `process_identifier`.
	/// `user_identifier`: User identifier (also known as `uid`). Unless the process has capability `CAP_SETUID`, this must be its own `user_identifier`, effective `user_identifier` or saved-set `user_identifier`.
	/// `group_identifier`: Group identifier (also known as `gid`). Unless the process has capability `CAP_SETGID`, this must be its own `group_identifier`, effective `group_identifier` or saved-set `group_identifier`.
	#[inline(always)]
	pub fn send_credentials(&self, credentials: Credentials) -> io::Result<()>
	{
		let credentials: [ucred; 1] =
		[
			unsafe { transmute(credentials) }
		];

		self.send_ancillary_data(SOL_SOCKET, SCM_CREDENTIALS, &credentials)
	}

	/// Send ancillary data over this socket.
	///
	/// `level`: A `SOL_*` constant such as `SOL_SOCKET`.
	/// `type`: A `SCM_*` constant such as `SCM_RIGHTS`.
	/// `array`: array of ancillary data to send.
	pub(crate) fn send_ancillary_data<T: Sized>(&self, level: c_int, type_: c_int, array: &[T]) -> io::Result<()>
	{
		let mut ancillary_data_buffer: Vec<u8> = Vec::with_capacity(cmsghdr::CMSG_SPACE(size_of::<T>() * array.len()));

		let mut message = msghdr::new(null_mut(), 0, null_mut(), 0, ancillary_data_buffer.as_mut_ptr() as *mut _, ancillary_data_buffer.len() as u32, 0);

		message.initialize_sole_header(level, type_, array);

		let result = unsafe { sendmsg(self.0, &message, SendFlags::NoSigPipeSignal.bits) };

		if likely!(result > 0)
		{
			Ok(())
		}
		else
		{
			use crate::ErrorKind::*;

			Err
			(
				io::Error::from
				(
					if likely!(result == 0)
					{
						WriteZero
					}
					else if likely!(result == -1)
					{
						match errno().0
						{
							EAGAIN => WouldBlock,
							EINTR => Interrupted,
							ENOMEM | ENOBUFS => Other,
							EPIPE => BrokenPipe,
							EACCES => PermissionDenied,
							ECONNRESET => ConnectionReset,
							EBADF => panic!("The argument `sockfd` is an invalid descriptor"),
							EFAULT => panic!("The receive buffer pointer(s) point outside the process's address space"),
							EINVAL => panic!("Invalid argument passed"),
							ENOTCONN => panic!("The socket is associated with a connection-oriented protocol and has not been connected"),
							ENOTSOCK => panic!("The argument `sockfd` does not refer to a socket"),
							EOPNOTSUPP => panic!("Some flags in the `flags` argument are inappropriate for the socket type"),
							EMSGSIZE => panic!("The socket type requires that message be sent atomically, and the size of the message to be sent made this impossible"),
							EISCONN => panic!("The connection-mode socket was connected already but a recipient was specified"),
							EDESTADDRREQ => panic!("The socket is not connection-mode, and no peer address is set"),
							_ => unreachable_code(format_args!("")),
						}
					}
					else
					{
						unreachable_code(format_args!(""))
					}
				)
			)
		}
	}

	/// Creates a new streaming Unix Domain server listener socket.
	///
	/// This is local socket akin to a Transmission Control Protocol (TCP) socket.
	#[inline(always)]
	pub fn new_streaming_unix_domain_socket_server_listener(unix_socket_address: &UnixSocketAddress<impl AsRef<Path>>, send_buffer_size_socket_option: SendBufferSizeSocketOption, receive_low_water_mark_in_bytes: ReceiveLowWaterMarkInBytes, back_log: BackLog, blocking: &Blocking, hyper_thread: HyperThread) -> Result<StreamingServerListenerSocketFileDescriptor<sockaddr_un>, NewSocketServerListenerError>
	{
		let this = SocketFileDescriptor::<sockaddr_un>::new_streaming_unix_domain_socket(send_buffer_size_socket_option, receive_low_water_mark_in_bytes, blocking)?;
		this.bind_unix_domain_socket(unix_socket_address)?;
		Ok(this.listen(back_log, hyper_thread)?)
	}

	/// Creates a new streaming Unix Domain client socket.
	///
	/// This is local socket akin to a Transmission Control Protocol (TCP) socket.
	#[inline(always)]
	pub fn new_streaming_unix_domain_socket_client(unix_socket_address: &UnixSocketAddress<impl AsRef<Path>>, send_buffer_size_socket_option: SendBufferSizeSocketOption, receive_low_water_mark_in_bytes: ReceiveLowWaterMarkInBytes, blocking: &Blocking) -> Result<StreamingSocketFileDescriptor<sockaddr_un>, NewSocketClientError>
	{
		let this = SocketFileDescriptor::<sockaddr_un>::new_streaming_unix_domain_socket(send_buffer_size_socket_option, receive_low_water_mark_in_bytes, blocking)?;
		this.connect_unix_domain_socket(unix_socket_address)?;
		Ok(StreamingSocketFileDescriptor(this))
	}

	/// Creates a new datagram Unix Domain server listener socket.
	///
	/// This is local socket akin to an User Datagram Protocol (UDP) socket.
	#[inline(always)]
	pub fn new_datagram_unix_domain_socket_server_listener(unix_socket_address: &UnixSocketAddress<impl AsRef<Path>>, send_buffer_size_socket_option: SendBufferSizeSocketOption, blocking: &Blocking, hyper_thread: HyperThread) -> Result<DatagramServerListenerSocketFileDescriptor<sockaddr_un>, NewSocketServerListenerError>
	{
		let this = SocketFileDescriptor::<sockaddr_un>::new_datagram_unix_domain_socket(send_buffer_size_socket_option, blocking)?;
		this.set_incoming_cpu(hyper_thread);
		this.bind_unix_domain_socket(unix_socket_address)?;
		Ok(DatagramServerListenerSocketFileDescriptor(this))
	}

	/// Creates a new datagram Unix Domain client socket.
	///
	/// This is local socket akin to an User Datagram Protocol (UDP) socket.
	#[inline(always)]
	pub fn new_datagram_unix_domain_socket_client(unix_socket_address: &UnixSocketAddress<impl AsRef<Path>>, send_buffer_size_socket_option: SendBufferSizeSocketOption, blocking: &Blocking) -> Result<DatagramClientSocketFileDescriptor<sockaddr_un>, NewSocketClientError>
	{
		let this = SocketFileDescriptor::<sockaddr_un>::new_datagram_unix_domain_socket(send_buffer_size_socket_option, blocking)?;
		this.connect_unix_domain_socket(unix_socket_address)?;
		Ok(DatagramClientSocketFileDescriptor(this))
	}
	
	/// This is local socket akin to an User Datagram Protocol (UDP) socket.
	#[inline(always)]
	pub fn new_datagram_unix_domain_socket_client_listener(unix_socket_address: &UnixSocketAddress<impl AsRef<Path>>, send_buffer_size_socket_option: SendBufferSizeSocketOption, blocking: &Blocking, hyper_thread: HyperThread) -> Result<DatagramClientListenerSocketFileDescriptor<sockaddr_un>, NewSocketClientListenerError>
	{
		let this = SocketFileDescriptor::<sockaddr_un>::new_datagram_unix_domain_socket(send_buffer_size_socket_option, blocking)?;
		this.set_incoming_cpu(hyper_thread);
		this.bind_unix_domain_socket(unix_socket_address)?;
		this.connect_unix_domain_socket(unix_socket_address)?;
		Ok(DatagramClientListenerSocketFileDescriptor(this))
	}

	/// Creates a new streaming Unix Domain client socket pair.
	///
	/// This is a pair of local sockets akin to Transmission Control Protocol (TCP) sockets.
	#[inline(always)]
	pub fn new_streaming_unix_domain_socket_pair(lefthand_send_buffer_size_socket_option: SendBufferSizeSocketOption, righthand_send_buffer_size_socket_option: SendBufferSizeSocketOption, blocking: &Blocking) -> Result<(StreamingSocketFileDescriptor<sockaddr_un>, StreamingSocketFileDescriptor<sockaddr_un>), NewSocketClientError>
	{
		let (lefthand, righthand) = Self::socketpair(SOCK_STREAM, lefthand_send_buffer_size_socket_option, righthand_send_buffer_size_socket_option, blocking)?;

		Ok((StreamingSocketFileDescriptor(lefthand), StreamingSocketFileDescriptor(righthand)))
	}

	/// Creates a new datagram Unix Domain client socket pair.
	///
	/// This is a pair of local sockets akin to User Datagram Protocol (UDP) sockets.
	#[inline(always)]
	pub fn new_datagram_unix_domain_socket_pair(lefthand_send_buffer_size_socket_option: SendBufferSizeSocketOption, righthand_send_buffer_size_socket_option: SendBufferSizeSocketOption, blocking: &Blocking) -> Result<(DatagramClientSocketFileDescriptor<sockaddr_un>, DatagramClientSocketFileDescriptor<sockaddr_un>), NewSocketClientError>
	{
		let (lefthand, righthand) = Self::socketpair(SOCK_DGRAM, lefthand_send_buffer_size_socket_option, righthand_send_buffer_size_socket_option, blocking)?;

		Ok((DatagramClientSocketFileDescriptor(lefthand), DatagramClientSocketFileDescriptor(righthand)))
	}
	
	/// Reconnects an Unix domain client socket.
	#[inline(always)]
	pub fn reconnect_unix_domain_client_socket(&self, unix_socket_address: &UnixSocketAddress<impl AsRef<Path>>) -> Result<(), SocketConnectError>
	{
		self.connect_unix_domain_socket(unix_socket_address)
	}

	#[inline(always)]
	fn connect_unix_domain_socket(&self, unix_socket_address: &UnixSocketAddress<impl AsRef<Path>>) -> Result<(), SocketConnectError>
	{
		use self::UnixSocketAddress::*;

		let (local_address, length) = match unix_socket_address
		{
			&File { ref socket_file_path, .. } => Self::unix_domain_socket_data_from_socket_file_path(socket_file_path),

			&Abstract { ref abstract_name } => Self::unix_domain_socket_data_from_abstract_name(&abstract_name[..]),
		};

		self.connect(&local_address, length, false)
	}

	/// `parent_folder_mode` is a octal mode, eg 0o0755.
	#[inline(always)]
	fn bind_unix_domain_socket(&self, unix_socket_address: &UnixSocketAddress<impl AsRef<Path>>) -> Result<(), SocketBindError>
	{
		fn ensure_parent_folder_exists_with_correct_permissions(socket_file_path: &Path, parent_folder_mode: AccessPermissions) -> Result<(), SocketBindError>
		{
			use self::SocketBindError::FilePathInvalid;
			use self::FilePathInvalidReason::*;

			// NOTE: canonicalize(), metadata(), set_permissions() and directory creation is not done atomically.
			let canonical_path = socket_file_path.canonicalize().map_err(|io_error| FilePathInvalid(CanonicalizationOfPathFailed(io_error)))?;

			let parent_folder_path = canonical_path.parent().ok_or(FilePathInvalid(DoesNotHaveAParentFolder))?;

			match parent_folder_path.metadata()
			{
				Ok(metadata) =>
				{
					if !metadata.is_dir()
					{
						return Err(FilePathInvalid(ParentExistsAndIsNotAFolder))
					}
					let mut permissions = metadata.permissions();
					permissions.set_mode(parent_folder_mode.into());
					set_permissions(&parent_folder_path, permissions).map_err(|io_error| FilePathInvalid(SetParentFolderPermissions(io_error)))
				}

				Err(_) => DirBuilder::new().recursive(true).mode(parent_folder_mode.into()).create(&parent_folder_path).map_err(|io_error| FilePathInvalid(ParentFolderRecursiveCreationFailed(io_error))),
			}
		}

		fn remove_if_previously_abandoned_socket_file_path(path: &Path) -> Result<(), SocketBindError>
		{
			if let Ok(metadata) = path.metadata()
			{
				let result = if metadata.is_dir()
				{
					remove_dir(path)
				}
				else
				{
					remove_file(path)
				};
				result.map_err(|io_error| SocketBindError::FilePathInvalid(FilePathInvalidReason::CouldNotRemovePreviousSocketFilePath(io_error)))
			}
			else
			{
				Ok(())
			}
		}

		use self::UnixSocketAddress::*;

		let (local_address, length) = match unix_socket_address
		{
			&File { ref socket_file_path, parent_folder_mode } =>
			{
				ensure_parent_folder_exists_with_correct_permissions(socket_file_path.as_ref(), parent_folder_mode)?;
				remove_if_previously_abandoned_socket_file_path(socket_file_path.as_ref())?;
				Self::unix_domain_socket_data_from_socket_file_path(socket_file_path)
			}

			&Abstract { ref abstract_name } => Self::unix_domain_socket_data_from_abstract_name(&abstract_name[..]),
		};

		bind_socket_with_length(self, &local_address, length)
	}

	#[inline(always)]
	fn unix_domain_socket_data_from_socket_file_path(socket_file_path: impl AsRef<Path>) -> (sockaddr_un, usize)
	{
		let mut socket_data = sockaddr_un::default();

		let path_bytes = path_bytes_without_trailing_nul(&socket_file_path);
		let path_bytes_length = path_bytes.len();
		debug_assert!(path_bytes_length < sockaddr_un::PathLength, "Path converted to bytes is equal to or more than sockaddr_un::PathLength bytes long");
		unsafe { socket_data.sun_path.as_mut_ptr().copy_from_nonoverlapping(path_bytes.as_ptr() as *const _, path_bytes_length) };

		// length is offsetof(struct sockaddr_un, sun_path) + strlen(sun_path) + 1
		(socket_data, size_of::<sa_family_t>() + path_bytes_length + 1)
	}

	#[inline(always)]
	fn unix_domain_socket_data_from_abstract_name(abstract_name: &[u8]) -> (sockaddr_un, usize)
	{
		let mut socket_data = sockaddr_un::default();

		let path_bytes_length = abstract_name.len();
		debug_assert!(path_bytes_length < sockaddr_un::PathLength, "Path converted to bytes is equal to or more than sockaddr_un::PathLength bytes long");

		unsafe { socket_data.sun_path.as_mut_ptr().copy_from_nonoverlapping(abstract_name.as_ptr().add(1) as *const _, path_bytes_length) };

		// length is offsetof(struct sockaddr_un, sun_path) + strlen(sun_path) + 1
		(socket_data, size_of::<sa_family_t>() + path_bytes_length + 1)
	}
}
