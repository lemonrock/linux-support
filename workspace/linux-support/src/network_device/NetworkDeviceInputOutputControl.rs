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
				unexpected @ _ => unreachable!("Unexpected error {} from ioctl(SIOCGIFINDEX)", unexpected),
			}
		)
	}
	
	/// Set a tunable.
	///
	/// Returns an error if out-of-range.
	#[inline(always)]
	pub fn set_tunable(&self, tunable: impl Tunable) -> Result<Option<()>, NetworkDeviceInputOutputControlError<TunableOutOfRangeError>>
	{
		self.ethtool_command
		(
			ethtool_tunable::new_set(tunable),
			|command| Ok(()),
			|errno| match errno.0
			{
				ERANGE => Err(TunableOutOfRangeError),
				
				_ => Self::error_is_unreachable(errno),
			},
			|_command| (),
		)
	}
	
	/// Try to set forward error correction (FEC).
	#[inline(always)]
	pub fn set_forward_error_correction(&self, forward_error_correction_code: ForwardErrorCorrectionCode) -> Result<Option<()>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		let mut command = match self.get_forward_error_correction()?
		{
			None => return Ok(None),
			Some(None) => return Ok(Some(())),
			Some(Some(command)) => command,
		};
		
		if !command.is_supported_forward_error_correction_code(forward_error_correction_code)
		{
			return Ok(Some(()))
		}
		
		if command.active_fec == forward_error_correction_code
		{
			return Ok(Some(()))
		}
		
		self.ethtool_command
		(
			ethtool_fecparam
			{
				cmd: ETHTOOL_SFECPARAM,
				active_fec: forward_error_correction_code,
				fec: 0,
				reserved: 0
			},
			|_command| Ok(()),
			Self::error_is_unreachable,
			|_command| (),
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
			Self::error_is_unreachable,
			|_command| (),
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
			Self::error_is_unreachable,
			|_command| (),
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
			Self::error_is_unreachable,
			|_command| PendingQueueDepths::new(ring_parameters.rx_pending, ring_parameters.rx_mini_pending, ring_parameters.rx_jumbo_pending, ring_parameters.tx_pending),
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
			Self::error_is_unreachable,
			|_command| Channels::new(channels.combined_count, channels.rx_count, channels.tx_count)
		)
	}
	
	/// Set driver message level.
	pub fn set_driver_message_level(&self, desired: NETIF_MSG) -> Result<Option<NETIF_MSG>, NetworkDeviceInputOutputControlError<Infallible>>
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
			Self::error_is_unreachable,
			|_command| NETIF_MSG::empty()
		)
	}
	
	/// Set features.
	pub fn set_features(&self, features_to_change: impl Iterator<Item=HashMap<NETIF_F, bool>>) -> Result<Option<()>, NetworkDeviceInputOutputControlError<Infallible>>
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
			Self::error_is_unreachable,
			|_command| ()
		)
	}
	
	/// Set private flags.
	#[inline(always)]
	pub fn set_private_flags(&self, all_string_sets: &AllStringSets, driver_specific_flags_to_change: &HashMap<ObjectName32, bool>) -> Result<Option<()>, NetworkDeviceInputOutputControlError<()>>
	{
		let option_private_flags = self.ethtool_command
		(
			ethtool_value
			{
				cmd: ETHTOOL_GPFLAGS,
				data: 0,
			},
			|command| Ok(command.data),
			Self::error_is_unreachable,
			|_command| 0
		)?;
		
		let mut bit_mask_of_flags_to_set = match option_private_flags
		{
			None => return Ok(None),
			Some(ethtool_value { data, .. }) => data
		};
		
		let private_flags_string_set = all_string_sets.get(&ethtool_stringset::ETH_SS_PRIV_FLAGS).unwrap();
		
		let mut bit_mask_of_flags_to_set = data;
		for (index, driver_specific_flag) in private_flags_string_set.iter().enumerate().take_while(|&(index, _driver_specific_flag)| index < 32)
		{
			match driver_specific_flags_to_change.get(driver_specific_flag)
			{
				None => (),
				
				Some(set) =>
				{
					let flag = (1 << index) as u32;
					if *set
					{
						bit_mask_of_flags_to_set |= flag
					}
					else
					{
						bit_mask_of_flags_to_set &= !flag
					}
				}
			}
		}
		
		self.ethtool_command
		(
			ethtool_value
			{
				cmd: ETHTOOL_GPFLAGS,
				data: bit_mask_of_flags_to_set,
			},
			|_command| Ok(()),
			Self::error_is_unreachable,
			|_command| (),
		)
	}
	
	/// Get all string string sets.
	pub fn get_all_string_sets(&self) -> Result<Option<AllStringSets>, NetworkDeviceInputOutputControlError<ObjectNameFromBytesError>>
	{
		let string_set_lengths = match self.get_all_string_set_lengths().map_err(|error| error.map_error())?
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
			let string_set = match self.get_string_set_known_length(supported_string_set, number_of_strings).map_err(|error| error.map_error())?
			{
				None => return Ok(None),
				Some(string_set) => string_set,
			};
			
			let raw_strings = string_set.array_elements();
			let mut strings = IndexSet::with_capacity(raw_strings.len());
			for raw_string in raw_strings
			{
				let string = ObjectName32::try_from(raw_string)?;
				strings.push(string)
			}
			string_sets.insert(supported_string_set, StringSet(strings))
		}
		
		Ok(Some(AllStringSets(string_sets)))
	}
	
	/// Forward error correction (FEC) information.
	#[inline(always)]
	pub fn get_forward_error_correction(&self) -> Result<Option<Option<ethtool_fecparam>>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		self.ethtool_command
		(
			ethtool_fecparam
			{
				cmd: ETHTOOL_GFECPARAM,
				active_fec: ForwardErrorCorrectionCode::ETHTOOL_FEC_NONE,
				fec: 0,
				reserved: 0,
			},
			|command| Ok(Some(command)),
			Self::error_is_unreachable,
			|_command| None
		)
	}
	
	/// Driver information.
	#[inline(always)]
	pub fn driver_info(&self) -> Result<Option<ethtool_drvinfo>, CreationError>
	{
		let mut command = ethtool_drvinfo::default();
		command.cmd = ETHTOOL_GDRVINFO;
		
		let result: Result<Option<ethtool_drvinfo>, NetworkDeviceInputOutputControlError<Infallible>> = self.ethtool_command
		(
			command,
			|command| Ok(command),
			Self::error_is_unreachable,
			|_command| panic!("Driver information should always be available")
		);
		
		use self::NetworkDeviceInputOutputControlError::*;
		
		match result
		{
			Ok(value) => Ok(value),
			
			Err(Creation(creation_error)) => Err(creation_error),
			
			Err(PermissionDenied) => panic!("Driver information should always be accessible"),
			
			Err(ControlOperation(Infallible)) => unreachable!("Control operation (ioctl) failures either panic or return `Ok(None)` - see logic above"),
		}
	}
	
	/// Get features.
	pub fn get_features(&self) -> Result<Option<VariablySizedEthtoolCommandWrapper<ethtool_gfeatures>>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		self.ethtool_command
		(
			ethtool_gfeatures::new(),
			|command| Ok(command),
			|errno| match errno.0
			{
				unexpected @ _ => unreachable!("Unexpected error {} from ioctl(SIOCETHTOOL)", unexpected),
			},
			|command|
			{
				for array_element in command.array_elements_mut()
				{
					array_element.available = 0;
					array_element.requested = 0;
					array_element.active = 0;
					array_element.never_changed = 0;
				}
				command
			}
		)
	}
	
	/// Get driver message level.
	pub fn get_driver_message_level(&self) -> Result<Option<NETIF_MSG>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		self.ethtool_command
		(
			command = ethtool_value
			{
				cmd: ETHTOOL_GMSGLVL,
				data: 0,
			},
			|command| Ok(NETIF_MSG::from_bits_truncate(command.data)),
			Self::error_is_unreachable,
			|_command| NETIF_MSG::empty()
		)
	}
	
	/// Set pause parameters.
	pub fn set_pause(&self, pause_configuration: PauseConfiguration) -> Result<Option<()>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		self.ethtool_command
		(
			ethtool_pauseparam::set(pause_configuration),
			|command| Ok(()),
			Self::error_is_unreachable,
			|_command| ()
		)
	}
	
	/// Set Energy Efficient Ethernet (EEE).
	pub fn set_energy_efficient_ethernet(&self, configuration: &EnergyEfficientEthernetConfiguration) -> Result<Option<()>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		let eee = match self.get_energy_efficient_ethernet()?
		{
			None => return Ok(None),
			Some(None) => return Ok(Some(())),
			Some(Some(eee)) => eee,
		};
		
		use self::EnergyEfficientEthernetConfiguration::*;
		
		let command = match configuration
		{
			&Disable => ethtool_eee
			{
				cmd: ETHTOOL_SEEE,
				supported: 0,
				advertised: 0,
				lp_advertised: 0,
				eee_active: 0,
				eee_enabled: 0,
				tx_lpi_enabled: 0,
				tx_lpi_timer: 0,
				reserved: [0; 2],
			},
			
			&Enable { ref advertise, ref transmit_low_power_idle_microseconds } =>
			{
				let mut command = ethtool_eee
				{
					cmd: ETHTOOL_SEEE,
					supported: 0,
					advertised: 0,
					lp_advertised: 0,
					eee_active: 0,
					eee_enabled: 1,
					tx_lpi_enabled: 0,
					tx_lpi_timer: 0,
					reserved: [0; 2],
				};
				for advertise in advertise.iter()
				{
					let advertise = *advertise;
					if eee.supports(advertise)
					{
						command.set_we_advertise(advertise)
					}
				}
				if let Some(transmit_low_power_idle_microseconds) = transmit_low_power_idle_microseconds
				{
					command.tx_lpi_enabled = 1;
					command.tx_lpi_timer = *transmit_low_power_idle_microseconds;
				}
			}
		};
		
		self.ethtool_command
		(
			command,
			|command| Ok(()),
			Self::error_is_unreachable,
			|_command| ()
		)
	}
	
	/// Energy Efficient Ethernet (EEE).
	pub fn get_energy_efficient_ethernet(&self) -> Result<Option<Option<ethtool_eee>>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		self.ethtool_command
		(
			ethtool_eee
			{
				cmd: ETHTOOL_GEEE,
				supported: 0,
				advertised: 0,
				lp_advertised: 0,
				eee_active: 0,
				eee_enabled: 0,
				tx_lpi_enabled: 0,
				tx_lpi_timer: 0,
				reserved: [0; 2],
			},
			|command| Ok(Some(command)),
			Self::error_is_unreachable,
			|_command| None
		)
	}
	
	// Wake-on-LAN.
	#[inline(always)]
	pub fn wake_on_lan(&self) -> Result<Option<Option<ethtool_wolinfo>>, NetworkDeviceInputOutputControlError<Infallible>>
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
			Self::error_is_unreachable,
			|_command| None
		)
	}
	
	/// Queue depths.
	#[inline(always)]
	pub fn receive_ring_queues_and_transmit_ring_queue_depths(&self) -> Result<Option<Option<ethtool_ringparam>>, NetworkDeviceInputOutputControlError<Infallible>>
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
			Self::error_is_unreachable,
			|_command| None
		)
	}
	
	/// Number of channels.
	#[inline(always)]
	pub fn number_of_channels(&self) -> Result<Option<Option<ethtool_channels>>, NetworkDeviceInputOutputControlError<Infallible>>
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
			Self::error_is_unreachable,
			|_command| None
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
	
	#[inline(always)]
	fn get_all_string_set_lengths(&self) -> Result<Option<VariablySizedEthtoolCommandWrapper<ethtool_sset_info>>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		self.ethtool_command
		(
			ethtool_sset_info::new(),
			|command| Ok(command),
			Self::error_is_unreachable,
			|command|
			{
				command.set_supported_string_sets_to_none();
				command
			}
		)
	}
	
	#[inline(always)]
	fn get_string_set_known_length(&self, string_set: ethtool_stringset, number_of_strings: u32) -> Result<Option<VariablySizedEthtoolCommandWrapper<ethtool_gstrings>>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		if number_of_strings == 0
		{
			return Ok(Some(ethtool_gstrings::new(string_set, 0)))
		}
		
		self.ethtool_command
		(
			ethtool_gstrings::new(string_set, number_of_strings),
			|command| Ok(command),
			Self::error_is_unreachable,
			|command| command
		)
	}
	
	#[inline(always)]
	fn ifreq_from_name<V: Sized, E: error::Error + 'static>(&self, request: i32, ok_handler: impl FnOnce(ifreq) -> Result<V, E>, error_handler: impl FnOnce(Errno) -> Result<Option<V>, E>) -> Result<Option<V>, NetworkDeviceInputOutputControlError<E>>
	{
		self.network_device_socket_file_descriptor.ifreq_from_name(request, self.network_interface_name(), ok_handler, error_handler)
	}
	
	#[inline(always)]
	fn ethtool_command<C: EthtoolCommand, V: Sized, E: error::Error + 'static>(&self, command: C, ok_handler: impl FnOnce(C) -> Result<V, E>, error_handler: impl FnOnce(Errno) -> Result<Option<V>, E>, not_supported: impl FnOnce(C) -> V) -> Result<Option<V>, NetworkDeviceInputOutputControlError<E>>
	{
		self.network_device_socket_file_descriptor.ethtool_command(self.network_interface_name(), command, ok_handler, error_handler, not_supported)
	}
	
	#[inline(always)]
	pub(crate) fn network_interface_name(&self) -> NetworkInterfaceName
	{
		self.network_interface_name.clone().into_owned()
	}
	
	#[inline(always)]
	fn error_is_unreachable(errno: Errno) -> !
	{
		unreachable!("Unexpected error {} from ioctl(SIOCETHTOOL)", errno)
	}
}
