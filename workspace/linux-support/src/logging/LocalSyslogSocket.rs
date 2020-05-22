// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This is a blocking socket.
#[derive(Debug)]
pub struct LocalSyslogSocket
{
	socket_file_descriptor: DatagramClientSocketUnixDomainFileDescriptor,
	is_connected: Cell<bool>,
	socket_file_path: PathBuf,
	buffer: Vec<u8>,
}

impl LocalSyslogSocket
{
	/// New.
	pub fn new(dev_path: &DevPath) -> Result<Self, NewSocketClientError>
	{
		let socket_file_path = dev_path.file_path("/dev/log");
		
		Ok
		(
			Self
			{
				socket_file_descriptor: Self::open(&socket_file_path)?,
				is_connected: Cell::new(true),
				socket_file_path,
				buffer:
				{
					const Size: usize = 4096;
					let mut buffer = Vec::with_capacity(Size);
					unsafe { buffer.set_len(Size) };
					buffer
				},
			}
		)
	}
	
	/// Log.
	pub fn log(&mut self, message_template: &impl MessageTemplate, timestamp: DateTime<Utc>, message: &str) -> Result<(), &'static str>
	{
		if !self.is_connected.get()
		{
			if self.reconnect().is_err()
			{
				return Err("NotConnected and could not reconnect")
			}
		}
		
		let length = message_template.format(&mut self.buffer[..], timestamp, message);
		
		self.buffer[length] = b'\n';
		let original_length = length + 1;
		
		let mut buffer = &self.buffer[ .. original_length];
		loop
		{
			use self::ErrorKind::*;
			
			match self.socket_file_descriptor.send(buffer, SendFlags::empty())
			{
				Ok(length_sent) => if likely!(length == length_sent)
				{
					return Ok(())
				}
				else
				{
					debug_assert!(length_sent < original_length);
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
						if buffer.len() != original_length
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
						if buffer.len() != original_length
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
	fn sleep_for_one_second(&self)
	{
		unsafe { sleep(1) };
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
