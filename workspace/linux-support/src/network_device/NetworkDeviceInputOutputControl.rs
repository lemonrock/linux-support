// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Controls a network device.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NetworkDeviceInputOutputControl<'a>
{
	network_device_socket_file_descriptor: NetworkDeviceSocketFileDescriptor,
	network_interface_name: Cow<'a, NetworkInterfaceName>,
}

impl<'a> NetworkDeviceInputOutputControl
{
	/// New instance.
	#[inline(always)]
	pub fn new(network_interface_name: Cow<'a, NetworkInterfaceName>) -> Result<Self, CreationError>
	{
		Ok
		(
			Self
			{
				network_device_socket_file_descriptor: NetworkDeviceSocketFileDescriptor::new()?,
				network_interface_name,
			}
		)
	}
	
	/// Derived from implementation of `if_nametoindex()` in musl libc, but with stronger error handling.
	#[inline(always)]
	pub fn network_interface_name_to_network_interface_index(&self) -> Result<Option<NetworkInterfaceIndex>, NetworkDeviceInputOutputControlError<ParseNumberError>>
	{
		self.ifreq_from_name
		(
			SIOCGIFINDEX,
			|ifreq| Ok(NetworkInterfaceIndex::try_from(unsafe { ifreq.ifr_ifru.ifru_ivalue })?),
			|errno| match errno.0
			{
				ENODEV | ENXIO => Ok(None),
				
				EPERM => panic!("Permission denied"),
				
				unexpected @ _ => unreachable!("Unexpected error {} from ioctl(SIOCGIFINDEX)", unexpected),
			}
		)
	}
	
	/// Set a tunable.
	#[inline(always)]
	pub fn set_tunable(&self, tunable: impl Tunable) -> Result<Option<()>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		self.ethtool_command
		(
			ethtool_tunable::new_set(tunable),
			|command| Ok(()),
			|errno, _command| match errno.0
			{
				ENODEV | ENXIO => Ok(None),
				
				EOPNOTSUPP => Ok(Some(())),
				
				ERANGE => panic!("Out of range permitted for this device"),
				
				EPERM => panic!("Permission denied"),
				
				unexpected @ _ => unreachable!("Unexpected error {} from ioctl(SIOCETHTOOL)", unexpected),
			}
		)
	}
	
	/// Disable Wake-on-LAN.
	#[inline(always)]
	pub fn disable_wake_on_lan(&self) -> Result<Option<()>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		let wake_on_lan_information = match self.wake_on_lan()?
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
				
				EPERM => panic!("Permission denied"),
				
