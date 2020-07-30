// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global network device configuration
#[derive(Default, Debug, Clone, Eq, PartialEq)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GlobalNetworkDeviceConfiguration
{
	/// Network interface name.
	pub network_interface_name: NetworkInterfaceName,

	/// Disable Wake-on-LAN (WoL).
	pub disable_wake_on_lan: bool,
	
	/// Change coalesce configuration.
	pub coalesce_configuration: Option<CoalesceConfiguration>,
	
	/// Maximize the number of channels?
	pub maximize_number_of_channels: bool,

	/// Maximize pending queue depths?
	pub maximize_pending_queue_depths: bool,
}

impl GlobalNetworkDeviceConfiguration
{
	/// Configures.
	#[inline(always)]
	pub fn configure(&self) -> Result<(Channels, PendingQueueDepths), GlobalNetworkDeviceConfigurationError>
	{
		use self::GlobalNetworkDeviceConfigurationError::*;
		
		let network_socket_file_descriptor = NetworkDeviceSocketFileDescriptor::new()?;
		
		if self.disable_wake_on_lan
		{
			network_socket_file_descriptor.disable_wake_on_lan(self.network_interface_name).map_err(CouldNotDisableWakeOnLan)?.ok_or(NetworkDeviceDoesNotExist(self.network_interface_name()))?
		}
		
		if let Some(ref coalesce_configuration) = self.coalesce_configuration
		{
			network_socket_file_descriptor.change_coalesce_configuration(self.network_interface_name, coalesce_configuration).map_err(CouldNotChangeCoalesceConfiguration)?.ok_or(NetworkDeviceDoesNotExist(self.network_interface_name()))?
		}
		
		let channels = network_socket_file_descriptor.maximize_number_of_channels(self.network_interface_name(), self.maximize_number_of_channels).map_err(CouldNotMaximizeChannels)?.ok_or(NetworkDeviceDoesNotExist(self.network_interface_name()))?;
		
		let pending_queue_depths = network_socket_file_descriptor.maximize_receive_ring_queues_and_transmit_ring_queue_depths(network_interface_name(), self.maximize_pending_queue_depths)?.map_err(CouldNotMaximizePendingQueueDepths).ok_or(NetworkDeviceDoesNotExist(self.network_interface_name()))?;
		
		// TODO: get a string set.
		
		// TODO: ETHTOOL_SPFLAGS
		// TODO: parse_rxfhashopts such as RXH_L2DA and dump_rxfhash
		// TODO: dump_per_queue_coalesce
		// TODO: ethtool_sfeatures
		// TODO: do_srxclass
		// TODO: do_srxntuple
		/*
		
		:: rx-flow-hash
		
		struct ethtool_rxnfc nfccmd;
		
		if (ctx->argc == 5) {
			flow_rss = true;
			nfccmd.rss_context = get_u32(ctx->argp[4], 0);
			
		nfccmd.cmd = ETHTOOL_SRXFH;
		nfccmd.flow_type = rx_fhash_set;
		nfccmd.data = rx_fhash_val;
		if (flow_rss)
			nfccmd.flow_type |= FLOW_RSS;

		:: flow-type
			
			/* attempt to add rule via N-tuple specifier */
			err = do_srxntuple(ctx, &rx_rule_fs);
			if (!err)
				return 0;
	
			/* attempt to add rule via network flow classifier */
			err = rxclass_rule_ins(ctx, &rx_rule_fs, rss_context);
			if (err < 0) {
				fprintf(stderr, "Cannot insert"
					" classification rule\n");
				return 1;
		
		:: delete
		rxclass_rule_del(ctx, rx_class_rule_del);
		 */
		 */
		
		Ok((channels, pending_queue_depths))
	}
	
	#[inline(always)]
	fn network_interface_name(&self) -> NetworkInterfaceName
	{
		self.network_interface_name.clone()
	}
}
