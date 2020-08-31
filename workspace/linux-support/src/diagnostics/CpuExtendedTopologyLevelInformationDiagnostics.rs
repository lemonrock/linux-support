// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CpuExtendedTopologyLevelInformationDiagnostics
{
	pub processors: u16,
	
	pub level_number: u8,
	
	pub level_type: String,
	
	pub x2apic_id: u32,
	
	pub shift_right_for_next_apic_id: u32,
}

impl CpuExtendedTopologyLevelInformationDiagnostics
{
	#[inline(always)]
	fn gather(extended_topology_level: ExtendedTopologyLevel) -> Self
	{
		use self::TopologyType::*;
		
		Self
		{
			processors: extended_topology_level.processors(),
			
			level_number: extended_topology_level.level_number(),
			
			level_type: match extended_topology_level.level_type
			{
				Invalid => "Invalid",
				SMT => "HyperThread",
				Core => "Core",
			}.to_string(),
			
			x2apic_id: extended_topology_level.x2apic_id(),
			
			shift_right_for_next_apic_id: extended_topology_level.shift_right_for_next_apic_id(),
		}
	}
}
