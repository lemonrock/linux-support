// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Controls a network device.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NetworkDeviceInputOutputControl<'a>
{
	network_device_socket_file_descriptor: NetworkDeviceSocketFileDescriptor,
	network_interface_name: Cow<'a, NetworkInterfaceName>,
}

impl<'a> NetworkDeviceInputOutputControl<'a>
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
	
	/// Set transmission queue length.
	#[inline(always)]
	pub fn set_transmission_queue_length(&self, transmission_queue_length: u32) -> Result<Option<()>, NetworkDeviceInputOutputControlError<TransmissionQueueLengthOutRangeError>>
	{
		self.set_ifreq_from_name
		(
			SIOCSIFTXQLEN,
			ifreq_ifru
			{
				ifru_ivalue: transmission_queue_length.try_into().map_err(|_| TransmissionQueueLengthOutRangeError)?,
			},
			|_ifreq| Ok(()),
			|errno| match errno.0
			{
				ERANGE => Err(TransmissionQueueLengthOutRangeError),
				
				unexpected @ _ => unreachable!("Unexpected error {} from ioctl(SIOCGIFINDEX)", unexpected),
			}
		)
	}
	
	/// Set maximum transmission unit (MTU).
	#[inline(always)]
	pub fn set_maximum_transmission_unit(&self, maximum_transmission_unit: MaximumTransmissionUnit) -> Result<Option<()>, NetworkDeviceInputOutputControlError<MaximumTransmissionUnitOutRangeError>>
	{
		self.set_ifreq_from_name
		(
			SIOCSIFMTU,
			ifreq_ifru
			{
				ifru_mtu: maximum_transmission_unit.into(),
			},
			|_ifreq| Ok(()),
			|errno| match errno.0
			{
				ERANGE => Err(MaximumTransmissionUnitOutRangeError),
				
				unexpected @ _ => unreachable!("Unexpected error {} from ioctl(SIOCGIFINDEX)", unexpected),
			}
		)
	}
	
	/// Link settings.
	pub fn link_settings(&self) -> Result<Option<Option<LinkSettings>>, NetworkDeviceInputOutputControlError<UndocumentedError>>
	{
		let (cmd, link_mode_masks_nwords) = match self.ethtool_command
		(
			ethtool_link_settings::to_get_link_mode_masks_nwords(),
			|command| Ok(Some((command.cmd, command.link_mode_masks_nwords))),
			|errno| Err(UndocumentedError(errno)),
			|_command| None,
		)?
		{
			None => return Ok(None),
			Some(None) => return Ok(Some(None)),
			Some(Some(value)) => value,
		};
		
		if cmd != ETHTOOL_GLINKSETTINGS || link_mode_masks_nwords >= 0
		{
			return Ok(Some(None))
		}
		
		let command = match self.ethtool_command
		(
			ethtool_link_settings::to_get(link_mode_masks_nwords),
			|command| Ok(Some(command)),
			|errno| Err(UndocumentedError(errno)),
			|_command| None,
		)?
		{
			None => return Ok(None),
			Some(None) => return Ok(Some(None)),
			Some(Some(command)) => if command.cmd != ETHTOOL_GLINKSETTINGS || command.link_mode_masks_nwords <= 0
			{
				return Ok(Some(None))
			}
			else
			{
				command
			},
		};
		
		use self::PORT::*;
		use self::PortConnector::*;
		Ok
		(
			Some
			(
				Some
				(
					LinkSettings
					{
						speed: command.speed,
						
						duplex: command.duplex,
						
						port_connector: match command.port
						{
							PORT_TP => Some
							(
								TwistedPair
								{
									mdi_x_status: command.eth_tp_mdix,
									mdi_x_control: command.eth_tp_mdix_ctrl,
								}
							),
							
							PORT_AUI => Some(AttachmentUnitInterface),
							
							PORT_BNC => Some(BayonetNeillConcelman),
							
							PORT_MII => Some(MediaIndependentInterface),
							
							PORT_FIBRE => Some(Fibre),
							
							PORT_DA => Some(DirectAttachCopper),
							
							PORT_OTHER => Some(Other),
							
							PORT_NONE => None,
						},
						
						phy_address: command.phy_address,
						
						auto_negotiation: command.autoneg,
						
						mdio_support: command.mdio_support,
						
						we_support: SpeedsPortConnectorsPausesAndForwardErrorConnectionsSettings::from_link_mode_bit_set(command.supported()),
						
						we_advertise: SpeedsPortConnectorsPausesAndForwardErrorConnectionsSettings::from_link_mode_bit_set(command.advertising()),
						
						our_link_partner_advertises: SpeedsPortConnectorsPausesAndForwardErrorConnectionsSettings::from_link_mode_bit_set(command.lp_advertising()),
					}
				)
			)
		)
	}
	
	pub fn link_is_up(&self) -> Result<Option<Option<bool>>, NetworkDeviceInputOutputControlError<UndocumentedError>>
	{
		self.ethtool_command
		(
			ethtool_value
			{
				cmd: ETHTOOL_GLINK,
				data: 0
			},
			|command| Ok(Some(command.data != 0)),
			|errno| Err(UndocumentedError(errno)),
			|_command| None,
		)
	}
	
	pub fn timestamping(&self) -> Result<Option<Option<NetworkDeviceTimestampingInformation>>, NetworkDeviceInputOutputControlError<UndocumentedError>>
	{
		self.ethtool_command
		(
			ethtool_ts_info::get(),
			|command|
			Ok
			(
				Some
				(
					NetworkDeviceTimestampingInformation
					{
						capabilities: command.capabilities(),
						precision_time_protocol_hardware_clock_index: command.precision_time_protocol_hardware_clock_index(),
						hardware_transmit_timestamp_modes: command.hardware_transmit_timestamp_modes(),
						hardware_receive_filter_modes: command.hardware_receive_filter_modes(),
					}
				)
			),
			|errno| Err(UndocumentedError(errno)),
			|_command| None,
		)
	}
	
	pub fn registers(&self, driver_and_device_information: &DriverAndDeviceInformation) -> Result<Option<Option<NetworkDeviceRegisters>>, NetworkDeviceInputOutputControlError<UndocumentedError>>
	{
		let length = match driver_and_device_information.device_registers_blob_size_in_bytes
		{
			None => return Ok(Some(None)),
			Some(non_zero_length) => non_zero_length.get()
		};
		
		self.ethtool_command
		(
			ethtool_regs::get_registers(length),
			|command|
			{
				let version = command.version;
				let binary_data = command.array_elements().to_vec().into_boxed_slice();
				
				Ok(Some(NetworkDeviceRegisters { version, binary_data }))
			},
			|errno| Err(UndocumentedError(errno)),
			|_command| None,
		)
	}
	
	pub fn expansion_eeprom(&self, driver_and_device_information: &DriverAndDeviceInformation) -> Result<Option<Option<ExpansionEepromBinaryData>>, NetworkDeviceInputOutputControlError<UndocumentedError>>
	{
		let length = match driver_and_device_information.device_eeprom_blob_size_in_bytes
		{
			None => return Ok(Some(None)),
			Some(non_zero_length) => non_zero_length.get()
		};
		
		self.ethtool_command
		(
			ethtool_eeprom::get_eeprom(length),
			|command|
			{
				let device_version = driver_and_device_information.device_expansion_eeprom_version.clone();
				let magic_cookie = command.magic;
				let binary_data = command.array_elements().to_vec().into_boxed_slice();
				
				Ok(Some(ExpansionEepromBinaryData { device_version, magic_cookie, binary_data }))
			},
			|errno| Err(UndocumentedError(errno)),
			|_command| None,
		)
	}
	
	pub fn plugin_module_eeprom(&self) -> Result<Option<Option<PluginModuleEepromBinaryData>>, NetworkDeviceInputOutputControlError<UndocumentedError>>
	{
		let (type_, length) = match self.ethtool_command
		(
			ethtool_modinfo::get(),
			|command| Ok(Some((command.type_, command.eeprom_len))),
			|errno| Err(UndocumentedError(errno)),
			|_command| None,
		)?
		{
			None => return Ok(None),
			Some(None) => return Ok(Some(None)),
			Some(Some(tuple)) => tuple,
		};
		
		if length == 0
		{
			return Ok(Some(None))
		}
		
		const PageSize: usize = BinaryData256::PageSize;
		const DoublePageSize: usize = PageSize * 2;
		const LargeSize: usize = BinaryData640::LargeSize;
		
		match self.ethtool_command
		(
			ethtool_eeprom::get_module_eeprom(length),
			|command| Ok(Some(command.array_elements().to_vec())),
			|errno| Err(UndocumentedError(errno)),
			|_command| None,
		)?
		{
			None => Ok(None),
			
			Some(None) => Ok(Some(None)),
			
			Some(Some(binary_data)) =>
			{
				use self::PluginModuleEepromBinaryData::*;
				
				let eeprom = match (type_, length as usize)
				{
					(ETH_MODULE_SFF_8079, PageSize) => SFF_8079 { page_A0: binary_data.into() },
					
					(ETH_MODULE_SFF_8472, PageSize) => SFF_8472 { page_A0: binary_data.into(), page_A2: None },
					
					(ETH_MODULE_SFF_8472, DoublePageSize) => SFF_8472 { page_A0: (&binary_data[0 .. PageSize]).into(), page_A2: Some((&binary_data[PageSize .. ]).into()) },
					
					(ETH_MODULE_SFF_8436, PageSize) => SFF_8436 { page_A0: binary_data.into() },
					
					(ETH_MODULE_SFF_8436, LargeSize) => SFF_8436_640 { binary_data: binary_data.into() },
					
					(ETH_MODULE_SFF_8636, PageSize) => SFF_8636 { page_A0: binary_data.into() },
					
					(ETH_MODULE_SFF_8636, LargeSize) => SFF_8636_640 { binary_data: binary_data.into() },
					
					// `ionic`   or  ?
					(0, PageSize)  |  _ => Unknown { type_, binary_data: binary_data.into_boxed_slice() },
				};
				
				Ok(Some(Some(eeprom)))
			}
		}
	}
	
	pub fn firmware(&self, driver_and_device_information: &DriverAndDeviceInformation) -> Result<Option<Option<NetworkDeviceFirmware>>, NetworkDeviceInputOutputControlError<UndocumentedError>>
	{
		let device_firmware_version = driver_and_device_information.device_firmware_version.clone();
		
		let (driver_specific_flags, version, length) = match self.ethtool_command
		(
			ethtool_dump::get_flag(),
			|command| Ok(Some((command.flag, command.version, command.array_length()))),
			|errno| Err(UndocumentedError(errno)),
			|_command| None,
		)?
		{
			None => return Ok(None),
			Some(None) => return Ok(Some(None)),
			Some(Some(tuple)) => tuple,
		};
		
		if length == 0 || driver_specific_flags == ETH_FW_DUMP_DISABLE
		{
			return Ok(Some(Some(NetworkDeviceFirmware { device_version: device_firmware_version, driver_specific_flags, version, binary_data: None })))
		}
		
		match self.ethtool_command
		(
			ethtool_dump::get_data(length),
			|command| Ok(Some(command.array_elements().to_vec().into_boxed_slice())),
			|errno| Err(UndocumentedError(errno)),
			|_command| None,
		)?
		{
			None => Ok(None),
			Some(None) => Ok(Some(Some(NetworkDeviceFirmware { device_version: device_firmware_version, driver_specific_flags, version, binary_data: None }))),
			Some(Some(binary_data)) => Ok(Some(Some(NetworkDeviceFirmware { device_version: device_firmware_version, driver_specific_flags, version, binary_data: Some(binary_data) }))),
		}
	}
	
	pub fn nic_statistics(&self, all_string_sets: &AllStringSets)-> Result<Option<Option<HashMap<ObjectName32, u64>>>, NetworkDeviceInputOutputControlError<UndocumentedError>>
	{
		self.raw_statistics(all_string_sets, ethtool_stringset::ETH_SS_STATS, ETHTOOL_GSTATS)
	}
	
	pub fn phy_statistics(&self, all_string_sets: &AllStringSets) -> Result<Option<Option<HashMap<ObjectName32, u64>>>, NetworkDeviceInputOutputControlError<UndocumentedError>>
	{
		self.raw_statistics(all_string_sets, ethtool_stringset::ETH_SS_PHY_STATS, ETHTOOL_GPHYSTATS)
	}
	
	#[inline(always)]
	fn raw_statistics(&self, all_string_sets: &AllStringSets, string_set_key: ethtool_stringset, command: u32) -> Result<Option<Option<HashMap<ObjectName32, u64>>>, NetworkDeviceInputOutputControlError<UndocumentedError>>
	{
		let statistics_string_set = match all_string_sets.get(&string_set_key)
		{
			None => return Ok(Some(None)),
			Some(string_set) => string_set,
		};
		
		let number_of_statistics = statistics_string_set.len();
		if number_of_statistics == 0
		{
			return Ok(Some(Some(HashMap::new())))
		}
		
		self.ethtool_command
		(
			ethtool_gstats::get(command, number_of_statistics as u32),
			|command|
			{
				let statistics_array = command.array_elements();
				let mut statistics = HashMap::with_capacity(number_of_statistics);
				for index in 0 .. number_of_statistics
				{
					let statistic_name = match statistics_string_set.get_index(index)
					{
						None => continue,
						Some(statistic_name) => statistic_name,
					};
					
					statistics.insert(statistic_name.clone(), unsafe { *statistics_array.get_unchecked(index) });
				}
				Ok(Some(statistics))
			},
			|errno| Err(UndocumentedError(errno)),
			|_command| None,
		)
	}
	
	/// Gets a tunable.
	#[inline(always)]
	pub fn tunable<T: Tunable>(&self) -> Result<Option<Option<T>>, NetworkDeviceInputOutputControlError<UndocumentedError>>
	{
		self.ethtool_command
		(
			ethtool_tunable::new_get(),
			|tunable| Ok(Some(tunable.data)),
			|errno| Err(UndocumentedError(errno)),
			|_command| None,
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
			|_command| Ok(()),
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
		let command = match self.raw_forward_error_correction()?
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
		
		if wake_on_lan_information.supported.is_empty()
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
	pub fn coalesce_configuration(&self) -> Result<Option<Option<CoalesceConfiguration>>, NetworkDeviceInputOutputControlError<AdaptiveCoalescingError>>
	{
		self.ethtool_command
		(
			ethtool_coalesce::get(),
			|command| command.as_coalesce_configuration().map(Some),
			Self::error_is_unreachable,
			|_command| None,
		)
	}
	
	#[allow(missing_docs)]
	pub fn change_coalesce_configuration(&self, coalesce_configuration: &CoalesceConfiguration) -> Result<Option<()>, NetworkDeviceInputOutputControlError<UndocumentedError>>
	{
		let command = coalesce_configuration.as_ethtool_coalesce();
		
		self.ethtool_command
		(
			command,
			|_command| Ok(()),
			|errno| Err(UndocumentedError(errno)),
			|_command| (),
		)
	}
	
	#[allow(missing_docs)]
	pub fn maximize_receive_ring_queues_and_transmit_ring_queue_depths(&self, maximize: bool) -> Result<Option<PendingQueueDepths>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		let (current, maximima) = match self.receive_ring_queues_and_transmit_ring_queue_depths()?
		{
			None => return Ok(None),
			Some(None)  => return Ok(Some(PendingQueueDepths::Unsupported)),
			Some(Some(current_and_maximima)) => current_and_maximima,
		};
		
		if !maximize
		{
			return Ok(Some(current))
		}
		
		self.ethtool_command
		(
			ethtool_ringparam
			{
				cmd: ETHTOOL_SRINGPARAM,
				rx_max_pending: None,
				rx_mini_max_pending: None,
				rx_jumbo_max_pending: None,
				tx_max_pending: None,
				rx_pending: maximima.receive_pending_queue_depth,
				rx_mini_pending: maximima.receive_mini_pending_queue_depth,
				rx_jumbo_pending: maximima.receive_jumbo_pending_queue_depth,
				tx_pending: maximima.transmit_pending_queue_depth,
			},
			|_command| Ok(maximima),
			Self::error_is_unreachable,
			|_command| current,
		)
	}
	
	/// NOTE: If a RX flow indirection table is configured (`netif_is_rxfh_configured()`) and the number of receive channels (combined + receive only) is reduced then `EINVAL` is returned.
	/// NOTE: If XDP user memory is in use against a channel (a queue to XDP) and the number of channels is reduced (which should not happen) `EINVAL` is returned.
	pub fn maximize_number_of_channels(&self, maximize: bool) -> Result<Option<Channels>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		let (current, maximima) = match self.number_of_channels()?
		{
			None => return Ok(None),
			Some(None)  => return Ok(Some(Channels::Unsupported)),
			Some(Some(current_and_maximima)) => current_and_maximima,
		};
		
		if !maximize
		{
			return Ok(Some(current))
		}
		
		self.ethtool_command
		(
			ethtool_channels
			{
				cmd: ETHTOOL_SCHANNELS,
				max_rx: None,
				max_tx: None,
				max_combined: None,
				max_other: None,
				combined_count: maximima.receive_and_transmit_channels_count,
				rx_count: maximima.receive_only_channels_count,
				tx_count: maximima.transmit_only_channels_count,
				other_count: maximima.other_channels_count,
			},
			|_command| Ok(maximima),
			Self::error_is_unreachable,
			|_command| current,
		)
	}
	
	/// The ring queue count can legitimately be zero.
	///
	/// If unsupported then the value `1` is returned.
	#[inline(always)]
	pub fn receive_ring_queue_count(&self) -> Result<Option<QueueCount>, NetworkDeviceInputOutputControlError<ParseNumberError>>
	{
		self.ethtool_command
		(
			ethtool_rxnfc
			{
				cmd: ETHTOOL_GRXRINGS,
				flow_type: 0,
				data: 0,
				fs: unsafe { zeroed() },
				rule_count_or_rss_context: unsafe { zeroed() },
				rule_locs: Default::default()
			},
			|command| QueueCount::try_from(command.data),
			Self::error_is_unreachable,
			|_command| QueueCount::InclusiveMinimum,
		)
	}
	
	pub fn new_configured_hash_settings_in_new_context(&self, configured_hash_settings: &ConfiguredHashSettings) -> Result<Option<Option<ContextIdentifier>>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		self.ethtool_command
		(
			ethtool_rxfh::set(Some(ContextIdentifier::Create), &configured_hash_settings),
			|command| Ok(command.rss_context),
			Self::error_is_unreachable,
			|_command| None,
		)
	}
	
	pub fn set_configured_hash_settings(&self, context_identifier: Option<ContextIdentifier>, configured_hash_settings: &ConfiguredHashSettings) -> Result<Option<bool>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		match self.ethtool_command
		(
			ethtool_rxfh::set(context_identifier, &configured_hash_settings),
			|_command| Ok(true),
			Self::error_is_unreachable,
			|_command| false,
		)?
		{
			None => Ok(None),
			
			Some(true) => Ok(Some(true)),
			
			Some(false) => match context_identifier
			{
				Some(_) => Ok(Some(false)),
				
				None => self.legacy_fallback_set_hash_indirection_table_to_defaults(configured_hash_settings),
			}
		}
	}
	
	fn legacy_fallback_set_hash_indirection_table_to_defaults(&self, configured_hash_settings: &ConfiguredHashSettings) -> Result<Option<bool>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		let indirection_table = configured_hash_settings.indirection_table.as_ref();
		let indirection_size = indirection_table.map(|vec| vec.len()).unwrap_or(0);
		
		let mut command = ethtool_rxfh_indir::new_with_initialized_header_but_uninitialized_array
		(
			ethtool_rxfh_indir
			{
				cmd: ETHTOOL_SRXFHINDIR,
				size: indirection_size as u32,
				ring_index: Default::default()
			}
		);
		
		if let Some(indirection_table) = indirection_table
		{
			unsafe { command.array_elements_mut().as_mut_ptr().copy_from_nonoverlapping(indirection_table.as_ptr(), indirection_size) };
		}
		
		self.ethtool_command
		(
			command,
			|_command| Ok(true),
			Self::error_is_unreachable,
			|_command| false,
		)
	}
	
	/// Returns `Some(true)` if managed to reset, `Some(false)` otherwise.
	/// Returns `None` if no network device.
	pub fn reset_configured_hash_settings(&self, context_identifier: Option<ContextIdentifier>) -> Result<Option<bool>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		match self.ethtool_command
		(
			ethtool_rxfh::reset(context_identifier),
			|_command| Ok(true),
			Self::error_is_unreachable,
			|_command| false,
		)?
		{
			None => Ok(None),
			
			Some(true) => Ok(Some(true)),
			
			Some(false) => match context_identifier
			{
				Some(_) => Ok(Some(false)),
				
				None => self.legacy_fallback_reset_hash_indirection_table_to_defaults(),
			}
		}
	}
	
	/// This may not work for all devices.
	///
	/// Returns whether it was supported.
	fn legacy_fallback_reset_hash_indirection_table_to_defaults(&self) -> Result<Option<bool>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		self.ethtool_command
		(
			ethtool_rxfh_indir
			{
				cmd: ETHTOOL_SRXFHINDIR,
				size: 0,
				ring_index: Default::default()
			},
			|_command| Ok(true),
			Self::error_is_unreachable,
			|_command| false,
		)
	}
	
	/// Gets configured hash settings.
	pub fn configured_receive_side_scaling_hash_settings(&self, context_identifier: Option<ContextIdentifier>) -> Result<Option<ConfiguredHashSettings>, NetworkDeviceInputOutputControlError<UnsupportedHashFunctionError>>
	{
		match self.modern_get_indirection_size_and_key_size(context_identifier)?
		{
			None => return Ok(None),
			
			Some(Some(sizes)) => self.modern_get_configured_hash_settings(context_identifier, sizes),
			
			Some(None) => match context_identifier
			{
				Some(_) => Ok(Some(ConfiguredHashSettings::Unsupported)),
				
				None => match self.legacy_fallback_get_indirection_size()?
				{
					None => Ok(None),
					
					Some(None) => Ok(Some(ConfiguredHashSettings::Unsupported)),
					
					Some(Some(indirection_size)) => match self.legacy_fallback_get_indirection_table(indirection_size)?
					{
						None => Ok(None),
						
						Some(indirection_table) => Ok(Some(ConfiguredHashSettings { function: None, indirection_table, key: None, }))
					},
				}
			}
		}
	}
	
	/// May not be supported, even for the default `context_identifier` (`None`).
	///
	/// In this case, one may fall back to `ETHTOOL_GRXFHINDIR` to get the hash indirection table but not the hash key for the default context (but not other contexts).
	#[inline(always)]
	fn modern_get_indirection_size_and_key_size(&self, context_identifier: Option<ContextIdentifier>) -> Result<Option<Option<(usize, usize)>>, NetworkDeviceInputOutputControlError<UnsupportedHashFunctionError>>
	{
		self.ethtool_command
		(
			ethtool_rxfh::get_indirection_table_size_and_key_size(context_identifier),
			|command| Ok(Some((command.indir_size as usize, command.key_size as usize))),
			Self::error_is_unreachable,
			|_command| None,
		)
	}
	
	#[inline(always)]
	fn modern_get_configured_hash_settings(&self, context_identifier: Option<ContextIdentifier>, (indirection_size, key_size): (usize, usize)) -> Result<Option<ConfiguredHashSettings>, NetworkDeviceInputOutputControlError<UnsupportedHashFunctionError>>
	{
		self.ethtool_command
		(
			ethtool_rxfh::get_indirection_table_and_key(context_identifier, indirection_size, key_size),
			|command| command.configured_hash_settings(),
			Self::error_is_unreachable,
			|_command| ConfiguredHashSettings::Unsupported,
		)
	}
	
	fn legacy_fallback_get_indirection_size(&self) -> Result<Option<Option<usize>>, NetworkDeviceInputOutputControlError<UnsupportedHashFunctionError>>
	{
		self.ethtool_command
		(
			ethtool_rxfh_indir
			{
				cmd: ETHTOOL_GRXFHINDIR,
				size: 0,
				ring_index: Default::default()
			},
			|command| Ok(Some(command.size as usize)),
			Self::error_is_unreachable,
			|_command| None,
		)
	}
	
	fn legacy_fallback_get_indirection_table(&self, indirection_size: usize) -> Result<Option<Option<IndirectionTable>>, NetworkDeviceInputOutputControlError<UnsupportedHashFunctionError>>
	{
		self.ethtool_command
		(
			ethtool_rxfh_indir::new_with_initialized_header_but_uninitialized_array
			(
				ethtool_rxfh_indir
				{
					cmd: ETHTOOL_GRXFHINDIR,
					size: indirection_size as u32,
					ring_index: Default::default()
				}
			),
			|command| Ok
			(
				if command.size == 0
				{
					None
				}
				else
				{
					Some(IndirectionTable(command.array_elements().to_vec()))
				}
			),
			Self::error_is_unreachable,
			|_command| None,
		)
	}
	
	/// Set driver message level.
	pub fn set_driver_message_level(&self, desired: NETIF_MSG) -> Result<Option<NETIF_MSG>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		let supported = match self.driver_message_level()?
		{
			None => return Ok(None),
			Some(supported) => supported,
		};
		
		if supported == NETIF_MSG::empty()
		{
			return Ok(Some(NETIF_MSG::empty()))
		}
		
		self.ethtool_command
		(
			ethtool_value
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
		let features = match self.raw_features()?
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
			|_command| Ok(()),
			Self::error_is_unreachable,
			|_command| ()
		)
	}
	
	/// Set private flags.
	#[inline(always)]
	pub fn set_private_flags(&self, all_string_sets: &AllStringSets, driver_specific_flags_to_change: &HashMap<ObjectName32, bool>) -> Result<Option<()>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		let mut bit_mask_of_flags_to_set = match self.raw_private_flags()?
		{
			None => return Ok(None),
			Some(bit_mask_of_flags) => bit_mask_of_flags
		};
		
		let private_flags_string_set = all_string_sets.get(&ethtool_stringset::ETH_SS_PRIV_FLAGS).unwrap();
		
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
				cmd: ETHTOOL_SPFLAGS,
				data: bit_mask_of_flags_to_set,
			},
			|_command| Ok(()),
			Self::error_is_unreachable,
			|_command| (),
		)
	}
	
	#[inline(always)]
	pub fn private_flags(&self, all_string_sets: &AllStringSets) -> Result<Option<HashSet<ObjectName32>>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		match self.raw_private_flags()?
		{
			None => Ok(None),
			Some(bit_mask_of_flags) =>
			{
				if bit_mask_of_flags == 0
				{
					return Ok(Some(HashSet::default()))
				}
				
				let private_flags_string_set = all_string_sets.get(&ethtool_stringset::ETH_SS_PRIV_FLAGS).ok_or(NetworkDeviceInputOutputControlError::Other("No string set for ethtool_stringset::ETH_SS_PRIV_FLAGS but private flags are present".to_string()))?;
				
				let length = private_flags_string_set.len();
				let mut private_flags = HashSet::with_capacity(length);
				
				for bit in 0 .. 31
				{
					if bit_mask_of_flags & (1 << bit) != 0
					{
						let private_flag_name = private_flags_string_set.get_index(bit as usize).ok_or(NetworkDeviceInputOutputControlError::Other(format!("No private flag name for private flag zero-based bit {}", bit)))?;
						
						private_flags.insert(private_flag_name.clone());
					}
				}
				
				Ok(Some(private_flags))
			}
		}
	}
	
	#[inline(always)]
	fn raw_private_flags(&self) -> Result<Option<u32>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		self.ethtool_command
		(
			ethtool_value
			{
				cmd: ETHTOOL_GPFLAGS,
				data: 0,
			},
			|command| Ok(command.data),
			Self::error_is_unreachable,
			|_command| 0
		)
	}
	
	/// Get all string string sets.
	pub fn all_string_sets(&self) -> Result<Option<AllStringSets>, NetworkDeviceInputOutputControlError<ObjectNameFromBytesError>>
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
				strings.insert(string);
			}
			string_sets.insert(supported_string_set, StringSet(strings));
		}
		
		Ok(Some(AllStringSets(string_sets)))
	}
	
	/// Forward error correction (FEC) information.
	#[inline(always)]
	pub fn forward_error_correction(&self) -> Result<Option<Option<HashSet<ForwardErrorCorrectionCode>>>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		match self.raw_forward_error_correction()?
		{
			None => Ok(None),
			
			Some(None) => Ok(Some(None)),
			
			Some(Some(raw_forward_error_correction)) => Ok(Some(Some(raw_forward_error_correction.to_forward_error_correction_codes()))),
		}
	}
	
	#[inline(always)]
	fn raw_forward_error_correction(&self) -> Result<Option<Option<ethtool_fecparam>>, NetworkDeviceInputOutputControlError<Infallible>>
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
	pub fn driver_and_device_information(&self) -> Result<Option<DriverAndDeviceInformation>, NetworkDeviceInputOutputControlError<ObjectNameFromBytesError>>
	{
		let mut command = ethtool_drvinfo::default();
		command.cmd = ETHTOOL_GDRVINFO;
		
		self.ethtool_command
		(
			command,
			|command| command.as_driver_and_device_information(),
			Self::error_is_unreachable,
			|_command| panic!("Driver information should always be available")
		)
	}
	
	/// Get features.
	pub fn features(&self) -> Result<Option<DeviceFeatures>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		Ok
		(
			self.raw_features()?.map(|raw_feature| DeviceFeatures
			{
				available: raw_feature.available_features(),
				requested: raw_feature.requested_features(),
				active: raw_feature.active_features(),
				never_changed: raw_feature.never_changed_features()
			})
		)
	}
	
	fn raw_features(&self) -> Result<Option<VariablySizedEthtoolCommandWrapper<ethtool_gfeatures>>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		self.ethtool_command
		(
			ethtool_gfeatures::new(),
			|command| Ok(command),
			|errno| match errno.0
			{
				unexpected @ _ => unreachable!("Unexpected error {} from ioctl(SIOCETHTOOL)", unexpected),
			},
			|mut command|
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
	pub fn driver_message_level(&self) -> Result<Option<NETIF_MSG>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		self.ethtool_command
		(
			ethtool_value
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
			|_command| Ok(()),
			Self::error_is_unreachable,
			|_command| ()
		)
	}
	
	/// Get pause parameters.
	pub fn pause(&self) -> Result<Option<Option<PauseConfiguration>>, NetworkDeviceInputOutputControlError<InvalidCombinationOfPauseSettingsError>>
	{
		self.ethtool_command
		(
			ethtool_pauseparam::get(),
			|command| command.as_pause_configuration().map(Some),
			Self::error_is_unreachable,
			|_command| None,
		)
	}
	
	/// Set Energy Efficient Ethernet (EEE).
	pub fn set_energy_efficient_ethernet(&self, configuration: &EnergyEfficientEthernetConfiguration) -> Result<Option<()>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		let energy_efficient_ethernet_information = match self.energy_efficient_ethernet()?
		{
			None => return Ok(None),
			Some(None) => return Ok(Some(())),
			Some(Some(energy_efficient_ethernet_information)) => energy_efficient_ethernet_information,
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
					if energy_efficient_ethernet_information.is_this_a_speed_we_could_advertise(advertise)
					{
						command.set_we_advertise_this_speed_to_our_link_partner(advertise)
					}
				}
				
				if let Some(transmit_low_power_idle_microseconds) = transmit_low_power_idle_microseconds
				{
					command.tx_lpi_enabled = 1;
					command.tx_lpi_timer = *transmit_low_power_idle_microseconds;
				}
				
				command
			}
		};
		
		self.ethtool_command
		(
			command,
			|_command| Ok(()),
			Self::error_is_unreachable,
			|_command| ()
		)
	}
	
	/// Energy Efficient Ethernet (EEE).
	pub fn energy_efficient_ethernet(&self) -> Result<Option<Option<EnergyEfficientEthernetInformation>>, NetworkDeviceInputOutputControlError<Infallible>>
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
			|command| Ok(Some(command.as_energy_efficient_ethernet_information())),
			Self::error_is_unreachable,
			|_command| None
		)
	}
	
	// Wake-on-LAN.
	#[inline(always)]
	pub fn wake_on_lan(&self) -> Result<Option<Option<WakeOnLanInformation>>, NetworkDeviceInputOutputControlError<Infallible>>
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
			|command| Ok(Some(command.to_wake_on_lan_information())),
			Self::error_is_unreachable,
			|_command| None
		)
	}
	
	/// Queue depths, current and maximima.
	#[inline(always)]
	pub fn receive_ring_queues_and_transmit_ring_queue_depths(&self) -> Result<Option<Option<(PendingQueueDepths, PendingQueueDepths)>>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		self.ethtool_command
		(
			ethtool_ringparam
			{
				cmd: ETHTOOL_GRINGPARAM,
				rx_max_pending: None,
				rx_mini_max_pending: None,
				rx_jumbo_max_pending: None,
				tx_max_pending: None,
				rx_pending: None,
				rx_mini_pending: None,
				rx_jumbo_pending: None,
				tx_pending: None,
			},
			|ring_parameters|
			{
				let current = PendingQueueDepths::new(ring_parameters.rx_pending, ring_parameters.rx_jumbo_pending, ring_parameters.rx_mini_pending, ring_parameters.tx_pending);
				let maximima = PendingQueueDepths::new(ring_parameters.rx_max_pending, ring_parameters.rx_mini_max_pending, ring_parameters.rx_jumbo_max_pending, ring_parameters.tx_max_pending);
				
				Ok(Some((current, maximima)))
			},
			Self::error_is_unreachable,
			|_command| None
		)
	}
	
	/// Number of channels, current and maximima.
	#[inline(always)]
	pub fn number_of_channels(&self) -> Result<Option<Option<(Channels, Channels)>>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		self.ethtool_command
		(
			ethtool_channels
			{
				cmd: ETHTOOL_GCHANNELS,
				max_rx: None,
				max_tx: None,
				max_other: None,
				max_combined: None,
				rx_count: None,
				tx_count: None,
				other_count: None,
				combined_count: None,
			},
			|command|
			{
				let current = Channels::new(command.combined_count, command.rx_count, command.tx_count, command.other_count);
				let maximima = Channels::new(command.max_combined, command.max_rx, command.max_tx, command.max_other);
				Ok(Some((current, maximima)))
			},
			Self::error_is_unreachable,
			|_command| None
		)
	}
	
	#[inline(always)]
	fn get_all_string_set_lengths(&self) -> Result<Option<VariablySizedEthtoolCommandWrapper<ethtool_sset_info>>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		self.ethtool_command
		(
			ethtool_sset_info::new(),
			|command| Ok(command),
			Self::error_is_unreachable,
			|mut command|
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
	fn set_ifreq_from_name<V: Sized, E: error::Error + 'static>(&self, request: i32, ifr_ifru: ifreq_ifru, ok_handler: impl FnOnce(ifreq) -> Result<V, E>, error_handler: impl FnOnce(Errno) -> Result<Option<V>, E>) -> Result<Option<V>, NetworkDeviceInputOutputControlError<E>>
	{
		self.network_device_socket_file_descriptor.set_ifreq_from_name(request, self.network_interface_name(), ifr_ifru, ok_handler, error_handler)
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
	fn error_is_unreachable<V: Sized, E: error::Error + 'static>(errno: Errno) -> Result<Option<V>, E>
	{
		unreachable!("Unexpected error {} from ioctl(SIOCETHTOOL)", errno)
	}
}