				unexpected @ _ => unreachable!("Unexpected error {} from ioctl(SIOCETHTOOL)", unexpected),
			}
		)
	}
	
	#[allow(missing_docs)]
	pub fn change_coalesce_configuration(&self, coalesce_configuration: &CoalesceConfiguration) -> Result<Option<()>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		let command = coalesce_configuration.as_ethtool_coalesce();
		
		self.ethtool_command
		(
			command,
			|command| Ok(()),
			|errno, _command| match errno.0
			{
				ENODEV | ENXIO => Ok(None),
				
				EOPNOTSUPP => Ok(Some(())),
				
				EPERM => panic!("Permission denied"),
				
				unexpected @ _ => unreachable!("Unexpected error {} from ioctl(SIOCETHTOOL)", unexpected),
			}
		)
	}
	
	#[allow(missing_docs)]
	pub fn maximize_receive_ring_queues_and_transmit_ring_queue_depths(&self, maximize: bool) -> Result<Option<PendingQueueDepths>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		let ring_parameters = match self.receive_ring_queues_and_transmit_ring_queue_depths()?
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
				
				EPERM => panic!("Permission denied"),
				
				unexpected @ _ => unreachable!("Unexpected error {} from ioctl(SIOCETHTOOL)", unexpected),
			}
		)
	}
	
	/// NOTE: If a RX flow indirection table is configured (`netif_is_rxfh_configured()`) and the number of receive channels (combined + receive only) is reduced then `EINVAL` is returned.
	/// NOTE: If XDP user memory is in use against a channel (a queue to XDP) and the number of channels is reduced (which should not happen) `EINVAL` is returned.
	pub fn maximize_number_of_channels(&self, maximize: bool) -> Result<Option<Channels>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		const One: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(1) };
		
		let channels = match self.number_of_channels()?
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
				
				EPERM => panic!("Permission denied"),
				
				unexpected @ _ => unreachable!("Unexpected error {} from ioctl(SIOCETHTOOL)", unexpected),
			}
		)
	}
	
	#[inline(always)]
	pub(crate) fn bus_device_address(&self) -> Result<Option<BusDeviceAddress>, NetworkDeviceInputOutputControlError<ObjectNameFromBytesError>>
	{
		match self.driver_info()?
		{
			None => Ok(None),
			
			Some(driver_info) => Ok(Some(BusDeviceAddress::from(ObjectName32::try_from(command.bus_info)?))),
		}
	}
	
	pub(crate) fn set_driver_message_level(&self, desired: NETIF_MSG) -> Result<Option<NETIF_MSG>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		let supported = match self.get_driver_message_level()?
		{
			None => return Ok(None),
			Some(supported) => supported,
		};
		
		if supported == NETIF_MSG::empty()
		{
			Ok(Some(NETIF_MSG::empty()))
		}
		
		self.ethtool_command
		(
			command = ethtool_value
			{
				cmd: ETHTOOL_SMSGLVL,
				data: (supported & desired).bits(),
			},
			|command| Ok(NETIF_MSG::from_bits_truncate(command.data)),
			|errno, command| match errno.0
			{
				ENODEV | ENXIO => Ok(None),
				
				EOPNOTSUPP => Ok(Some(NETIF_MSG::empty())),
				
				EPERM => panic!("Permission denied"),
				
				unexpected @ _ => unreachable!("Unexpected error {} from ioctl(SIOCETHTOOL)", unexpected),
			}
		)
	}
	
	pub(crate) fn set_features(&self, features_to_change: impl Iterator<Item=HashMap<NETIF_F, bool>>) -> Result<Option<()>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		let features = match self.get_features()?
		{
			None => return Ok(None),
			Some(features) => features,
		};
		
		let mut set_features = ethtool_sfeatures::new();
		
		let mut have_changes: bool = false;
		for features_to_change in features_to_change
		{
			let at_least_one_feature_to_change = features.change_features_if_possible(&features_to_change, &mut set_features);
			have_changes |= at_least_one_feature_to_change;
		}
		
		if !have_changes
		{
			return Ok(Some(()))
		}
		
		self.ethtool_command
		(
			set_features,
			|command| Ok(()),
			|errno, _command| match errno.0
			{
				ENODEV | ENXIO => Ok(None),
				
				EOPNOTSUPP => Ok(Some(())),
				
				EPERM => panic!("Permission denied"),
				
				unexpected @ _ => unreachable!("Unexpected error {} from ioctl(SIOCETHTOOL)", unexpected),
			}
		)
	}
	
	#[inline(always)]
	pub(crate) fn get_all_string_sets(&self) -> Result<Option<HashMap<ethtool_stringset, Vec<ObjectName32>>>, NetworkDeviceInputOutputControlError<()>>
	{
		let string_set_lengths = match self.get_all_string_set_lengths()?
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
			let string_set = match self.get_string_set_known_length(supported_string_set, number_of_strings)?
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
	fn get_all_string_set_lengths(&self) -> Result<Option<VariablySizedEthtoolCommandWrapper<ethtool_sset_info>>, NetworkDeviceInputOutputControlError<()>>
	{
		self.ethtool_command
		(
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
				
				EPERM => panic!("Permission denied"),
				
				EFAULT => unreachable!("We passed a bad memory address"),
				
				unexpected @ _ => unreachable!("Unexpected error {} from ioctl(SIOCETHTOOL)", unexpected),
			}
		)
	}
	
	#[inline(always)]
	fn get_string_set_known_length(&self, string_set: ethtool_stringset, number_of_strings: u32) -> Result<Option<VariablySizedEthtoolCommandWrapper<ethtool_gstrings>>, NetworkDeviceInputOutputControlError<()>>
	{
		if number_of_strings == 0
		{
			return Ok(Some(ethtool_gstrings::new(string_set, 0)))
		}
		
		self.ethtool_command
		(
			ethtool_gstrings::new(string_set, number_of_strings),
			|command| Ok(command),
			|errno, command| match errno.0
			{
				ENODEV | ENXIO => Ok(None),
				
				EOPNOTSUPP => Ok(Some(command)),
				
				ENOMEM => Err(()),
				
				EPERM => panic!("Permission denied"),
				
				EFAULT => unreachable!("We passed a bad memory address"),
				
				unexpected @ _ => unreachable!("Unexpected error {} from ioctl(SIOCETHTOOL)", unexpected),
			}
		)
	}
	
	#[inline(always)]
	fn driver_info(&self) -> Result<Option<ethtool_drvinfo>, CreationError>
	{
		let mut command = ethtool_drvinfo::default();
		command.cmd = ETHTOOL_GDRVINFO;
		
		let result: Result<Option<ethtool_drvinfo>, NetworkDeviceInputOutputControlError<Infallible>> = self.ethtool_command
		(
			command,
			|command| Ok(command),
			|errno, _command| match errno.0
			{
				ENODEV | ENXIO => Ok(None),
				
				EPERM => panic!("Permission denied"),
				
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
	
	fn get_features(&self) -> Result<Option<VariablySizedEthtoolCommandWrapper<ethtool_gfeatures>>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		self.ethtool_command
		(
			ethtool_gfeatures::new(),
			|command| Ok(command),
			|errno, _command| match errno.0
			{
				ENODEV | ENXIO => Ok(None),
				
				EPERM => panic!("Permission denied"),
				
				EFAULT => panic!("Not enough memory allocated to hold returned features"),
				
				unexpected @ _ => unreachable!("Unexpected error {} from ioctl(SIOCETHTOOL)", unexpected),
			}
		)
	}
	
	fn get_driver_message_level(&self) -> Result<Option<NETIF_MSG>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		self.ethtool_command
		(
			command = ethtool_value
			{
				cmd: ETHTOOL_GMSGLVL,
				data: 0,
			},
			|command| Ok(NETIF_MSG::from_bits_truncate(command.data)),
			|errno, command| match errno.0
			{
				ENODEV | ENXIO => Ok(None),
				
				EOPNOTSUPP => Ok(Some(NETIF_MSG::empty())),
				
				EPERM => panic!("Permission denied"),
				
				unexpected @ _ => unreachable!("Unexpected error {} from ioctl(SIOCETHTOOL)", unexpected),
			}
		)
	}
	
	#[inline(always)]
	fn wake_on_lan(&self) -> Result<Option<Option<ethtool_wolinfo>>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		self.ethtool_command
		(
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
				
				EPERM => panic!("Permission denied"),
				
				unexpected @ _ => unreachable!("Unexpected error {} from ioctl(SIOCETHTOOL)", unexpected),
			}
		)
	}
	
	#[inline(always)]
	fn receive_ring_queues_and_transmit_ring_queue_depths(&self) -> Result<Option<Option<ethtool_ringparam>>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		self.ethtool_command
		(
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
				
				EPERM => panic!("Permission denied"),
				
				unexpected @ _ => unreachable!("Unexpected error {} from ioctl(SIOCETHTOOL)", unexpected),
			}
		)
	}
	
	#[inline(always)]
	fn number_of_channels(&self) -> Result<Option<Option<ethtool_channels>>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		self.ethtool_command
		(
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
				
				EPERM => panic!("Permission denied"),
				
				unexpected @ _ => unreachable!("Unexpected error {} from ioctl(SIOCETHTOOL)", unexpected),
			}
		)
	}
	
	#[inline(always)]
	fn ifreq_from_name<V: Sized, E: error::Error + 'static>(&self, request: i32, ok_handler: impl FnOnce(ifreq) -> Result<V, E>, error_handler: impl FnOnce(Errno) -> Result<Option<V>, E>) -> Result<Option<V>, NetworkDeviceInputOutputControlError<E>>
	{
		self.network_device_socket_file_descriptor.ifreq_from_name(request, self.network_interface_name(), ok_handler, error_handler)
	}
	
	#[inline(always)]
	pub(crate) fn ethtool_command<C: EthtoolCommand, V: Sized, E: error::Error + 'static>(&self, command: C, ok_handler: impl FnOnce(C) -> Result<V, E>, error_handler: impl FnOnce(Errno, C) -> Result<Option<V>, E>) -> Result<Option<V>, NetworkDeviceInputOutputControlError<E>>
	{
		self.network_device_socket_file_descriptor.ethtool_command(self.network_interface_name(), command, ok_handler, error_handler)
	}
	
	#[inline(always)]
	pub(crate) fn network_interface_name(&self) -> NetworkInterfaceName
	{
		self.network_interface_name.clone().into_owned()
	}
}
