// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Either a receive or a transmit queue.
pub trait SysfsQueue<'a>: Sized
{
	#[doc(hidden)]
	const Prefix: &str;
	
	#[doc(hidden)]
	fn new(network_interface_name: &'a NetworkInterfaceName, queue_identifier: QueueIdentifier) -> Self;
	
	#[doc(hidden)]
	#[inline(always)]
	fn file_path(&self, sys_path: &SysPath, file_name: &str) -> PathBuf
	{
		sys_path.network_interface_class_net_queues_file_path(self.network_interface_name(), Self::Prefix, self.queue_identifier(), file_name)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn validate_file_name(dir_entry: DirEntry) -> Result<QueueIdentifier, ParseNumberError>
	{
		const Hyphen: u8 = b'-';
		const LengthOfHyphen: usize = 1;
		
		let prefix = Self::Prefix.as_bytes();
		
		let prefix_length = prefix.len();
		let minimum_queue_folder_name_length = prefix_length + LengthOfHyphen + 1;
		let file_name = dir_entry.file_name().into_vec();
		if file_name.len() < minimum_queue_folder_name_length
		{
			Err(ParseNumberError::TooShort)
		}
		if !file_name.starts_with(prefix.as_bytes())
		{
			return Err(ParseNumberError::DoesNotStartWithPrefix { prefix })
		}
		if unsafe { * file_name.get_unchecked(prefix_length) } != Hyphen
		{
			return Err(ParseNumberError::InvalidByte { byte: Hyphen })
		}
		QueueIdentifier::parse_decimal_number(&file_name[prefix_length .. ])
	}
	
	/// `NetworkInterfaceName`.
	fn network_interface_name(&self) -> &'a NetworkInterfaceName;
	
	/// `QueueIdentifier`.
	fn queue_identifier(&self) -> QueueIdentifier;
}
