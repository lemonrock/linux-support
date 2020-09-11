// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SpeedsPortConnectorsPausesAndForwardErrorConnectionsSettings
{
	/// Speeds.
	pub speeds: HashSet<ethtool_link_mode_bit_indices_speed>,
	
	/// Port connectors.
	pub port_connectors: HashSet<ethtool_link_mode_bit_indices_ports>,
	
	/// Pauses.
	pub pauses: HashSet<ethtool_link_mode_bit_indices_pause>,
	
	/// Forward Error Corrections (FECs).
	pub forward_error_corrections: HashSet<ethtool_link_mode_bit_indices_forward_error_correction>,
}

impl SpeedsPortConnectorsPausesAndForwardErrorConnectionsSettings
{
	pub(crate) fn from_link_mode_bit_set(link_mode_bit_set: LinkModeBitSet) -> Self
	{
		Self
		{
			speeds: link_mode_bit_set.speeds(),
			
			port_connectors: link_mode_bit_set.port_connectors(),
			
			pauses: link_mode_bit_set.pauses(),
			
			forward_error_corrections: link_mode_bit_set.forward_error_corrections(),
		}
	}
}
