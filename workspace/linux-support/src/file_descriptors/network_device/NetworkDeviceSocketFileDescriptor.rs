// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a socket instance suitable for use with network_device ioctl commands.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub(crate) struct NetworkDeviceSocketFileDescriptor(RawFd);

impl Drop for NetworkDeviceSocketFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.0.close()
	}
}

impl AsRawFd for NetworkDeviceSocketFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl IntoRawFd for NetworkDeviceSocketFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.0
	}
}

impl FromRawFd for NetworkDeviceSocketFileDescriptor
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(fd)
	}
}

impl FileDescriptor for NetworkDeviceSocketFileDescriptor
{
}

impl NetworkDeviceSocketFileDescriptor
{
	#[inline(always)]
	pub fn new() -> Result<Self, CreationError>
	{
		// Use of `AF_INET` and `SOCK_DGRAM` is the same as the code in ethtool, ethtool.c, function `ioctl_init()`.
		new_socket(AF_INET, SOCK_DGRAM, 0, false).map(|file_descriptor| Self(file_descriptor))
	}
	
	/// Derived from implementation of `if_indextoname` in musl libc, but without a malloc and copy.
	pub fn network_interface_index_to_network_interface_name(&self, network_interface_index: NetworkInterfaceIndex) -> Result<Option<NetworkInterfaceName>, NetworkDeviceInputOutputControlError<ObjectNameFromBytesError>>
	{
		self.ifreq
		(
			SIOCGIFNAME,
			ifreq
			{
				ifr_ifrn: Default::default(),
				ifr_ifru: ifreq_ifru
				{
					ifru_ivalue: network_interface_index.into(),
				},
			},
			|ifreq| Ok(NetworkInterfaceName::from(ObjectName16::try_from(unsafe { ifreq.ifr_ifrn.ifrn_name })?)),
			|errno| match errno.0
			{
				ENODEV | ENXIO => Ok(None),
				
				unexpected @ _ => unreachable!("Unexpected error {} from ioctl(SIOCGIFNAME)", unexpected),
			}
		)
	}
	
	/// Derived from implementation of `if_nametoindex()` in musl libc, but with stronger error handling.
	pub fn network_interface_name_to_network_interface_index(&self, network_interface_name: NetworkInterfaceName) -> Result<Option<NetworkInterfaceIndex>, NetworkDeviceInputOutputControlError<ParseNumberError>>
	{
		self.ifreq_from_name
		(
			SIOCGIFINDEX,
			network_interface_name,
			|ifreq| Ok(NetworkInterfaceIndex::try_from(unsafe { ifreq.ifr_ifru.ifru_ivalue })?),
			|errno| match errno.0
			{
				ENODEV | ENXIO => Ok(None),
				
				unexpected @ _ => unreachable!("Unexpected error {} from ioctl(SIOCGIFINDEX)", unexpected),
			}
		)
	}
	
	#[inline(always)]
	pub fn disable_wake_on_lan(&self, network_interface_name: NetworkInterfaceName) -> Result<Option<()>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		let wake_on_lan_information = match self.wake_on_lan(network_interface_name)?
		{
			None => return Ok(None),
			Some(None)  => return Ok(Some(())),
			Some(Some(wake_on_lan_information)) => wake_on_lan_information,
		};
		
		if wake_on_lan_information.supported == WAKE::empty()
		{
			return Ok(Some(()))
		}
		
