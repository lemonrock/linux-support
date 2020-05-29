// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[thread_local] static mut PerThreadLocalSyslogSocket: StaticInitializedOnce<LocalSyslogSocket> = StaticInitializedOnce::uninitialized();

/// This is a blocking socket.
#[derive(Debug)]
pub struct LocalSyslogSocket
{
	socket_file_descriptor: DatagramClientSocketUnixDomainFileDescriptor,
	is_connected: Cell<bool>,
	buffer: Vec<u8>,
	socket_file_path: PathBuf,
	log_messages_have_been_truncated: bool,
}

impl LocalSyslogSocket
{
	#[inline(always)]
	pub(crate) unsafe fn configure_per_thread_local_syslog_socket() -> Result<(), NewSocketClientError>
	{
		PerThreadLocalSyslogSocket.initialize_once(LocalSyslogSocket::new(StaticLoggingConfiguration::instance())?);
		Ok(())
	}
	
	/// Syslog.
	#[inline(always)]
	pub fn syslog(message_template: &impl MessageTemplate, message: &str) -> Result<(), &'static str>
	{
		Self::instance_mut().log(message_template, message)
	}
	
	#[inline(always)]
	fn instance_mut() -> &'static mut Self
	{
		unsafe { PerThreadLocalSyslogSocket.value_mut() }
	}
	
	/// New.
	fn new(configuration: &StaticLoggingConfiguration) -> Result<Self, NewSocketClientError>
	{
		let socket_file_path = configuration.dev_path.file_path("log");
		Ok
		(
			Self
			{
				socket_file_descriptor: Self::open(&socket_file_path)?,
				is_connected: Cell::new(true),
				buffer:
				{
					let mut buffer = Vec::with_capacity(configuration.logging_buffer_size.get());
					unsafe { buffer.set_len(buffer.capacity()) };
					buffer
				},
				socket_file_path,
				log_messages_have_been_truncated: false,
			}
		)
	}
	
	/// Log.
	fn log(&mut self, message_template: &impl MessageTemplate, message: &str) -> Result<(), &'static str>
	{
		if !self.is_connected.get()
		{
			if self.reconnect().is_err()
			{
				return Err("NotConnected and could not reconnect")
			}
		}
		
		let buffer_length = self.write_message_to_buffer(message_template, message);
		
		let mut buffer = &self.buffer[ ..buffer_length];
		loop
		{
			use crate::ErrorKind::*;
			
			match self.socket_file_descriptor.send(buffer, SendFlags::empty())
			{
				Ok(length_sent) => if likely!(buffer_length == length_sent)
				{
					return Ok(())
				}
				else
				{
					debug_assert!(length_sent < buffer_length);
					buffer = &buffer[length_sent .. ];
					continue
				},
				
				Err(error) => match error.kind()
				{
					Interrupted => continue,
					
					Other =>
					{
						self.sleep_for_one_second();
						continue
					},
					
					PermissionDenied =>
					{
						if self.reconnect().is_err()
						{
							return Err("PermissionDenied and could not reconnect")
						}
						if buffer.len() != buffer_length
						{
							return Err("PermissionDenied, reconnected but partial log message send has occurred")
						}
						continue
					},
					
					WouldBlock => unreachable!("socket has been opened blocking"),
					
					_ =>
					{
						if self.reconnect().is_err()
						{
							return Err("Errored and could not reconnect")
						}
						if buffer.len() != buffer_length
						{
							return Err("Errored, reconnected but partial log message send has occurred")
						}
						continue
					}
				}
			}
		}
	}
	
	#[inline(always)]
	fn write_message_to_buffer(&mut self, message_template: &impl MessageTemplate, message: &str) -> usize
	{
		let timestamp = Utc::now();
		
		let written_length =
		{
			let reserve_space_for_terminal_line_feed = self.buffer.len() - 1;
			let (written_length, truncated) = message_template.format(&mut self.buffer[..reserve_space_for_terminal_line_feed], timestamp, message);
			if unlikely!(truncated)
			{
				self.log_messages_have_been_truncated = truncated;
			}
			written_length
		};
		
		self.write_terminal_line_feed_to_buffer(written_length)
	}
	
	#[inline(always)]
	fn write_terminal_line_feed_to_buffer(&mut self, written_length: usize) -> usize
	{
		const LineFeed: u8 = b'\n';
		unsafe { * self.buffer.get_unchecked_mut(written_length) = LineFeed };
		written_length + 1
	}
	
	#[inline(always)]
	fn sleep_for_one_second(&self)
	{
		let OneSecond = Duration::new(1, 0);
		sleep(OneSecond);
	}
	
	#[inline(always)]
	fn reconnect(&self) -> Result<(), ()>
	{
		self.sleep_for_one_second();
		
		match self.socket_file_descriptor.reconnect_unix_domain_client_socket(&Self::dev_log(&self.socket_file_path))
		{
			Ok(()) =>
			{
				self.is_connected.set(true);
				Ok(())
			}
			
			Err(_) =>
			{
				self.is_connected.set(false);
				Err(())
			}
		}
	}
	
	#[inline(always)]
	fn open(socket_file_path: &Path) -> Result<DatagramClientSocketUnixDomainFileDescriptor, NewSocketClientError>
	{
		const send_buffer_size_in_bytes: usize = 4096;
		SocketFileDescriptor::new_datagram_unix_domain_socket_client(&Self::dev_log(socket_file_path), send_buffer_size_in_bytes, false)
	}
	
	#[inline(always)]
	fn dev_log(socket_file_path: &Path) -> UnixSocketAddress<&Path>
	{
		UnixSocketAddress::File
		{
			socket_file_path,
			parent_folder_mode: AccessPermissions::from(0o0666u32)
		}
	}
}