		self.ethtool_command
		(
			network_interface_name,
			ethtool_wolinfo
			{
				cmd: ETHTOOL_SWOL,
				supported: WAKE::empty(),
				wolopts: WAKE::empty(),
				sopass: unsafe { zeroed() }
			},
			|_command| Ok(()),
			|errno, _command| match errno.0
			{
				ENODEV | ENXIO => Ok(None),
				
				EOPNOTSUPP => Ok(Some(())),
				
				unexpected @ _ => unreachable!("Unexpected error {} from ioctl(SIOCETHTOOL)", unexpected),
			}
		)
	}
	
	#[allow(missing_docs)]
	pub fn change_coalesce_configuration(&self, network_interface_name: NetworkInterfaceName, coalesce_configuration: &CoalesceConfiguration) -> Result<Option<()>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		let command = coalesce_configuration.as_ethtool_coalesce();
		
		self.ethtool_command
		(
			network_interface_name,
			command,
			|command| Ok(()),
			|errno, _command| match errno.0
			{
				ENODEV | ENXIO => Ok(None),
				
				EOPNOTSUPP => Ok(Some(())),
				
				unexpected @ _ => unreachable!("Unexpected error {} from ioctl(SIOCETHTOOL)", unexpected),
			}
		)
	}
	
	#[allow(missing_docs)]
	pub fn maximize_receive_ring_queues_and_transmit_ring_queue_depths(&self, network_interface_name: NetworkInterfaceName, maximize: bool) -> Result<Option<PendingQueueDepths>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		let ring_parameters = match self.receive_ring_queues_and_transmit_ring_queue_depths(network_interface_name)?
		{
			None => return Ok(None),
			Some(None)  => return Ok(Some(PendingQueueDepths::new(None, None, None, None))),
			Some(Some(ring_parameters)) => ring_parameters,
		};
		
		if !maximize
		{
			let receive_pending_queue_depth = ring_parameters.rx_pending;
			let receive_mini_pending_queue_depth = ring_parameters.rx_mini_pending;
			let receive_jumbo_pending_queue_depth = ring_parameters.rx_jumbo_pending;
			let transmit_pending_queue_depth = ring_parameters.tx_pending;
			return Ok(Some(PendingQueueDepths::new(receive_jumbo_pending_queue_depth, receive_pending_queue_depth, receive_mini_pending_queue_depth, transmit_pending_queue_depth)))
		}
		
		let receive_pending_queue_depth = NonZeroU32::new(channels.max_rx);
		let receive_mini_pending_queue_depth = NonZeroU32::new(channels.max_tx);
		let receive_jumbo_pending_queue_depth = NonZeroU32::new(channels.max_combined);
		let transmit_pending_queue_depth = NonZeroU32::new(channels.max_other);
		
		self.ethtool_command
		(
			network_interface_name,
			ethtool_ringparam
			{
				cmd: ETHTOOL_SRINGPARAM,
				rx_max_pending: 0,
				rx_mini_max_pending: 0,
				rx_jumbo_max_pending: 0,
				tx_max_pending: 0,
				rx_pending: receive_pending_queue_depth,
				rx_mini_pending: receive_mini_pending_queue_depth,
				rx_jumbo_pending: receive_jumbo_pending_queue_depth,
				tx_pending: transmit_pending_queue_depth,
			},
			|_command| Ok(PendingQueueDepths::new(receive_jumbo_pending_queue_depth, receive_pending_queue_depth, receive_mini_pending_queue_depth, transmit_pending_queue_depth)),
			|errno, _command| match errno.0
			{
				ENODEV | ENXIO => Ok(None),
				
				EOPNOTSUPP => Ok(Some(PendingQueueDepths::new(ring_parameters.rx_pending, ring_parameters.rx_mini_pending, ring_parameters.rx_jumbo_pending, ring_parameters.tx_pending))),
				
				unexpected @ _ => unreachable!("Unexpected error {} from ioctl(SIOCETHTOOL)", unexpected),
			}
		)
	}
	
	/// NOTE: If a RX flow indirection table is configured (`netif_is_rxfh_configured()`) and the number of receive channels (combined + receive only) is reduced then `EINVAL` is returned.
	/// NOTE: If XDP user memory is in use against a channel (a queue to XDP) and the number of channels is reduced (which should not happen) `EINVAL` is returned.
	pub fn maximize_number_of_channels(&self, network_interface_name: NetworkInterfaceName, maximize: bool) -> Result<Option<Channels>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		const One: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(1) };
		
		let channels = match self.number_of_channels(network_interface_name)?
		{
			None => return Ok(None),
			Some(None)  => return Ok(Some(Channels::new(None, None, None))),
			Some(Some(channels)) => channels,
		};
		
		if !maximize
		{
			let receive_and_transmit_channels_count = channels.combined_count;
			let receive_channels_count = channels.rx_count;
			let transmit_channels_count = channels.tx_count;
			return Ok(Some(Channels::new(receive_and_transmit_channels_count, receive_channels_count, transmit_channels_count)))
		}
		
		let receive_and_transmit_channels_count = NonZeroU32::new(channels.max_combined);
		let receive_channels_count = NonZeroU32::new(channels.max_rx);
		let transmit_channels_count = NonZeroU32::new(channels.max_tx);
		let other_channels_count = NonZeroU32::new(channels.max_other);
		
		self.ethtool_command
		(
			network_interface_name,
			ethtool_channels
			{
				cmd: ETHTOOL_SCHANNELS,
				max_rx: 0,
				max_tx: 0,
				max_combined: 0,
				max_other: 0,
				combined_count: receive_and_transmit_channels_count,
				rx_count: receive_channels_count,
				tx_count: transmit_channels_count,
				other_count: other_channels_count,
			},
			|_command| Ok(Channels::new(receive_and_transmit_channels_count, receive_channels_count, transmit_channels_count)),
			|errno, _command| match errno.0
			{
				ENODEV | ENXIO => Ok(None),
				
				EOPNOTSUPP => Ok(Some(Channels::new(channels.combined_count, channels.rx_count, channels.tx_count))),
				
				unexpected @ _ => unreachable!("Unexpected error {} from ioctl(SIOCETHTOOL)", unexpected),
			}
		)
	}
	
	#[inline(always)]
	pub(crate) fn bus_device_address(&self, network_interface_name: NetworkInterfaceName) -> Result<Option<BusDeviceAddress>, NetworkDeviceInputOutputControlError<ObjectNameFromBytesError>>
	{
		match self.driver_info(network_interface_name)?
		{
			None => Ok(None),
			
			Some(driver_info) => Ok(Some(BusDeviceAddress::from(ObjectName32::try_from(command.bus_info)?))),
		}
	}
	
	#[inline(always)]
	fn driver_info(&self, network_interface_name: NetworkInterfaceName) -> Result<Option<ethtool_drvinfo>, CreationError>
	{
		let mut command = ethtool_drvinfo::default();
		command.cmd = ETHTOOL_GDRVINFO;
		
		let result: Result<Option<ethtool_drvinfo>, NetworkDeviceInputOutputControlError<Infallible>> = self.ethtool_command
		(
			network_interface_name,
			command,
			|command| Ok(command),
			|errno, _command| match errno.0
			{
				ENODEV | ENXIO => Ok(None),
				
				unexpected @ _ => unreachable!("Unexpected error {} from ioctl(SIOCETHTOOL)", unexpected),
			}
		);
		
		use self::NetworkDeviceInputOutputControlError::*;
		
		match result
		{
			Ok(value) => Ok(value),
			
			Err(Creation(creation_error)) => Err(creation_error),
			
			Err(ControlOperation(Infallible)) => unreachable!("Control operation (ioctl) failures either panic or return `Ok(None)` - see logic above"),
		}
	}
	
	#[inline(always)]
	pub fn get_all_string_sets(&self, network_interface_name: NetworkInterfaceName) -> Result<Option<HashMap<ethtool_stringset, ObjectName32>>, NetworkDeviceInputOutputControlError<()>>
	{
		let string_set_lengths = match self.get_all_string_set_lengths(network_interface_name.clone())?
		{
			None => return Ok(None),
			Some(string_set_lengths) => string_set_lengths,
		};
		
		let supported_string_sets = string_set_lengths.supported_string_sets();
		let supported_string_sets_lengths = string_set_lengths.array_elements();
		let mut string_sets = HashMap::with_capacity(supported_string_sets_lengths.len());
		for (index, supported_string_set) in supported_string_sets.enumerate()
		{
			let number_of_strings = unsafe { *supported_string_sets_lengths.get_unchecked(index) };
			let string_set = match self.get_string_set_known_length(network_interface_name.clone(), supported_string_set, number_of_strings)?
			{
				None => return Ok(None),
				Some(string_set) => string_set,
			};
			
			let raw_strings = string_set.array_elements();
			let mut strings = Vec::with_capacity(raw_strings.len());
			for raw_string in raw_strings
			{
				let string = ObjectName32::try_from(raw_string).map_err(|_error| NetworkDeviceInputOutputControlError::ControlOperation(()))?;
				strings.push(string)
			}
			string_sets.insert(supported_string_set, strings)
		}
		
		Ok(Some(string_sets))
	}
	
	#[inline(always)]
	fn get_all_string_set_lengths(&self, network_interface_name: NetworkInterfaceName) -> Result<Option<VariablySizedEthtoolCommandWrapper<ethtool_sset_info>>, NetworkDeviceInputOutputControlError<()>>
	{
		self.ethtool_command
		(
			network_interface_name,
			ethtool_sset_info::new(),
			|command| Ok(command),
			|errno, command| match errno.0
			{
				ENODEV | ENXIO => Ok(None),
				
				EOPNOTSUPP =>
				{
					command.set_supported_string_sets_to_none();
					Ok(Some(command))
				},
				
				ENOMEM => Err(()),
				
				EFAULT => unreachable!("We passed a bad memory address"),
				
				unexpected @ _ => unreachable!("Unexpected error {} from ioctl(SIOCETHTOOL)", unexpected),
			}
		)
	}
	
	#[inline(always)]
	fn get_string_set_known_length(&self, network_interface_name: NetworkInterfaceName, string_set: ethtool_stringset, number_of_strings: u32) -> Result<Option<VariablySizedEthtoolCommandWrapper<ethtool_gstrings>>, NetworkDeviceInputOutputControlError<()>>
	{
		if number_of_strings == 0
		{
			return Ok(Some(ethtool_gstrings::new(string_set, 0)))
		}
		
		self.ethtool_command
		(
			network_interface_name,
			ethtool_gstrings::new(string_set, number_of_strings),
			|command| Ok(command),
			|errno, command| match errno.0
			{
				ENODEV | ENXIO => Ok(None),
				
				EOPNOTSUPP => Ok(Some(command)),
				
				ENOMEM => Err(()),
				
				EFAULT => unreachable!("We passed a bad memory address"),
				
				unexpected @ _ => unreachable!("Unexpected error {} from ioctl(SIOCETHTOOL)", unexpected),
			}
		)
	}
	
	#[inline(always)]
	fn wake_on_lan(&self, network_interface_name: NetworkInterfaceName) -> Result<Option<Option<ethtool_wolinfo>>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		self.ethtool_command
		(
			network_interface_name,
			ethtool_wolinfo
			{
				cmd: ETHTOOL_GWOL,
				supported: WAKE::empty(),
				wolopts: WAKE::empty(),
				sopass: unsafe { zeroed() }
			},
			|command| Ok(Some(command)),
			|errno, _command| match errno.0
			{
				ENODEV | ENXIO => Ok(None),
				
				EOPNOTSUPP => Ok(Some(None)),
				
				unexpected @ _ => unreachable!("Unexpected error {} from ioctl(SIOCETHTOOL)", unexpected),
			}
		)
	}
	
	#[inline(always)]
	fn receive_ring_queues_and_transmit_ring_queue_depths(&self, network_interface_name: NetworkInterfaceName) -> Result<Option<Option<ethtool_ringparam>>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		self.ethtool_command
		(
			network_interface_name,
			ethtool_ringparam
			{
				cmd: ETHTOOL_GRINGPARAM,
				rx_max_pending: 0,
				rx_mini_max_pending: 0,
				rx_jumbo_max_pending: 0,
				tx_max_pending: 0,
				rx_pending: None,
				rx_mini_pending: None,
				rx_jumbo_pending: None,
				tx_pending: None,
			},
			|command| Ok(Some(command)),
			|errno, _command| match errno.0
			{
				ENODEV | ENXIO => Ok(None),
				
				EOPNOTSUPP => Ok(Some(None)),
				
				unexpected @ _ => unreachable!("Unexpected error {} from ioctl(SIOCETHTOOL)", unexpected),
			}
		)
	}
	
	#[inline(always)]
	fn number_of_channels(&self, network_interface_name: NetworkInterfaceName) -> Result<Option<Option<ethtool_channels>>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		self.ethtool_command
		(
			network_interface_name,
			ethtool_channels
			{
				cmd: ETHTOOL_GCHANNELS,
				max_rx: 0,
				max_tx: 0,
				max_other: 0,
				max_combined: 0,
				rx_count: None,
				tx_count: None,
				other_count: None,
				combined_count: None,
			},
			|command| Ok(Some(command)),
			|errno, _command| match errno.0
			{
				ENODEV | ENXIO => Ok(None),
				
				EOPNOTSUPP => Ok(Some(None)),
				
				unexpected @ _ => unreachable!("Unexpected error {} from ioctl(SIOCETHTOOL)", unexpected),
			}
		)
	}
	
	#[inline(always)]
	fn ifreq_from_name<V: Sized, E: error::Error + 'static>(&self, request: i32, network_interface_name: NetworkInterfaceName, ok_handler: impl FnOnce(ifreq) -> Result<V, E>, error_handler: impl FnOnce(Errno) -> Result<Option<V>, E>) -> Result<Option<V>, NetworkDeviceInputOutputControlError<E>>
	{
		self.ifreq
		(
			request,
			ifreq
			{
				ifr_ifrn: ifreq_ifrn
				{
					ifrn_name: network_interface_name.into(),
				},
				ifr_ifru: Default::default(),
			},
			ok_handler,
			error_handler
		)
	}
	
	#[inline(always)]
	fn ifreq<V: Sized, E: error::Error + 'static>(&self, request: i32, mut ifreq: ifreq, ok_handler: impl FnOnce(ifreq) -> Result<V, E>, error_handler: impl FnOnce(Errno) -> Result<Option<V>, E>) -> Result<Option<V>, NetworkDeviceInputOutputControlError<E>>
	{
		use self::NetworkDeviceInputOutputControlError::*;
		
		match self.input_output_control(request, &mut ifreq)
		{
			Ok(()) => Ok(Some(ok_handler(ifreq).map_err(ControlOperation)?)),
			
			Err(errno) => error_handler(errno).map_err(ControlOperation),
			
			Err(unexpected @ _) => unreachable!("Unexpected error {} from ioctl({})", unexpected, request),
		}
	}
	
	#[inline(always)]
	fn ethtool_command<C: EthtoolCommand, V: Sized, E: error::Error + 'static>(&self, network_interface_name: NetworkInterfaceName, mut command: C, ok_handler: impl FnOnce(C) -> Result<V, E>, error_handler: impl FnOnce(Errno, C) -> Result<Option<V>, E>) -> Result<Option<V>, NetworkDeviceInputOutputControlError<E>>
	{
		let mut ifr = ifreq
		{
			ifr_ifrn: ifreq_ifrn
			{
				ifrn_name: network_interface_name.into(),
			},
			ifr_ifru: ifreq_ifru
			{
				ifru_data: &mut command as *mut C as *mut c_void,
			},
		};
		
		use self::NetworkDeviceInputOutputControlError::*;
		
		match self.input_output_control(SIOCETHTOOL, &mut ifr)
		{
			Ok(()) => Ok(Some(ok_handler(command).map_err(ControlOperation)?)),
			
			Err(errno) => error_handler(errno, command).map_err(ControlOperation),
		}
	}
	
	#[inline(always)]
	fn input_output_control(&self, request: i32, ifreq: &mut ifreq) -> Result<(), Errno>
	{
		let result = unsafe { ioctl(self.as_raw_fd(), request, &mut ifreq as *mut _ as *mut c_void) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			Err(errno())
		}
		else
		{
			unreachable!("Unexpected result {} from ioctl()", result)
		}
	}
}
